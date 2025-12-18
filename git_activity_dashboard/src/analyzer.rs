use crate::classifier::{FileClassification, FileClassifier};
use crate::traits::{Analytics, PeriodStrategy};
use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Null byte delimiter for git log format - safe from commit message injection
pub const GIT_LOG_DELIMITER: char = '\x00';

/// Git log format string using null byte delimiter
/// Use git's %x00 format specifier which outputs actual null bytes
pub const GIT_LOG_FORMAT: &str = "%H%x00%an%x00%ae%x00%aI%x00%s";

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidCommitFormat(String),
    InvalidDate(String),
    EmptyInput,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidCommitFormat(line) => write!(f, "Invalid commit format: {}", line),
            ParseError::InvalidDate(date) => write!(f, "Invalid date format: {}", date),
            ParseError::EmptyInput => write!(f, "Empty git log input"),
        }
    }
}

impl std::error::Error for ParseError {}

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
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub contribution_types: HashMap<String, u32>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub languages: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStats {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub technologies: Vec<String>,
    pub total_commits: u32,
    pub total_lines_added: u32,
    pub total_lines_removed: u32,
    pub total_files_changed: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_commit_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_date: Option<DateTime<Utc>>,
    /// Last known HEAD commit hash (for incremental updates)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_hash: Option<String>,
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
            last_commit_hash: None,
            languages: HashMap::new(),
            contribution_types: HashMap::new(),
            file_extensions: HashMap::new(),
            commits: Vec::new(),
        }
    }
}

impl RepoStats {
    /// Merge new stats into existing (for incremental updates)
    pub fn merge(&mut self, other: &RepoStats) {
        self.total_commits += other.total_commits;
        self.total_lines_added += other.total_lines_added;
        self.total_lines_removed += other.total_lines_removed;
        self.total_files_changed += other.total_files_changed;

        // Update date range
        if let Some(other_first) = other.first_commit_date {
            self.first_commit_date = Some(match self.first_commit_date {
                Some(self_first) => self_first.min(other_first),
                None => other_first,
            });
        }
        if let Some(other_last) = other.last_commit_date {
            self.last_commit_date = Some(match self.last_commit_date {
                Some(self_last) => self_last.max(other_last),
                None => other_last,
            });
        }

        // Update last commit hash
        if other.last_commit_hash.is_some() {
            self.last_commit_hash = other.last_commit_hash.clone();
        }

        // Merge language counts
        for (lang, count) in &other.languages {
            *self.languages.entry(lang.clone()).or_insert(0) += count;
        }

        // Merge contribution types
        for (ctype, count) in &other.contribution_types {
            *self.contribution_types.entry(ctype.clone()).or_insert(0) += count;
        }

        // Merge file extensions
        for (ext, count) in &other.file_extensions {
            *self.file_extensions.entry(ext.clone()).or_insert(0) += count;
        }

        // Merge commits (if stored)
        self.commits.extend(other.commits.iter().cloned());
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
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub contribution_breakdown: HashMap<String, u32>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
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

/// Options for parsing git logs
#[derive(Debug, Clone, Default)]
pub struct ParseOptions {
    /// Whether to store individual commits (memory intensive for large repos)
    pub store_commits: bool,
    /// Use legacy pipe delimiter (for backwards compatibility)
    pub legacy_delimiter: bool,
}

pub struct GitAnalyzer {
    pub author_email: Option<String>,
    pub author_name: Option<String>,
    classifier: FileClassifier,
    repos: Vec<RepoStats>,
    /// Cached total stats (invalidated on repo changes)
    cached_stats: Option<TotalStats>,
    /// Default parse options
    parse_options: ParseOptions,
}

impl GitAnalyzer {
    pub fn new(author_email: Option<String>, author_name: Option<String>) -> Self {
        Self {
            author_email,
            author_name,
            classifier: FileClassifier::new(),
            repos: Vec::new(),
            cached_stats: None,
            parse_options: ParseOptions::default(),
        }
    }

    pub fn with_options(mut self, options: ParseOptions) -> Self {
        self.parse_options = options;
        self
    }

    /// Set whether to store individual commits
    pub fn set_store_commits(&mut self, store: bool) {
        self.parse_options.store_commits = store;
    }

    /// Add pre-parsed repository data (used by WASM when git operations happen in JS)
    pub fn add_repo_data(&mut self, stats: RepoStats) {
        self.repos.push(stats);
        self.cached_stats = None; // Invalidate cache
    }

    /// Parse raw git log output and add to repos
    /// Supports both null-byte delimiter (preferred) and legacy pipe delimiter
    pub fn parse_git_log(&mut self, repo_name: &str, repo_path: &str, log_output: &str) -> Result<RepoStats, ParseError> {
        if log_output.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let mut stats = RepoStats {
            name: repo_name.to_string(),
            path: repo_path.to_string(),
            ..Default::default()
        };

        let mut current_commit: Option<CommitInfo> = None;
        let mut first_hash: Option<String> = None;
        let mut parse_errors: Vec<ParseError> = Vec::new();

        // Detect delimiter: if line contains null byte, use that; otherwise use pipe
        let delimiter = if log_output.contains('\x00') || !self.parse_options.legacy_delimiter {
            '\x00'
        } else {
            '|'
        };

        for line in log_output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Check if this is a commit line
            let is_commit_line = if delimiter == '\x00' {
                line.contains('\x00')
            } else {
                line.contains('|') && line.matches('|').count() >= 4
            };

            if is_commit_line {
                let parts: Vec<&str> = line.splitn(5, delimiter).collect();
                if parts.len() == 5 {
                    // Save previous commit
                    if let Some(commit) = current_commit.take() {
                        if self.parse_options.store_commits {
                            stats.commits.push(commit);
                        }
                    }

                    let date = DateTime::parse_from_rfc3339(parts[3])
                        .map(|d| d.with_timezone(&Utc))
                        .map_err(|_| ParseError::InvalidDate(parts[3].to_string()));

                    match date {
                        Ok(date) => {
                            if first_hash.is_none() {
                                first_hash = Some(parts[0].to_string());
                            }

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
                                contribution_types: HashMap::new(),
                                languages: HashMap::new(),
                            });
                        }
                        Err(e) => {
                            parse_errors.push(e);
                        }
                    }
                    continue;
                }
            }

            // Check if this is a numstat line (additions\tdeletions\tfilename)
            if let Some(ref mut commit) = current_commit {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 3 {
                    // Binary files show "-" for added/removed
                    let added: u32 = parts[0].parse().unwrap_or(0);
                    let removed: u32 = parts[1].parse().unwrap_or(0);
                    let filepath = parts[2];

                    let classification = self.classifier.classify(filepath, added, removed);

                    // Track language
                    if let Some(ref lang) = classification.language {
                        *stats.languages.entry(lang.clone()).or_insert(0) += added + removed;
                    }

                    // Track contribution type
                    let type_key = format!("{:?}", classification.contribution_type).to_lowercase();
                    *stats.contribution_types.entry(type_key).or_insert(0) += added + removed;

                    // Track file extension
                    let ext = Self::get_file_extension(filepath);
                    *stats.file_extensions.entry(ext).or_insert(0) += added + removed;

                    commit.lines_added += added;
                    commit.lines_removed += removed;
                    commit.files_changed += 1;

                    // Update totals inline (avoid second iteration)
                    stats.total_lines_added += added;
                    stats.total_lines_removed += removed;
                    stats.total_files_changed += 1;

                    if self.parse_options.store_commits {
                        commit.file_classifications.push(classification);
                    }
                }
            }
        }

        // Don't forget the last commit
        if let Some(commit) = current_commit {
            // Update date range from last commit
            if stats.first_commit_date.is_none() || commit.date < stats.first_commit_date.unwrap() {
                stats.first_commit_date = Some(commit.date);
            }
            if stats.last_commit_date.is_none() || commit.date > stats.last_commit_date.unwrap() {
                stats.last_commit_date = Some(commit.date);
            }

            stats.total_commits += 1;

            if self.parse_options.store_commits {
                stats.commits.push(commit);
            }
        }

        // Count commits we processed
        stats.total_commits = if self.parse_options.store_commits {
            stats.commits.len() as u32
        } else {
            // Count from first_hash presence
            log_output.lines()
                .filter(|l| l.contains(delimiter) || (delimiter == '|' && l.matches('|').count() >= 4))
                .count() as u32
        };

        // Update date range from stored commits if available
        if self.parse_options.store_commits && !stats.commits.is_empty() {
            let dates: Vec<_> = stats.commits.iter().map(|c| c.date).collect();
            stats.first_commit_date = dates.iter().min().copied();
            stats.last_commit_date = dates.iter().max().copied();
        }

        // Store the latest commit hash for incremental updates
        stats.last_commit_hash = first_hash;

        self.repos.push(stats.clone());
        self.cached_stats = None; // Invalidate cache

        Ok(stats)
    }

    /// Parse git log with legacy pipe delimiter (backwards compatibility)
    pub fn parse_git_log_legacy(&mut self, repo_name: &str, repo_path: &str, log_output: &str) -> Result<RepoStats, ParseError> {
        let old_legacy = self.parse_options.legacy_delimiter;
        self.parse_options.legacy_delimiter = true;
        let result = self.parse_git_log(repo_name, repo_path, log_output);
        self.parse_options.legacy_delimiter = old_legacy;
        result
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

    /// Get a mutable reference to repos (for incremental updates)
    pub fn get_repos_mut(&mut self) -> &mut Vec<RepoStats> {
        self.cached_stats = None;
        &mut self.repos
    }

    /// Find a repo by name
    pub fn find_repo(&self, name: &str) -> Option<&RepoStats> {
        self.repos.iter().find(|r| r.name == name)
    }

    /// Find a repo by name (mutable)
    pub fn find_repo_mut(&mut self, name: &str) -> Option<&mut RepoStats> {
        self.cached_stats = None;
        self.repos.iter_mut().find(|r| r.name == name)
    }

    pub fn get_total_stats(&self) -> TotalStats {
        // Return cached stats if available
        if let Some(ref cached) = self.cached_stats {
            return cached.clone();
        }

        self.compute_total_stats()
    }

    fn compute_total_stats(&self) -> TotalStats {
        let total_commits: u32 = self.repos.iter().map(|r| r.total_commits).sum();
        let total_lines_added: u32 = self.repos.iter().map(|r| r.total_lines_added).sum();
        let total_lines_removed: u32 = self.repos.iter().map(|r| r.total_lines_removed).sum();
        let total_files_changed: u32 = self.repos.iter().map(|r| r.total_files_changed).sum();

        // Pre-allocate with estimated capacity
        let est_capacity = self.repos.len() * 5;
        let mut all_languages: HashMap<String, u32> = HashMap::with_capacity(est_capacity);
        let mut all_contribution_types: HashMap<String, u32> = HashMap::with_capacity(8);
        let mut all_file_extensions: HashMap<String, u32> = HashMap::with_capacity(est_capacity);

        for repo in &self.repos {
            for (lang, count) in &repo.languages {
                *all_languages.entry(lang.clone()).or_insert(0) += count;
            }
            for (ctype, count) in &repo.contribution_types {
                *all_contribution_types.entry(ctype.clone()).or_insert(0) += count;
            }
            for (ext, count) in &repo.file_extensions {
                *all_file_extensions.entry(ext.clone()).or_insert(0) += count;
            }
        }

        // Calculate percentages
        let contribution_percentages = Self::calculate_percentages(&all_contribution_types);
        let language_percentages = Self::calculate_percentages(&all_languages);
        let file_extension_percentages = Self::calculate_percentages(&all_file_extensions);

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

    fn calculate_percentages(map: &HashMap<String, u32>) -> HashMap<String, f64> {
        let total: u32 = map.values().sum();
        if total == 0 {
            return HashMap::new();
        }

        map.iter()
            .map(|(k, v)| {
                let pct = (*v as f64 / total as f64) * 100.0;
                (k.clone(), (pct * 10.0).round() / 10.0)
            })
            .collect()
    }

    /// Cache the total stats (call after all repos are added)
    pub fn cache_stats(&mut self) {
        self.cached_stats = Some(self.compute_total_stats());
    }

    pub fn get_daily_activity(&self, days: u32) -> Vec<ActivitySummary> {
        let now = Utc::now();
        let mut summaries = Vec::with_capacity(days as usize);
        let mut active_repos: HashSet<&String> = HashSet::new();

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

            active_repos.clear();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);

                        // Aggregate contribution types for this period
                        for (ctype, lines) in &commit.contribution_types {
                            *summary.contribution_breakdown.entry(ctype.clone()).or_insert(0) += lines;
                        }

                        // Aggregate languages for this period
                        for (lang, lines) in &commit.languages {
                            *summary.language_breakdown.entry(lang.clone()).or_insert(0) += lines;
                        }
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
        let mut summaries = Vec::with_capacity(weeks as usize);
        let mut active_repos: HashSet<&String> = HashSet::new();

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

            active_repos.clear();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);

                        // Aggregate contribution types for this period
                        for (ctype, lines) in &commit.contribution_types {
                            *summary.contribution_breakdown.entry(ctype.clone()).or_insert(0) += lines;
                        }

                        // Aggregate languages for this period
                        for (lang, lines) in &commit.languages {
                            *summary.language_breakdown.entry(lang.clone()).or_insert(0) += lines;
                        }
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
        let mut summaries = Vec::with_capacity(months as usize);
        let mut active_repos: HashSet<&String> = HashSet::new();

        for i in 0..months {
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

            active_repos.clear();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);

                        // Aggregate contribution types for this period
                        for (ctype, lines) in &commit.contribution_types {
                            *summary.contribution_breakdown.entry(ctype.clone()).or_insert(0) += lines;
                        }

                        // Aggregate languages for this period
                        for (lang, lines) in &commit.languages {
                            *summary.language_breakdown.entry(lang.clone()).or_insert(0) += lines;
                        }
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

    /// Generic activity aggregation using a PeriodStrategy
    /// This enables extensible time-based grouping (Open/Closed Principle)
    pub fn aggregate_activity<P: PeriodStrategy>(&self, strategy: &P, count: u32) -> Vec<ActivitySummary> {
        let mut summaries = Vec::with_capacity(count as usize);
        let mut active_repos: HashSet<&String> = HashSet::new();

        for i in 0..count {
            let (start, end) = strategy.boundaries(i);

            let mut summary = ActivitySummary {
                period_start: start,
                period_end: end,
                period_label: strategy.label(i),
                commits: 0,
                lines_added: 0,
                lines_removed: 0,
                files_changed: 0,
                repos_active: 0,
                contribution_breakdown: HashMap::new(),
                language_breakdown: HashMap::new(),
            };

            active_repos.clear();

            for repo in &self.repos {
                for commit in &repo.commits {
                    if commit.date >= start && commit.date <= end {
                        summary.commits += 1;
                        summary.lines_added += commit.lines_added;
                        summary.lines_removed += commit.lines_removed;
                        summary.files_changed += commit.files_changed;
                        active_repos.insert(&repo.name);

                        // Aggregate contribution types for this period
                        for (ctype, lines) in &commit.contribution_types {
                            *summary.contribution_breakdown.entry(ctype.clone()).or_insert(0) += lines;
                        }

                        // Aggregate languages for this period
                        for (lang, lines) in &commit.languages {
                            *summary.language_breakdown.entry(lang.clone()).or_insert(0) += lines;
                        }
                    }
                }
            }

            summary.repos_active = active_repos.len() as u32;
            summaries.push(summary);
        }

        summaries
    }
}

// Implement the Analytics trait for GitAnalyzer (Dependency Inversion Principle)
impl Analytics for GitAnalyzer {
    fn total_stats(&self) -> TotalStats {
        self.get_total_stats()
    }

    fn repos(&self) -> &[RepoStats] {
        self.get_repos()
    }

    fn daily_activity(&self, days: u32) -> Vec<ActivitySummary> {
        self.get_daily_activity(days)
    }

    fn weekly_activity(&self, weeks: u32) -> Vec<ActivitySummary> {
        self.get_weekly_activity(weeks)
    }

    fn monthly_activity(&self, months: u32) -> Vec<ActivitySummary> {
        self.get_monthly_activity(months)
    }
}

/// Returns the recommended git log command for a repo
pub fn git_log_command(since_hash: Option<&str>) -> String {
    let range = match since_hash {
        Some(hash) => format!("{}..HEAD", hash),
        None => String::new(),
    };
    format!(
        "git log {} --format='{}' --numstat",
        range, GIT_LOG_FORMAT
    )
}

#[cfg(test)]
#[path = "tests/analyzer_tests.rs"]
mod tests;
