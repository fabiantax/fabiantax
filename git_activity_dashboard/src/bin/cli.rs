use clap::Parser;
use git_activity_dashboard::{
    GitAnalyzer, BadgeExporter, LinkedInExporter, MarkdownExporter, PortfolioExporter,
    analyze_repo, find_repos, is_git_repo, AnalyzeOptions,
    GitHubClient, GitHubScanOptions, StatsGrouping,
    parse_date, get_month_range, get_last_month_range,
};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-activity")]
#[command(about = "Git Activity Dashboard - Analyze your git contributions across repositories")]
#[command(version)]
struct Cli {
    /// Specific repository paths to analyze
    #[arg(short, long, value_name = "PATH")]
    repos: Vec<PathBuf>,

    /// Scan directory for git repositories
    #[arg(short, long, value_name = "DIR")]
    scan: Option<PathBuf>,

    /// Maximum depth when scanning for repos
    #[arg(short, long, default_value = "3")]
    depth: usize,

    /// Filter commits by author email
    #[arg(short, long)]
    email: Option<String>,

    /// Filter commits by author name
    #[arg(short, long)]
    author: Option<String>,

    /// Maximum commits to analyze per repo (default: all)
    #[arg(long)]
    max_commits: Option<usize>,

    /// Export to JSON file
    #[arg(long, value_name = "FILE")]
    json: Option<PathBuf>,

    /// Export to Markdown file
    #[arg(long, value_name = "FILE")]
    markdown: Option<PathBuf>,

    /// Export LinkedIn-ready summary
    #[arg(long, value_name = "FILE")]
    linkedin: Option<PathBuf>,

    /// Export project portfolio
    #[arg(long, value_name = "FILE")]
    portfolio: Option<PathBuf>,

    /// Export README badge/widget
    #[arg(long, value_name = "FILE")]
    badge: Option<PathBuf>,

    /// Export all formats to specified directory
    #[arg(long, value_name = "DIR")]
    all_exports: Option<PathBuf>,

    /// Suppress console output
    #[arg(short, long)]
    quiet: bool,

    /// Include files that match .gitignore patterns (disabled by default)
    #[arg(long)]
    include_ignored: bool,

    // ─────────────────────────────────────────────────────────────
    // GitHub Integration (requires GITHUB_TOKEN environment variable)
    // ─────────────────────────────────────────────────────────────

    /// Scan all GitHub repos for a user (stats only, no cloning)
    #[arg(long, value_name = "USERNAME")]
    github_stats: Option<Option<String>>,

    /// Scan GitHub repos and clone missing ones for full analysis
    #[arg(long, value_name = "USERNAME")]
    github_scan: Option<Option<String>>,

    /// Directory to clone GitHub repos into (default: ./github_repos)
    #[arg(long, value_name = "DIR", default_value = "./github_repos")]
    github_clone_dir: PathBuf,

    /// Include forked repositories
    #[arg(long)]
    include_forks: bool,

    /// Include archived repositories
    #[arg(long)]
    include_archived: bool,

    /// Filter repos by activity since date (YYYY-MM-DD or relative: "1 month ago")
    #[arg(long, value_name = "DATE")]
    since: Option<String>,

    /// Filter repos by activity until date (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    until: Option<String>,

    /// Filter to repos with activity in the last month
    #[arg(long)]
    last_month: bool,

    /// Filter to repos with activity in a specific month (e.g., "november", "2024-11")
    #[arg(long, value_name = "MONTH")]
    month: Option<String>,

    /// Group stats by time period: week, week-filetype, week-repo, week-repo-filetype, week-category, week-lang, month, month-repo, month-filetype, month-category, month-lang
    #[arg(long, value_name = "GROUPING")]
    group_by: Option<String>,

    /// Limit number of periods to show (default: 20)
    #[arg(long, value_name = "N")]
    limit: Option<usize>,
}

fn print_summary(analyzer: &GitAnalyzer) {
    let stats = analyzer.get_total_stats();

    println!("\n{}", "=".repeat(60));
    println!("GIT ACTIVITY DASHBOARD");
    println!("{}", "=".repeat(60));

    println!("\nRepositories analyzed: {}", stats.total_repos);
    println!("Total commits: {}", stats.total_commits);
    println!("Lines added: {}", stats.total_lines_added);
    println!("Lines removed: {}", stats.total_lines_removed);
    println!("Files changed: {}", stats.total_files_changed);

    println!("\n{}", "-".repeat(40));
    println!("CONTRIBUTION BREAKDOWN");
    println!("{}", "-".repeat(40));

    let mut sorted_types: Vec<_> = stats.contribution_types.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));

    for (ctype, _) in &sorted_types {
        let pct = stats.contribution_percentages.get(*ctype).unwrap_or(&0.0);
        let label = match ctype.as_str() {
            "productioncode" => "Production Code",
            "tests" => "Tests",
            "documentation" => "Documentation",
            "specsconfig" => "Specs & Config",
            "infrastructure" => "Infrastructure",
            "styling" => "Styling",
            "buildartifacts" => "Build Artifacts",
            "assets" => "Assets",
            "generated" => "Generated",
            "data" => "Data",
            "other" => "Other",
            _ => ctype.as_str(),
        };
        let bar = "█".repeat((*pct / 2.0) as usize);
        println!("  {:20} {:5.1}% {}", label, pct, bar);
    }

    if !stats.languages.is_empty() {
        println!("\n{}", "-".repeat(40));
        println!("PROGRAMMING LANGUAGES");
        println!("{}", "-".repeat(40));

        let mut sorted_langs: Vec<_> = stats.languages.iter().collect();
        sorted_langs.sort_by(|a, b| b.1.cmp(a.1));

        for (lang, _) in sorted_langs.iter().take(8) {
            let pct = stats.language_percentages.get(*lang).unwrap_or(&0.0);
            let bar = "█".repeat((*pct / 2.0) as usize);
            println!("  {:20} {:5.1}% {}", lang, pct, bar);
        }
    }

    // File extensions
    if !stats.file_extensions.is_empty() {
        println!("\n{}", "-".repeat(40));
        println!("FILE TYPES (by extension)");
        println!("{}", "-".repeat(40));

        let mut sorted_exts: Vec<_> = stats.file_extensions.iter().collect();
        sorted_exts.sort_by(|a, b| b.1.cmp(a.1));

        for (ext, _) in sorted_exts.iter().take(10) {
            let pct = stats.file_extension_percentages.get(*ext).unwrap_or(&0.0);
            let bar = "█".repeat((*pct / 2.0) as usize);
            println!("  {:20} {:5.1}% {}", ext, pct, bar);
        }
    }

    // Weekly activity
    let weekly = analyzer.get_weekly_activity(4);
    if !weekly.is_empty() {
        println!("\n{}", "-".repeat(40));
        println!("WEEKLY ACTIVITY");
        println!("{}", "-".repeat(40));

        for week in weekly {
            let total_lines = week.lines_added + week.lines_removed;
            let bar = "█".repeat((week.commits as usize).min(20));
            println!("  {:20} {:3} commits  {:6} lines  {}",
                week.period_label, week.commits, total_lines, bar);
        }
    }

    // Monthly activity
    let monthly = analyzer.get_monthly_activity(6);
    if !monthly.is_empty() {
        println!("\n{}", "-".repeat(40));
        println!("MONTHLY ACTIVITY");
        println!("{}", "-".repeat(40));

        for month in monthly {
            let total_lines = month.lines_added + month.lines_removed;
            let bar = "█".repeat((month.commits as usize / 5).min(20));
            println!("  {:20} {:4} commits  {:7} lines  {}",
                month.period_label, month.commits, total_lines, bar);
        }
    }

    // Repository list with detailed breakdown
    println!("\n{}", "-".repeat(40));
    println!("REPOSITORIES (per-repo breakdown)");
    println!("{}", "-".repeat(40));

    let mut repos: Vec<_> = analyzer.get_repos().to_vec();
    repos.sort_by(|a, b| b.total_commits.cmp(&a.total_commits));

    for repo in repos.iter().take(10) {
        println!("\n  {} ({} commits)", repo.name, repo.total_commits);
        println!("    Lines: +{} / -{}", repo.total_lines_added, repo.total_lines_removed);

        // Top languages for this repo
        if !repo.languages.is_empty() {
            let mut langs: Vec<_> = repo.languages.iter().collect();
            langs.sort_by(|a, b| b.1.cmp(a.1));
            let top_langs: Vec<_> = langs.iter().take(3).map(|(l, _)| l.as_str()).collect();
            println!("    Languages: {}", top_langs.join(", "));
        }

        // Top contribution types for this repo
        if !repo.contribution_types.is_empty() {
            let mut types: Vec<_> = repo.contribution_types.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));
            let top_types: Vec<_> = types.iter().take(3).map(|(t, _)| t.as_str()).collect();
            println!("    Focus: {}", top_types.join(", "));
        }

        // Top file extensions for this repo
        if !repo.file_extensions.is_empty() {
            let mut exts: Vec<_> = repo.file_extensions.iter().collect();
            exts.sort_by(|a, b| b.1.cmp(a.1));
            let top_exts: Vec<_> = exts.iter().take(4).map(|(e, _)| e.as_str()).collect();
            println!("    File types: {}", top_exts.join(", "));
        }
    }

    println!("\n{}\n", "=".repeat(60));
}

fn handle_exports(cli: &Cli, analyzer: &GitAnalyzer) {
    if let Some(export_dir) = &cli.all_exports {
        fs::create_dir_all(export_dir).expect("Failed to create export directory");

        let json = serde_json::to_string_pretty(&analyzer.get_dashboard_data()).unwrap_or_default();
        fs::write(export_dir.join("activity.json"), json).expect("Failed to write JSON");
        fs::write(export_dir.join("report.md"), MarkdownExporter::export(analyzer)).expect("Failed to write Markdown");
        fs::write(export_dir.join("linkedin.txt"), LinkedInExporter::export(analyzer)).expect("Failed to write LinkedIn");
        fs::write(export_dir.join("portfolio.md"), PortfolioExporter::export(analyzer)).expect("Failed to write Portfolio");
        fs::write(export_dir.join("badge.md"), BadgeExporter::export(analyzer)).expect("Failed to write Badge");

        if !cli.quiet {
            println!("All exports saved to: {}", export_dir.display());
        }
    } else {
        if let Some(path) = &cli.json {
            let json = serde_json::to_string_pretty(&analyzer.get_dashboard_data()).unwrap_or_default();
            fs::write(path, json).expect("Failed to write JSON");
            if !cli.quiet {
                println!("JSON exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.markdown {
            fs::write(path, MarkdownExporter::export(analyzer)).expect("Failed to write Markdown");
            if !cli.quiet {
                println!("Markdown exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.linkedin {
            fs::write(path, LinkedInExporter::export(analyzer)).expect("Failed to write LinkedIn");
            if !cli.quiet {
                println!("LinkedIn summary exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.portfolio {
            fs::write(path, PortfolioExporter::export(analyzer)).expect("Failed to write Portfolio");
            if !cli.quiet {
                println!("Portfolio exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.badge {
            fs::write(path, BadgeExporter::export(analyzer)).expect("Failed to write Badge");
            if !cli.quiet {
                println!("Badge exported to: {}", path.display());
            }
        }
    }
}

/// Parse date range from CLI options
fn get_date_range(cli: &Cli) -> (Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>) {
    // --last-month takes priority
    if cli.last_month {
        let (start, end) = get_last_month_range();
        return (Some(start), Some(end));
    }

    // --month takes next priority
    if let Some(ref month) = cli.month {
        if let Some((start, end)) = get_month_range(month) {
            return (Some(start), Some(end));
        } else {
            eprintln!("Warning: Could not parse month '{}', ignoring filter", month);
        }
    }

    // Parse --since and --until
    let since = cli.since.as_ref().and_then(|s| {
        let parsed = parse_date(s);
        if parsed.is_none() {
            eprintln!("Warning: Could not parse date '{}', ignoring --since", s);
        }
        parsed
    });

    let until = cli.until.as_ref().and_then(|s| {
        let parsed = parse_date(s);
        if parsed.is_none() {
            eprintln!("Warning: Could not parse date '{}', ignoring --until", s);
        }
        parsed
    });

    (since, until)
}

fn main() {
    let cli = Cli::parse();

    // ─────────────────────────────────────────────────────────────
    // GitHub Stats Mode (API only, no cloning)
    // ─────────────────────────────────────────────────────────────
    if cli.github_stats.is_some() {
        let username = cli.github_stats.as_ref().and_then(|u| u.clone());

        let client = match GitHubClient::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error creating GitHub client: {}", e);
                std::process::exit(1);
            }
        };

        if !client.is_authenticated() {
            eprintln!("Error: GITHUB_TOKEN environment variable not set.");
            eprintln!("Set it with: export GITHUB_TOKEN=your_token");
            std::process::exit(1);
        }

        let (since, until) = get_date_range(&cli);

        let options = GitHubScanOptions {
            username: username.clone(),
            clone_dir: cli.github_clone_dir.clone(),
            include_forks: cli.include_forks,
            include_archived: cli.include_archived,
            include_private: true,
            skip_clone: true,
            since,
            until,
        };

        // Check if weekly grouping is requested
        if let Some(ref group_by_str) = cli.group_by {
            let grouping = match StatsGrouping::from_str(group_by_str) {
                Some(g) => g,
                None => {
                    eprintln!("Invalid --group-by value: {}", group_by_str);
                    eprintln!("Valid options: week, week-filetype, week-repo, week-repo-filetype, week-category, week-lang, month, month-repo, month-filetype, month-category, month-lang");
                    std::process::exit(1);
                }
            };

            // Get the authenticated user for API calls
            let owner = match client.get_authenticated_user() {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("Error getting authenticated user: {}", e);
                    std::process::exit(1);
                }
            };

            // First get list of repos
            let repos = match client.list_repos(None) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error listing repos: {}", e);
                    std::process::exit(1);
                }
            };

            // Filter repos based on options
            let filtered_repos: Vec<_> = repos
                .into_iter()
                .filter(|r| options.include_forks || !r.fork)
                .filter(|r| options.include_archived || !r.archived)
                .collect();

            println!("Fetching weekly stats for {} repositories...\n", filtered_repos.len());

            match client.get_weekly_stats(&filtered_repos, &grouping, &owner) {
                Ok(weekly_stats) => {
                    if !cli.quiet {
                        GitHubClient::print_weekly_stats(&weekly_stats, &grouping, cli.limit);
                    }

                    // Export to JSON if requested
                    if let Some(path) = &cli.json {
                        let json = serde_json::to_string_pretty(&weekly_stats).unwrap_or_default();
                        fs::write(path, json).expect("Failed to write JSON");
                        if !cli.quiet {
                            println!("JSON exported to: {}", path.display());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching weekly stats: {}", e);
                    std::process::exit(1);
                }
            }

            return;
        }

        // Regular stats (no grouping)
        match client.get_all_repo_stats(username.as_deref(), &options) {
            Ok(stats) => {
                if !cli.quiet {
                    GitHubClient::print_stats_summary(&stats);
                }

                // Export to JSON if requested
                if let Some(path) = &cli.json {
                    let json = serde_json::to_string_pretty(&stats).unwrap_or_default();
                    fs::write(path, json).expect("Failed to write JSON");
                    if !cli.quiet {
                        println!("JSON exported to: {}", path.display());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching GitHub stats: {}", e);
                std::process::exit(1);
            }
        }

        return;
    }

    // ─────────────────────────────────────────────────────────────
    // GitHub Scan Mode (clone and analyze)
    // ─────────────────────────────────────────────────────────────
    if cli.github_scan.is_some() {
        let username = cli.github_scan.as_ref().and_then(|u| u.clone());

        let client = match GitHubClient::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error creating GitHub client: {}", e);
                std::process::exit(1);
            }
        };

        if !client.is_authenticated() {
            eprintln!("Error: GITHUB_TOKEN environment variable not set.");
            eprintln!("Set it with: export GITHUB_TOKEN=your_token");
            std::process::exit(1);
        }

        let (since, until) = get_date_range(&cli);

        let options = GitHubScanOptions {
            username: username.clone(),
            clone_dir: cli.github_clone_dir.clone(),
            include_forks: cli.include_forks,
            include_archived: cli.include_archived,
            include_private: true,
            skip_clone: false,
            since,
            until,
        };

        let scan_result = match client.scan_repos(&options) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error scanning GitHub repos: {}", e);
                std::process::exit(1);
            }
        };

        if !cli.quiet {
            println!("\nScan complete:");
            println!("  Cloned: {}", scan_result.cloned.len());
            println!("  Already existed: {}", scan_result.existing.len());
            println!("  Skipped: {}", scan_result.skipped.len());
            println!("  Failed: {}", scan_result.failed.len());
        }

        // Now analyze the repos
        let mut analyzer = GitAnalyzer::new(cli.email.clone(), cli.author.clone());

        let analyze_options = AnalyzeOptions {
            author_email: cli.email.clone(),
            author_name: cli.author.clone(),
            since_commit: None,
            max_commits: cli.max_commits,
            store_commits: false,
            respect_gitignore: !cli.include_ignored,
        };

        for path in &scan_result.repo_paths {
            if !cli.quiet {
                println!("Analyzing: {}", path.display());
            }

            match analyze_repo(path, &analyze_options) {
                Ok(stats) => {
                    analyzer.add_repo_data(stats);
                }
                Err(e) => {
                    if !cli.quiet {
                        eprintln!("Warning: Failed to analyze {}: {}", path.display(), e);
                    }
                }
            }
        }

        if !analyzer.get_repos().is_empty() {
            analyzer.cache_stats();
            if !cli.quiet {
                print_summary(&analyzer);
            }
            handle_exports(&cli, &analyzer);
        }

        return;
    }

    // ─────────────────────────────────────────────────────────────
    // Local Repository Mode (original behavior)
    // ─────────────────────────────────────────────────────────────

    // Determine which repos to analyze
    let repo_paths: Vec<PathBuf> = if !cli.repos.is_empty() {
        cli.repos.iter().map(|r| r.canonicalize().unwrap_or(r.clone())).collect()
    } else if let Some(scan_path) = &cli.scan {
        let scan_path = scan_path.canonicalize().unwrap_or(scan_path.clone());
        if !scan_path.is_dir() {
            eprintln!("Error: {} is not a directory", scan_path.display());
            std::process::exit(1);
        }

        // Use walkdir via our git module
        let repos = find_repos(&scan_path, cli.depth);
        if repos.is_empty() {
            eprintln!("No git repositories found in {}", scan_path.display());
            std::process::exit(1);
        }
        if !cli.quiet {
            println!("Found {} repositories", repos.len());
        }
        repos
    } else {
        // Default to current directory
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        if is_git_repo(&cwd) {
            vec![cwd]
        } else {
            eprintln!("Error: Current directory is not a git repository.");
            eprintln!("Use -r to specify repos or -s to scan a directory.");
            std::process::exit(1);
        }
    };

    // Create analyzer
    let mut analyzer = GitAnalyzer::new(cli.email.clone(), cli.author.clone());

    // Set up analysis options
    let options = AnalyzeOptions {
        author_email: cli.email.clone(),
        author_name: cli.author.clone(),
        since_commit: None,
        max_commits: cli.max_commits,
        store_commits: false, // Don't store individual commits, just aggregate
        respect_gitignore: !cli.include_ignored, // Skip .gitignore files by default
    };

    // Analyze repos using git2 (no shell!)
    for path in &repo_paths {
        if !cli.quiet {
            println!("Analyzing: {}", path.display());
        }

        match analyze_repo(path, &options) {
            Ok(stats) => {
                analyzer.add_repo_data(stats);
            }
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Warning: Failed to analyze {}: {}", path.display(), e);
                }
            }
        }
    }

    if analyzer.get_repos().is_empty() {
        eprintln!("No repositories were successfully analyzed.");
        std::process::exit(1);
    }

    // Cache stats for better performance
    analyzer.cache_stats();

    // Print summary
    if !cli.quiet {
        print_summary(&analyzer);
    }

    // Handle exports
    handle_exports(&cli, &analyzer);
}
