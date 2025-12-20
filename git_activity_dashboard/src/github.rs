//! GitHub API integration for fetching and cloning user repositories
//!
//! This module provides functionality to:
//! - List all repositories for an authenticated user or specific username
//! - Clone repositories that don't exist locally
//! - Skip already-cloned repositories for efficiency
//! - Cache API responses for faster subsequent runs

use git2::{Cred, FetchOptions, RemoteCallbacks};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

/// GitHub repository metadata from the API
#[derive(Debug, Deserialize, Clone)]
pub struct GitHubRepo {
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub private: bool,
    pub fork: bool,
    pub archived: bool,
    pub size: u64,
    #[serde(default)]
    pub language: Option<String>,
    pub default_branch: String,
    /// When the repo was last pushed to
    pub pushed_at: Option<String>,
    /// When the repo was last updated
    pub updated_at: Option<String>,
    /// When the repo was created
    pub created_at: Option<String>,
}

/// Language statistics from GitHub API (bytes per language)
pub type LanguageStats = std::collections::HashMap<String, u64>;

/// File extension statistics (count and total size)
pub type FileTypeStats = std::collections::HashMap<String, (u64, u64)>; // (count, bytes)

/// Repository stats fetched from GitHub API (no cloning required)
#[derive(Debug, Clone, Serialize)]
pub struct GitHubRepoStats {
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub fork: bool,
    pub archived: bool,
    pub size_kb: u64,
    pub primary_language: Option<String>,
    /// Bytes of code per language
    pub languages: LanguageStats,
    /// File counts and sizes by extension
    pub file_types: FileTypeStats,
    /// Total file count
    pub file_count: u64,
    /// Estimated lines of code (rough calculation from bytes)
    pub estimated_loc: u64,
}

/// Weekly commit activity for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyCommitActivity {
    /// Unix timestamp of the week start (Sunday)
    pub week: i64,
    /// Total commits for the week
    pub total: u64,
}

/// Weekly stats grouped by different levels
#[derive(Debug, Clone, Serialize)]
pub struct WeeklyStats {
    /// Week start date (ISO format)
    pub week: String,
    /// Total commits
    pub commits: u64,
    /// Lines added (if available)
    pub additions: u64,
    /// Lines deleted (if available)
    pub deletions: u64,
}

/// Weekly stats per repository
#[derive(Debug, Clone, Serialize)]
pub struct WeeklyRepoStats {
    pub week: String,
    pub repo: String,
    pub commits: u64,
    pub additions: u64,
    pub deletions: u64,
    pub file_types: std::collections::HashMap<String, FileTypeWeeklyStats>,
}

/// Weekly stats per file type
#[derive(Debug, Clone, Serialize, Default)]
pub struct FileTypeWeeklyStats {
    pub commits: u64,
    pub additions: u64,
    pub deletions: u64,
}

/// Grouping level for stats
#[derive(Debug, Clone, PartialEq)]
pub enum StatsGrouping {
    Week,
    WeekFileType,
    WeekRepo,
    WeekRepoFileType,
}

impl StatsGrouping {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "week" => Some(Self::Week),
            "week-filetype" | "weekfiletype" | "week-file" => Some(Self::WeekFileType),
            "week-repo" | "weekrepo" => Some(Self::WeekRepo),
            "week-repo-filetype" | "weekrepofiletype" | "week-repo-file" => Some(Self::WeekRepoFileType),
            _ => None,
        }
    }
}

/// Cached commit data for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRepoCommits {
    pub repo: String,
    pub fetched_at: String,
    pub commits: Vec<CachedCommit>,
}

/// Cached commit details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCommit {
    pub sha: String,
    pub week: String,
    pub additions: u64,
    pub deletions: u64,
    pub file_extensions: Vec<String>,
}

/// Cache manager for GitHub API responses
pub struct GitHubCache {
    cache_dir: PathBuf,
}

impl GitHubCache {
    pub fn new() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("git-activity-dashboard");

        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir).ok();

        Self { cache_dir }
    }

    fn cache_path(&self, repo: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", repo.replace('/', "_")))
    }

    pub fn get(&self, repo: &str) -> Option<CachedRepoCommits> {
        let path = self.cache_path(repo);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cached) = serde_json::from_str::<CachedRepoCommits>(&content) {
                    // Check if cache is less than 1 hour old
                    if let Ok(fetched) = chrono::DateTime::parse_from_rfc3339(&cached.fetched_at) {
                        let age = chrono::Utc::now().signed_duration_since(fetched);
                        if age.num_hours() < 1 {
                            return Some(cached);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set(&self, repo: &str, commits: &CachedRepoCommits) {
        let path = self.cache_path(repo);
        if let Ok(json) = serde_json::to_string_pretty(commits) {
            fs::write(path, json).ok();
        }
    }
}

/// Options for GitHub scanning
#[derive(Debug, Clone)]
pub struct GitHubScanOptions {
    /// GitHub username to scan (if None, uses authenticated user)
    pub username: Option<String>,
    /// Directory to clone repos into
    pub clone_dir: PathBuf,
    /// Include forked repositories
    pub include_forks: bool,
    /// Include archived repositories
    pub include_archived: bool,
    /// Include private repositories (requires auth)
    pub include_private: bool,
    /// Skip cloning, only analyze existing repos
    pub skip_clone: bool,
    /// Filter repos with activity since this date
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    /// Filter repos with activity until this date
    pub until: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for GitHubScanOptions {
    fn default() -> Self {
        Self {
            username: None,
            clone_dir: PathBuf::from("./github_repos"),
            include_forks: false,
            include_archived: false,
            include_private: true,
            skip_clone: false,
            since: None,
            until: None,
        }
    }
}

/// Parse a date string into a DateTime
/// Supports: YYYY-MM-DD, month names (november, nov), YYYY-MM, relative (1 month ago)
pub fn parse_date(input: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::{Datelike, NaiveDate, TimeZone, Utc};

    let input = input.trim().to_lowercase();
    let now = Utc::now();

    // Try YYYY-MM-DD format
    if let Ok(date) = NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
        return Some(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0)?));
    }

    // Try YYYY-MM format (first day of month)
    if let Ok(date) = NaiveDate::parse_from_str(&format!("{}-01", input), "%Y-%m-%d") {
        return Some(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0)?));
    }

    // Try month name (current year or previous year if month is in future)
    let month_num = match input.as_str() {
        "january" | "jan" => Some(1),
        "february" | "feb" => Some(2),
        "march" | "mar" => Some(3),
        "april" | "apr" => Some(4),
        "may" => Some(5),
        "june" | "jun" => Some(6),
        "july" | "jul" => Some(7),
        "august" | "aug" => Some(8),
        "september" | "sep" | "sept" => Some(9),
        "october" | "oct" => Some(10),
        "november" | "nov" => Some(11),
        "december" | "dec" => Some(12),
        _ => None,
    };

    if let Some(month) = month_num {
        let year = if month > now.month() {
            now.year() - 1
        } else {
            now.year()
        };
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, 1) {
            return Some(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0)?));
        }
    }

    // Try relative date "X month(s) ago"
    if input.contains("month") && input.contains("ago") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some(num_str) = parts.first() {
            if let Ok(months) = num_str.parse::<i32>() {
                let target = now - chrono::Duration::days(months as i64 * 30);
                return Some(target);
            }
        }
    }

    // Try relative date "X week(s) ago"
    if input.contains("week") && input.contains("ago") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some(num_str) = parts.first() {
            if let Ok(weeks) = num_str.parse::<i64>() {
                let target = now - chrono::Duration::weeks(weeks);
                return Some(target);
            }
        }
    }

    None
}

/// Get the start and end dates for a given month
pub fn get_month_range(input: &str) -> Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    use chrono::{Datelike, NaiveDate, TimeZone, Utc};

    let start = parse_date(input)?;

    // Get first day of next month
    let (next_year, next_month) = if start.month() == 12 {
        (start.year() + 1, 1)
    } else {
        (start.year(), start.month() + 1)
    };

    let end_date = NaiveDate::from_ymd_opt(next_year, next_month, 1)?;
    let end = Utc.from_utc_datetime(&end_date.and_hms_opt(0, 0, 0)?);

    Some((start, end))
}

/// Get the date range for "last month"
pub fn get_last_month_range() -> (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>) {
    use chrono::{Datelike, NaiveDate, TimeZone, Utc};

    let now = Utc::now();

    // First day of current month (end of range)
    let end_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
    let end = Utc.from_utc_datetime(&end_date.and_hms_opt(0, 0, 0).unwrap());

    // First day of previous month (start of range)
    let (prev_year, prev_month) = if now.month() == 1 {
        (now.year() - 1, 12)
    } else {
        (now.year(), now.month() - 1)
    };
    let start_date = NaiveDate::from_ymd_opt(prev_year, prev_month, 1).unwrap();
    let start = Utc.from_utc_datetime(&start_date.and_hms_opt(0, 0, 0).unwrap());

    (start, end)
}

/// Result of scanning GitHub repositories
#[derive(Debug)]
pub struct ScanResult {
    /// Paths to repositories ready for analysis
    pub repo_paths: Vec<PathBuf>,
    /// Repositories that were newly cloned
    pub cloned: Vec<String>,
    /// Repositories that already existed locally
    pub existing: Vec<String>,
    /// Repositories that were skipped (forks, archived, etc.)
    pub skipped: Vec<String>,
    /// Repositories that failed to clone
    pub failed: Vec<(String, String)>,
}

/// GitHub API client
pub struct GitHubClient {
    client: Client,
    token: Option<String>,
}

impl GitHubClient {
    /// Create a new GitHub client, optionally with authentication
    pub fn new() -> Result<Self, String> {
        let token = env::var("GITHUB_TOKEN").ok();

        let client = Client::builder()
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self { client, token })
    }

    /// Create a client with an explicit token
    pub fn with_token(token: String) -> Result<Self, String> {
        let client = Client::builder()
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            token: Some(token),
        })
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    /// Get the authenticated user's login name
    pub fn get_authenticated_user(&self) -> Result<String, String> {
        let token = self.token.as_ref().ok_or("No GitHub token configured")?;

        let response = self
            .client
            .get("https://api.github.com/user")
            .header(USER_AGENT, "git-activity-dashboard")
            .header(ACCEPT, "application/vnd.github+json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .map_err(|e| format!("Failed to fetch user: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().unwrap_or_default()
            ));
        }

        #[derive(Deserialize)]
        struct User {
            login: String,
        }

        let user: User = response
            .json()
            .map_err(|e| format!("Failed to parse user response: {}", e))?;

        Ok(user.login)
    }

    /// Fetch all repositories for a user
    pub fn list_repos(&self, username: Option<&str>) -> Result<Vec<GitHubRepo>, String> {
        let mut all_repos = Vec::new();
        let mut page = 1;
        let per_page = 100;

        loop {
            let url = if let Some(user) = username {
                format!(
                    "https://api.github.com/users/{}/repos?per_page={}&page={}&sort=updated",
                    user, per_page, page
                )
            } else {
                // Authenticated user's repos (includes private)
                format!(
                    "https://api.github.com/user/repos?per_page={}&page={}&sort=updated&affiliation=owner",
                    per_page, page
                )
            };

            let mut request = self
                .client
                .get(&url)
                .header(USER_AGENT, "git-activity-dashboard")
                .header(ACCEPT, "application/vnd.github+json");

            if let Some(token) = &self.token {
                request = request.header(AUTHORIZATION, format!("Bearer {}", token));
            }

            let response = request
                .send()
                .map_err(|e| format!("Failed to fetch repos: {}", e))?;

            if !response.status().is_success() {
                return Err(format!(
                    "GitHub API error: {} - {}",
                    response.status(),
                    response.text().unwrap_or_default()
                ));
            }

            let repos: Vec<GitHubRepo> = response
                .json()
                .map_err(|e| format!("Failed to parse repos: {}", e))?;

            if repos.is_empty() {
                break;
            }

            all_repos.extend(repos);
            page += 1;

            // Safety limit
            if page > 50 {
                break;
            }
        }

        Ok(all_repos)
    }

    /// Fetch the file tree for a repository to get file type statistics
    pub fn get_repo_tree(&self, owner: &str, repo: &str, branch: &str) -> Result<FileTypeStats, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
            owner, repo, branch
        );

        let mut request = self
            .client
            .get(&url)
            .header(USER_AGENT, "git-activity-dashboard")
            .header(ACCEPT, "application/vnd.github+json");

        if let Some(token) = &self.token {
            request = request.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        let response = request
            .send()
            .map_err(|e| format!("Failed to fetch tree: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().unwrap_or_default()
            ));
        }

        #[derive(Deserialize)]
        struct TreeItem {
            path: String,
            #[serde(rename = "type")]
            item_type: String,
            #[serde(default)]
            size: Option<u64>,
        }

        #[derive(Deserialize)]
        struct TreeResponse {
            tree: Vec<TreeItem>,
            #[allow(dead_code)]
            truncated: bool,
        }

        let tree_response: TreeResponse = response
            .json()
            .map_err(|e| format!("Failed to parse tree: {}", e))?;

        let mut file_types: FileTypeStats = std::collections::HashMap::new();

        for item in tree_response.tree {
            if item.item_type != "blob" {
                continue; // Skip directories
            }

            // Extract file extension
            let ext = item
                .path
                .rsplit('.')
                .next()
                .map(|e| e.to_lowercase())
                .unwrap_or_else(|| "no_extension".to_string());

            // Skip if the extension is the full filename (no actual extension)
            let ext = if ext == item.path.to_lowercase() {
                "no_extension".to_string()
            } else {
                ext
            };

            let size = item.size.unwrap_or(0);
            let entry = file_types.entry(ext).or_insert((0, 0));
            entry.0 += 1; // count
            entry.1 += size; // bytes
        }

        Ok(file_types)
    }

    /// Fetch language statistics for a repository (no cloning required)
    pub fn get_repo_languages(&self, owner: &str, repo: &str) -> Result<LanguageStats, String> {
        let url = format!("https://api.github.com/repos/{}/{}/languages", owner, repo);

        let mut request = self
            .client
            .get(&url)
            .header(USER_AGENT, "git-activity-dashboard")
            .header(ACCEPT, "application/vnd.github+json");

        if let Some(token) = &self.token {
            request = request.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        let response = request
            .send()
            .map_err(|e| format!("Failed to fetch languages: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().unwrap_or_default()
            ));
        }

        response
            .json()
            .map_err(|e| format!("Failed to parse languages: {}", e))
    }

    /// Check if a repo was active within the given date range
    fn is_repo_in_date_range(repo: &GitHubRepo, since: Option<&chrono::DateTime<chrono::Utc>>, until: Option<&chrono::DateTime<chrono::Utc>>) -> bool {
        use chrono::DateTime;

        // Parse the pushed_at date
        let pushed_at = repo.pushed_at.as_ref().and_then(|s| {
            DateTime::parse_from_rfc3339(s).ok().map(|d| d.with_timezone(&chrono::Utc))
        });

        match (pushed_at, since, until) {
            (Some(pushed), Some(start), Some(end)) => pushed >= *start && pushed < *end,
            (Some(pushed), Some(start), None) => pushed >= *start,
            (Some(pushed), None, Some(end)) => pushed < *end,
            (None, _, _) => true, // Include repos without date info
            (Some(_), None, None) => true,
        }
    }

    /// Get stats for all repos without cloning (API only)
    pub fn get_all_repo_stats(&self, username: Option<&str>, options: &GitHubScanOptions) -> Result<Vec<GitHubRepoStats>, String> {
        // Determine if we're fetching for the authenticated user or a specific user
        let (display_username, is_authenticated_user) = if let Some(user) = username {
            (user.to_string(), false)
        } else if let Some(ref user) = options.username {
            (user.clone(), false)
        } else {
            (self.get_authenticated_user()?, true)
        };

        println!("Fetching repository statistics for: {}", display_username);

        // Print date filter info
        if let Some(since) = &options.since {
            println!("  Filtering: activity since {}", since.format("%Y-%m-%d"));
        }
        if let Some(until) = &options.until {
            println!("  Filtering: activity until {}", until.format("%Y-%m-%d"));
        }

        // Use /user/repos for authenticated user (includes private repos), /users/{}/repos for others
        let repos = if is_authenticated_user {
            self.list_repos(None)?
        } else {
            self.list_repos(Some(&display_username))?
        };
        println!("Found {} repositories, fetching stats...\n", repos.len());

        let mut stats = Vec::new();
        let mut skipped_date = 0;

        for repo in repos {
            // Filter based on options
            if repo.fork && !options.include_forks {
                continue;
            }
            if repo.archived && !options.include_archived {
                continue;
            }
            if repo.private && !options.include_private {
                continue;
            }

            // Filter by date range
            if !Self::is_repo_in_date_range(&repo, options.since.as_ref(), options.until.as_ref()) {
                skipped_date += 1;
                continue;
            }

            print!("  {} ", repo.name);

            // Fetch languages
            let languages = match self.get_repo_languages(&display_username, &repo.name) {
                Ok(langs) => langs,
                Err(_) => std::collections::HashMap::new(),
            };

            // Fetch file tree for file type stats
            let (file_types, file_count) = match self.get_repo_tree(&display_username, &repo.name, &repo.default_branch) {
                Ok(types) => {
                    let count: u64 = types.values().map(|(c, _)| c).sum();
                    (types, count)
                }
                Err(_) => (std::collections::HashMap::new(), 0),
            };

            let total_bytes: u64 = languages.values().sum();
            let estimated_loc = total_bytes / 40;

            println!("- {} files, {} LOC", file_count, estimated_loc);

            stats.push(GitHubRepoStats {
                name: repo.name,
                full_name: repo.full_name,
                private: repo.private,
                fork: repo.fork,
                archived: repo.archived,
                size_kb: repo.size,
                primary_language: repo.language,
                languages,
                file_types,
                file_count,
                estimated_loc,
            });
        }

        if skipped_date > 0 {
            println!("\n  (Skipped {} repos outside date range)", skipped_date);
        }

        Ok(stats)
    }

    /// Print a summary of GitHub stats (no cloning)
    pub fn print_stats_summary(stats: &[GitHubRepoStats]) {
        use std::collections::HashMap;

        let mut total_bytes: u64 = 0;
        let mut total_loc: u64 = 0;
        let mut total_files: u64 = 0;
        let mut language_totals: HashMap<String, u64> = HashMap::new();
        let mut file_type_totals: HashMap<String, (u64, u64)> = HashMap::new(); // (count, bytes)

        for repo in stats {
            total_loc += repo.estimated_loc;
            total_files += repo.file_count;

            for (lang, bytes) in &repo.languages {
                total_bytes += bytes;
                *language_totals.entry(lang.clone()).or_insert(0) += bytes;
            }

            for (ext, (count, bytes)) in &repo.file_types {
                let entry = file_type_totals.entry(ext.clone()).or_insert((0, 0));
                entry.0 += count;
                entry.1 += bytes;
            }
        }

        println!("\n{}", "=".repeat(60));
        println!("GITHUB REPOSITORY STATISTICS");
        println!("{}", "=".repeat(60));

        println!("\nRepositories analyzed: {}", stats.len());
        println!("Total files: {}", total_files);
        println!("Total code size: {:.2} MB", total_bytes as f64 / 1_000_000.0);
        println!("Estimated lines of code: {}", total_loc);

        // Sort languages by bytes
        let mut sorted_langs: Vec<_> = language_totals.into_iter().collect();
        sorted_langs.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n{}", "-".repeat(40));
        println!("LANGUAGES BY SIZE");
        println!("{}", "-".repeat(40));

        for (lang, bytes) in sorted_langs.iter().take(15) {
            let pct = if total_bytes > 0 { (*bytes as f64 / total_bytes as f64) * 100.0 } else { 0.0 };
            let bar = "█".repeat((pct / 2.0) as usize);
            let loc = bytes / 40; // Rough estimate
            println!("  {:20} {:5.1}%  ~{:>8} LOC  {}", lang, pct, loc, bar);
        }

        // File types by count
        let mut sorted_file_types: Vec<_> = file_type_totals.into_iter().collect();
        sorted_file_types.sort_by(|a, b| b.1.0.cmp(&a.1.0)); // Sort by count

        println!("\n{}", "-".repeat(40));
        println!("FILE TYPES (by extension)");
        println!("{}", "-".repeat(40));

        for (ext, (count, bytes)) in sorted_file_types.iter().take(20) {
            let pct = if total_files > 0 { (*count as f64 / total_files as f64) * 100.0 } else { 0.0 };
            let bar = "█".repeat((pct / 2.0) as usize);
            let kb = bytes / 1024;
            println!("  .{:19} {:5} files {:5.1}%  {:>6} KB  {}", ext, count, pct, kb, bar);
        }

        println!("\n{}", "-".repeat(40));
        println!("REPOSITORIES");
        println!("{}", "-".repeat(40));

        // Sort repos by estimated LOC
        let mut sorted_repos: Vec<_> = stats.iter().collect();
        sorted_repos.sort_by(|a, b| b.estimated_loc.cmp(&a.estimated_loc));

        for repo in sorted_repos.iter().take(15) {
            let lang = repo.primary_language.as_deref().unwrap_or("Unknown");
            let private_badge = if repo.private { "[private]" } else { "" };
            println!(
                "  {:30} {:>6} files {:>8} LOC  {:15} {}",
                repo.name, repo.file_count, repo.estimated_loc, lang, private_badge
            );
        }

        println!("\n{}\n", "=".repeat(60));
    }

    /// Fetch weekly commit activity for a repository
    pub fn get_repo_commit_activity(&self, owner: &str, repo: &str) -> Result<Vec<WeeklyCommitActivity>, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/stats/commit_activity",
            owner, repo
        );

        let mut request = self
            .client
            .get(&url)
            .header(USER_AGENT, "git-activity-dashboard")
            .header(ACCEPT, "application/vnd.github+json");

        if let Some(token) = &self.token {
            request = request.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        let response = request
            .send()
            .map_err(|e| format!("Failed to fetch commit activity: {}", e))?;

        // GitHub may return 202 if stats are being computed
        if response.status().as_u16() == 202 {
            return Ok(Vec::new());
        }

        if !response.status().is_success() {
            return Ok(Vec::new()); // Silently skip on error
        }

        let activity: Vec<WeeklyCommitActivity> = response
            .json()
            .unwrap_or_default();

        Ok(activity)
    }

    /// Fetch commit stats (additions/deletions) for a repository with caching
    pub fn get_repo_commits_stats(&self, owner: &str, repo: &str, _since: Option<&str>, _until: Option<&str>) -> Result<Vec<(String, u64, u64, u64, Vec<String>)>, String> {
        // Returns: (week, commits, additions, deletions, file_extensions)
        use chrono::{DateTime, Datelike, Utc};

        let cache = GitHubCache::new();
        let repo_key = format!("{}/{}", owner, repo);

        // Check cache first
        if let Some(cached) = cache.get(&repo_key) {
            // Convert cached data to result format
            let mut weekly_stats: HashMap<String, (u64, u64, u64, Vec<String>)> = HashMap::new();
            for commit in cached.commits {
                let entry = weekly_stats.entry(commit.week.clone()).or_insert((0, 0, 0, Vec::new()));
                entry.0 += 1;
                entry.1 += commit.additions;
                entry.2 += commit.deletions;
                entry.3.extend(commit.file_extensions);
            }
            let mut result: Vec<_> = weekly_stats
                .into_iter()
                .map(|(week, (commits, additions, deletions, exts))| {
                    (week, commits, additions, deletions, exts)
                })
                .collect();
            result.sort_by(|a, b| b.0.cmp(&a.0));
            return Ok(result);
        }

        // Fetch from API
        let mut all_commits = Vec::new();
        let mut page = 1;
        let per_page = 100;

        loop {
            let url = format!(
                "https://api.github.com/repos/{}/{}/commits?per_page={}&page={}",
                owner, repo, per_page, page
            );

            let mut request = self
                .client
                .get(&url)
                .header(USER_AGENT, "git-activity-dashboard")
                .header(ACCEPT, "application/vnd.github+json");

            if let Some(token) = &self.token {
                request = request.header(AUTHORIZATION, format!("Bearer {}", token));
            }

            let response = request
                .send()
                .map_err(|e| format!("Failed to fetch commits: {}", e))?;

            if !response.status().is_success() {
                break;
            }

            #[derive(Deserialize)]
            struct CommitInfo {
                sha: String,
                commit: CommitDetail,
            }

            #[derive(Deserialize)]
            struct CommitDetail {
                author: Option<CommitAuthor>,
            }

            #[derive(Deserialize)]
            struct CommitAuthor {
                date: Option<String>,
            }

            let commits: Vec<CommitInfo> = response.json().unwrap_or_default();

            if commits.is_empty() {
                break;
            }

            for commit in commits {
                if let Some(author) = &commit.commit.author {
                    if let Some(date) = &author.date {
                        all_commits.push((commit.sha, date.clone()));
                    }
                }
            }

            page += 1;
            if page > 10 {
                break; // Limit to 1000 commits per repo
            }
        }

        // Group by week and fetch stats for each commit
        let mut weekly_stats: HashMap<String, (u64, u64, u64, Vec<String>)> = HashMap::new();
        let mut cached_commits = Vec::new();

        for (sha, date_str) in all_commits.iter().take(100) { // Limit API calls
            // Parse date and get week
            let week = if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
                let dt_utc: DateTime<Utc> = dt.into();
                let iso_week = dt_utc.iso_week();
                format!("{}-W{:02}", iso_week.year(), iso_week.week())
            } else {
                continue;
            };

            // Fetch commit details for additions/deletions
            let commit_url = format!(
                "https://api.github.com/repos/{}/{}/commits/{}",
                owner, repo, sha
            );

            let mut request = self
                .client
                .get(&commit_url)
                .header(USER_AGENT, "git-activity-dashboard")
                .header(ACCEPT, "application/vnd.github+json");

            if let Some(token) = &self.token {
                request = request.header(AUTHORIZATION, format!("Bearer {}", token));
            }

            if let Ok(response) = request.send() {
                if response.status().is_success() {
                    #[derive(Deserialize)]
                    struct CommitStats {
                        stats: Option<StatsDetail>,
                        files: Option<Vec<FileDetail>>,
                    }

                    #[derive(Deserialize)]
                    struct StatsDetail {
                        additions: u64,
                        deletions: u64,
                    }

                    #[derive(Deserialize)]
                    struct FileDetail {
                        filename: String,
                    }

                    if let Ok(commit_data) = response.json::<CommitStats>() {
                        let mut additions = 0u64;
                        let mut deletions = 0u64;
                        let mut file_exts = Vec::new();

                        if let Some(stats) = commit_data.stats {
                            additions = stats.additions;
                            deletions = stats.deletions;
                        }

                        if let Some(files) = commit_data.files {
                            for file in files {
                                if let Some(ext) = std::path::Path::new(&file.filename)
                                    .extension()
                                    .and_then(|e| e.to_str())
                                {
                                    file_exts.push(ext.to_string());
                                }
                            }
                        }

                        // Store in weekly stats
                        let entry = weekly_stats.entry(week.clone()).or_insert((0, 0, 0, Vec::new()));
                        entry.0 += 1;
                        entry.1 += additions;
                        entry.2 += deletions;
                        entry.3.extend(file_exts.clone());

                        // Store for cache
                        cached_commits.push(CachedCommit {
                            sha: sha.clone(),
                            week,
                            additions,
                            deletions,
                            file_extensions: file_exts,
                        });
                    }
                }
            }
        }

        // Save to cache
        let cached_data = CachedRepoCommits {
            repo: repo_key.clone(),
            fetched_at: chrono::Utc::now().to_rfc3339(),
            commits: cached_commits,
        };
        cache.set(&repo_key, &cached_data);

        // Convert to sorted vector
        let mut result: Vec<_> = weekly_stats
            .into_iter()
            .map(|(week, (commits, additions, deletions, exts))| {
                (week, commits, additions, deletions, exts)
            })
            .collect();

        result.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by week descending
        Ok(result)
    }

    /// Get weekly stats for all repos with specified grouping
    pub fn get_weekly_stats(
        &self,
        repos: &[GitHubRepo],
        _grouping: &StatsGrouping,
        owner: &str,
    ) -> Result<Vec<WeeklyRepoStats>, String> {
        use std::collections::HashMap;

        let mut all_stats: HashMap<String, HashMap<String, WeeklyRepoStats>> = HashMap::new();

        for repo in repos {
            print!("  {} ", repo.name);
            std::io::Write::flush(&mut std::io::stdout()).ok();

            match self.get_repo_commits_stats(owner, &repo.name, None, None) {
                Ok(commits) => {
                    for (week, commit_count, additions, deletions, file_exts) in commits {
                        let week_entry = all_stats.entry(week.clone()).or_insert_with(HashMap::new);

                        let repo_entry = week_entry.entry(repo.name.clone()).or_insert_with(|| {
                            WeeklyRepoStats {
                                week: week.clone(),
                                repo: repo.name.clone(),
                                commits: 0,
                                additions: 0,
                                deletions: 0,
                                file_types: HashMap::new(),
                            }
                        });

                        repo_entry.commits += commit_count;
                        repo_entry.additions += additions;
                        repo_entry.deletions += deletions;

                        // Count file types
                        for ext in file_exts {
                            let ft = repo_entry.file_types.entry(ext).or_insert_with(FileTypeWeeklyStats::default);
                            ft.commits += 1;
                            ft.additions += additions / std::cmp::max(1, commit_count);
                            ft.deletions += deletions / std::cmp::max(1, commit_count);
                        }
                    }
                    println!("✓");
                }
                Err(_) => {
                    println!("(skipped)");
                }
            }
        }

        // Flatten based on grouping level
        let mut result = Vec::new();
        let mut sorted_weeks: Vec<_> = all_stats.keys().cloned().collect();
        sorted_weeks.sort_by(|a, b| b.cmp(a));

        for week in sorted_weeks {
            if let Some(repos_map) = all_stats.get(&week) {
                for (_, repo_stats) in repos_map {
                    result.push(repo_stats.clone());
                }
            }
        }

        Ok(result)
    }

    /// Print weekly stats with specified grouping
    pub fn print_weekly_stats(stats: &[WeeklyRepoStats], grouping: &StatsGrouping) {
        use std::collections::HashMap;

        println!("\n{}", "=".repeat(70));
        println!("WEEKLY ACTIVITY BREAKDOWN");
        println!("{}", "=".repeat(70));

        match grouping {
            StatsGrouping::Week => {
                // Aggregate by week only
                let mut weekly: HashMap<String, (u64, u64, u64)> = HashMap::new();
                for s in stats {
                    let entry = weekly.entry(s.week.clone()).or_insert((0, 0, 0));
                    entry.0 += s.commits;
                    entry.1 += s.additions;
                    entry.2 += s.deletions;
                }

                let mut sorted: Vec<_> = weekly.into_iter().collect();
                sorted.sort_by(|a, b| b.0.cmp(&a.0));

                println!("\n{:15} {:>10} {:>12} {:>12}", "Week", "Commits", "Additions", "Deletions");
                println!("{}", "-".repeat(50));

                for (week, (commits, additions, deletions)) in sorted.iter().take(20) {
                    let bar = "█".repeat(std::cmp::min(*commits as usize, 30));
                    println!("{:15} {:>10} {:>+12} {:>-12}  {}", week, commits, additions, deletions, bar);
                }
            }

            StatsGrouping::WeekFileType => {
                // Aggregate by week and file type (across all repos)
                let mut weekly_filetypes: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();
                for s in stats {
                    let week_entry = weekly_filetypes.entry(s.week.clone()).or_insert_with(HashMap::new);
                    for (ext, ft) in &s.file_types {
                        let entry = week_entry.entry(ext.clone()).or_insert((0, 0, 0));
                        entry.0 += ft.commits;
                        entry.1 += ft.additions;
                        entry.2 += ft.deletions;
                    }
                }

                let mut sorted_weeks: Vec<_> = weekly_filetypes.keys().cloned().collect();
                sorted_weeks.sort_by(|a, b| b.cmp(a));

                for week in sorted_weeks.iter().take(12) {
                    println!("\n{}", "-".repeat(70));
                    println!("Week: {}", week);
                    println!("{}", "-".repeat(70));
                    println!("{:20} {:>10} {:>12} {:>12}", "File Type", "Changes", "Additions", "Deletions");

                    if let Some(filetypes) = weekly_filetypes.get(week) {
                        let mut sorted_types: Vec<_> = filetypes.iter().collect();
                        sorted_types.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (ext, (changes, additions, deletions)) in sorted_types.iter().take(15) {
                            let bar = "█".repeat(std::cmp::min(*changes as usize, 20));
                            println!(".{:19} {:>10} {:>+12} {:>-12}  {}",
                                ext, changes, additions, deletions, bar);
                        }
                    }
                }
            }

            StatsGrouping::WeekRepo => {
                // Group by week, then show repos
                let mut by_week: HashMap<String, Vec<&WeeklyRepoStats>> = HashMap::new();
                for s in stats {
                    by_week.entry(s.week.clone()).or_insert_with(Vec::new).push(s);
                }

                let mut sorted_weeks: Vec<_> = by_week.keys().cloned().collect();
                sorted_weeks.sort_by(|a, b| b.cmp(a));

                for week in sorted_weeks.iter().take(12) {
                    println!("\n{}", "-".repeat(70));
                    println!("Week: {}", week);
                    println!("{}", "-".repeat(70));
                    println!("{:30} {:>10} {:>12} {:>12}", "Repository", "Commits", "Additions", "Deletions");

                    if let Some(repos) = by_week.get(week) {
                        let mut sorted_repos = repos.clone();
                        sorted_repos.sort_by(|a, b| b.commits.cmp(&a.commits));

                        for repo in sorted_repos.iter().take(10) {
                            let bar = "█".repeat(std::cmp::min(repo.commits as usize, 20));
                            println!("{:30} {:>10} {:>+12} {:>-12}  {}",
                                repo.repo, repo.commits, repo.additions, repo.deletions, bar);
                        }
                    }
                }
            }

            StatsGrouping::WeekRepoFileType => {
                // Group by week, repo, and file type
                let mut by_week: HashMap<String, Vec<&WeeklyRepoStats>> = HashMap::new();
                for s in stats {
                    by_week.entry(s.week.clone()).or_insert_with(Vec::new).push(s);
                }

                let mut sorted_weeks: Vec<_> = by_week.keys().cloned().collect();
                sorted_weeks.sort_by(|a, b| b.cmp(a));

                for week in sorted_weeks.iter().take(8) {
                    println!("\n{}", "=".repeat(70));
                    println!("Week: {}", week);
                    println!("{}", "=".repeat(70));

                    if let Some(repos) = by_week.get(week) {
                        let mut sorted_repos = repos.clone();
                        sorted_repos.sort_by(|a, b| b.commits.cmp(&a.commits));

                        for repo in sorted_repos.iter().take(5) {
                            println!("\n  {} ({} commits, +{} -{}):",
                                repo.repo, repo.commits, repo.additions, repo.deletions);

                            let mut sorted_types: Vec<_> = repo.file_types.iter().collect();
                            sorted_types.sort_by(|a, b| b.1.commits.cmp(&a.1.commits));

                            for (ext, ft) in sorted_types.iter().take(8) {
                                println!("    .{:15} {:>5} changes", ext, ft.commits);
                            }
                        }
                    }
                }
            }
        }

        println!("\n{}\n", "=".repeat(70));
    }

    /// Clone a repository to the specified directory
    fn clone_repo(&self, repo: &GitHubRepo, target_dir: &Path) -> Result<PathBuf, String> {
        let repo_path = target_dir.join(&repo.name);

        // Check if already exists
        if repo_path.exists() && repo_path.join(".git").exists() {
            return Ok(repo_path);
        }

        // Create parent directory if needed
        if !target_dir.exists() {
            std::fs::create_dir_all(target_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Set up authentication callbacks for private repos
        let mut callbacks = RemoteCallbacks::new();

        if let Some(token) = &self.token {
            let token = token.clone();
            callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                Cred::userpass_plaintext("x-access-token", &token)
            });
        }

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        // Clone using HTTPS (works with token auth)
        builder
            .clone(&repo.clone_url, &repo_path)
            .map_err(|e| format!("Failed to clone {}: {}", repo.name, e))?;

        Ok(repo_path)
    }

    /// Scan and optionally clone all repositories for a user
    pub fn scan_repos(&self, options: &GitHubScanOptions) -> Result<ScanResult, String> {
        // Determine if we're fetching for the authenticated user or a specific user
        let (username, is_authenticated_user) = if let Some(ref user) = options.username {
            (user.clone(), false)
        } else {
            (self.get_authenticated_user()?, true)
        };

        println!("Fetching repositories for: {}", username);

        // Use /user/repos for authenticated user (includes private repos), /users/{}/repos for others
        let repos = if is_authenticated_user {
            self.list_repos(None)?
        } else {
            self.list_repos(Some(&username))?
        };
        println!("Found {} repositories", repos.len());

        let mut result = ScanResult {
            repo_paths: Vec::new(),
            cloned: Vec::new(),
            existing: Vec::new(),
            skipped: Vec::new(),
            failed: Vec::new(),
        };

        // Create clone directory
        if !options.skip_clone && !options.clone_dir.exists() {
            std::fs::create_dir_all(&options.clone_dir)
                .map_err(|e| format!("Failed to create clone directory: {}", e))?;
        }

        for repo in repos {
            // Filter based on options
            if repo.fork && !options.include_forks {
                result.skipped.push(format!("{} (fork)", repo.name));
                continue;
            }

            if repo.archived && !options.include_archived {
                result.skipped.push(format!("{} (archived)", repo.name));
                continue;
            }

            if repo.private && !options.include_private {
                result.skipped.push(format!("{} (private)", repo.name));
                continue;
            }

            let repo_path = options.clone_dir.join(&repo.name);

            // Check if repo already exists locally
            if repo_path.exists() && repo_path.join(".git").exists() {
                println!("  [exists] {}", repo.name);
                result.existing.push(repo.name.clone());
                result.repo_paths.push(repo_path);
                continue;
            }

            // Skip cloning if requested
            if options.skip_clone {
                result.skipped.push(format!("{} (not cloned locally)", repo.name));
                continue;
            }

            // Clone the repository
            print!("  [cloning] {}... ", repo.name);
            match self.clone_repo(&repo, &options.clone_dir) {
                Ok(path) => {
                    println!("done");
                    result.cloned.push(repo.name.clone());
                    result.repo_paths.push(path);
                }
                Err(e) => {
                    println!("failed");
                    result.failed.push((repo.name.clone(), e));
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_client_creation() {
        // Should not panic even without token
        let client = GitHubClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_scan_options_default() {
        let options = GitHubScanOptions::default();
        assert!(!options.include_forks);
        assert!(!options.include_archived);
        assert!(options.include_private);
    }
}
