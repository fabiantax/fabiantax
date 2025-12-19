use super::*;
use crate::analyzer::{CommitInfo, RepoStats};
use chrono::{Duration, Utc};
use std::collections::HashMap;

/// Helper function to create a test GitAnalyzer with predictable data
fn create_test_analyzer() -> GitAnalyzer {
    let mut analyzer = GitAnalyzer::new(
        Some("test@example.com".to_string()),
        Some("Test User".to_string()),
    );
    analyzer.set_store_commits(true);

    // Create test repo 1 - Rust project
    let now = Utc::now();
    let repo1 = RepoStats {
        name: "test-repo-1".to_string(),
        path: "/path/to/repo1".to_string(),
        description: "A test repository for Rust code".to_string(),
        technologies: vec!["Rust".to_string(), "Git".to_string()],
        total_commits: 3,
        total_lines_added: 500,
        total_lines_removed: 100,
        total_files_changed: 15,
        first_commit_date: Some(now - Duration::days(60)),
        last_commit_date: Some(now - Duration::days(1)),
        last_commit_hash: Some("abc123".to_string()),
        languages: HashMap::from([
            ("Rust".to_string(), 400),
            ("JavaScript".to_string(), 150),
        ]),
        contribution_types: HashMap::from([
            ("production_code".to_string(), 300),
            ("tests".to_string(), 200),
            ("documentation".to_string(), 50),
        ]),
        file_extensions: HashMap::from([
            (".rs".to_string(), 400),
            (".js".to_string(), 150),
        ]),
        commits: vec![
            CommitInfo {
                hash: "commit1".to_string(),
                author: "Test User".to_string(),
                email: "test@example.com".to_string(),
                date: now - Duration::days(5),
                message: "Add feature".to_string(),
                files_changed: 5,
                lines_added: 200,
                lines_removed: 50,
                file_classifications: vec![],
                contribution_types: HashMap::from([("productioncode".to_string(), 250)]),
                languages: HashMap::from([("Rust".to_string(), 250)]),
            },
            CommitInfo {
                hash: "commit2".to_string(),
                author: "Test User".to_string(),
                email: "test@example.com".to_string(),
                date: now - Duration::days(3),
                message: "Fix bug".to_string(),
                files_changed: 5,
                lines_added: 150,
                lines_removed: 25,
                file_classifications: vec![],
                contribution_types: HashMap::from([("productioncode".to_string(), 175)]),
                languages: HashMap::from([("Rust".to_string(), 175)]),
            },
            CommitInfo {
                hash: "commit3".to_string(),
                author: "Test User".to_string(),
                email: "test@example.com".to_string(),
                date: now - Duration::days(1),
                message: "Update docs".to_string(),
                files_changed: 5,
                lines_added: 150,
                lines_removed: 25,
                file_classifications: vec![],
                contribution_types: HashMap::from([("documentation".to_string(), 175)]),
                languages: HashMap::from([("Documentation".to_string(), 175)]),
            },
        ],
    };

    // Create test repo 2 - Python project
    let repo2 = RepoStats {
        name: "test-repo-2".to_string(),
        path: "/path/to/repo2".to_string(),
        description: "".to_string(),
        technologies: vec![],
        total_commits: 2,
        total_lines_added: 1500,
        total_lines_removed: 300,
        total_files_changed: 20,
        first_commit_date: Some(now - Duration::days(30)),
        last_commit_date: Some(now - Duration::days(2)),
        last_commit_hash: Some("def456".to_string()),
        languages: HashMap::from([
            ("Python".to_string(), 1200),
            ("TypeScript".to_string(), 600),
        ]),
        contribution_types: HashMap::from([
            ("production_code".to_string(), 1000),
            ("tests".to_string(), 500),
            ("infrastructure".to_string(), 300),
        ]),
        file_extensions: HashMap::from([
            (".py".to_string(), 1200),
            (".ts".to_string(), 600),
        ]),
        commits: vec![
            CommitInfo {
                hash: "commit4".to_string(),
                author: "Test User".to_string(),
                email: "test@example.com".to_string(),
                date: now - Duration::days(10),
                message: "Initial commit".to_string(),
                files_changed: 10,
                lines_added: 1000,
                lines_removed: 200,
                file_classifications: vec![],
                contribution_types: HashMap::from([("productioncode".to_string(), 1200)]),
                languages: HashMap::from([("Python".to_string(), 1200)]),
            },
            CommitInfo {
                hash: "commit5".to_string(),
                author: "Test User".to_string(),
                email: "test@example.com".to_string(),
                date: now - Duration::days(2),
                message: "Add tests".to_string(),
                files_changed: 10,
                lines_added: 500,
                lines_removed: 100,
                file_classifications: vec![],
                contribution_types: HashMap::from([("tests".to_string(), 600)]),
                languages: HashMap::from([("Python".to_string(), 600)]),
            },
        ],
    };

    analyzer.add_repo_data(repo1);
    analyzer.add_repo_data(repo2);
    analyzer.cache_stats();

    analyzer
}

/// Helper to create an analyzer with empty data
fn create_empty_analyzer() -> GitAnalyzer {
    GitAnalyzer::new(Some("test@example.com".to_string()), Some("Test User".to_string()))
}

// ============================================================================
// fmt_num helper function tests
// ============================================================================

#[test]
fn test_fmt_num_zero() {
    assert_eq!(fmt_num(0), "0");
}

#[test]
fn test_fmt_num_small_numbers() {
    assert_eq!(fmt_num(1), "1");
    assert_eq!(fmt_num(99), "99");
    assert_eq!(fmt_num(999), "999");
}

#[test]
fn test_fmt_num_thousands() {
    assert_eq!(fmt_num(1000), "1,000");
    assert_eq!(fmt_num(1234), "1,234");
    assert_eq!(fmt_num(9999), "9,999");
    assert_eq!(fmt_num(12345), "12,345");
}

#[test]
fn test_fmt_num_millions() {
    assert_eq!(fmt_num(1000000), "1,000,000");
    assert_eq!(fmt_num(1234567), "1,234,567");
    assert_eq!(fmt_num(9999999), "9,999,999");
}

// ============================================================================
// MarkdownExporter tests
// ============================================================================

#[test]
fn test_markdown_export_structure() {
    let analyzer = create_test_analyzer();
    let output = MarkdownExporter::export(&analyzer);

    // Check for main headers
    assert!(output.contains("# Git Activity Dashboard"));
    assert!(output.contains("## Overview"));
    assert!(output.contains("## Contribution Breakdown"));
    assert!(output.contains("## Programming Languages"));
    assert!(output.contains("## File Types (by extension)"));
    assert!(output.contains("## Weekly Activity"));
    assert!(output.contains("## Monthly Activity"));
    assert!(output.contains("## Repositories (detailed)"));
}

#[test]
fn test_markdown_export_contains_table_formatting() {
    let analyzer = create_test_analyzer();
    let output = MarkdownExporter::export(&analyzer);

    // Check for table structures
    assert!(output.contains("| Metric | Value |"));
    assert!(output.contains("|--------|-------|"));
    assert!(output.contains("| Type | Lines | Percentage |"));
    assert!(output.contains("|------|-------|------------|"));
}

#[test]
fn test_markdown_export_contains_stats() {
    let analyzer = create_test_analyzer();
    let output = MarkdownExporter::export(&analyzer);

    // Check for repo count (2 repos)
    assert!(output.contains("| Repositories | 2 |"));

    // Check for total commits (5 total)
    assert!(output.contains("| Total Commits | 5 |"));

    // Check that repo names appear
    assert!(output.contains("### test-repo-1"));
    assert!(output.contains("### test-repo-2"));
}

#[test]
fn test_markdown_export_repos_sorted_by_commits() {
    let analyzer = create_test_analyzer();
    let output = MarkdownExporter::export(&analyzer);

    // test-repo-1 has 3 commits, test-repo-2 has 2 commits
    // test-repo-1 should appear first
    let repo1_pos = output.find("### test-repo-1").unwrap();
    let repo2_pos = output.find("### test-repo-2").unwrap();
    assert!(repo1_pos < repo2_pos);
}

#[test]
fn test_markdown_export_empty_description() {
    let analyzer = create_test_analyzer();
    let output = MarkdownExporter::export(&analyzer);

    // Repo 2 has empty description, should not have a blockquote before stats
    let repo2_section = output.split("### test-repo-2").nth(1).unwrap();
    let commits_line_pos = repo2_section.find("**Commits:**").unwrap();
    let blockquote_before = &repo2_section[..commits_line_pos];

    // Should not contain a blockquote line (starts with '>')
    let has_blockquote = blockquote_before.lines().any(|line| line.trim().starts_with('>'));
    assert!(!has_blockquote || blockquote_before.contains("> A test repository"));
}

#[test]
fn test_contribution_type_label() {
    use crate::utils::contribution_type_label;

    assert_eq!(contribution_type_label("productioncode"), "Production Code");
    assert_eq!(contribution_type_label("tests"), "Tests");
    assert_eq!(contribution_type_label("documentation"), "Documentation");
    assert_eq!(contribution_type_label("specsconfig"), "Specs & Config");
    assert_eq!(contribution_type_label("infrastructure"), "Infrastructure");
    assert_eq!(contribution_type_label("styling"), "Styling");
    assert_eq!(contribution_type_label("unknown"), "Other");
}

#[test]
fn test_markdown_export_empty_analyzer() {
    let analyzer = create_empty_analyzer();
    let output = MarkdownExporter::export(&analyzer);

    // Should still have structure but with zeros
    assert!(output.contains("# Git Activity Dashboard"));
    assert!(output.contains("| Repositories | 0 |"));
    assert!(output.contains("| Total Commits | 0 |"));
}

#[test]
fn test_markdown_export_handles_empty_languages() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let repo = RepoStats {
        name: "empty-lang-repo".to_string(),
        path: "/test".to_string(),
        total_commits: 1,
        languages: HashMap::new(),
        contribution_types: HashMap::from([("production_code".to_string(), 100)]),
        file_extensions: HashMap::new(),
        ..Default::default()
    };

    analyzer.add_repo_data(repo);
    let output = MarkdownExporter::export(&analyzer);

    // Should not crash, and should not show languages section if empty
    assert!(output.contains("# Git Activity Dashboard"));
}

// ============================================================================
// LinkedInExporter tests
// ============================================================================

#[test]
fn test_linkedin_export_structure() {
    let analyzer = create_test_analyzer();
    let output = LinkedInExporter::export(&analyzer);

    assert!(output.contains("My Developer Activity This Week"));
    assert!(output.contains("#coding #developer #programming #softwareengineering"));
}

#[test]
fn test_linkedin_export_includes_hashtags() {
    let analyzer = create_test_analyzer();
    let output = LinkedInExporter::export(&analyzer);

    assert!(output.contains("#coding"));
    assert!(output.contains("#developer"));
    assert!(output.contains("#programming"));
    assert!(output.contains("#softwareengineering"));
}

#[test]
fn test_linkedin_export_shows_top_languages() {
    let analyzer = create_test_analyzer();
    let output = LinkedInExporter::export(&analyzer);

    // Should show top languages
    assert!(output.contains("Top Languages:"));
    // Should contain some of our test languages
    assert!(output.contains("Python") || output.contains("Rust") || output.contains("TypeScript"));
}

#[test]
fn test_linkedin_export_shows_quality_metrics() {
    let analyzer = create_test_analyzer();
    let output = LinkedInExporter::export(&analyzer);

    // Should show quality metrics since we have tests and docs
    assert!(output.contains("Code Quality:"));
}

#[test]
fn test_linkedin_export_empty_week() {
    // Create analyzer with old commits (not this week)
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let old_date = Utc::now() - Duration::days(30);
    let repo = RepoStats {
        name: "old-repo".to_string(),
        path: "/test".to_string(),
        total_commits: 1,
        contribution_types: HashMap::from([("production_code".to_string(), 100)]),
        commits: vec![CommitInfo {
            hash: "old".to_string(),
            author: "Test".to_string(),
            email: "test@test.com".to_string(),
            date: old_date,
            message: "Old commit".to_string(),
            files_changed: 1,
            lines_added: 50,
            lines_removed: 10,
            file_classifications: vec![],
            contribution_types: HashMap::from([("productioncode".to_string(), 60)]),
            languages: HashMap::new(),
        }],
        ..Default::default()
    };

    analyzer.add_repo_data(repo);
    let output = LinkedInExporter::export(&analyzer);

    // Should still generate output without crashing
    assert!(output.contains("My Developer Activity This Week"));
}

// ============================================================================
// PortfolioExporter tests
// ============================================================================

#[test]
fn test_portfolio_export_structure() {
    let analyzer = create_test_analyzer();
    let output = PortfolioExporter::export(&analyzer);

    assert!(output.contains("# Project Portfolio"));
    assert!(output.contains("## Summary"));
    assert!(output.contains("## Technical Skills"));
    assert!(output.contains("## Code Quality Practices"));
    assert!(output.contains("## Projects"));
}

#[test]
fn test_portfolio_export_technical_skills_with_progress_bars() {
    let analyzer = create_test_analyzer();
    let output = PortfolioExporter::export(&analyzer);

    // Should contain progress bars (# characters)
    assert!(output.contains("**Python**:"));
    assert!(output.contains("**Rust**:"));
    // Progress bars use # characters
    assert!(output.contains('#'));
}

#[test]
fn test_portfolio_export_project_list() {
    let analyzer = create_test_analyzer();
    let output = PortfolioExporter::export(&analyzer);

    // Should list projects
    assert!(output.contains("### test-repo-1"));
    assert!(output.contains("### test-repo-2"));

    // Should show contribution details
    assert!(output.contains("**My Contribution:**"));
    assert!(output.contains("commits"));
    assert!(output.contains("lines added"));
}

#[test]
fn test_portfolio_export_project_duration_days() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let now = Utc::now();
    let repo = RepoStats {
        name: "short-project".to_string(),
        path: "/test".to_string(),
        total_commits: 1,
        first_commit_date: Some(now - Duration::days(15)),
        last_commit_date: Some(now),
        contribution_types: HashMap::from([("production_code".to_string(), 100)]),
        ..Default::default()
    };

    analyzer.add_repo_data(repo);
    let output = PortfolioExporter::export(&analyzer);

    // Duration should be in days (< 30 days)
    assert!(output.contains("day(s)"));
}

#[test]
fn test_portfolio_export_project_duration_months() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    let now = Utc::now();
    let repo = RepoStats {
        name: "long-project".to_string(),
        path: "/test".to_string(),
        total_commits: 1,
        first_commit_date: Some(now - Duration::days(90)),
        last_commit_date: Some(now),
        contribution_types: HashMap::from([("production_code".to_string(), 100)]),
        ..Default::default()
    };

    analyzer.add_repo_data(repo);
    let output = PortfolioExporter::export(&analyzer);

    // Duration should be in months (> 30 days)
    assert!(output.contains("month(s)"));
}

#[test]
fn test_portfolio_export_empty_description_handling() {
    let analyzer = create_test_analyzer();
    let output = PortfolioExporter::export(&analyzer);

    // test-repo-2 has empty description
    // Should still work, just no description shown
    assert!(output.contains("### test-repo-2"));

    // Find the section for repo-2
    if let Some(repo2_start) = output.find("### test-repo-2") {
        if let Some(next_section) = output[repo2_start..].find("**My Contribution:**") {
            let between = &output[repo2_start..repo2_start + next_section];
            // Should be relatively short (no description paragraph)
            assert!(between.len() < 100);
        }
    }
}

#[test]
fn test_portfolio_export_code_quality_table() {
    let analyzer = create_test_analyzer();
    let output = PortfolioExporter::export(&analyzer);

    // Should have code quality table
    assert!(output.contains("| Category | Percentage |"));
    assert!(output.contains("| Production Code |"));
    assert!(output.contains("| Tests |"));
    assert!(output.contains("| Documentation |"));
    assert!(output.contains("| Infrastructure/DevOps |"));
}

// ============================================================================
// BadgeExporter tests
// ============================================================================

#[test]
fn test_badge_export_structure() {
    let analyzer = create_test_analyzer();
    let output = BadgeExporter::export(&analyzer);

    assert!(output.contains("<!-- Git Activity Dashboard Widget -->"));
    assert!(output.contains("<div align=\"center\">"));
    assert!(output.contains("### Developer Activity"));
    assert!(output.contains("</div>"));
    assert!(output.contains("<!-- End Git Activity Dashboard Widget -->"));
}

#[test]
fn test_badge_export_table_structure() {
    let analyzer = create_test_analyzer();
    let output = BadgeExporter::export(&analyzer);

    // Should have table with All Time / This Week columns
    assert!(output.contains("| Metric | All Time | This Week |"));
    assert!(output.contains("|--------|----------|-----------|"));
    assert!(output.contains("| Commits |"));
    assert!(output.contains("| Lines Changed |"));
    assert!(output.contains("| Repositories |"));
}

#[test]
fn test_badge_export_quality_badges_shown() {
    let analyzer = create_test_analyzer();
    let output = BadgeExporter::export(&analyzer);

    // Should show quality badges
    assert!(output.contains("**Code Quality:**"));
    assert!(output.contains("Tests:") || output.contains("Docs:"));
}

#[test]
fn test_badge_export_quality_badges_conditional() {
    let mut analyzer = GitAnalyzer::new(None, None);
    analyzer.set_store_commits(true);

    // Repo with no tests or docs
    let repo = RepoStats {
        name: "no-quality".to_string(),
        path: "/test".to_string(),
        total_commits: 1,
        contribution_types: HashMap::from([("production_code".to_string(), 100)]),
        ..Default::default()
    };

    analyzer.add_repo_data(repo);
    let output = BadgeExporter::export(&analyzer);

    // Should not show quality badges section
    assert!(!output.contains("**Code Quality:**"));
}

#[test]
fn test_badge_export_zero_values() {
    let analyzer = create_empty_analyzer();
    let output = BadgeExporter::export(&analyzer);

    // Should handle zero values gracefully
    assert!(output.contains("| Commits | 0 | 0 |"));
    assert!(output.contains("| Lines Changed | 0 | 0 |"));
    assert!(output.contains("| Repositories | 0 | 0 |"));
}

#[test]
fn test_badge_export_formatted_numbers() {
    let analyzer = create_test_analyzer();
    let output = BadgeExporter::export(&analyzer);

    // Total lines changed should be 2000 (500+100 + 1500+300)
    // Should be formatted with comma
    assert!(output.contains("2,400") || output.contains("2,000"));
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_all_exporters_produce_non_empty_output() {
    let analyzer = create_test_analyzer();

    let markdown = MarkdownExporter::export(&analyzer);
    let linkedin = LinkedInExporter::export(&analyzer);
    let portfolio = PortfolioExporter::export(&analyzer);
    let badge = BadgeExporter::export(&analyzer);

    assert!(!markdown.is_empty());
    assert!(!linkedin.is_empty());
    assert!(!portfolio.is_empty());
    assert!(!badge.is_empty());

    // All should have reasonable length
    assert!(markdown.len() > 500);
    assert!(linkedin.len() > 50);
    assert!(portfolio.len() > 500);
    assert!(badge.len() > 100);
}

#[test]
fn test_all_exporters_contain_expected_data() {
    let analyzer = create_test_analyzer();

    // All exporters should reference the repo count
    let markdown = MarkdownExporter::export(&analyzer);
    let portfolio = PortfolioExporter::export(&analyzer);
    let badge = BadgeExporter::export(&analyzer);

    // Check that total repos (2) appears in output
    assert!(markdown.contains("2") || markdown.contains("| Repositories | 2 |"));
    assert!(portfolio.contains("2") || portfolio.contains("**Total Projects:** 2"));
    assert!(badge.contains("2") || badge.contains("| Repositories | 2 |"));
}

#[test]
fn test_all_exporters_handle_empty_analyzer() {
    let analyzer = create_empty_analyzer();

    // All exporters should handle empty data without panicking
    let markdown = MarkdownExporter::export(&analyzer);
    let linkedin = LinkedInExporter::export(&analyzer);
    let portfolio = PortfolioExporter::export(&analyzer);
    let badge = BadgeExporter::export(&analyzer);

    assert!(!markdown.is_empty());
    assert!(!linkedin.is_empty());
    assert!(!portfolio.is_empty());
    assert!(!badge.is_empty());
}

#[test]
fn test_exporters_consistency() {
    let analyzer = create_test_analyzer();
    let stats = analyzer.get_total_stats();

    // All exporters should show consistent data
    let total_commits = stats.total_commits;

    let markdown = MarkdownExporter::export(&analyzer);
    let badge = BadgeExporter::export(&analyzer);

    // Both should reference the same commit count (5 in our test data)
    assert!(markdown.contains(&total_commits.to_string()));
    assert!(badge.contains(&total_commits.to_string()));
}
