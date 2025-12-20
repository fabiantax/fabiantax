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
    #[serde(default)]
    pub file_paths: Vec<String>, // Legacy: kept for backward compatibility
    #[serde(default)]
    pub file_stats: Vec<FileStats>, // Per-file additions/deletions
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
    WeekCategory,
    WeekRepoCategory,
    WeekLanguage,
    Month,
    MonthRepo,
    MonthFileType,
    MonthCategory,
    MonthRepoCategory,
    MonthLanguage,
}

impl StatsGrouping {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "week" => Some(Self::Week),
            "week-filetype" | "weekfiletype" | "week-file" => Some(Self::WeekFileType),
            "week-repo" | "weekrepo" => Some(Self::WeekRepo),
            "week-repo-filetype" | "weekrepofiletype" | "week-repo-file" => Some(Self::WeekRepoFileType),
            "week-category" | "weekcategory" | "week-cat" => Some(Self::WeekCategory),
            "week-repo-category" | "weekrepocategory" | "week-repo-cat" => Some(Self::WeekRepoCategory),
            "week-lang" | "weeklang" | "week-language" => Some(Self::WeekLanguage),
            "month" => Some(Self::Month),
            "month-repo" | "monthrepo" => Some(Self::MonthRepo),
            "month-filetype" | "monthfiletype" | "month-file" => Some(Self::MonthFileType),
            "month-category" | "monthcategory" | "month-cat" => Some(Self::MonthCategory),
            "month-repo-category" | "monthrepocategory" | "month-repo-cat" => Some(Self::MonthRepoCategory),
            "month-lang" | "monthlang" | "month-language" => Some(Self::MonthLanguage),
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
    #[serde(default)]
    pub version: u32, // Cache version for invalidation
}

/// Per-file statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub path: String,
    pub additions: u64,
    pub deletions: u64,
}

/// Cached commit details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCommit {
    pub sha: String,
    pub week: String,
    pub additions: u64,
    pub deletions: u64,
    pub file_extensions: Vec<String>,
    #[serde(default)]
    pub file_paths: Vec<String>, // Legacy: kept for cache migration
    #[serde(default)]
    pub file_stats: Vec<FileStats>, // New: per-file additions/deletions
}

/// Programming language detected from file extension
/// Follows Single Responsibility Principle - only handles language detection
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum ProgrammingLanguage {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    CSharp,
    Go,
    Java,
    Kotlin,
    Swift,
    Ruby,
    Cpp,
    C,
    PHP,
    Scala,
    Elixir,
    Haskell,
    Shell,
    SQL,
    HTML,
    CSS,
    Markdown,
    YAML,
    JSON,
    TOML,
    Other(String),
}

impl ProgrammingLanguage {
    /// Detect programming language from file path
    /// Open/Closed Principle - easy to extend by adding new match arms
    pub fn from_path(path: &str) -> Self {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            // Rust
            "rs" => Self::Rust,

            // TypeScript (check before JavaScript)
            "ts" | "tsx" | "mts" | "cts" => Self::TypeScript,

            // JavaScript
            "js" | "jsx" | "mjs" | "cjs" => Self::JavaScript,

            // Python
            "py" | "pyw" | "pyi" | "pyx" => Self::Python,

            // C#
            "cs" | "csx" => Self::CSharp,

            // Go
            "go" => Self::Go,

            // Java
            "java" => Self::Java,

            // Kotlin
            "kt" | "kts" => Self::Kotlin,

            // Swift
            "swift" => Self::Swift,

            // Ruby
            "rb" | "rake" | "gemspec" => Self::Ruby,

            // C++
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "h++" => Self::Cpp,

            // C
            "c" | "h" => Self::C,

            // PHP
            "php" | "php3" | "php4" | "php5" | "phtml" => Self::PHP,

            // Scala
            "scala" | "sc" => Self::Scala,

            // Elixir
            "ex" | "exs" => Self::Elixir,

            // Haskell
            "hs" | "lhs" => Self::Haskell,

            // Shell
            "sh" | "bash" | "zsh" | "fish" | "ps1" | "psm1" => Self::Shell,

            // SQL
            "sql" => Self::SQL,

            // HTML
            "html" | "htm" | "xhtml" => Self::HTML,

            // CSS
            "css" | "scss" | "sass" | "less" | "styl" => Self::CSS,

            // Markdown
            "md" | "markdown" | "mdx" => Self::Markdown,

            // YAML
            "yaml" | "yml" => Self::YAML,

            // JSON
            "json" | "jsonc" | "json5" => Self::JSON,

            // TOML
            "toml" => Self::TOML,

            // Other - store the extension for visibility
            other if !other.is_empty() => Self::Other(other.to_string()),
            _ => Self::Other("unknown".to_string()),
        }
    }

    /// Get human-readable name for the language
    pub fn as_str(&self) -> &str {
        match self {
            Self::Rust => "Rust",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Python => "Python",
            Self::CSharp => "C#",
            Self::Go => "Go",
            Self::Java => "Java",
            Self::Kotlin => "Kotlin",
            Self::Swift => "Swift",
            Self::Ruby => "Ruby",
            Self::Cpp => "C++",
            Self::C => "C",
            Self::PHP => "PHP",
            Self::Scala => "Scala",
            Self::Elixir => "Elixir",
            Self::Haskell => "Haskell",
            Self::Shell => "Shell",
            Self::SQL => "SQL",
            Self::HTML => "HTML",
            Self::CSS => "CSS",
            Self::Markdown => "Markdown",
            Self::YAML => "YAML",
            Self::JSON => "JSON",
            Self::TOML => "TOML",
            Self::Other(ext) => ext,
        }
    }

    /// Check if this is a programming language (not markup/config)
    pub fn is_code(&self) -> bool {
        !matches!(
            self,
            Self::Markdown | Self::YAML | Self::JSON | Self::TOML | Self::HTML | Self::CSS
        )
    }
}

/// File category based on folder patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileCategory {
    Docs,      // docs/, documentation/, README, etc.
    Specs,     // specs/, spec/, specifications/, schemas/
    Tests,     // tests/, test/, __tests__/, *_test.*, *_spec.*
    Config,    // .json, .yaml, .toml in root or config/
    CI,        // .github/, .gitlab-ci.yml, Jenkinsfile, etc.
    Generated, // dist/, build/, node_modules/, vendor/, lock files
    Assets,    // images/, assets/, static/, public/
    Code,      // Everything else
    Excluded,  // Files to exclude from stats entirely
}

impl FileCategory {
    pub fn from_path(path: &str) -> Self {
        let path_lower = path.to_lowercase();
        let filename = std::path::Path::new(path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_lowercase();

        // EXCLUDED: Lock files and dependency folders (these inflate stats massively)
        if filename == "package-lock.json"
            || filename == "yarn.lock"
            || filename == "pnpm-lock.yaml"
            || filename == "cargo.lock"
            || filename == "gemfile.lock"
            || filename == "poetry.lock"
            || filename == "composer.lock"
            || path_lower.contains("/node_modules/")
            || path_lower.contains("/vendor/")
            || path_lower.starts_with("node_modules/")
            || path_lower.starts_with("vendor/")
        {
            return Self::Excluded;
        }

        // GENERATED: Build outputs and generated code
        if path_lower.contains("/dist/")
            || path_lower.contains("/build/")
            || path_lower.contains("/out/")
            || path_lower.contains("/.next/")
            || path_lower.contains("/target/")
            || path_lower.contains("/__pycache__/")
            || path_lower.contains("/bin/debug/")
            || path_lower.contains("/bin/release/")
            || path_lower.contains("/obj/")
            || path_lower.starts_with("dist/")
            || path_lower.starts_with("build/")
            || path_lower.starts_with("out/")
            || path_lower.starts_with(".next/")
            || path_lower.starts_with("target/")
            || path_lower.starts_with("__pycache__/")
            || path_lower.starts_with("obj/")
            || path_lower.starts_with("bin/debug/")   // C#
            || path_lower.starts_with("bin/release/") // C#
            || filename.ends_with(".min.js")
            || filename.ends_with(".min.css")
            || filename.ends_with(".bundle.js")
        {
            return Self::Generated;
        }

        // CI/CD: Continuous integration and deployment
        if path_lower.contains("/.github/")
            || path_lower.contains("/.gitlab/")
            || path_lower.starts_with(".github/")
            || path_lower.starts_with(".gitlab/")
            || path_lower.starts_with(".circleci/")
            || filename == ".gitlab-ci.yml"
            || filename == "jenkinsfile"
            || filename == ".travis.yml"
            || filename == "azure-pipelines.yml"
            || filename == "bitbucket-pipelines.yml"
            || path_lower.contains("/.circleci/")
        {
            return Self::CI;
        }

        // ASSETS: Images, fonts, static files
        if path_lower.contains("/assets/")
            || path_lower.contains("/images/")
            || path_lower.contains("/static/")
            || path_lower.contains("/public/")
            || path_lower.contains("/fonts/")
            || path_lower.contains("/media/")
            || path_lower.starts_with("assets/")
            || path_lower.starts_with("images/")
            || path_lower.starts_with("static/")
            || path_lower.starts_with("public/")
        {
            let ext = std::path::Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            // Only count actual asset files, not code in these folders
            if matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp"
                | "woff" | "woff2" | "ttf" | "eot" | "otf" | "mp3" | "mp4" | "wav" | "pdf") {
                return Self::Assets;
            }
        }

        // DOCS: Documentation
        if path_lower.contains("/docs/")
            || path_lower.contains("/documentation/")
            || path_lower.starts_with("docs/")
            || path_lower.starts_with("documentation/")
            || filename == "readme.md"
            || filename == "readme"
            || filename == "changelog.md"
            || filename == "changelog"
            || filename == "contributing.md"
            || filename == "contributing"
            || filename == "license.md"
            || filename == "license"
        {
            return Self::Docs;
        }

        // SPECS: API specs, schemas
        if path_lower.contains("/specs/")
            || path_lower.contains("/spec/")
            || path_lower.contains("/specifications/")
            || path_lower.contains("/schemas/")
            || path_lower.contains("/schema/")
            || path_lower.starts_with("specs/")
            || path_lower.starts_with("spec/")
            || path_lower.starts_with("schemas/")
            || path_lower.starts_with("schema/")
            || path_lower.starts_with("specifications/")
            || filename == "openapi.yaml"
            || filename == "openapi.json"
            || filename == "swagger.yaml"
            || filename == "swagger.json"
        {
            return Self::Specs;
        }

        // TESTS: Test files
        if path_lower.contains("/tests/")
            || path_lower.contains("/test/")
            || path_lower.contains("/__tests__/")
            || path_lower.contains("/_tests/")
            || path_lower.starts_with("tests/")
            || path_lower.starts_with("test/")
            || filename.contains("_test.")
            || filename.contains("_spec.")
            || filename.contains(".test.")
            || filename.contains(".spec.")
            || filename.starts_with("test_")  // Python: test_*.py
        {
            return Self::Tests;
        }

        // CONFIG: Configuration files in root or config folders
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if (ext == "json" || ext == "yaml" || ext == "yml" || ext == "toml")
            && (path_lower.contains("/config/")
                || path_lower.contains("/configuration/")
                || !path.contains("/")  // Root level
                || path_lower.starts_with("config/")
                || path_lower.starts_with("configuration/"))
        {
            return Self::Config;
        }

        // Also catch common config files by name
        if filename == "tsconfig.json"
            || filename == "jsconfig.json"
            || filename == "eslintrc.json"
            || filename == ".eslintrc.json"
            || filename == ".prettierrc"
            || filename == "prettier.config.js"
            || filename == "babel.config.js"
            || filename == ".babelrc"
            || filename == "webpack.config.js"
            || filename == "vite.config.ts"
            || filename == "vite.config.js"
            || filename == "next.config.js"
            || filename == "tailwind.config.js"
            || filename == "postcss.config.js"
        {
            return Self::Config;
        }

        Self::Code
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Docs => "docs",
            Self::Specs => "specs",
            Self::Tests => "tests",
            Self::Config => "config",
            Self::CI => "ci",
            Self::Generated => "generated",
            Self::Assets => "assets",
            Self::Code => "code",
            Self::Excluded => "excluded",
        }
    }

    /// Returns true if this category should be excluded from stats
    pub fn is_excluded(&self) -> bool {
        matches!(self, Self::Excluded)
    }
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

    const CACHE_VERSION: u32 = 5; // v5: pagination for commits with >300 files

    pub fn get(&self, repo: &str) -> Option<CachedRepoCommits> {
        use chrono::{Datelike, Utc};

        let path = self.cache_path(repo);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cached) = serde_json::from_str::<CachedRepoCommits>(&content) {
                    // Check cache version - invalidate old format
                    if cached.version < Self::CACHE_VERSION {
                        return None;
                    }

                    // Get current week
                    let now = Utc::now();
                    let current_week = format!("{}-W{:02}", now.iso_week().year(), now.iso_week().week());

                    // Check if cache has any commits from current week
                    let has_current_week = cached.commits.iter().any(|c| c.week == current_week);

                    if has_current_week {
                        // If cache has current week data, only use if less than 1 hour old
                        if let Ok(fetched) = chrono::DateTime::parse_from_rfc3339(&cached.fetched_at) {
                            let age = now.signed_duration_since(fetched);
                            if age.num_hours() < 1 {
                                return Some(cached);
                            }
                        }
                    } else {
                        // Old commits never change - cache indefinitely
                        return Some(cached);
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

/// Parse GitHub Link header to extract the "next" page URL
/// Format: <url>; rel="next", <url>; rel="last"
fn parse_link_header_next(link_header: &str) -> Option<String> {
    for part in link_header.split(',') {
        let part = part.trim();
        if part.contains("rel=\"next\"") {
            // Extract URL between < and >
            if let Some(start) = part.find('<') {
                if let Some(end) = part.find('>') {
                    return Some(part[start + 1..end].to_string());
                }
            }
        }
    }
    None
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
    pub fn get_repo_commits_stats(&self, owner: &str, repo: &str, _since: Option<&str>, _until: Option<&str>) -> Result<Vec<(String, u64, u64, u64, Vec<String>, Vec<String>, Vec<FileStats>)>, String> {
        // Returns: (week, commits, additions, deletions, file_extensions, file_paths, file_stats)
        use chrono::{DateTime, Datelike, Utc};

        let cache = GitHubCache::new();
        let repo_key = format!("{}/{}", owner, repo);

        // Check cache first
        if let Some(cached) = cache.get(&repo_key) {
            // Convert cached data to result format
            let mut weekly_stats: HashMap<String, (u64, u64, u64, Vec<String>, Vec<String>, Vec<FileStats>)> = HashMap::new();
            for commit in cached.commits {
                let entry = weekly_stats.entry(commit.week.clone()).or_insert((0, 0, 0, Vec::new(), Vec::new(), Vec::new()));
                entry.0 += 1;
                entry.1 += commit.additions;
                entry.2 += commit.deletions;
                entry.3.extend(commit.file_extensions);
                entry.4.extend(commit.file_paths);
                entry.5.extend(commit.file_stats);
            }
            let mut result: Vec<_> = weekly_stats
                .into_iter()
                .map(|(week, (commits, additions, deletions, exts, paths, stats))| {
                    (week, commits, additions, deletions, exts, paths, stats)
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
        let mut weekly_stats: HashMap<String, (u64, u64, u64, Vec<String>, Vec<String>, Vec<FileStats>)> = HashMap::new();
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

            // Fetch commit details for additions/deletions with pagination support
            let commit_url = format!(
                "https://api.github.com/repos/{}/{}/commits/{}",
                owner, repo, sha
            );

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

            #[derive(Deserialize, Clone)]
            struct FileDetail {
                filename: String,
                #[serde(default)]
                additions: u64,
                #[serde(default)]
                deletions: u64,
            }

            let mut additions = 0u64;
            let mut deletions = 0u64;
            let mut file_exts = Vec::new();
            let mut file_paths = Vec::new();
            let mut file_stats = Vec::new();
            let mut current_url = Some(commit_url);
            let mut page_count = 0;
            const MAX_PAGES: usize = 10; // Up to 3000 files (300 per page)

            // Fetch all pages of files for this commit
            while let Some(url) = current_url.take() {
                if page_count >= MAX_PAGES {
                    break;
                }
                page_count += 1;

                let mut request = self
                    .client
                    .get(&url)
                    .header(USER_AGENT, "git-activity-dashboard")
                    .header(ACCEPT, "application/vnd.github+json");

                if let Some(token) = &self.token {
                    request = request.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                if let Ok(response) = request.send() {
                    if response.status().is_success() {
                        // Extract Link header before consuming body
                        let next_url = response
                            .headers()
                            .get("link")
                            .and_then(|h| h.to_str().ok())
                            .and_then(parse_link_header_next);

                        if let Ok(commit_data) = response.json::<CommitStats>() {
                            // Only get stats from first page
                            if page_count == 1 {
                                if let Some(stats) = commit_data.stats {
                                    additions = stats.additions;
                                    deletions = stats.deletions;
                                }
                            }

                            if let Some(files) = commit_data.files {
                                for file in &files {
                                    file_paths.push(file.filename.clone());
                                    file_stats.push(FileStats {
                                        path: file.filename.clone(),
                                        additions: file.additions,
                                        deletions: file.deletions,
                                    });
                                    if let Some(ext) = std::path::Path::new(&file.filename)
                                        .extension()
                                        .and_then(|e| e.to_str())
                                    {
                                        file_exts.push(ext.to_string());
                                    }
                                }
                            }

                            // Continue to next page if available
                            current_url = next_url;
                        }
                    }
                }
            }

            // Only store if we got valid data
            if additions > 0 || deletions > 0 || !file_stats.is_empty() {
                // Store in weekly stats
                let entry = weekly_stats.entry(week.clone()).or_insert((0, 0, 0, Vec::new(), Vec::new(), Vec::new()));
                entry.0 += 1;
                entry.1 += additions;
                entry.2 += deletions;
                entry.3.extend(file_exts.clone());
                entry.4.extend(file_paths.clone());
                entry.5.extend(file_stats.clone());

                // Store for cache
                cached_commits.push(CachedCommit {
                    sha: sha.clone(),
                    week,
                    additions,
                    deletions,
                    file_extensions: file_exts,
                    file_paths,
                    file_stats,
                });
            }
        }

        // Save to cache
        let cached_data = CachedRepoCommits {
            repo: repo_key.clone(),
            fetched_at: chrono::Utc::now().to_rfc3339(),
            commits: cached_commits,
            version: GitHubCache::CACHE_VERSION,
        };
        cache.set(&repo_key, &cached_data);

        // Convert to sorted vector
        let mut result: Vec<_> = weekly_stats
            .into_iter()
            .map(|(week, (commits, additions, deletions, exts, paths, stats))| {
                (week, commits, additions, deletions, exts, paths, stats)
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
                    for (week, commit_count, additions, deletions, file_exts, file_paths, file_stats) in commits {
                        let week_entry = all_stats.entry(week.clone()).or_insert_with(HashMap::new);

                        let repo_entry = week_entry.entry(repo.name.clone()).or_insert_with(|| {
                            WeeklyRepoStats {
                                week: week.clone(),
                                repo: repo.name.clone(),
                                commits: 0,
                                additions: 0,
                                deletions: 0,
                                file_types: HashMap::new(),
                                file_paths: Vec::new(),
                                file_stats: Vec::new(),
                            }
                        });

                        repo_entry.commits += commit_count;
                        repo_entry.additions += additions;
                        repo_entry.deletions += deletions;
                        repo_entry.file_paths.extend(file_paths);
                        repo_entry.file_stats.extend(file_stats);

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

    /// Convert week string (YYYY-Www) to month string (YYYY-MM)
    fn week_to_month(week: &str) -> String {
        // Parse week like "2025-W51" and convert to "2025-12"
        if let Some(caps) = week.strip_prefix("20").and_then(|s| {
            let parts: Vec<&str> = s.split("-W").collect();
            if parts.len() == 2 {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        }) {
            let year = format!("20{}", caps.0);
            if let Ok(week_num) = caps.1.parse::<u32>() {
                // Approximate month from week number
                let month = match week_num {
                    1..=4 => 1,
                    5..=8 => 2,
                    9..=13 => 3,
                    14..=17 => 4,
                    18..=22 => 5,
                    23..=26 => 6,
                    27..=30 => 7,
                    31..=35 => 8,
                    36..=39 => 9,
                    40..=44 => 10,
                    45..=48 => 11,
                    _ => 12,
                };
                return format!("{}-{:02}", year, month);
            }
        }
        week.to_string()
    }

    /// Print weekly stats with specified grouping
    pub fn print_weekly_stats(stats: &[WeeklyRepoStats], grouping: &StatsGrouping, limit: Option<usize>) {
        use std::collections::HashMap;

        let period_limit = limit.unwrap_or(20);

        println!("\n{}", "=".repeat(80));
        println!("ACTIVITY BREAKDOWN");
        println!("{}", "=".repeat(80));

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

                println!("\n{:12} {:>8} {:>12} {:>12} {:>12}", "Week", "Commits", "Additions", "Deletions", "Net LOC");
                println!("{}", "-".repeat(60));

                for (week, (commits, additions, deletions)) in sorted.iter().take(period_limit) {
                    let net = *additions as i64 - *deletions as i64;
                    let bar = "█".repeat(std::cmp::min(*commits as usize, 20));
                    println!("{:12} {:>8} {:>+12} {:>12} {:>+12}  {}", week, commits, additions, deletions, net, bar);
                }
            }

            StatsGrouping::Month => {
                // Aggregate by month
                let mut monthly: HashMap<String, (u64, u64, u64)> = HashMap::new();
                for s in stats {
                    let month = Self::week_to_month(&s.week);
                    let entry = monthly.entry(month).or_insert((0, 0, 0));
                    entry.0 += s.commits;
                    entry.1 += s.additions;
                    entry.2 += s.deletions;
                }

                let mut sorted: Vec<_> = monthly.into_iter().collect();
                sorted.sort_by(|a, b| b.0.cmp(&a.0));

                println!("\n{:12} {:>8} {:>12} {:>12} {:>12}", "Month", "Commits", "Additions", "Deletions", "Net LOC");
                println!("{}", "-".repeat(60));

                for (month, (commits, additions, deletions)) in sorted.iter().take(period_limit) {
                    let net = *additions as i64 - *deletions as i64;
                    let bar = "█".repeat(std::cmp::min((*commits / 10) as usize, 20));
                    println!("{:12} {:>8} {:>+12} {:>12} {:>+12}  {}", month, commits, additions, deletions, net, bar);
                }
            }

            StatsGrouping::MonthRepo => {
                // Group by month, then show repos within each month
                let mut by_month: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();
                for s in stats {
                    let month = Self::week_to_month(&s.week);
                    let month_entry = by_month.entry(month).or_insert_with(HashMap::new);
                    let repo_entry = month_entry.entry(s.repo.clone()).or_insert((0, 0, 0));
                    repo_entry.0 += s.commits;
                    repo_entry.1 += s.additions;
                    repo_entry.2 += s.deletions;
                }

                let mut sorted_months: Vec<_> = by_month.keys().cloned().collect();
                sorted_months.sort_by(|a, b| b.cmp(a));

                for month in sorted_months.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Month: {}", month);
                    println!("{}", "-".repeat(80));
                    println!("{:25} {:>8} {:>12} {:>12} {:>12}", "Repository", "Commits", "Additions", "Deletions", "Net LOC");

                    if let Some(repos) = by_month.get(month) {
                        let mut sorted_repos: Vec<_> = repos.iter().collect();
                        sorted_repos.sort_by(|a, b| b.1.0.cmp(&a.1.0)); // Sort by commits

                        for (repo, (commits, additions, deletions)) in sorted_repos.iter().take(10) {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*commits as usize, 15));
                            println!("{:25} {:>8} {:>+12} {:>12} {:>+12}  {}",
                                repo, commits, additions, deletions, net, bar);
                        }
                    }
                }
            }

            StatsGrouping::MonthFileType => {
                // Aggregate by month and file type
                let mut monthly_filetypes: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();
                for s in stats {
                    let month = Self::week_to_month(&s.week);
                    let month_entry = monthly_filetypes.entry(month).or_insert_with(HashMap::new);
                    for (ext, ft) in &s.file_types {
                        let entry = month_entry.entry(ext.clone()).or_insert((0, 0, 0));
                        entry.0 += ft.commits;
                        entry.1 += ft.additions;
                        entry.2 += ft.deletions;
                    }
                }

                let mut sorted_months: Vec<_> = monthly_filetypes.keys().cloned().collect();
                sorted_months.sort_by(|a, b| b.cmp(a));

                for month in sorted_months.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Month: {}", month);
                    println!("{}", "-".repeat(80));
                    println!("{:15} {:>10} {:>12} {:>12} {:>12}", "File Type", "Changes", "Additions", "Deletions", "Net LOC");

                    if let Some(filetypes) = monthly_filetypes.get(month) {
                        let mut sorted_types: Vec<_> = filetypes.iter().collect();
                        sorted_types.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (ext, (changes, additions, deletions)) in sorted_types.iter().take(15) {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*changes as usize, 15));
                            println!(".{:14} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                ext, changes, additions, deletions, net, bar);
                        }
                    }
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

                for week in sorted_weeks.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Week: {}", week);
                    println!("{}", "-".repeat(80));
                    println!("{:15} {:>10} {:>12} {:>12} {:>12}", "File Type", "Changes", "Additions", "Deletions", "Net LOC");

                    if let Some(filetypes) = weekly_filetypes.get(week) {
                        let mut sorted_types: Vec<_> = filetypes.iter().collect();
                        sorted_types.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (ext, (changes, additions, deletions)) in sorted_types.iter().take(15) {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*changes as usize, 15));
                            println!(".{:14} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                ext, changes, additions, deletions, net, bar);
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

                for week in sorted_weeks.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Week: {}", week);
                    println!("{}", "-".repeat(80));
                    println!("{:25} {:>8} {:>12} {:>12} {:>12}", "Repository", "Commits", "Additions", "Deletions", "Net LOC");

                    if let Some(repos) = by_week.get(week) {
                        let mut sorted_repos = repos.clone();
                        sorted_repos.sort_by(|a, b| b.commits.cmp(&a.commits));

                        for repo in sorted_repos.iter().take(10) {
                            let net = repo.additions as i64 - repo.deletions as i64;
                            let bar = "█".repeat(std::cmp::min(repo.commits as usize, 15));
                            println!("{:25} {:>8} {:>+12} {:>12} {:>+12}  {}",
                                repo.repo, repo.commits, repo.additions, repo.deletions, net, bar);
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

                for week in sorted_weeks.iter().take(period_limit) {
                    println!("\n{}", "=".repeat(80));
                    println!("Week: {}", week);
                    println!("{}", "=".repeat(80));

                    if let Some(repos) = by_week.get(week) {
                        let mut sorted_repos = repos.clone();
                        sorted_repos.sort_by(|a, b| b.commits.cmp(&a.commits));

                        for repo in sorted_repos.iter().take(5) {
                            let net = repo.additions as i64 - repo.deletions as i64;
                            println!("\n  {} ({} commits, +{} -{} = {} net):",
                                repo.repo, repo.commits, repo.additions, repo.deletions, net);

                            let mut sorted_types: Vec<_> = repo.file_types.iter().collect();
                            sorted_types.sort_by(|a, b| b.1.commits.cmp(&a.1.commits));

                            for (ext, ft) in sorted_types.iter().take(8) {
                                println!("    .{:15} {:>5} changes", ext, ft.commits);
                            }
                        }
                    }
                }
            }

            StatsGrouping::WeekCategory => {
                // Aggregate by week and file category (docs, specs, tests, config, ci, generated, assets, code)
                let mut weekly_categories: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();
                let mut excluded_count: u64 = 0;

                for s in stats {
                    let week_entry = weekly_categories.entry(s.week.clone()).or_insert_with(HashMap::new);

                    // Use per-file stats for accurate categorization
                    for file_stat in &s.file_stats {
                        let category = FileCategory::from_path(&file_stat.path);
                        if category.is_excluded() {
                            excluded_count += 1;
                            continue;
                        }
                        let entry = week_entry.entry(category.as_str().to_string()).or_insert((0, 0, 0));
                        entry.0 += 1; // file count
                        entry.1 += file_stat.additions;
                        entry.2 += file_stat.deletions;
                    }
                }

                let mut sorted_weeks: Vec<_> = weekly_categories.keys().cloned().collect();
                sorted_weeks.sort_by(|a, b| b.cmp(a));

                if excluded_count > 0 {
                    println!("\n(Excluded {} files: lock files, node_modules, vendor)", excluded_count);
                }

                for week in sorted_weeks.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Week: {}", week);
                    println!("{}", "-".repeat(80));
                    println!("{:15} {:>10} {:>12} {:>12} {:>12}", "Category", "Files", "Additions", "Deletions", "Net LOC");

                    if let Some(categories) = weekly_categories.get(week) {
                        // Sort by file count descending
                        let mut sorted_cats: Vec<_> = categories.iter().collect();
                        sorted_cats.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (cat, (files, additions, deletions)) in sorted_cats {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*files as usize / 2, 15));
                            println!("{:15} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                cat, files, additions, deletions, net, bar);
                        }
                    }
                }
            }

            StatsGrouping::WeekRepoCategory => {
                // Aggregate by week, then repo, then category
                let mut weekly_repo_categories: HashMap<String, HashMap<String, HashMap<String, (u64, u64, u64)>>> = HashMap::new();
                let mut excluded_count: u64 = 0;

                for s in stats {
                    let week_entry = weekly_repo_categories.entry(s.week.clone()).or_insert_with(HashMap::new);
                    let repo_entry = week_entry.entry(s.repo.clone()).or_insert_with(HashMap::new);

                    for file_stat in &s.file_stats {
                        let category = FileCategory::from_path(&file_stat.path);
                        if category.is_excluded() {
                            excluded_count += 1;
                            continue;
                        }
                        let entry = repo_entry.entry(category.as_str().to_string()).or_insert((0, 0, 0));
                        entry.0 += 1;
                        entry.1 += file_stat.additions;
                        entry.2 += file_stat.deletions;
                    }
                }

                let mut sorted_weeks: Vec<_> = weekly_repo_categories.keys().cloned().collect();
                sorted_weeks.sort_by(|a, b| b.cmp(a));

                if excluded_count > 0 {
                    println!("\n(Excluded {} files: lock files, node_modules, vendor)", excluded_count);
                }

                for week in sorted_weeks.iter().take(period_limit) {
                    println!("\n{}", "=".repeat(80));
                    println!("Week: {}", week);
                    println!("{}", "=".repeat(80));

                    if let Some(repos) = weekly_repo_categories.get(week) {
                        // Sort repos by total additions
                        let mut sorted_repos: Vec<_> = repos.iter().collect();
                        sorted_repos.sort_by(|a, b| {
                            let total_a: u64 = a.1.values().map(|v| v.1).sum();
                            let total_b: u64 = b.1.values().map(|v| v.1).sum();
                            total_b.cmp(&total_a)
                        });

                        for (repo, categories) in sorted_repos.iter().take(5) {
                            let total_files: u64 = categories.values().map(|v| v.0).sum();
                            let total_add: u64 = categories.values().map(|v| v.1).sum();
                            let total_del: u64 = categories.values().map(|v| v.2).sum();
                            let total_net = total_add as i64 - total_del as i64;

                            println!("\n  {} ({} files, {:+} LOC)", repo, total_files, total_net);
                            println!("  {:15} {:>10} {:>12} {:>12} {:>12}", "Category", "Files", "Additions", "Deletions", "Net LOC");

                            let mut sorted_cats: Vec<_> = categories.iter().collect();
                            sorted_cats.sort_by(|a, b| b.1.1.cmp(&a.1.1)); // Sort by additions

                            for (cat, (files, additions, deletions)) in sorted_cats {
                                let net = *additions as i64 - *deletions as i64;
                                let bar = "█".repeat(std::cmp::min(*files as usize / 2, 15));
                                println!("  {:15} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                    cat, files, additions, deletions, net, bar);
                            }
                        }
                    }
                }
            }

            StatsGrouping::MonthCategory => {
                // Aggregate by month and file category
                let mut monthly_categories: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();
                let mut excluded_count: u64 = 0;

                for s in stats {
                    let month = Self::week_to_month(&s.week);
                    let month_entry = monthly_categories.entry(month).or_insert_with(HashMap::new);

                    // Use per-file stats for accurate categorization
                    for file_stat in &s.file_stats {
                        let category = FileCategory::from_path(&file_stat.path);
                        if category.is_excluded() {
                            excluded_count += 1;
                            continue;
                        }
                        let entry = month_entry.entry(category.as_str().to_string()).or_insert((0, 0, 0));
                        entry.0 += 1; // file count
                        entry.1 += file_stat.additions;
                        entry.2 += file_stat.deletions;
                    }
                }

                let mut sorted_months: Vec<_> = monthly_categories.keys().cloned().collect();
                sorted_months.sort_by(|a, b| b.cmp(a));

                if excluded_count > 0 {
                    println!("\n(Excluded {} files: lock files, node_modules, vendor)", excluded_count);
                }

                for month in sorted_months.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Month: {}", month);
                    println!("{}", "-".repeat(80));
                    println!("{:15} {:>10} {:>12} {:>12} {:>12}", "Category", "Files", "Additions", "Deletions", "Net LOC");

                    if let Some(categories) = monthly_categories.get(month) {
                        // Sort by file count descending
                        let mut sorted_cats: Vec<_> = categories.iter().collect();
                        sorted_cats.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (cat, (files, additions, deletions)) in sorted_cats {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*files as usize / 5, 15));
                            println!("{:15} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                cat, files, additions, deletions, net, bar);
                        }
                    }
                }
            }

            StatsGrouping::MonthRepoCategory => {
                // Aggregate by month, then repo, then category
                let mut monthly_repo_categories: HashMap<String, HashMap<String, HashMap<String, (u64, u64, u64)>>> = HashMap::new();
                let mut excluded_count: u64 = 0;

                for s in stats {
                    let month = Self::week_to_month(&s.week);
                    let month_entry = monthly_repo_categories.entry(month).or_insert_with(HashMap::new);
                    let repo_entry = month_entry.entry(s.repo.clone()).or_insert_with(HashMap::new);

                    for file_stat in &s.file_stats {
                        let category = FileCategory::from_path(&file_stat.path);
                        if category.is_excluded() {
                            excluded_count += 1;
                            continue;
                        }
                        let entry = repo_entry.entry(category.as_str().to_string()).or_insert((0, 0, 0));
                        entry.0 += 1;
                        entry.1 += file_stat.additions;
                        entry.2 += file_stat.deletions;
                    }
                }

                let mut sorted_months: Vec<_> = monthly_repo_categories.keys().cloned().collect();
                sorted_months.sort_by(|a, b| b.cmp(a));

                if excluded_count > 0 {
                    println!("\n(Excluded {} files: lock files, node_modules, vendor)", excluded_count);
                }

                for month in sorted_months.iter().take(period_limit) {
                    println!("\n{}", "=".repeat(80));
                    println!("Month: {}", month);
                    println!("{}", "=".repeat(80));

                    if let Some(repos) = monthly_repo_categories.get(month) {
                        // Sort repos by total additions
                        let mut sorted_repos: Vec<_> = repos.iter().collect();
                        sorted_repos.sort_by(|a, b| {
                            let total_a: u64 = a.1.values().map(|v| v.1).sum();
                            let total_b: u64 = b.1.values().map(|v| v.1).sum();
                            total_b.cmp(&total_a)
                        });

                        for (repo, categories) in sorted_repos.iter().take(10) {
                            let total_files: u64 = categories.values().map(|v| v.0).sum();
                            let total_add: u64 = categories.values().map(|v| v.1).sum();
                            let total_del: u64 = categories.values().map(|v| v.2).sum();
                            let total_net = total_add as i64 - total_del as i64;

                            println!("\n  {} ({} files, {:+} LOC)", repo, total_files, total_net);
                            println!("  {:15} {:>10} {:>12} {:>12} {:>12}", "Category", "Files", "Additions", "Deletions", "Net LOC");

                            let mut sorted_cats: Vec<_> = categories.iter().collect();
                            sorted_cats.sort_by(|a, b| b.1.1.cmp(&a.1.1)); // Sort by additions

                            for (cat, (files, additions, deletions)) in sorted_cats {
                                let net = *additions as i64 - *deletions as i64;
                                let bar = "█".repeat(std::cmp::min(*files as usize / 5, 15));
                                println!("  {:15} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                    cat, files, additions, deletions, net, bar);
                            }
                        }
                    }
                }
            }

            StatsGrouping::WeekLanguage => {
                // Aggregate by week and programming language
                let mut weekly_langs: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();

                for s in stats {
                    let week_entry = weekly_langs.entry(s.week.clone()).or_insert_with(HashMap::new);

                    // Use per-file stats for accurate language categorization
                    for file_stat in &s.file_stats {
                        if FileCategory::from_path(&file_stat.path).is_excluded() {
                            continue;
                        }
                        let lang = ProgrammingLanguage::from_path(&file_stat.path);
                        let entry = week_entry.entry(lang.as_str().to_string()).or_insert((0, 0, 0));
                        entry.0 += 1;
                        entry.1 += file_stat.additions;
                        entry.2 += file_stat.deletions;
                    }
                }

                let mut sorted_weeks: Vec<_> = weekly_langs.keys().cloned().collect();
                sorted_weeks.sort_by(|a, b| b.cmp(a));

                for week in sorted_weeks.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Week: {}", week);
                    println!("{}", "-".repeat(80));
                    println!("{:15} {:>10} {:>12} {:>12} {:>12}", "Language", "Files", "Additions", "Deletions", "Net LOC");

                    if let Some(langs) = weekly_langs.get(week) {
                        let mut sorted_langs: Vec<_> = langs.iter().collect();
                        sorted_langs.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (lang, (files, additions, deletions)) in sorted_langs.iter().take(15) {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*files as usize / 2, 15));
                            println!("{:15} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                lang, files, additions, deletions, net, bar);
                        }
                    }
                }
            }

            StatsGrouping::MonthLanguage => {
                // Aggregate by month and programming language
                let mut monthly_langs: HashMap<String, HashMap<String, (u64, u64, u64)>> = HashMap::new();

                for s in stats {
                    let month = Self::week_to_month(&s.week);
                    let month_entry = monthly_langs.entry(month).or_insert_with(HashMap::new);

                    // Use per-file stats for accurate language categorization
                    for file_stat in &s.file_stats {
                        if FileCategory::from_path(&file_stat.path).is_excluded() {
                            continue;
                        }
                        let lang = ProgrammingLanguage::from_path(&file_stat.path);
                        let entry = month_entry.entry(lang.as_str().to_string()).or_insert((0, 0, 0));
                        entry.0 += 1;
                        entry.1 += file_stat.additions;
                        entry.2 += file_stat.deletions;
                    }
                }

                let mut sorted_months: Vec<_> = monthly_langs.keys().cloned().collect();
                sorted_months.sort_by(|a, b| b.cmp(a));

                for month in sorted_months.iter().take(period_limit) {
                    println!("\n{}", "-".repeat(80));
                    println!("Month: {}", month);
                    println!("{}", "-".repeat(80));
                    println!("{:15} {:>10} {:>12} {:>12} {:>12}", "Language", "Files", "Additions", "Deletions", "Net LOC");

                    if let Some(langs) = monthly_langs.get(month) {
                        let mut sorted_langs: Vec<_> = langs.iter().collect();
                        sorted_langs.sort_by(|a, b| b.1.0.cmp(&a.1.0));

                        for (lang, (files, additions, deletions)) in sorted_langs.iter().take(15) {
                            let net = *additions as i64 - *deletions as i64;
                            let bar = "█".repeat(std::cmp::min(*files as usize / 5, 15));
                            println!("{:15} {:>10} {:>+12} {:>12} {:>+12}  {}",
                                lang, files, additions, deletions, net, bar);
                        }
                    }
                }
            }
        }

        println!("\n{}\n", "=".repeat(80));
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

    // =====================================================
    // FileCategory Tests
    // =====================================================

    #[test]
    fn test_file_category_excluded_lock_files() {
        // Lock files should be excluded
        assert_eq!(FileCategory::from_path("package-lock.json"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("yarn.lock"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("pnpm-lock.yaml"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("Cargo.lock"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("Gemfile.lock"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("poetry.lock"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("composer.lock"), FileCategory::Excluded);
    }

    #[test]
    fn test_file_category_excluded_node_modules() {
        assert_eq!(FileCategory::from_path("node_modules/lodash/index.js"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("src/node_modules/package/file.js"), FileCategory::Excluded);
    }

    #[test]
    fn test_file_category_excluded_vendor() {
        assert_eq!(FileCategory::from_path("vendor/package/file.php"), FileCategory::Excluded);
        assert_eq!(FileCategory::from_path("src/vendor/lib/code.go"), FileCategory::Excluded);
    }

    #[test]
    fn test_file_category_generated_build_outputs() {
        // Build outputs
        assert_eq!(FileCategory::from_path("dist/bundle.js"), FileCategory::Generated);
        assert_eq!(FileCategory::from_path("build/output.css"), FileCategory::Generated);
        assert_eq!(FileCategory::from_path("out/main.js"), FileCategory::Generated);
        assert_eq!(FileCategory::from_path(".next/static/file.js"), FileCategory::Generated);
        assert_eq!(FileCategory::from_path("target/release/app"), FileCategory::Generated);  // Rust
        assert_eq!(FileCategory::from_path("bin/Debug/App.dll"), FileCategory::Generated);   // C#
        assert_eq!(FileCategory::from_path("bin/Release/App.exe"), FileCategory::Generated); // C#
        assert_eq!(FileCategory::from_path("obj/Debug/file.cs"), FileCategory::Generated);   // C#
        assert_eq!(FileCategory::from_path("__pycache__/module.pyc"), FileCategory::Generated); // Python
    }

    #[test]
    fn test_file_category_generated_minified() {
        assert_eq!(FileCategory::from_path("assets/app.min.js"), FileCategory::Generated);
        assert_eq!(FileCategory::from_path("css/styles.min.css"), FileCategory::Generated);
        assert_eq!(FileCategory::from_path("dist/vendor.bundle.js"), FileCategory::Generated);
    }

    #[test]
    fn test_file_category_ci_github() {
        assert_eq!(FileCategory::from_path(".github/workflows/ci.yml"), FileCategory::CI);
        assert_eq!(FileCategory::from_path(".github/CODEOWNERS"), FileCategory::CI);
        assert_eq!(FileCategory::from_path(".github/dependabot.yml"), FileCategory::CI);
    }

    #[test]
    fn test_file_category_ci_gitlab() {
        assert_eq!(FileCategory::from_path(".gitlab-ci.yml"), FileCategory::CI);
        assert_eq!(FileCategory::from_path(".gitlab/ci/test.yml"), FileCategory::CI);
    }

    #[test]
    fn test_file_category_ci_other() {
        assert_eq!(FileCategory::from_path("Jenkinsfile"), FileCategory::CI);
        assert_eq!(FileCategory::from_path(".travis.yml"), FileCategory::CI);
        assert_eq!(FileCategory::from_path("azure-pipelines.yml"), FileCategory::CI);
        assert_eq!(FileCategory::from_path(".circleci/config.yml"), FileCategory::CI);
    }

    #[test]
    fn test_file_category_docs() {
        assert_eq!(FileCategory::from_path("docs/guide.md"), FileCategory::Docs);
        assert_eq!(FileCategory::from_path("documentation/api.md"), FileCategory::Docs);
        assert_eq!(FileCategory::from_path("README.md"), FileCategory::Docs);
        assert_eq!(FileCategory::from_path("CHANGELOG.md"), FileCategory::Docs);
        assert_eq!(FileCategory::from_path("CONTRIBUTING.md"), FileCategory::Docs);
        assert_eq!(FileCategory::from_path("LICENSE.md"), FileCategory::Docs);
        assert_eq!(FileCategory::from_path("LICENSE"), FileCategory::Docs);
    }

    #[test]
    fn test_file_category_specs() {
        assert_eq!(FileCategory::from_path("specs/api.yaml"), FileCategory::Specs);
        assert_eq!(FileCategory::from_path("spec/feature.md"), FileCategory::Specs);
        assert_eq!(FileCategory::from_path("schemas/user.json"), FileCategory::Specs);
        assert_eq!(FileCategory::from_path("schema/types.graphql"), FileCategory::Specs);
        assert_eq!(FileCategory::from_path("openapi.yaml"), FileCategory::Specs);
        assert_eq!(FileCategory::from_path("swagger.json"), FileCategory::Specs);
    }

    #[test]
    fn test_file_category_tests_folders() {
        assert_eq!(FileCategory::from_path("tests/unit/auth.rs"), FileCategory::Tests);
        assert_eq!(FileCategory::from_path("test/integration/api.py"), FileCategory::Tests);
        assert_eq!(FileCategory::from_path("__tests__/component.test.js"), FileCategory::Tests);
        assert_eq!(FileCategory::from_path("src/__tests__/utils.test.ts"), FileCategory::Tests);
    }

    #[test]
    fn test_file_category_tests_file_patterns() {
        // Rust tests
        assert_eq!(FileCategory::from_path("src/auth_test.rs"), FileCategory::Tests);
        // Python tests
        assert_eq!(FileCategory::from_path("src/test_auth.py"), FileCategory::Tests);
        // JavaScript/TypeScript tests
        assert_eq!(FileCategory::from_path("src/utils.test.ts"), FileCategory::Tests);
        assert_eq!(FileCategory::from_path("src/utils.spec.ts"), FileCategory::Tests);
        assert_eq!(FileCategory::from_path("src/utils_spec.js"), FileCategory::Tests);
    }

    #[test]
    fn test_file_category_config_root() {
        assert_eq!(FileCategory::from_path("tsconfig.json"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("package.json"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("Cargo.toml"), FileCategory::Config);
        assert_eq!(FileCategory::from_path(".eslintrc.json"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("vite.config.ts"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("webpack.config.js"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("tailwind.config.js"), FileCategory::Config);
    }

    #[test]
    fn test_file_category_config_folder() {
        assert_eq!(FileCategory::from_path("config/database.yaml"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("config/settings.json"), FileCategory::Config);
        assert_eq!(FileCategory::from_path("configuration/app.toml"), FileCategory::Config);
    }

    #[test]
    fn test_file_category_code() {
        // Regular source files should be Code
        assert_eq!(FileCategory::from_path("src/main.rs"), FileCategory::Code);
        assert_eq!(FileCategory::from_path("src/app.ts"), FileCategory::Code);
        assert_eq!(FileCategory::from_path("lib/utils.py"), FileCategory::Code);
        assert_eq!(FileCategory::from_path("Controllers/HomeController.cs"), FileCategory::Code);
        assert_eq!(FileCategory::from_path("src/components/Button.tsx"), FileCategory::Code);
    }

    #[test]
    fn test_file_category_is_excluded() {
        assert!(FileCategory::Excluded.is_excluded());
        assert!(!FileCategory::Code.is_excluded());
        assert!(!FileCategory::Tests.is_excluded());
        assert!(!FileCategory::Docs.is_excluded());
    }

    #[test]
    fn test_file_category_as_str() {
        assert_eq!(FileCategory::Code.as_str(), "code");
        assert_eq!(FileCategory::Tests.as_str(), "tests");
        assert_eq!(FileCategory::Docs.as_str(), "docs");
        assert_eq!(FileCategory::Specs.as_str(), "specs");
        assert_eq!(FileCategory::Config.as_str(), "config");
        assert_eq!(FileCategory::CI.as_str(), "ci");
        assert_eq!(FileCategory::Generated.as_str(), "generated");
        assert_eq!(FileCategory::Assets.as_str(), "assets");
        assert_eq!(FileCategory::Excluded.as_str(), "excluded");
    }

    // =====================================================
    // ProgrammingLanguage Tests
    // =====================================================

    #[test]
    fn test_lang_rust() {
        assert_eq!(ProgrammingLanguage::from_path("src/main.rs"), ProgrammingLanguage::Rust);
        assert_eq!(ProgrammingLanguage::from_path("lib.rs"), ProgrammingLanguage::Rust);
    }

    #[test]
    fn test_lang_typescript() {
        assert_eq!(ProgrammingLanguage::from_path("src/app.ts"), ProgrammingLanguage::TypeScript);
        assert_eq!(ProgrammingLanguage::from_path("component.tsx"), ProgrammingLanguage::TypeScript);
        assert_eq!(ProgrammingLanguage::from_path("module.mts"), ProgrammingLanguage::TypeScript);
    }

    #[test]
    fn test_lang_javascript() {
        assert_eq!(ProgrammingLanguage::from_path("index.js"), ProgrammingLanguage::JavaScript);
        assert_eq!(ProgrammingLanguage::from_path("component.jsx"), ProgrammingLanguage::JavaScript);
        assert_eq!(ProgrammingLanguage::from_path("module.mjs"), ProgrammingLanguage::JavaScript);
    }

    #[test]
    fn test_lang_python() {
        assert_eq!(ProgrammingLanguage::from_path("main.py"), ProgrammingLanguage::Python);
        assert_eq!(ProgrammingLanguage::from_path("types.pyi"), ProgrammingLanguage::Python);
    }

    #[test]
    fn test_lang_csharp() {
        assert_eq!(ProgrammingLanguage::from_path("Program.cs"), ProgrammingLanguage::CSharp);
        assert_eq!(ProgrammingLanguage::from_path("script.csx"), ProgrammingLanguage::CSharp);
    }

    #[test]
    fn test_lang_go() {
        assert_eq!(ProgrammingLanguage::from_path("main.go"), ProgrammingLanguage::Go);
    }

    #[test]
    fn test_lang_java_kotlin() {
        assert_eq!(ProgrammingLanguage::from_path("Main.java"), ProgrammingLanguage::Java);
        assert_eq!(ProgrammingLanguage::from_path("App.kt"), ProgrammingLanguage::Kotlin);
        assert_eq!(ProgrammingLanguage::from_path("build.kts"), ProgrammingLanguage::Kotlin);
    }

    #[test]
    fn test_lang_cpp_c() {
        assert_eq!(ProgrammingLanguage::from_path("main.cpp"), ProgrammingLanguage::Cpp);
        assert_eq!(ProgrammingLanguage::from_path("file.cc"), ProgrammingLanguage::Cpp);
        assert_eq!(ProgrammingLanguage::from_path("header.hpp"), ProgrammingLanguage::Cpp);
        assert_eq!(ProgrammingLanguage::from_path("main.c"), ProgrammingLanguage::C);
        assert_eq!(ProgrammingLanguage::from_path("header.h"), ProgrammingLanguage::C);
    }

    #[test]
    fn test_lang_web() {
        assert_eq!(ProgrammingLanguage::from_path("index.html"), ProgrammingLanguage::HTML);
        assert_eq!(ProgrammingLanguage::from_path("styles.css"), ProgrammingLanguage::CSS);
        assert_eq!(ProgrammingLanguage::from_path("styles.scss"), ProgrammingLanguage::CSS);
    }

    #[test]
    fn test_lang_config_markup() {
        assert_eq!(ProgrammingLanguage::from_path("README.md"), ProgrammingLanguage::Markdown);
        assert_eq!(ProgrammingLanguage::from_path("config.yaml"), ProgrammingLanguage::YAML);
        assert_eq!(ProgrammingLanguage::from_path("package.json"), ProgrammingLanguage::JSON);
        assert_eq!(ProgrammingLanguage::from_path("Cargo.toml"), ProgrammingLanguage::TOML);
    }

    #[test]
    fn test_lang_shell() {
        assert_eq!(ProgrammingLanguage::from_path("build.sh"), ProgrammingLanguage::Shell);
        assert_eq!(ProgrammingLanguage::from_path("script.ps1"), ProgrammingLanguage::Shell);
    }

    #[test]
    fn test_lang_is_code() {
        assert!(ProgrammingLanguage::Rust.is_code());
        assert!(ProgrammingLanguage::TypeScript.is_code());
        assert!(ProgrammingLanguage::Python.is_code());
        assert!(!ProgrammingLanguage::Markdown.is_code());
        assert!(!ProgrammingLanguage::YAML.is_code());
        assert!(!ProgrammingLanguage::JSON.is_code());
    }

    #[test]
    fn test_lang_as_str() {
        assert_eq!(ProgrammingLanguage::Rust.as_str(), "Rust");
        assert_eq!(ProgrammingLanguage::TypeScript.as_str(), "TypeScript");
        assert_eq!(ProgrammingLanguage::CSharp.as_str(), "C#");
        assert_eq!(ProgrammingLanguage::Cpp.as_str(), "C++");
        assert_eq!(ProgrammingLanguage::Other("xml".to_string()).as_str(), "xml");
    }
}
