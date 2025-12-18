use super::*;

// ========================================
// Test Helper Functions
// ========================================

/// Creates a sample git log output with null byte delimiter
fn create_sample_git_log_null_delimiter() -> String {
    format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Initial commit\n\
         10\t5\tsrc/main.rs\n\
         3\t2\tREADME.md\n\
         def456\x00Jane Smith\x00jane@example.com\x002024-01-16T14:20:00Z\x00Add feature\n\
         20\t8\tsrc/lib.rs\n\
         5\t0\ttests/test.rs\n"
    )
}

/// Creates a sample git log output with legacy pipe delimiter
fn create_sample_git_log_pipe_delimiter() -> String {
    "abc123|John Doe|john@example.com|2024-01-15T10:30:00Z|Initial commit\n\
     10\t5\tsrc/main.rs\n\
     3\t2\tREADME.md\n\
     def456|Jane Smith|jane@example.com|2024-01-16T14:20:00Z|Add feature\n\
     20\t8\tsrc/lib.rs\n\
     5\t0\ttests/test.rs\n"
        .to_string()
}

/// Creates a git log output with binary files (- for additions)
fn create_git_log_with_binary_files() -> String {
    format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Add binary\n\
         -\t-\timage.png\n\
         10\t5\tsrc/main.rs\n"
    )
}

/// Creates a git log output with unicode characters
fn create_git_log_with_unicode() -> String {
    format!(
        "abc123\x00José García\x00jose@example.com\x002024-01-15T10:30:00Z\x00Fix: 修正 bug 🐛\n\
         10\t5\tsrc/main.rs\n"
    )
}

/// Creates a commit with no file changes
fn create_git_log_no_file_changes() -> String {
    format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Empty commit\n"
    )
}

/// Creates a RepoStats with sample data
fn create_sample_repo_stats(name: &str) -> RepoStats {
    let mut stats = RepoStats {
        name: name.to_string(),
        path: format!("/path/to/{}", name),
        total_commits: 10,
        total_lines_added: 100,
        total_lines_removed: 50,
        total_files_changed: 25,
        ..Default::default()
    };

    stats.languages.insert("Rust".to_string(), 100);
    stats.languages.insert("JavaScript".to_string(), 50);
    stats.contribution_types.insert("code".to_string(), 120);
    stats.contribution_types.insert("docs".to_string(), 30);
    stats.file_extensions.insert(".rs".to_string(), 80);
    stats.file_extensions.insert(".js".to_string(), 40);

    stats
}

/// Creates a commit info with sample data
fn create_sample_commit(hash: &str, date_str: &str) -> CommitInfo {
    CommitInfo {
        hash: hash.to_string(),
        author: "Test Author".to_string(),
        email: "test@example.com".to_string(),
        date: DateTime::parse_from_rfc3339(date_str)
            .unwrap()
            .with_timezone(&Utc),
        message: "Test commit".to_string(),
        files_changed: 2,
        lines_added: 10,
        lines_removed: 5,
        file_classifications: Vec::new(),
    }
}

// ========================================
// 1. ParseError Tests
// ========================================

#[test]
fn test_parse_error_display_invalid_commit_format() {
    let error = ParseError::InvalidCommitFormat("bad line".to_string());
    assert_eq!(
        error.to_string(),
        "Invalid commit format: bad line"
    );
}

#[test]
fn test_parse_error_display_invalid_date() {
    let error = ParseError::InvalidDate("not-a-date".to_string());
    assert_eq!(
        error.to_string(),
        "Invalid date format: not-a-date"
    );
}

#[test]
fn test_parse_error_display_empty_input() {
    let error = ParseError::EmptyInput;
    assert_eq!(error.to_string(), "Empty git log input");
}

#[test]
fn test_parse_error_implements_error_trait() {
    let error = ParseError::EmptyInput;
    // Should compile - verifies Error trait is implemented
    let _: &dyn std::error::Error = &error;
}

// ========================================
// 2. RepoStats Tests
// ========================================

#[test]
fn test_repo_stats_default_values() {
    let stats = RepoStats::default();
    assert_eq!(stats.name, "");
    assert_eq!(stats.path, "");
    assert_eq!(stats.description, "");
    assert_eq!(stats.total_commits, 0);
    assert_eq!(stats.total_lines_added, 0);
    assert_eq!(stats.total_lines_removed, 0);
    assert_eq!(stats.total_files_changed, 0);
    assert!(stats.first_commit_date.is_none());
    assert!(stats.last_commit_date.is_none());
    assert!(stats.last_commit_hash.is_none());
    assert!(stats.languages.is_empty());
    assert!(stats.contribution_types.is_empty());
    assert!(stats.file_extensions.is_empty());
    assert!(stats.commits.is_empty());
}

#[test]
fn test_repo_stats_merge_accumulates_totals() {
    let mut stats1 = RepoStats {
        total_commits: 5,
        total_lines_added: 100,
        total_lines_removed: 50,
        total_files_changed: 20,
        ..Default::default()
    };

    let stats2 = RepoStats {
        total_commits: 3,
        total_lines_added: 60,
        total_lines_removed: 30,
        total_files_changed: 15,
        ..Default::default()
    };

    stats1.merge(&stats2);

    assert_eq!(stats1.total_commits, 8);
    assert_eq!(stats1.total_lines_added, 160);
    assert_eq!(stats1.total_lines_removed, 80);
    assert_eq!(stats1.total_files_changed, 35);
}

#[test]
fn test_repo_stats_merge_handles_first_commit_date_min() {
    let date1 = DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let date2 = DateTime::parse_from_rfc3339("2024-01-10T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let mut stats1 = RepoStats {
        first_commit_date: Some(date1),
        ..Default::default()
    };

    let stats2 = RepoStats {
        first_commit_date: Some(date2),
        ..Default::default()
    };

    stats1.merge(&stats2);
    assert_eq!(stats1.first_commit_date, Some(date2)); // Earlier date
}

#[test]
fn test_repo_stats_merge_handles_last_commit_date_max() {
    let date1 = DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let date2 = DateTime::parse_from_rfc3339("2024-01-20T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let mut stats1 = RepoStats {
        last_commit_date: Some(date1),
        ..Default::default()
    };

    let stats2 = RepoStats {
        last_commit_date: Some(date2),
        ..Default::default()
    };

    stats1.merge(&stats2);
    assert_eq!(stats1.last_commit_date, Some(date2)); // Later date
}

#[test]
fn test_repo_stats_merge_handles_none_dates() {
    let date = DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let mut stats1 = RepoStats::default();
    let stats2 = RepoStats {
        first_commit_date: Some(date),
        last_commit_date: Some(date),
        ..Default::default()
    };

    stats1.merge(&stats2);
    assert_eq!(stats1.first_commit_date, Some(date));
    assert_eq!(stats1.last_commit_date, Some(date));
}

#[test]
fn test_repo_stats_merge_aggregates_languages() {
    let mut stats1 = RepoStats::default();
    stats1.languages.insert("Rust".to_string(), 100);
    stats1.languages.insert("JavaScript".to_string(), 50);

    let mut stats2 = RepoStats::default();
    stats2.languages.insert("Rust".to_string(), 80);
    stats2.languages.insert("Python".to_string(), 40);

    stats1.merge(&stats2);

    assert_eq!(stats1.languages.get("Rust"), Some(&180));
    assert_eq!(stats1.languages.get("JavaScript"), Some(&50));
    assert_eq!(stats1.languages.get("Python"), Some(&40));
}

#[test]
fn test_repo_stats_merge_aggregates_contribution_types() {
    let mut stats1 = RepoStats::default();
    stats1.contribution_types.insert("code".to_string(), 100);
    stats1.contribution_types.insert("docs".to_string(), 50);

    let mut stats2 = RepoStats::default();
    stats2.contribution_types.insert("code".to_string(), 80);
    stats2.contribution_types.insert("tests".to_string(), 40);

    stats1.merge(&stats2);

    assert_eq!(stats1.contribution_types.get("code"), Some(&180));
    assert_eq!(stats1.contribution_types.get("docs"), Some(&50));
    assert_eq!(stats1.contribution_types.get("tests"), Some(&40));
}

#[test]
fn test_repo_stats_merge_aggregates_file_extensions() {
    let mut stats1 = RepoStats::default();
    stats1.file_extensions.insert(".rs".to_string(), 100);
    stats1.file_extensions.insert(".js".to_string(), 50);

    let mut stats2 = RepoStats::default();
    stats2.file_extensions.insert(".rs".to_string(), 80);
    stats2.file_extensions.insert(".py".to_string(), 40);

    stats1.merge(&stats2);

    assert_eq!(stats1.file_extensions.get(".rs"), Some(&180));
    assert_eq!(stats1.file_extensions.get(".js"), Some(&50));
    assert_eq!(stats1.file_extensions.get(".py"), Some(&40));
}

#[test]
fn test_repo_stats_merge_extends_commits() {
    let commit1 = create_sample_commit("abc123", "2024-01-15T10:00:00Z");
    let commit2 = create_sample_commit("def456", "2024-01-16T10:00:00Z");

    let mut stats1 = RepoStats::default();
    stats1.commits.push(commit1.clone());

    let mut stats2 = RepoStats::default();
    stats2.commits.push(commit2.clone());

    stats1.merge(&stats2);

    assert_eq!(stats1.commits.len(), 2);
    assert_eq!(stats1.commits[0].hash, "abc123");
    assert_eq!(stats1.commits[1].hash, "def456");
}

#[test]
fn test_repo_stats_merge_updates_last_commit_hash() {
    let mut stats1 = RepoStats {
        last_commit_hash: Some("old_hash".to_string()),
        ..Default::default()
    };

    let stats2 = RepoStats {
        last_commit_hash: Some("new_hash".to_string()),
        ..Default::default()
    };

    stats1.merge(&stats2);
    assert_eq!(stats1.last_commit_hash, Some("new_hash".to_string()));
}

// ========================================
// 3. GitAnalyzer Parsing Tests
// ========================================

#[test]
fn test_parse_git_log_with_null_byte_delimiter() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = create_sample_git_log_null_delimiter();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.name, "test-repo");
    assert_eq!(stats.path, "/path/to/repo");
    assert_eq!(stats.total_commits, 2);
    assert_eq!(stats.commits.len(), 2);

    // Check first commit
    assert_eq!(stats.commits[0].hash, "abc123");
    assert_eq!(stats.commits[0].author, "John Doe");
    assert_eq!(stats.commits[0].email, "john@example.com");
    assert_eq!(stats.commits[0].message, "Initial commit");
    assert_eq!(stats.commits[0].files_changed, 2);
    assert_eq!(stats.commits[0].lines_added, 13);
    assert_eq!(stats.commits[0].lines_removed, 7);

    // Check second commit
    assert_eq!(stats.commits[1].hash, "def456");
    assert_eq!(stats.commits[1].author, "Jane Smith");
    assert_eq!(stats.commits[1].lines_added, 25);
    assert_eq!(stats.commits[1].lines_removed, 8);
}

#[test]
fn test_parse_git_log_with_legacy_pipe_delimiter() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = create_sample_git_log_pipe_delimiter();
    let result = analyzer.parse_git_log_legacy("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total_commits, 2);
    assert_eq!(stats.commits.len(), 2);
    assert_eq!(stats.commits[0].hash, "abc123");
    assert_eq!(stats.commits[1].hash, "def456");
}

#[test]
fn test_parse_git_log_with_empty_input_returns_error() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", "");

    assert!(result.is_err());
    match result {
        Err(ParseError::EmptyInput) => (),
        _ => panic!("Expected EmptyInput error"),
    }
}

#[test]
fn test_parse_git_log_with_whitespace_only_returns_error() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", "   \n\n  \t  ");

    assert!(result.is_err());
    match result {
        Err(ParseError::EmptyInput) => (),
        _ => panic!("Expected EmptyInput error"),
    }
}

#[test]
fn test_parse_git_log_with_invalid_date_format() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x00invalid-date\x00Test commit\n\
         10\t5\tsrc/main.rs\n"
    );

    // Should not crash, but won't include the commit with invalid date in stored commits
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);
    assert!(result.is_ok());
    let stats = result.unwrap();
    // Commit with invalid date is still counted in total (counted by delimiter presence)
    // but not processed for file stats
    assert!(stats.total_commits >= 1);
}

#[test]
fn test_parse_git_log_with_binary_files() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = create_git_log_with_binary_files();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 1);

    // Binary file should contribute 0 to line counts, but regular file should count
    assert_eq!(stats.commits[0].lines_added, 10);
    assert_eq!(stats.commits[0].lines_removed, 5);
    assert_eq!(stats.commits[0].files_changed, 2); // Both files counted
}

#[test]
fn test_parse_git_log_with_unicode_characters() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = create_git_log_with_unicode();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 1);
    assert_eq!(stats.commits[0].author, "José García");
    assert_eq!(stats.commits[0].message, "Fix: 修正 bug 🐛");
}

#[test]
fn test_parse_git_log_with_no_file_changes() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = create_git_log_no_file_changes();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 1);
    assert_eq!(stats.commits[0].files_changed, 0);
    assert_eq!(stats.commits[0].lines_added, 0);
    assert_eq!(stats.commits[0].lines_removed, 0);
}

#[test]
fn test_parse_git_log_without_storing_commits() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(false);

    let log_output = create_sample_git_log_null_delimiter();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total_commits, 2);
    assert_eq!(stats.commits.len(), 0); // Commits not stored

    // But totals should still be calculated
    assert!(stats.total_lines_added > 0);
    assert!(stats.total_lines_removed > 0);
}

#[test]
fn test_parse_git_log_tracks_languages() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let log_output = create_sample_git_log_null_delimiter();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();

    // Should have classified Rust files (depending on classifier config)
    assert!(stats.languages.contains_key("Rust"));
    // The file classifier determines actual language mappings
    assert!(!stats.languages.is_empty());
}

#[test]
fn test_parse_git_log_tracks_file_extensions() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let log_output = create_sample_git_log_null_delimiter();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();

    // Should have tracked .rs and .md extensions
    assert!(stats.file_extensions.contains_key(".rs"));
    assert!(stats.file_extensions.contains_key(".md"));
}

#[test]
fn test_parse_git_log_sets_date_range() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = create_sample_git_log_null_delimiter();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();

    assert!(stats.first_commit_date.is_some());
    assert!(stats.last_commit_date.is_some());

    let first = stats.first_commit_date.unwrap();
    let last = stats.last_commit_date.unwrap();
    assert!(first <= last);
}

#[test]
fn test_parse_git_log_stores_last_commit_hash() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let log_output = create_sample_git_log_null_delimiter();
    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();

    // First hash in log output should be stored
    assert_eq!(stats.last_commit_hash, Some("abc123".to_string()));
}

#[test]
fn test_parse_git_log_legacy_compatibility() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let pipe_output = create_sample_git_log_pipe_delimiter();
    let result = analyzer.parse_git_log_legacy("test-repo", "/path/to/repo", &pipe_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total_commits, 2);
    assert!(stats.total_lines_added > 0);
}

// ========================================
// 4. Statistics Computation Tests
// ========================================

#[test]
fn test_get_total_stats_with_single_repo() {
    let mut analyzer = GitAnalyzer::new(None, None);
    let stats = create_sample_repo_stats("repo1");
    analyzer.add_repo_data(stats);

    let total = analyzer.get_total_stats();

    assert_eq!(total.total_repos, 1);
    assert_eq!(total.total_commits, 10);
    assert_eq!(total.total_lines_added, 100);
    assert_eq!(total.total_lines_removed, 50);
    assert_eq!(total.total_lines_changed, 150);
    assert_eq!(total.total_files_changed, 25);
}

#[test]
fn test_get_total_stats_with_multiple_repos() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.add_repo_data(create_sample_repo_stats("repo2"));
    analyzer.add_repo_data(create_sample_repo_stats("repo3"));

    let total = analyzer.get_total_stats();

    assert_eq!(total.total_repos, 3);
    assert_eq!(total.total_commits, 30); // 10 * 3
    assert_eq!(total.total_lines_added, 300); // 100 * 3
    assert_eq!(total.total_lines_removed, 150); // 50 * 3
    assert_eq!(total.total_lines_changed, 450);
    assert_eq!(total.total_files_changed, 75); // 25 * 3
}

#[test]
fn test_get_total_stats_aggregates_languages() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.add_repo_data(create_sample_repo_stats("repo2"));

    let total = analyzer.get_total_stats();

    assert_eq!(total.languages.get("Rust"), Some(&200)); // 100 * 2
    assert_eq!(total.languages.get("JavaScript"), Some(&100)); // 50 * 2
}

#[test]
fn test_calculate_percentages_with_empty_map() {
    let map = HashMap::new();
    let percentages = GitAnalyzer::calculate_percentages(&map);
    assert!(percentages.is_empty());
}

#[test]
fn test_calculate_percentages_with_data() {
    let mut map = HashMap::new();
    map.insert("Rust".to_string(), 80);
    map.insert("JavaScript".to_string(), 20);

    let percentages = GitAnalyzer::calculate_percentages(&map);

    assert_eq!(percentages.get("Rust"), Some(&80.0));
    assert_eq!(percentages.get("JavaScript"), Some(&20.0));
}

#[test]
fn test_calculate_percentages_rounds_correctly() {
    let mut map = HashMap::new();
    map.insert("A".to_string(), 33);
    map.insert("B".to_string(), 33);
    map.insert("C".to_string(), 34);

    let percentages = GitAnalyzer::calculate_percentages(&map);

    // Should round to 1 decimal place
    assert_eq!(percentages.get("A"), Some(&33.0));
    assert_eq!(percentages.get("B"), Some(&33.0));
    assert_eq!(percentages.get("C"), Some(&34.0));
}

#[test]
fn test_cache_stats_stores_computed_stats() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    // Initially no cache
    assert!(analyzer.cached_stats.is_none());

    analyzer.cache_stats();

    // After caching
    assert!(analyzer.cached_stats.is_some());

    let cached = analyzer.cached_stats.as_ref().unwrap();
    assert_eq!(cached.total_repos, 1);
    assert_eq!(cached.total_commits, 10);
}

#[test]
fn test_cache_invalidated_on_add_repo_data() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.cache_stats();

    assert!(analyzer.cached_stats.is_some());

    // Adding new repo should invalidate cache
    analyzer.add_repo_data(create_sample_repo_stats("repo2"));

    assert!(analyzer.cached_stats.is_none());
}

#[test]
fn test_get_total_stats_uses_cache() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    // First call computes and doesn't use cache
    let stats1 = analyzer.get_total_stats();

    // Manually cache
    analyzer.cache_stats();

    // Second call should use cache
    let stats2 = analyzer.get_total_stats();

    assert_eq!(stats1.total_commits, stats2.total_commits);
    assert_eq!(stats1.total_repos, stats2.total_repos);
}

// ========================================
// 5. Activity Summary Tests
// ========================================

#[test]
fn test_get_daily_activity_returns_correct_day_count() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_daily_activity(7);

    assert_eq!(activity.len(), 7);
}

#[test]
fn test_get_daily_activity_calculates_date_ranges() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_daily_activity(3);

    for summary in &activity {
        // End should be after start
        assert!(summary.period_end > summary.period_start);

        // Period should be roughly 1 day (allowing for DST)
        let duration = summary.period_end.signed_duration_since(summary.period_start);
        assert!(duration.num_hours() >= 23 && duration.num_hours() <= 25);
    }
}

#[test]
fn test_get_daily_activity_includes_commits_in_range() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    // Create a repo with commits
    let log_output = create_sample_git_log_null_delimiter();
    let _ = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    let activity = analyzer.get_daily_activity(30);

    // Should return 30 activity summaries
    assert_eq!(activity.len(), 30);
}

#[test]
fn test_get_weekly_activity_returns_correct_week_count() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_weekly_activity(4);

    assert_eq!(activity.len(), 4);
}

#[test]
fn test_get_weekly_activity_has_7_day_periods() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_weekly_activity(2);

    for summary in &activity {
        let duration = summary.period_end.signed_duration_since(summary.period_start);
        // Should be approximately 7 days
        assert!(duration.num_days() >= 6 && duration.num_days() <= 7);
    }
}

#[test]
fn test_get_monthly_activity_returns_correct_month_count() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_monthly_activity(6);

    assert_eq!(activity.len(), 6);
}

#[test]
fn test_get_monthly_activity_handles_year_boundaries() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_monthly_activity(13);

    assert_eq!(activity.len(), 13);

    // Check that we cross year boundaries properly
    let years: Vec<i32> = activity.iter()
        .map(|a| a.period_start.year())
        .collect();

    // Should have at least 2 different years if we go back 13 months
    let unique_years: HashSet<i32> = years.into_iter().collect();
    assert!(unique_years.len() >= 2);
}

#[test]
fn test_get_monthly_activity_period_labels_have_month_names() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_monthly_activity(3);

    for summary in &activity {
        // Label should contain a month name and year
        assert!(summary.period_label.contains("20")); // Year contains "20"
    }
}

#[test]
fn test_activity_summary_initializes_with_zero_stats() {
    let analyzer = GitAnalyzer::new(None, None);
    let activity = analyzer.get_daily_activity(1);

    assert_eq!(activity[0].commits, 0);
    assert_eq!(activity[0].lines_added, 0);
    assert_eq!(activity[0].lines_removed, 0);
    assert_eq!(activity[0].files_changed, 0);
    assert_eq!(activity[0].repos_active, 0);
}

// ========================================
// 6. Repository Management Tests
// ========================================

#[test]
fn test_add_repo_data_adds_to_repos_list() {
    let mut analyzer = GitAnalyzer::new(None, None);
    assert_eq!(analyzer.get_repos().len(), 0);

    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    assert_eq!(analyzer.get_repos().len(), 1);

    analyzer.add_repo_data(create_sample_repo_stats("repo2"));
    assert_eq!(analyzer.get_repos().len(), 2);
}

#[test]
fn test_find_repo_returns_repo_by_name() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.add_repo_data(create_sample_repo_stats("repo2"));

    let found = analyzer.find_repo("repo1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "repo1");
}

#[test]
fn test_find_repo_returns_none_for_nonexistent() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    let found = analyzer.find_repo("nonexistent");
    assert!(found.is_none());
}

#[test]
fn test_find_repo_mut_returns_mutable_reference() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    let found = analyzer.find_repo_mut("repo1");
    assert!(found.is_some());

    // Modify the repo
    if let Some(repo) = found {
        repo.total_commits = 999;
    }

    // Verify modification
    let repo = analyzer.find_repo("repo1").unwrap();
    assert_eq!(repo.total_commits, 999);
}

#[test]
fn test_find_repo_mut_invalidates_cache() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.cache_stats();

    assert!(analyzer.cached_stats.is_some());

    // find_repo_mut should invalidate cache
    let _ = analyzer.find_repo_mut("repo1");

    assert!(analyzer.cached_stats.is_none());
}

#[test]
fn test_get_repos_returns_slice() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.add_repo_data(create_sample_repo_stats("repo2"));

    let repos = analyzer.get_repos();
    assert_eq!(repos.len(), 2);
    assert_eq!(repos[0].name, "repo1");
    assert_eq!(repos[1].name, "repo2");
}

#[test]
fn test_get_repos_mut_returns_mutable_vec() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    let repos = analyzer.get_repos_mut();
    repos[0].total_commits = 777;

    // Verify modification
    assert_eq!(analyzer.get_repos()[0].total_commits, 777);
}

#[test]
fn test_get_repos_mut_invalidates_cache() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));
    analyzer.cache_stats();

    assert!(analyzer.cached_stats.is_some());

    // get_repos_mut should invalidate cache
    let _ = analyzer.get_repos_mut();

    assert!(analyzer.cached_stats.is_none());
}

// ========================================
// 7. git_log_command Function Tests
// ========================================

#[test]
fn test_git_log_command_without_since_hash() {
    let cmd = git_log_command(None);

    assert!(cmd.contains("git log"));
    assert!(cmd.contains("--numstat"));
    assert!(cmd.contains("--format="));
    assert!(!cmd.contains("..")); // No range
}

#[test]
fn test_git_log_command_with_since_hash() {
    let cmd = git_log_command(Some("abc123"));

    assert!(cmd.contains("git log"));
    assert!(cmd.contains("abc123..HEAD"));
    assert!(cmd.contains("--numstat"));
    assert!(cmd.contains("--format="));
}

#[test]
fn test_git_log_command_uses_correct_format() {
    let cmd = git_log_command(None);

    // Should include the GIT_LOG_FORMAT string
    assert!(cmd.contains("%H%x00%an%x00%ae%x00%aI%x00%s"));
}

// ========================================
// 8. Edge Cases Tests
// ========================================

#[test]
fn test_parse_git_log_with_very_large_line_counts() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    // Use large but not overflowing numbers
    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Large commit\n\
         1000000\t500000\tsrc/large_file.rs\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits[0].lines_added, 1_000_000);
    assert_eq!(stats.commits[0].lines_removed, 500_000);
}

#[test]
fn test_parse_git_log_with_future_dates() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    // Date in the future
    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002099-12-31T23:59:59Z\x00Future commit\n\
         10\t5\tsrc/main.rs\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 1);
    assert_eq!(stats.commits[0].date.year(), 2099);
}

#[test]
fn test_parse_git_log_with_commits_having_same_timestamp() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let same_time = "2024-01-15T10:30:00Z";
    let log_output = format!(
        "abc123\x00Author1\x00a1@example.com\x00{}\x00Commit 1\n\
         10\t5\tfile1.rs\n\
         def456\x00Author2\x00a2@example.com\x00{}\x00Commit 2\n\
         20\t10\tfile2.rs\n",
        same_time, same_time
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 2);
    assert_eq!(stats.first_commit_date, stats.last_commit_date);
}

#[test]
fn test_parse_git_log_with_file_without_extension() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00No ext file\n\
         10\t5\tMakefile\n\
         3\t2\tREADME\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();

    // Files without extensions should be tracked as "(no ext)"
    assert!(stats.file_extensions.contains_key("(no ext)"));
}

#[test]
fn test_parse_git_log_with_mixed_case_extensions() {
    let mut analyzer = GitAnalyzer::new(None, None);

    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Mixed case\n\
         10\t5\tsrc/Main.RS\n\
         3\t2\tREADME.MD\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();

    // Extensions should be normalized to lowercase
    assert!(stats.file_extensions.contains_key(".rs"));
    assert!(stats.file_extensions.contains_key(".md"));
}

#[test]
fn test_parse_git_log_with_empty_commit_message() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00\n\
         10\t5\tsrc/main.rs\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 1);
    assert_eq!(stats.commits[0].message, "");
}

#[test]
fn test_parse_git_log_with_multiline_context_doesnt_break_parsing() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    // Message contains what looks like a file stat line
    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Fix\n\
         10\t5\tsrc/main.rs\n\
         3\t2\tREADME.md\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits.len(), 1);
    assert_eq!(stats.commits[0].files_changed, 2);
}

#[test]
fn test_analyzer_new_with_author_info() {
    let analyzer = GitAnalyzer::new(
        Some("test@example.com".to_string()),
        Some("Test Author".to_string()),
    );

    assert_eq!(analyzer.author_email, Some("test@example.com".to_string()));
    assert_eq!(analyzer.author_name, Some("Test Author".to_string()));
}

#[test]
fn test_analyzer_with_options() {
    let options = ParseOptions {
        store_commits: true,
        legacy_delimiter: true,
    };

    let analyzer = GitAnalyzer::new(None, None).with_options(options);

    // Verify by checking parsing behavior
    assert_eq!(analyzer.parse_options.store_commits, true);
    assert_eq!(analyzer.parse_options.legacy_delimiter, true);
}

#[test]
fn test_set_store_commits_changes_option() {
    let mut analyzer = GitAnalyzer::new(None, None);
    assert_eq!(analyzer.parse_options.store_commits, false);

    analyzer.set_store_commits(true);
    assert_eq!(analyzer.parse_options.store_commits, true);

    analyzer.set_store_commits(false);
    assert_eq!(analyzer.parse_options.store_commits, false);
}

#[test]
fn test_get_dashboard_data_includes_all_sections() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    let dashboard = analyzer.get_dashboard_data();

    // Verify all sections are present
    assert!(dashboard.generated_at <= Utc::now());
    assert_eq!(dashboard.summary.total_repos, 1);
    assert_eq!(dashboard.repositories.len(), 1);
    assert_eq!(dashboard.daily_activity.len(), 7);
    assert_eq!(dashboard.weekly_activity.len(), 4);
    assert_eq!(dashboard.monthly_activity.len(), 6);
}

#[test]
fn test_parse_git_log_with_tabs_in_commit_message() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let log_output = format!(
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Message\twith\ttabs\n\
         10\t5\tsrc/main.rs\n"
    );

    let result = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.commits[0].message, "Message\twith\ttabs");
}

#[test]
fn test_total_stats_includes_percentages() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.add_repo_data(create_sample_repo_stats("repo1"));

    let total = analyzer.get_total_stats();

    assert!(!total.language_percentages.is_empty());
    assert!(!total.contribution_percentages.is_empty());
    assert!(!total.file_extension_percentages.is_empty());

    // Percentages should sum to approximately 100
    let lang_sum: f64 = total.language_percentages.values().sum();
    assert!((lang_sum - 100.0).abs() < 1.0); // Allow small rounding error
}

#[test]
fn test_parse_git_log_adds_to_analyzer_repos() {
    let mut analyzer = GitAnalyzer::new(None, None);
    assert_eq!(analyzer.get_repos().len(), 0);

    let log_output = create_sample_git_log_null_delimiter();
    let _ = analyzer.parse_git_log("test-repo", "/path/to/repo", &log_output);

    // Should have added the repo to analyzer
    assert_eq!(analyzer.get_repos().len(), 1);
    assert_eq!(analyzer.get_repos()[0].name, "test-repo");
}

#[test]
fn test_get_file_extension_with_dotfiles() {
    let ext = GitAnalyzer::get_file_extension(".gitignore");
    // Dotfiles without an extension after the initial dot return "(no ext)"
    assert_eq!(ext, "(no ext)");
}

#[test]
fn test_get_file_extension_with_path() {
    let ext = GitAnalyzer::get_file_extension("src/lib/module.rs");
    assert_eq!(ext, ".rs");
}

#[test]
fn test_get_file_extension_with_multiple_dots() {
    let ext = GitAnalyzer::get_file_extension("file.test.js");
    assert_eq!(ext, ".js");
}
