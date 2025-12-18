use crate::analyzer::GitAnalyzer;
use crate::traits::{Analytics, Exporter};
use crate::utils::{format_number_full, contribution_type_label, sort_by_value_ref};
use chrono::Utc;

/// Format number with thousands separator (delegated to utils)
fn fmt_num(n: u32) -> String {
    format_number_full(n)
}

/// Get human-readable label for contribution type (delegated to utils)
fn type_label(ctype: &str) -> &'static str {
    contribution_type_label(ctype)
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
            let label = type_label(ctype);
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
                    lines.push(format!("- {}: {:.1}%", type_label(ctype), pct));
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
}

// Implement Exporter trait for MarkdownExporter
impl Exporter for MarkdownExporter {
    fn export(&self, analytics: &dyn Analytics) -> String {
        // Create a temporary GitAnalyzer-like wrapper or use the analytics directly
        // For now, we'll keep the static method for backward compatibility
        // and implement this as a wrapper
        let stats = analytics.total_stats();
        let _repos = analytics.repos(); // Available for future use

        let mut lines = Vec::new();
        lines.push("# Git Activity Dashboard".to_string());
        lines.push(String::new());
        lines.push(format!("*Generated on {}*", Utc::now().format("%Y-%m-%d %H:%M")));
        lines.push(String::new());
        lines.push("## Overview".to_string());
        lines.push(String::new());
        lines.push("| Metric | Value |".to_string());
        lines.push("|--------|-------|".to_string());
        lines.push(format!("| Repositories | {} |", stats.total_repos));
        lines.push(format!("| Total Commits | {} |", fmt_num(stats.total_commits)));
        lines.push(format!("| Lines Added | {} |", fmt_num(stats.total_lines_added)));
        lines.push(format!("| Lines Removed | {} |", fmt_num(stats.total_lines_removed)));
        lines.push(String::new());

        // Contribution breakdown
        lines.push("## Contribution Breakdown".to_string());
        lines.push(String::new());
        for (ctype, count) in sort_by_value_ref(&stats.contribution_types).iter().take(10) {
            let pct = stats.contribution_percentages.get(*ctype).unwrap_or(&0.0);
            lines.push(format!("- **{}**: {} lines ({:.1}%)", type_label(ctype), fmt_num(**count), pct));
        }
        lines.push(String::new());

        // Languages
        lines.push("## Languages".to_string());
        lines.push(String::new());
        for (lang, _) in sort_by_value_ref(&stats.languages).iter().take(8) {
            let pct = stats.language_percentages.get(*lang).unwrap_or(&0.0);
            lines.push(format!("- **{}**: {:.1}%", lang, pct));
        }

        lines.join("\n")
    }

    fn name(&self) -> &'static str {
        "markdown"
    }

    fn extension(&self) -> &'static str {
        "md"
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

// Implement Exporter trait for LinkedInExporter
impl Exporter for LinkedInExporter {
    fn export(&self, analytics: &dyn Analytics) -> String {
        let stats = analytics.total_stats();
        let weekly = analytics.weekly_activity(1);

        let mut lines = Vec::new();
        lines.push("My Developer Activity".to_string());
        lines.push(String::new());

        if let Some(week) = weekly.first() {
            lines.push(format!("This week: {} commits, {} lines changed",
                week.commits, week.lines_added + week.lines_removed));
        }

        lines.push(String::new());
        lines.push(format!("Total: {} commits across {} repos",
            stats.total_commits, stats.total_repos));
        lines.push(String::new());
        lines.push("#coding #developer #programming".to_string());

        lines.join("\n")
    }

    fn name(&self) -> &'static str {
        "linkedin"
    }

    fn extension(&self) -> &'static str {
        "txt"
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

// Implement Exporter trait for PortfolioExporter
impl Exporter for PortfolioExporter {
    fn export(&self, analytics: &dyn Analytics) -> String {
        let stats = analytics.total_stats();

        let mut lines = Vec::new();
        lines.push("# Project Portfolio".to_string());
        lines.push(String::new());
        lines.push(format!("**Total Projects:** {}", stats.total_repos));
        lines.push(format!("**Total Commits:** {}", fmt_num(stats.total_commits)));
        lines.push(format!("**Lines of Code:** {}", fmt_num(stats.total_lines_added)));
        lines.push(String::new());

        // Skills
        lines.push("## Technical Skills".to_string());
        for (lang, _) in sort_by_value_ref(&stats.languages).iter().take(8) {
            let pct = stats.language_percentages.get(*lang).unwrap_or(&0.0);
            lines.push(format!("- **{}**: {:.1}%", lang, pct));
        }

        lines.join("\n")
    }

    fn name(&self) -> &'static str {
        "portfolio"
    }

    fn extension(&self) -> &'static str {
        "md"
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

// Implement Exporter trait for BadgeExporter
impl Exporter for BadgeExporter {
    fn export(&self, analytics: &dyn Analytics) -> String {
        let stats = analytics.total_stats();

        let mut lines = Vec::new();
        lines.push("<!-- Git Activity Dashboard Badge -->".to_string());
        lines.push("<div align=\"center\">".to_string());
        lines.push(String::new());
        lines.push(format!("**{} commits** | **{} lines** | **{} repos**",
            fmt_num(stats.total_commits),
            fmt_num(stats.total_lines_added),
            stats.total_repos));
        lines.push(String::new());
        lines.push("</div>".to_string());

        lines.join("\n")
    }

    fn name(&self) -> &'static str {
        "badge"
    }

    fn extension(&self) -> &'static str {
        "md"
    }
}

#[cfg(test)]
#[path = "tests/exporters_tests.rs"]
mod tests;
