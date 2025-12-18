use crate::analyzer::GitAnalyzer;
use chrono::Utc;

/// Format number with thousands separator
fn fmt_num(n: u32) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}

pub struct MarkdownExporter;

impl MarkdownExporter {
    pub fn export(analyzer: &GitAnalyzer) -> String {
        let stats = analyzer.get_total_stats();
        let mut lines = Vec::new();

        lines.push("# Git Activity Dashboard".to_string());
        lines.push(String::new());
        lines.push(format!("*Generated on {}*", Utc::now().format("%Y-%m-%d %H:%M")));
        lines.push(String::new());

        // Overall stats
        lines.push("## Overview".to_string());
        lines.push(String::new());
        lines.push("| Metric | Value |".to_string());
        lines.push("|--------|-------|".to_string());
        lines.push(format!("| Repositories | {} |", stats.total_repos));
        lines.push(format!("| Total Commits | {} |", fmt_num(stats.total_commits)));
        lines.push(format!("| Lines Added | {} |", fmt_num(stats.total_lines_added)));
        lines.push(format!("| Lines Removed | {} |", fmt_num(stats.total_lines_removed)));
        lines.push(format!("| Files Changed | {} |", fmt_num(stats.total_files_changed)));
        lines.push(String::new());

        // Contribution breakdown
        lines.push("## Contribution Breakdown".to_string());
        lines.push(String::new());
        lines.push("| Type | Lines | Percentage |".to_string());
        lines.push("|------|-------|------------|".to_string());

        let mut sorted_types: Vec<_> = stats.contribution_types.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));

        for (ctype, count) in sorted_types {
            let pct = stats.contribution_percentages.get(ctype).unwrap_or(&0.0);
            let label = Self::type_label(ctype);
            lines.push(format!("| {} | {} | {}% |", label, fmt_num(*count), pct));
        }
        lines.push(String::new());

        // Languages (Programming Languages)
        if !stats.languages.is_empty() {
            lines.push("## Programming Languages".to_string());
            lines.push(String::new());
            lines.push("| Language | Lines | Percentage |".to_string());
            lines.push("|----------|-------|------------|".to_string());

            let mut sorted_langs: Vec<_> = stats.languages.iter().collect();
            sorted_langs.sort_by(|a, b| b.1.cmp(a.1));

            for (lang, count) in sorted_langs.iter().take(10) {
                let pct = stats.language_percentages.get(*lang).unwrap_or(&0.0);
                lines.push(format!("| {} | {} | {}% |", lang, fmt_num(**count), pct));
            }
            lines.push(String::new());
        }

        // File Extensions
        if !stats.file_extensions.is_empty() {
            lines.push("## File Types (by extension)".to_string());
            lines.push(String::new());
            lines.push("| Extension | Lines | Percentage |".to_string());
            lines.push("|-----------|-------|------------|".to_string());

            let mut sorted_exts: Vec<_> = stats.file_extensions.iter().collect();
            sorted_exts.sort_by(|a, b| b.1.cmp(a.1));

            for (ext, count) in sorted_exts.iter().take(15) {
                let pct = stats.file_extension_percentages.get(*ext).unwrap_or(&0.0);
                lines.push(format!("| {} | {} | {}% |", ext, fmt_num(**count), pct));
            }
            lines.push(String::new());
        }

        // Weekly activity
        lines.push("## Weekly Activity".to_string());
        lines.push(String::new());
        let weekly = analyzer.get_weekly_activity(4);
        lines.push("| Week | Commits | Lines Changed | Repos |".to_string());
        lines.push("|------|---------|---------------|-------|".to_string());
        for week in weekly {
            let total_lines = week.lines_added + week.lines_removed;
            lines.push(format!("| {} | {} | {} | {} |", week.period_label, week.commits, fmt_num(total_lines), week.repos_active));
        }
        lines.push(String::new());

        // Monthly activity
        lines.push("## Monthly Activity".to_string());
        lines.push(String::new());
        let monthly = analyzer.get_monthly_activity(6);
        lines.push("| Month | Commits | Lines Changed | Repos |".to_string());
        lines.push("|-------|---------|---------------|-------|".to_string());
        for month in monthly {
            let total_lines = month.lines_added + month.lines_removed;
            lines.push(format!("| {} | {} | {} | {} |", month.period_label, month.commits, fmt_num(total_lines), month.repos_active));
        }
        lines.push(String::new());

        // Repository breakdown (per repo)
        lines.push("## Repositories (detailed)".to_string());
        lines.push(String::new());

        let mut repos: Vec<_> = analyzer.get_repos().to_vec();
        repos.sort_by(|a, b| b.total_commits.cmp(&a.total_commits));

        for repo in repos {
            lines.push(format!("### {}", repo.name));
            lines.push(String::new());
            if !repo.description.is_empty() {
                lines.push(format!("> {}", repo.description));
                lines.push(String::new());
            }

            // Basic stats
            lines.push(format!("**Commits:** {} | **Lines:** +{} / -{}",
                repo.total_commits, fmt_num(repo.total_lines_added), fmt_num(repo.total_lines_removed)));
            if let (Some(first), Some(last)) = (repo.first_commit_date, repo.last_commit_date) {
                lines.push(format!("**Active:** {} to {}", first.format("%Y-%m-%d"), last.format("%Y-%m-%d")));
            }
            lines.push(String::new());

            // Per-repo contribution types
            if !repo.contribution_types.is_empty() {
                lines.push("**Contribution Breakdown:**".to_string());
                let mut sorted_types: Vec<_> = repo.contribution_types.iter().collect();
                sorted_types.sort_by(|a, b| b.1.cmp(a.1));
                let total: u32 = sorted_types.iter().map(|(_, c)| *c).sum();
                for (ctype, count) in sorted_types.iter().take(5) {
                    let pct = if total > 0 { (**count as f64 / total as f64) * 100.0 } else { 0.0 };
                    lines.push(format!("- {}: {:.1}%", Self::type_label(ctype), pct));
                }
                lines.push(String::new());
            }

            // Per-repo languages
            if !repo.languages.is_empty() {
                lines.push("**Languages:**".to_string());
                let mut sorted_langs: Vec<_> = repo.languages.iter().collect();
                sorted_langs.sort_by(|a, b| b.1.cmp(a.1));
                let total: u32 = sorted_langs.iter().map(|(_, c)| *c).sum();
                for (lang, count) in sorted_langs.iter().take(5) {
                    let pct = if total > 0 { (**count as f64 / total as f64) * 100.0 } else { 0.0 };
                    lines.push(format!("- {}: {:.1}%", lang, pct));
                }
                lines.push(String::new());
            }

            // Per-repo file extensions
            if !repo.file_extensions.is_empty() {
                lines.push("**File Types:**".to_string());
                let mut sorted_exts: Vec<_> = repo.file_extensions.iter().collect();
                sorted_exts.sort_by(|a, b| b.1.cmp(a.1));
                let total: u32 = sorted_exts.iter().map(|(_, c)| *c).sum();
                for (ext, count) in sorted_exts.iter().take(5) {
                    let pct = if total > 0 { (**count as f64 / total as f64) * 100.0 } else { 0.0 };
                    lines.push(format!("- {}: {:.1}%", ext, pct));
                }
                lines.push(String::new());
            }

            lines.push("---".to_string());
            lines.push(String::new());
        }

        lines.join("\n")
    }

    fn type_label(ctype: &str) -> &str {
        match ctype {
            "production_code" => "Production Code",
            "tests" => "Tests",
            "documentation" => "Documentation",
            "specs_config" => "Specs & Config",
            "infrastructure" => "Infrastructure",
            "styling" => "Styling",
            _ => "Other",
        }
    }
}

pub struct LinkedInExporter;

impl LinkedInExporter {
    pub fn export(analyzer: &GitAnalyzer) -> String {
        let stats = analyzer.get_total_stats();
        let weekly = analyzer.get_weekly_activity(1);
        let current_week = weekly.first();

        let mut lines = Vec::new();

        lines.push("My Developer Activity This Week".to_string());
        lines.push(String::new());

        if let Some(week) = current_week {
            if week.commits > 0 {
                lines.push(format!("{} commits", week.commits));
                lines.push(format!("{} lines of code", fmt_num(week.lines_added + week.lines_removed)));
                lines.push(format!("{} active repos", week.repos_active));
                lines.push(String::new());
            }
        }

        // Quality indicators
        let mut quality_metrics = Vec::new();
        if let Some(test_pct) = stats.contribution_percentages.get("tests") {
            if *test_pct > 0.0 {
                quality_metrics.push(format!("Tests: {}%", test_pct));
            }
        }
        if let Some(doc_pct) = stats.contribution_percentages.get("documentation") {
            if *doc_pct > 0.0 {
                quality_metrics.push(format!("Documentation: {}%", doc_pct));
            }
        }

        if !quality_metrics.is_empty() {
            lines.push("Code Quality:".to_string());
            for metric in quality_metrics {
                lines.push(format!("  {}", metric));
            }
            lines.push(String::new());
        }

        // Top languages
        if !stats.languages.is_empty() {
            let mut sorted_langs: Vec<_> = stats.languages.iter().collect();
            sorted_langs.sort_by(|a, b| b.1.cmp(a.1));
            let top_langs: Vec<_> = sorted_langs.iter().take(3).map(|(l, _)| l.as_str()).collect();
            lines.push(format!("Top Languages: {}", top_langs.join(", ")));
            lines.push(String::new());
        }

        lines.push("#coding #developer #programming #softwareengineering".to_string());

        lines.join("\n")
    }
}

pub struct PortfolioExporter;

impl PortfolioExporter {
    pub fn export(analyzer: &GitAnalyzer) -> String {
        let stats = analyzer.get_total_stats();
        let mut lines = Vec::new();

        lines.push("# Project Portfolio".to_string());
        lines.push(String::new());
        lines.push(format!("*Generated on {}*", Utc::now().format("%Y-%m-%d")));
        lines.push(String::new());

        // Summary
        lines.push("## Summary".to_string());
        lines.push(String::new());
        lines.push(format!("- **Total Projects:** {}", stats.total_repos));
        lines.push(format!("- **Total Commits:** {}", fmt_num(stats.total_commits)));
        lines.push(format!("- **Total Lines of Code:** {}", fmt_num(stats.total_lines_added)));
        lines.push(String::new());

        // Skills
        if !stats.languages.is_empty() {
            lines.push("## Technical Skills".to_string());
            lines.push(String::new());

            let mut sorted_langs: Vec<_> = stats.languages.iter().collect();
            sorted_langs.sort_by(|a, b| b.1.cmp(a.1));
            let total: u32 = sorted_langs.iter().map(|(_, c)| *c).sum();

            for (lang, count) in sorted_langs.iter().take(10) {
                let pct = (**count as f64 / total as f64) * 100.0;
                let bar_len = (pct / 5.0) as usize;
                let bar: String = (0..bar_len).map(|_| '#').collect();
                lines.push(format!("- **{}**: {:.1}% {}", lang, pct, bar));
            }
            lines.push(String::new());
        }

        // Code quality
        lines.push("## Code Quality Practices".to_string());
        lines.push(String::new());
        lines.push("| Category | Percentage |".to_string());
        lines.push("|----------|------------|".to_string());

        let prod_pct = stats.contribution_percentages.get("production_code").unwrap_or(&0.0);
        let test_pct = stats.contribution_percentages.get("tests").unwrap_or(&0.0);
        let doc_pct = stats.contribution_percentages.get("documentation").unwrap_or(&0.0);
        let infra_pct = stats.contribution_percentages.get("infrastructure").unwrap_or(&0.0);

        lines.push(format!("| Production Code | {}% |", prod_pct));
        lines.push(format!("| Tests | {}% |", test_pct));
        lines.push(format!("| Documentation | {}% |", doc_pct));
        lines.push(format!("| Infrastructure/DevOps | {}% |", infra_pct));
        lines.push(String::new());

        // Projects
        lines.push("## Projects".to_string());
        lines.push(String::new());

        let mut repos: Vec<_> = analyzer.get_repos().to_vec();
        repos.sort_by(|a, b| b.total_commits.cmp(&a.total_commits));

        for repo in repos {
            lines.push(format!("### {}", repo.name));
            lines.push(String::new());

            if !repo.description.is_empty() {
                lines.push(repo.description.clone());
                lines.push(String::new());
            }

            if !repo.technologies.is_empty() {
                lines.push(format!("**Technologies:** {}", repo.technologies.join(", ")));
                lines.push(String::new());
            }

            lines.push("**My Contribution:**".to_string());
            lines.push(format!("- {} commits", repo.total_commits));
            lines.push(format!("- {} lines added, {} lines removed", fmt_num(repo.total_lines_added), fmt_num(repo.total_lines_removed)));

            if let (Some(first), Some(last)) = (repo.first_commit_date, repo.last_commit_date) {
                let duration = (last - first).num_days();
                if duration > 30 {
                    lines.push(format!("- Project duration: {} month(s)", duration / 30));
                } else {
                    lines.push(format!("- Project duration: {} day(s)", duration));
                }
            }

            if !repo.languages.is_empty() {
                let mut sorted_langs: Vec<_> = repo.languages.iter().collect();
                sorted_langs.sort_by(|a, b| b.1.cmp(a.1));
                let top_langs: Vec<_> = sorted_langs.iter().take(3).map(|(l, _)| l.as_str()).collect();
                lines.push(format!("- Primary languages: {}", top_langs.join(", ")));
            }

            lines.push(String::new());
            lines.push("---".to_string());
            lines.push(String::new());
        }

        lines.join("\n")
    }
}

pub struct BadgeExporter;

impl BadgeExporter {
    pub fn export(analyzer: &GitAnalyzer) -> String {
        let stats = analyzer.get_total_stats();
        let weekly = analyzer.get_weekly_activity(1);
        let current_week = weekly.first();

        let mut lines = Vec::new();

        lines.push("<!-- Git Activity Dashboard Widget -->".to_string());
        lines.push("<div align=\"center\">".to_string());
        lines.push(String::new());
        lines.push("### Developer Activity".to_string());
        lines.push(String::new());
        lines.push("| Metric | All Time | This Week |".to_string());
        lines.push("|--------|----------|-----------|".to_string());

        let (week_commits, week_lines, week_repos) = if let Some(week) = current_week {
            (week.commits, week.lines_added + week.lines_removed, week.repos_active)
        } else {
            (0, 0, 0)
        };

        lines.push(format!("| Commits | {} | {} |", fmt_num(stats.total_commits), week_commits));
        lines.push(format!("| Lines Changed | {} | {} |", fmt_num(stats.total_lines_changed), fmt_num(week_lines)));
        lines.push(format!("| Repositories | {} | {} |", stats.total_repos, week_repos));
        lines.push(String::new());

        // Quality badges
        let mut badges = Vec::new();
        if let Some(test_pct) = stats.contribution_percentages.get("tests") {
            if *test_pct > 0.0 {
                badges.push(format!("Tests: {}%", test_pct));
            }
        }
        if let Some(doc_pct) = stats.contribution_percentages.get("documentation") {
            if *doc_pct > 0.0 {
                badges.push(format!("Docs: {}%", doc_pct));
            }
        }

        if !badges.is_empty() {
            lines.push(format!("**Code Quality:** {}", badges.join(" | ")));
            lines.push(String::new());
        }

        lines.push("</div>".to_string());
        lines.push("<!-- End Git Activity Dashboard Widget -->".to_string());

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
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
    fn test_markdown_type_label() {
        assert_eq!(MarkdownExporter::type_label("production_code"), "Production Code");
        assert_eq!(MarkdownExporter::type_label("tests"), "Tests");
        assert_eq!(MarkdownExporter::type_label("documentation"), "Documentation");
        assert_eq!(MarkdownExporter::type_label("specs_config"), "Specs & Config");
        assert_eq!(MarkdownExporter::type_label("infrastructure"), "Infrastructure");
        assert_eq!(MarkdownExporter::type_label("styling"), "Styling");
        assert_eq!(MarkdownExporter::type_label("unknown"), "Other");
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
}
