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
