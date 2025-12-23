//! Git repository operations using libgit2
//!
//! This module provides native git access without shelling out to the git command.

use crate::classifier::FileClassifier;
use crate::analyzer::{RepoStats, CommitInfo};
use chrono::{DateTime, TimeZone, Utc};
use git2::{Commit, DiffOptions, Repository};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
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
    /// Respect .gitignore patterns (skip ignored files)
    pub respect_gitignore: bool,
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

    // Load .gitignore if respect_gitignore is enabled
    let gitignore = if options.respect_gitignore {
        load_gitignore(path)
    } else {
        None
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
        let diff_stats = get_commit_diff_stats(&repo, &commit, &classifier, gitignore.as_ref(), &mut languages, &mut contribution_types, &mut file_extensions)?;

        stats.total_lines_added += diff_stats.insertions;
        stats.total_lines_removed += diff_stats.deletions;
        stats.total_files_changed += diff_stats.files_changed;
        commit_count += 1;

        // Store commit info for time-based grouping (weekly/monthly activity)
        let commit_info = CommitInfo {
            hash: oid.to_string(),
            author: commit.author().name().unwrap_or("Unknown").to_string(),
            email: commit.author().email().unwrap_or("").to_string(),
            date: datetime,
            message: commit.message().unwrap_or("").lines().next().unwrap_or("").to_string(),
            files_changed: diff_stats.files_changed,
            lines_added: diff_stats.insertions,
            lines_removed: diff_stats.deletions,
            file_classifications: Vec::new(),
            contribution_types: diff_stats.contribution_types,
            languages: diff_stats.languages,
        };
        stats.commits.push(commit_info);
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

/// Load .gitignore patterns from a repository (including nested gitignores)
pub fn load_gitignore(repo_path: &Path) -> Option<Gitignore> {
    use walkdir::WalkDir;

    let mut builder = GitignoreBuilder::new(repo_path);

    // Load root .gitignore
    let root_gitignore = repo_path.join(".gitignore");
    if root_gitignore.exists() {
        let _ = builder.add(&root_gitignore);
    }

    // Walk and find all nested .gitignore files
    for entry in WalkDir::new(repo_path)
        .max_depth(10)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with('.') || s == ".gitignore")
                .unwrap_or(false)
        })
    {
        if let Ok(entry) = entry {
            if entry.file_name() == ".gitignore" && entry.file_type().is_file() {
                let _ = builder.add(entry.path());
            }
        }
    }

    // Always add common build artifact patterns (using ** for any path depth)
    // These patterns match anywhere in the path
    let _ = builder.add_line(None, "**/target/**");
    let _ = builder.add_line(None, "**/node_modules/**");
    let _ = builder.add_line(None, "**/build/**");
    let _ = builder.add_line(None, "**/dist/**");
    let _ = builder.add_line(None, "**/.fingerprint/**");
    let _ = builder.add_line(None, "**/incremental/**");
    let _ = builder.add_line(None, "**/__pycache__/**");
    let _ = builder.add_line(None, "**/*.o");
    let _ = builder.add_line(None, "**/*.rlib");
    let _ = builder.add_line(None, "**/*.rmeta");
    let _ = builder.add_line(None, "**/*.d");
    let _ = builder.add_line(None, "**/*.so");
    let _ = builder.add_line(None, "**/*.dylib");
    let _ = builder.add_line(None, "**/*.dll");
    let _ = builder.add_line(None, "**/*.exe");
    let _ = builder.add_line(None, "**/*.timestamp");
    let _ = builder.add_line(None, "**/*.pyc");

    builder.build().ok()
}

/// Stats returned from analyzing a single commit's diff
struct CommitDiffStats {
    insertions: u32,
    deletions: u32,
    files_changed: u32,
    languages: HashMap<String, u32>,
    contribution_types: HashMap<String, u32>,
}

/// Get diff stats for a single commit
fn get_commit_diff_stats(
    repo: &Repository,
    commit: &Commit,
    classifier: &FileClassifier,
    gitignore: Option<&Gitignore>,
    repo_languages: &mut HashMap<String, u32>,
    repo_contribution_types: &mut HashMap<String, u32>,
    repo_file_extensions: &mut HashMap<String, u32>,
) -> Result<CommitDiffStats, GitError> {
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

    // Per-commit tracking
    let mut commit_languages: HashMap<String, u32> = HashMap::new();
    let mut commit_contribution_types: HashMap<String, u32> = HashMap::new();

    // Iterate through file changes for classification
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                let path_str = path.to_string_lossy();

                // Skip files that match .gitignore patterns
                if let Some(gi) = gitignore {
                    if gi.matched(path, false).is_ignore() {
                        return true; // Skip this file
                    }
                }

                // Classify the file
                let classification = classifier.classify(&path_str, insertions, deletions);

                // Track language (both repo-level and per-commit)
                if let Some(ref lang) = classification.language {
                    *repo_languages.entry(lang.clone()).or_insert(0) += insertions + deletions;
                    *commit_languages.entry(lang.clone()).or_insert(0) += insertions + deletions;
                }

                // Track contribution type (both repo-level and per-commit)
                let type_key = format!("{:?}", classification.contribution_type).to_lowercase();
                *repo_contribution_types.entry(type_key.clone()).or_insert(0) += insertions + deletions;
                *commit_contribution_types.entry(type_key).or_insert(0) += insertions + deletions;

                // Track file extension (repo-level only)
                let ext = get_file_extension(&path_str);
                *repo_file_extensions.entry(ext).or_insert(0) += insertions + deletions;
            }
            true
        },
        None,
        None,
        None,
    ).map_err(|e| GitError::DiffError(e.message().to_string()))?;

    Ok(CommitDiffStats {
        insertions,
        deletions,
        files_changed,
        languages: commit_languages,
        contribution_types: commit_contribution_types,
    })
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
#[path = "tests/git_tests.rs"]
mod tests;
