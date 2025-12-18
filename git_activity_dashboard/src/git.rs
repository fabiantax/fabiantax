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

    #[test]
    fn test_find_repos() {
        // This test assumes we're in a git repo
        let cwd = std::env::current_dir().unwrap();
        let repos = find_repos(&cwd, 1);
        assert!(!repos.is_empty() || !is_git_repo(&cwd));
    }
}
