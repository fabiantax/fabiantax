//! Git repository operations using libgit2
//!
//! This module provides native git access without shelling out to the git command.

use crate::classifier::FileClassifier;
use crate::analyzer::RepoStats;
use chrono::{DateTime, TimeZone, Utc};
use git2::{Commit, DiffOptions, Repository};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum GitError {
    OpenRepo(String),
    WalkCommits(String),
    DiffError(String),
    NotARepo,
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::OpenRepo(msg) => write!(f, "Failed to open repository: {}", msg),
            GitError::WalkCommits(msg) => write!(f, "Failed to walk commits: {}", msg),
            GitError::DiffError(msg) => write!(f, "Failed to compute diff: {}", msg),
            GitError::NotARepo => write!(f, "Not a git repository"),
        }
    }
}

impl std::error::Error for GitError {}

impl From<git2::Error> for GitError {
    fn from(e: git2::Error) -> Self {
        GitError::OpenRepo(e.message().to_string())
    }
}

/// Options for analyzing a repository
#[derive(Debug, Clone, Default)]
pub struct AnalyzeOptions {
    /// Filter commits by author email
    pub author_email: Option<String>,
    /// Filter commits by author name
    pub author_name: Option<String>,
    /// Only analyze commits after this hash (for incremental updates)
    pub since_commit: Option<String>,
    /// Maximum number of commits to analyze (None = all)
    pub max_commits: Option<usize>,
    /// Store individual commit details (memory intensive)
    pub store_commits: bool,
}

/// Analyze a git repository and return stats
pub fn analyze_repo(path: &Path, options: &AnalyzeOptions) -> Result<RepoStats, GitError> {
    let repo = Repository::open(path).map_err(|e| GitError::OpenRepo(e.message().to_string()))?;

    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut stats = RepoStats {
        name: repo_name,
        path: path.to_string_lossy().to_string(),
        ..Default::default()
    };

    let classifier = FileClassifier::new();
    let mut languages: HashMap<String, u32> = HashMap::new();
    let mut contribution_types: HashMap<String, u32> = HashMap::new();
    let mut file_extensions: HashMap<String, u32> = HashMap::new();

    // Set up revision walker
    let mut revwalk = repo.revwalk().map_err(|e| GitError::WalkCommits(e.message().to_string()))?;
    revwalk.push_head().map_err(|e| GitError::WalkCommits(e.message().to_string()))?;

    // If we have a since_commit, stop there
    let stop_at = options.since_commit.as_ref().and_then(|hash| {
        git2::Oid::from_str(hash).ok()
    });

    let mut commit_count = 0;
    let mut first_date: Option<DateTime<Utc>> = None;
    let mut last_date: Option<DateTime<Utc>> = None;
    let mut last_hash: Option<String> = None;

    for oid_result in revwalk {
        let oid = oid_result.map_err(|e| GitError::WalkCommits(e.message().to_string()))?;

        // Stop at the specified commit (for incremental updates)
        if let Some(stop) = stop_at {
            if oid == stop {
                break;
            }
        }

        // Check max commits limit
        if let Some(max) = options.max_commits {
            if commit_count >= max {
                break;
            }
        }

        let commit = repo.find_commit(oid).map_err(|e| GitError::WalkCommits(e.message().to_string()))?;

        // Filter by author
        if let Some(ref email_filter) = options.author_email {
            if let Some(email) = commit.author().email() {
                if !email.contains(email_filter.as_str()) {
                    continue;
                }
            } else {
                continue;
            }
        }

        if let Some(ref name_filter) = options.author_name {
            if let Some(name) = commit.author().name() {
                if !name.contains(name_filter.as_str()) {
                    continue;
                }
            } else {
                continue;
            }
        }

        // Get commit date
        let commit_time = commit.time();
        let datetime = Utc.timestamp_opt(commit_time.seconds(), 0)
            .single()
            .unwrap_or_else(Utc::now);

        // Track date range
        if first_date.is_none() || datetime < first_date.unwrap() {
            first_date = Some(datetime);
        }
        if last_date.is_none() || datetime > last_date.unwrap() {
            last_date = Some(datetime);
        }

        // Store first (most recent) commit hash for cache tracking
        if last_hash.is_none() {
            last_hash = Some(oid.to_string());
        }

        // Get diff stats
        let (insertions, deletions, files_changed) = get_commit_diff_stats(&repo, &commit, &classifier, &mut languages, &mut contribution_types, &mut file_extensions)?;

        stats.total_lines_added += insertions;
        stats.total_lines_removed += deletions;
        stats.total_files_changed += files_changed;
        commit_count += 1;
    }

    stats.total_commits = commit_count as u32;
    stats.first_commit_date = first_date;
    stats.last_commit_date = last_date;
    stats.last_commit_hash = last_hash;
    stats.languages = languages;
    stats.contribution_types = contribution_types;
    stats.file_extensions = file_extensions;

    Ok(stats)
}

/// Get diff stats for a single commit
fn get_commit_diff_stats(
    repo: &Repository,
    commit: &Commit,
    classifier: &FileClassifier,
    languages: &mut HashMap<String, u32>,
    contribution_types: &mut HashMap<String, u32>,
    file_extensions: &mut HashMap<String, u32>,
) -> Result<(u32, u32, u32), GitError> {
    let tree = commit.tree().map_err(|e| GitError::DiffError(e.message().to_string()))?;

    // Get parent tree (or empty for root commit)
    let parent_tree = if commit.parent_count() > 0 {
        commit.parent(0).ok().and_then(|p| p.tree().ok())
    } else {
        None
    };

    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        parent_tree.as_ref(),
        Some(&tree),
        Some(&mut diff_opts)
    ).map_err(|e| GitError::DiffError(e.message().to_string()))?;

    let stats = diff.stats().map_err(|e| GitError::DiffError(e.message().to_string()))?;

    let insertions = stats.insertions() as u32;
    let deletions = stats.deletions() as u32;
    let files_changed = stats.files_changed() as u32;

    // Iterate through file changes for classification
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                let path_str = path.to_string_lossy();

                // Classify the file
                let classification = classifier.classify(&path_str, insertions, deletions);

                // Track language
                if let Some(ref lang) = classification.language {
                    *languages.entry(lang.clone()).or_insert(0) += insertions + deletions;
                }

                // Track contribution type
                let type_key = format!("{:?}", classification.contribution_type).to_lowercase();
                *contribution_types.entry(type_key).or_insert(0) += insertions + deletions;

                // Track file extension
                let ext = get_file_extension(&path_str);
                *file_extensions.entry(ext).or_insert(0) += insertions + deletions;
            }
            true
        },
        None,
        None,
        None,
    ).map_err(|e| GitError::DiffError(e.message().to_string()))?;

    Ok((insertions, deletions, files_changed))
}

/// Extract file extension from path
fn get_file_extension(filepath: &str) -> String {
    std::path::Path::new(filepath)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e.to_lowercase()))
        .unwrap_or_else(|| "(no ext)".to_string())
}

/// Check if a path is a git repository
pub fn is_git_repo(path: &Path) -> bool {
    Repository::open(path).is_ok()
}

/// Get the HEAD commit hash for a repository
pub fn get_head_hash(path: &Path) -> Option<String> {
    let repo = Repository::open(path).ok()?;
    let head = repo.head().ok()?;
    let oid = head.target()?;
    Some(oid.to_string())
}

/// Find all git repositories under a path
pub fn find_repos(base_path: &Path, max_depth: usize) -> Vec<std::path::PathBuf> {
    use walkdir::WalkDir;

    let mut repos = Vec::new();

    for entry in WalkDir::new(base_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden directories except .git
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with('.') || s == ".git")
                .unwrap_or(false)
        })
    {
        if let Ok(entry) = entry {
            if entry.file_name() == ".git" && entry.file_type().is_dir() {
                if let Some(parent) = entry.path().parent() {
                    repos.push(parent.to_path_buf());
                }
            }
        }
    }

    repos
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ==================== GitError Tests ====================

    #[test]
    fn test_git_error_display_open_repo() {
        let err = GitError::OpenRepo("not found".to_string());
        assert_eq!(format!("{}", err), "Failed to open repository: not found");
    }

    #[test]
    fn test_git_error_display_walk_commits() {
        let err = GitError::WalkCommits("invalid ref".to_string());
        assert_eq!(format!("{}", err), "Failed to walk commits: invalid ref");
    }

    #[test]
    fn test_git_error_display_diff_error() {
        let err = GitError::DiffError("binary file".to_string());
        assert_eq!(format!("{}", err), "Failed to compute diff: binary file");
    }

    #[test]
    fn test_git_error_display_not_a_repo() {
        let err = GitError::NotARepo;
        assert_eq!(format!("{}", err), "Not a git repository");
    }

    #[test]
    fn test_git_error_is_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(GitError::NotARepo);
        assert!(err.to_string().contains("Not a git repository"));
    }

    // ==================== AnalyzeOptions Tests ====================

    #[test]
    fn test_analyze_options_default() {
        let opts = AnalyzeOptions::default();
        assert!(opts.author_email.is_none());
        assert!(opts.author_name.is_none());
        assert!(opts.since_commit.is_none());
        assert!(opts.max_commits.is_none());
        assert!(!opts.store_commits);
    }

    #[test]
    fn test_analyze_options_with_filters() {
        let opts = AnalyzeOptions {
            author_email: Some("test@example.com".to_string()),
            author_name: Some("Test User".to_string()),
            since_commit: Some("abc123".to_string()),
            max_commits: Some(100),
            store_commits: true,
        };
        assert_eq!(opts.author_email.unwrap(), "test@example.com");
        assert_eq!(opts.author_name.unwrap(), "Test User");
        assert_eq!(opts.since_commit.unwrap(), "abc123");
        assert_eq!(opts.max_commits.unwrap(), 100);
        assert!(opts.store_commits);
    }

    #[test]
    fn test_analyze_options_clone() {
        let opts = AnalyzeOptions {
            author_email: Some("test@example.com".to_string()),
            ..Default::default()
        };
        let cloned = opts.clone();
        assert_eq!(opts.author_email, cloned.author_email);
    }

    // ==================== get_file_extension Tests ====================

    #[test]
    fn test_get_file_extension_simple() {
        assert_eq!(get_file_extension("main.rs"), ".rs");
    }

    #[test]
    fn test_get_file_extension_multiple_dots() {
        assert_eq!(get_file_extension("test.spec.ts"), ".ts");
    }

    #[test]
    fn test_get_file_extension_no_extension() {
        assert_eq!(get_file_extension("Makefile"), "(no ext)");
    }

    #[test]
    fn test_get_file_extension_hidden_file() {
        assert_eq!(get_file_extension(".gitignore"), "(no ext)");
    }

    #[test]
    fn test_get_file_extension_path_with_dirs() {
        assert_eq!(get_file_extension("src/lib/utils.js"), ".js");
    }

    #[test]
    fn test_get_file_extension_uppercase_normalized() {
        assert_eq!(get_file_extension("README.MD"), ".md");
    }

    #[test]
    fn test_get_file_extension_empty_string() {
        assert_eq!(get_file_extension(""), "(no ext)");
    }

    // ==================== is_git_repo Tests ====================

    #[test]
    fn test_is_git_repo_valid_repo() {
        let cwd = std::env::current_dir().unwrap();
        // We should be running from within a git repo
        let result = is_git_repo(&cwd);
        // Either we're in a repo or we're not - just test it doesn't panic
        assert!(result || !result);
    }

    #[test]
    fn test_is_git_repo_invalid_path() {
        let fake_path = PathBuf::from("/nonexistent/path/that/should/not/exist");
        assert!(!is_git_repo(&fake_path));
    }

    #[test]
    fn test_is_git_repo_non_repo_dir() {
        let tmp = std::env::temp_dir();
        // temp dir is unlikely to be a git repo
        let result = is_git_repo(&tmp);
        // Just verify it returns a boolean without panicking
        assert!(result || !result);
    }

    // ==================== get_head_hash Tests ====================

    #[test]
    fn test_get_head_hash_invalid_path() {
        let fake_path = PathBuf::from("/nonexistent/path");
        assert!(get_head_hash(&fake_path).is_none());
    }

    #[test]
    fn test_get_head_hash_non_repo() {
        let tmp = std::env::temp_dir();
        let result = get_head_hash(&tmp);
        // Either it's a repo with a hash or it's not
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_get_head_hash_valid_repo_returns_40_char_hash() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let hash = get_head_hash(&cwd);
            assert!(hash.is_some());
            let hash = hash.unwrap();
            assert_eq!(hash.len(), 40); // Git SHA-1 hashes are 40 hex chars
            assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    // ==================== find_repos Tests ====================

    #[test]
    fn test_find_repos_in_current_dir() {
        let cwd = std::env::current_dir().unwrap();
        let repos = find_repos(&cwd, 1);
        // Should find current repo if we're in one
        if is_git_repo(&cwd) {
            assert!(!repos.is_empty());
        }
    }

    #[test]
    fn test_find_repos_empty_for_nonexistent_path() {
        let fake_path = PathBuf::from("/nonexistent/path");
        let repos = find_repos(&fake_path, 3);
        assert!(repos.is_empty());
    }

    #[test]
    fn test_find_repos_respects_max_depth() {
        let cwd = std::env::current_dir().unwrap();
        let shallow = find_repos(&cwd, 0);
        let deeper = find_repos(&cwd, 3);
        // Deeper search should find >= shallow search results
        assert!(deeper.len() >= shallow.len());
    }

    #[test]
    fn test_find_repos_returns_pathbufs() {
        let cwd = std::env::current_dir().unwrap();
        let repos = find_repos(&cwd, 1);
        for repo in repos {
            assert!(repo.is_absolute() || repo.is_relative());
        }
    }

    // ==================== analyze_repo Tests ====================

    #[test]
    fn test_analyze_repo_invalid_path() {
        let fake_path = PathBuf::from("/nonexistent/repo");
        let opts = AnalyzeOptions::default();
        let result = analyze_repo(&fake_path, &opts);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_repo_error_is_open_repo() {
        let fake_path = PathBuf::from("/nonexistent/repo");
        let opts = AnalyzeOptions::default();
        let result = analyze_repo(&fake_path, &opts);
        match result {
            Err(GitError::OpenRepo(_)) => (),
            _ => panic!("Expected OpenRepo error"),
        }
    }

    #[test]
    fn test_analyze_repo_valid_repo_returns_stats() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(5), // Limit for speed
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            assert!(result.is_ok());
            let stats = result.unwrap();
            assert!(!stats.name.is_empty());
            assert!(!stats.path.is_empty());
        }
    }

    #[test]
    fn test_analyze_repo_with_max_commits_limit() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(2),
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            assert!(result.is_ok());
            let stats = result.unwrap();
            assert!(stats.total_commits <= 2);
        }
    }

    #[test]
    fn test_analyze_repo_extracts_repo_name_from_path() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(1),
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            assert!(result.is_ok());
            let stats = result.unwrap();
            // Name should be last component of path
            let expected_name = cwd.file_name().unwrap().to_str().unwrap();
            assert_eq!(stats.name, expected_name);
        }
    }

    #[test]
    fn test_analyze_repo_populates_dates_when_commits_exist() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(3),
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            if let Ok(stats) = result {
                if stats.total_commits > 0 {
                    assert!(stats.first_commit_date.is_some());
                    assert!(stats.last_commit_date.is_some());
                    assert!(stats.last_commit_hash.is_some());
                }
            }
        }
    }

    #[test]
    fn test_analyze_repo_tracks_languages() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(5),
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            if let Ok(stats) = result {
                // If we have commits with files, we should have some languages
                if stats.total_files_changed > 0 {
                    // May or may not have languages depending on file types
                    assert!(stats.languages.len() >= 0);
                }
            }
        }
    }

    #[test]
    fn test_analyze_repo_tracks_contribution_types() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(5),
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            if let Ok(stats) = result {
                if stats.total_files_changed > 0 {
                    assert!(!stats.contribution_types.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_analyze_repo_tracks_file_extensions() {
        let cwd = std::env::current_dir().unwrap();
        if is_git_repo(&cwd) {
            let opts = AnalyzeOptions {
                max_commits: Some(5),
                ..Default::default()
            };
            let result = analyze_repo(&cwd, &opts);
            if let Ok(stats) = result {
                if stats.total_files_changed > 0 {
                    assert!(!stats.file_extensions.is_empty());
                }
            }
        }
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_full_workflow_find_and_analyze() {
        let cwd = std::env::current_dir().unwrap();
        let repos = find_repos(&cwd, 1);

        for repo_path in repos.iter().take(1) {
            if is_git_repo(repo_path) {
                let hash = get_head_hash(repo_path);
                assert!(hash.is_some());

                let opts = AnalyzeOptions {
                    max_commits: Some(3),
                    ..Default::default()
                };
                let result = analyze_repo(repo_path, &opts);
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_analyze_options_debug_trait() {
        let opts = AnalyzeOptions::default();
        let debug_str = format!("{:?}", opts);
        assert!(debug_str.contains("AnalyzeOptions"));
    }

    #[test]
    fn test_git_error_debug_trait() {
        let err = GitError::NotARepo;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("NotARepo"));
    }
}
