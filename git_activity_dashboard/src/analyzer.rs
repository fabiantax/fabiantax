use crate::classifier::{FileClassification, FileClassifier};
use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: DateTime<Utc>,
    pub message: String,
    pub files_changed: u32,
    pub lines_added: u32,
    pub lines_removed: u32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub file_classifications: Vec<FileClassification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStats {
    pub name: String,
    pub path: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub total_commits: u32,
    pub total_lines_added: u32,
    pub total_lines_removed: u32,
    pub total_files_changed: u32,
    pub first_commit_date: Option<DateTime<Utc>>,
    pub last_commit_date: Option<DateTime<Utc>>,
    pub languages: HashMap<String, u32>,
    pub contribution_types: HashMap<String, u32>,
    pub file_extensions: HashMap<String, u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub commits: Vec<CommitInfo>,
}

impl Default for RepoStats {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            description: String::new(),
            technologies: Vec::new(),
            total_commits: 0,
            total_lines_added: 0,
            total_lines_removed: 0,
            total_files_changed: 0,
            first_commit_date: None,
            last_commit_date: None,
            languages: HashMap::new(),
            contribution_types: HashMap::new(),
            file_extensions: HashMap::new(),
            commits: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySummary {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub period_label: String,
    pub commits: u32,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub files_changed: u32,
    pub repos_active: u32,
    pub contribution_breakdown: HashMap<String, u32>,
    pub language_breakdown: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalStats {
    pub total_repos: u32,
    pub total_commits: u32,
    pub total_lines_added: u32,
    pub total_lines_removed: u32,
    pub total_lines_changed: u32,
    pub total_files_changed: u32,
    pub languages: HashMap<String, u32>,
    pub language_percentages: HashMap<String, f64>,
    pub contribution_types: HashMap<String, u32>,
    pub contribution_percentages: HashMap<String, f64>,
    pub file_extensions: HashMap<String, u32>,
    pub file_extension_percentages: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub generated_at: DateTime<Utc>,
    pub summary: TotalStats,
    pub repositories: Vec<RepoStats>,
    pub daily_activity: Vec<ActivitySummary>,
    pub weekly_activity: Vec<ActivitySummary>,
    pub monthly_activity: Vec<ActivitySummary>,
}

pub struct GitAnalyzer {
    pub author_email: Option<String>,
    pub author_name: Option<String>,
    classifier: FileClassifier,
    repos: Vec<RepoStats>,
}

impl GitAnalyzer {
    pub fn new(author_email: Option<String>, author_name: Option<String>) -> Self {
        Self {
            author_email,
            author_name,
            classifier: FileClassifier::new(),
            repos: Vec::new(),
        }
    }

    /// Add pre-parsed repository data (used by WASM when git operations happen in JS)
    pub fn add_repo_data(&mut self, stats: RepoStats) {
        self.repos.push(stats);
    }

    /// Parse raw git log output and add to repos
    pub fn parse_git_log(&mut self, repo_name: &str, repo_path: &str, log_output: &str) -> RepoStats {
        let mut stats = RepoStats {
            name: repo_name.to_string(),
            path: repo_path.to_string(),
            ..Default::default()
        };

        let mut current_commit: Option<CommitInfo> = None;
        let mut languages: HashMap<String, u32> = HashMap::new();
        let mut contribution_types: HashMap<String, u32> = HashMap::new();
        let mut file_extensions: HashMap<String, u32> = HashMap::new();

        for line in log_output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Check if this is a commit line (format: hash|author|email|date|message)
            if line.contains('|') && line.matches('|').count() >= 4 {
                let parts: Vec<&str> = line.splitn(5, '|').collect();
                if parts.len() == 5 {
                    // Save previous commit
                    if let Some(commit) = current_commit.take() {
                        stats.commits.push(commit);
                    }

                    let date = DateTime::parse_from_rfc3339(parts[3])
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());

                    current_commit = Some(CommitInfo {
                        hash: parts[0].to_string(),
                        author: parts[1].to_string(),
                        email: parts[2].to_string(),
                        date,
                        message: parts[4].to_string(),
                        files_changed: 0,
                        lines_added: 0,
                        lines_removed: 0,
                        file_classifications: Vec::new(),
                    });
                    continue;
                }
            }

            // Check if this is a numstat line (additions\tdeletions\tfilename)
            if let Some(ref mut commit) = current_commit {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 3 {
                    let added: u32 = parts[0].parse().unwrap_or(0);
                    let removed: u32 = parts[1].parse().unwrap_or(0);
                    let filepath = parts[2];

                    let classification = self.classifier.classify(filepath, added, removed);

                    // Track language
                    if let Some(ref lang) = classification.language {
                        *languages.entry(lang.clone()).or_insert(0) += added + removed;
                    }

                    // Track contribution type
                    let type_key = serde_json::to_string(&classification.contribution_type)
                        .unwrap_or_else(|_| "\"other\"".to_string())
                        .trim_matches('"')
                        .to_string();
                    *contribution_types.entry(type_key).or_insert(0) += added + removed;

                    // Track file extension
                    let ext = Self::get_file_extension(filepath);
                    *file_extensions.entry(ext).or_insert(0) += added + removed;

                    commit.lines_added += added;
                    commit.lines_removed += removed;
                    commit.files_changed += 1;
                    commit.file_classifications.push(classification);
                }
            }
        }

        // Don't forget the last commit
        if let Some(commit) = current_commit {
            stats.commits.push(commit);
        }

        // Calculate totals
        stats.total_commits = stats.commits.len() as u32;
        stats.languages = languages;
        stats.contribution_types = contribution_types;
        stats.file_extensions = file_extensions;

        for commit in &stats.commits {
            stats.total_lines_added += commit.lines_added;
            stats.total_lines_removed += commit.lines_removed;
            stats.total_files_changed += commit.files_changed;
        }

        if !stats.commits.is_empty() {
            let dates: Vec<_> = stats.commits.iter().map(|c| c.date).collect();
            stats.first_commit_date = dates.iter().min().copied();
            stats.last_commit_date = dates.iter().max().copied();
        }

        self.repos.push(stats.clone());
        stats
    }

    /// Extract file extension from a path
    fn get_file_extension(filepath: &str) -> String {
        let path = std::path::Path::new(filepath);
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e.to_lowercase()))
            .unwrap_or_else(|| "(no ext)".to_string())
    }

    pub fn get_repos(&self) -> &[RepoStats] {
        &self.repos
    }

    pub fn get_total_stats(&self) -> TotalStats {
        let total_commits: u32 = self.repos.iter().map(|r| r.total_commits).sum();
        let total_lines_added: u32 = self.repos.iter().map(|r| r.total_lines_added).sum();
        let total_lines_removed: u32 = self.repos.iter().map(|r| r.total_lines_removed).sum();
        let total_files_changed: u32 = self.repos.iter().map(|r| r.total_files_changed).sum();

        // Aggregate languages
        let mut all_languages: HashMap<String, u32> = HashMap::new();
        for repo in &self.repos {
            for (lang, count) in &repo.languages {
                *all_languages.entry(lang.clone()).or_insert(0) += count;
            }
        }

        // Aggregate contribution types
        let mut all_contribution_types: HashMap<String, u32> = HashMap::new();
        for repo in &self.repos {
            for (ctype, count) in &repo.contribution_types {
                *all_contribution_types.entry(ctype.clone()).or_insert(0) += count;
            }
        }

        // Aggregate file extensions
        let mut all_file_extensions: HashMap<String, u32> = HashMap::new();
        for repo in &self.repos {
            for (ext, count) in &repo.file_extensions {
                *all_file_extensions.entry(ext.clone()).or_insert(0) += count;
            }
        }

        // Calculate contribution type percentages
        let total_lines: u32 = all_contribution_types.values().sum();
        let mut contribution_percentages: HashMap<String, f64> = HashMap::new();
        if total_lines > 0 {
            for (ctype, count) in &all_contribution_types {
                let pct = (*count as f64 / total_lines as f64) * 100.0;
                contribution_percentages.insert(ctype.clone(), (pct * 10.0).round() / 10.0);
            }
        }

        // Calculate language percentages
        let total_lang_lines: u32 = all_languages.values().sum();
        let mut language_percentages: HashMap<String, f64> = HashMap::new();
        if total_lang_lines > 0 {
            for (lang, count) in &all_languages {
                let pct = (*count as f64 / total_lang_lines as f64) * 100.0;
                language_percentages.insert(lang.clone(), (pct * 10.0).round() / 10.0);
            }
        }

        // Calculate file extension percentages
        let total_ext_lines: u32 = all_file_extensions.values().sum();
        let mut file_extension_percentages: HashMap<String, f64> = HashMap::new();
        if total_ext_lines > 0 {
            for (ext, count) in &all_file_extensions {
                let pct = (*count as f64 / total_ext_lines as f64) * 100.0;
                file_extension_percentages.insert(ext.clone(), (pct * 10.0).round() / 10.0);
            }
        }

        TotalStats {
            total_repos: self.repos.len() as u32,
            total_commits,
            total_lines_added,
            total_lines_removed,
            total_lines_changed: total_lines_added + total_lines_removed,
            total_files_changed,
            languages: all_languages,
            language_percentages,
            contribution_types: all_contribution_types,
            contribution_percentages,
            file_extensions: all_file_extensions,
            file_extension_percentages,
        }
    }

    pub fn get_daily_activity(&self, days: u32) -> Vec<ActivitySummary> {
        let now = Utc::now();
        let mut summaries = Vec::new();

        for i in 0..days {
            let day = now - Duration::days(i as i64);
            let start = day.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
            let end = day.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();

            let mut summary = ActivitySummary {
                period_start: start,
                period_end: end,
                period_label: day.format("%A, %b %d").to_string(),
                commits: 0,
                lines_added: 0,
                lines_removed: 0,
                files_changed: 0,
                repos_active: 0,
                contribution_breakdown: HashMap::new(),
                language_breakdown: HashMap::new(),
            };

            let mut active_repos = std::collections::HashSet::new();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);
                    }
                }
            }

            summary.repos_active = active_repos.len() as u32;
            summaries.push(summary);
        }

        summaries
    }

    pub fn get_weekly_activity(&self, weeks: u32) -> Vec<ActivitySummary> {
        let now = Utc::now();
        let mut summaries = Vec::new();

        for i in 0..weeks {
            let week_start = now - Duration::days((now.weekday().num_days_from_monday() + i * 7) as i64);
            let start = week_start.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
            let end = start + Duration::days(7) - Duration::seconds(1);

            let mut summary = ActivitySummary {
                period_start: start,
                period_end: end,
                period_label: format!("Week of {}", start.format("%b %d")),
                commits: 0,
                lines_added: 0,
                lines_removed: 0,
                files_changed: 0,
                repos_active: 0,
                contribution_breakdown: HashMap::new(),
                language_breakdown: HashMap::new(),
            };

            let mut active_repos = std::collections::HashSet::new();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);
                    }
                }
            }

            summary.repos_active = active_repos.len() as u32;
            summaries.push(summary);
        }

        summaries
    }

    pub fn get_monthly_activity(&self, months: u32) -> Vec<ActivitySummary> {
        let now = Utc::now();
        let mut summaries = Vec::new();

        for i in 0..months {
            // Calculate month start
            let mut year = now.year();
            let mut month = now.month() as i32 - i as i32;

            while month <= 0 {
                month += 12;
                year -= 1;
            }

            let month = month as u32;
            let start = chrono::NaiveDate::from_ymd_opt(year, month, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc();

            // Calculate month end
            let next_month = if month == 12 { 1 } else { month + 1 };
            let next_year = if month == 12 { year + 1 } else { year };
            let end = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc() - Duration::seconds(1);

            let month_name = start.format("%B %Y").to_string();

            let mut summary = ActivitySummary {
                period_start: start,
                period_end: end,
                period_label: month_name,
                commits: 0,
                lines_added: 0,
                lines_removed: 0,
                files_changed: 0,
                repos_active: 0,
                contribution_breakdown: HashMap::new(),
                language_breakdown: HashMap::new(),
            };

            let mut active_repos = std::collections::HashSet::new();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);
                    }
                }
            }

            summary.repos_active = active_repos.len() as u32;
            summaries.push(summary);
        }

        summaries
    }

    pub fn get_dashboard_data(&self) -> DashboardData {
        DashboardData {
            generated_at: Utc::now(),
            summary: self.get_total_stats(),
            repositories: self.repos.clone(),
            daily_activity: self.get_daily_activity(7),
            weekly_activity: self.get_weekly_activity(4),
            monthly_activity: self.get_monthly_activity(6),
        }
    }
}
