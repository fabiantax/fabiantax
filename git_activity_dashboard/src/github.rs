//! GitHub API integration for fetching and cloning user repositories
//!
//! This module provides functionality to:
//! - List all repositories for an authenticated user or specific username
//! - Clone repositories that don't exist locally
//! - Skip already-cloned repositories for efficiency

use git2::{Cred, FetchOptions, RemoteCallbacks};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

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
        let username = if let Some(user) = username {
            user.to_string()
        } else if let Some(ref user) = options.username {
            user.clone()
        } else {
            self.get_authenticated_user()?
        };

        println!("Fetching repository statistics for: {}", username);

        // Print date filter info
        if let Some(since) = &options.since {
            println!("  Filtering: activity since {}", since.format("%Y-%m-%d"));
        }
        if let Some(until) = &options.until {
            println!("  Filtering: activity until {}", until.format("%Y-%m-%d"));
        }

        let repos = self.list_repos(Some(&username))?;
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
            let languages = match self.get_repo_languages(&username, &repo.name) {
                Ok(langs) => langs,
                Err(_) => std::collections::HashMap::new(),
            };

            // Fetch file tree for file type stats
            let (file_types, file_count) = match self.get_repo_tree(&username, &repo.name, &repo.default_branch) {
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
        let username = if let Some(ref user) = options.username {
            user.clone()
        } else {
            self.get_authenticated_user()?
        };

        println!("Fetching repositories for: {}", username);

        let repos = self.list_repos(Some(&username))?;
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
