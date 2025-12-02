use clap::Parser;
use git_activity_dashboard::{
    GitAnalyzer, BadgeExporter, LinkedInExporter, MarkdownExporter, PortfolioExporter,
    ParseOptions, GIT_LOG_FORMAT,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    depth: u32,

    /// Filter commits by author email
    #[arg(short, long)]
    email: Option<String>,

    /// Filter commits by author name
    #[arg(short, long)]
    author: Option<String>,

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
}

fn find_git_repos(base_path: &Path, max_depth: u32) -> Vec<PathBuf> {
    let mut repos = Vec::new();
    find_git_repos_recursive(base_path, max_depth, 0, &mut repos);
    repos
}

fn find_git_repos_recursive(path: &Path, max_depth: u32, current_depth: u32, repos: &mut Vec<PathBuf>) {
    if current_depth > max_depth {
        return;
    }

    if path.join(".git").is_dir() {
        repos.push(path.to_path_buf());
        return; // Don't search inside git repos
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') {
                        find_git_repos_recursive(&entry_path, max_depth, current_depth + 1, repos);
                    }
                }
            }
        }
    }
}

fn get_git_log(repo_path: &Path, author_email: &Option<String>, author_name: &Option<String>) -> Option<String> {
    let mut cmd = Command::new("git");
    cmd.current_dir(repo_path)
        .arg("log")
        .arg(format!("--format={}", GIT_LOG_FORMAT))
        .arg("--numstat");

    if let Some(email) = author_email {
        cmd.arg("--author").arg(email);
    } else if let Some(name) = author_name {
        cmd.arg("--author").arg(name);
    }

    let output = cmd.output().ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
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

    for (ctype, count) in &sorted_types {
        let pct = stats.contribution_percentages.get(*ctype).unwrap_or(&0.0);
        let label = match ctype.as_str() {
            "production_code" => "Production Code",
            "tests" => "Tests",
            "documentation" => "Documentation",
            "specs_config" => "Specs & Config",
            "infrastructure" => "Infrastructure",
            "styling" => "Styling",
            _ => "Other",
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

        for (lang, _count) in sorted_langs.iter().take(8) {
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

        for (ext, _count) in sorted_exts.iter().take(10) {
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

fn main() {
    let cli = Cli::parse();

    // Determine which repos to analyze
    let repo_paths: Vec<PathBuf> = if !cli.repos.is_empty() {
        cli.repos.iter().map(|r| r.canonicalize().unwrap_or(r.clone())).collect()
    } else if let Some(scan_path) = &cli.scan {
        let scan_path = scan_path.canonicalize().unwrap_or(scan_path.clone());
        if !scan_path.is_dir() {
            eprintln!("Error: {} is not a directory", scan_path.display());
            std::process::exit(1);
        }
        let repos = find_git_repos(&scan_path, cli.depth);
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
        if cwd.join(".git").is_dir() {
            vec![cwd]
        } else {
            eprintln!("Error: Current directory is not a git repository.");
            eprintln!("Use -r to specify repos or -s to scan a directory.");
            std::process::exit(1);
        }
    };

    // Create analyzer with options
    let parse_options = ParseOptions {
        store_commits: true,  // Need commits for activity views
        legacy_delimiter: false,
    };
    let mut analyzer = GitAnalyzer::new(cli.email.clone(), cli.author.clone())
        .with_options(parse_options);

    // Analyze repos
    for path in &repo_paths {
        if !cli.quiet {
            println!("Analyzing: {}", path.display());
        }

        if let Some(log_output) = get_git_log(path, &cli.email, &cli.author) {
            let repo_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            match analyzer.parse_git_log(&repo_name, &path.to_string_lossy(), &log_output) {
                Ok(_) => {},
                Err(e) => {
                    if !cli.quiet {
                        eprintln!("Warning: Failed to parse {}: {}", repo_name, e);
                    }
                }
            }
        }
    }

    // Cache stats for better performance
    analyzer.cache_stats();

    if analyzer.get_repos().is_empty() {
        eprintln!("No repositories were successfully analyzed.");
        std::process::exit(1);
    }

    // Print summary
    if !cli.quiet {
        print_summary(&analyzer);
    }

    // Handle exports
    if let Some(export_dir) = &cli.all_exports {
        fs::create_dir_all(export_dir).expect("Failed to create export directory");

        let json = serde_json::to_string_pretty(&analyzer.get_dashboard_data()).unwrap_or_default();
        fs::write(export_dir.join("activity.json"), json).expect("Failed to write JSON");
        fs::write(export_dir.join("report.md"), MarkdownExporter::export(&analyzer)).expect("Failed to write Markdown");
        fs::write(export_dir.join("linkedin.txt"), LinkedInExporter::export(&analyzer)).expect("Failed to write LinkedIn");
        fs::write(export_dir.join("portfolio.md"), PortfolioExporter::export(&analyzer)).expect("Failed to write Portfolio");
        fs::write(export_dir.join("badge.md"), BadgeExporter::export(&analyzer)).expect("Failed to write Badge");

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
            fs::write(path, MarkdownExporter::export(&analyzer)).expect("Failed to write Markdown");
            if !cli.quiet {
                println!("Markdown exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.linkedin {
            fs::write(path, LinkedInExporter::export(&analyzer)).expect("Failed to write LinkedIn");
            if !cli.quiet {
                println!("LinkedIn summary exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.portfolio {
            fs::write(path, PortfolioExporter::export(&analyzer)).expect("Failed to write Portfolio");
            if !cli.quiet {
                println!("Portfolio exported to: {}", path.display());
            }
        }

        if let Some(path) = &cli.badge {
            fs::write(path, BadgeExporter::export(&analyzer)).expect("Failed to write Badge");
            if !cli.quiet {
                println!("Badge exported to: {}", path.display());
            }
        }
    }
}
