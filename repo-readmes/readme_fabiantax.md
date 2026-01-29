# Git Activity Dashboard

A high-performance Rust library and CLI tool for analyzing git contributions across repositories. Generates detailed analytics, activity summaries, and exports for portfolios, LinkedIn, and README badges.

## Features

### Core Analytics
- **Multi-Repository Analysis** - Analyze single repos or scan directories for all git repositories
- **Author Filtering** - Filter commits by author email or name
- **Contribution Breakdown** - Categorize changes into production code, tests, documentation, infrastructure, config, styling, and more
- **Language Detection** - Automatically detect 30+ programming languages from file extensions
- **Activity Tracking** - Daily, weekly, and monthly activity summaries with per-period breakdowns
- **Incremental Updates** - Support for analyzing only new commits since last analysis

### File Classification
Smart file classification with 11 contribution types:
- **Production Code** - Application source code
- **Tests** - Unit tests, integration tests, e2e tests
- **Documentation** - README, docs, guides, API documentation
- **Specs & Config** - Package configs, CI/CD, linting rules
- **Infrastructure** - Docker, Kubernetes, Terraform, deployment scripts
- **Styling** - CSS, SCSS, Sass, styling frameworks
- **Build Artifacts** - Compiled outputs, binaries, cached files
- **Assets** - Images, fonts, media files
- **Generated** - Lock files, type definitions, minified code
- **Data** - CSV, databases, fixtures, migrations
- **Other** - Unclassified files

### Performance Optimizations
- **SIMD-Accelerated Pattern Matching** - Uses Aho-Corasick automaton for O(n) multi-pattern matching
- **Perfect Hash Functions (PHF)** - O(1) compile-time hash lookups for language/extension detection
- **Gitignore Filtering** - Respects `.gitignore` patterns to skip build artifacts and dependencies
- **Lazy Initialization** - Pattern matchers built once on first use
- **Caching** - Computed stats are cached for repeated access

### Export Formats
Multiple export options for different use cases:
- **JSON** - Full structured data for custom integrations
- **Markdown** - Detailed report with tables and charts
- **LinkedIn** - Social media ready summary with hashtags
- **Portfolio** - Project showcase format for personal websites
- **Badge/Widget** - README-embeddable activity summary

### Platform Support
- **Native CLI** - Fast command-line tool using libgit2 (no shell commands)
- **WebAssembly (WASM)** - Browser-compatible library for web applications
- **Library** - Embeddable Rust crate with full API

## Installation

### From Source
```bash
cd git_activity_dashboard
cargo build --release
```

### Run CLI
```bash
cargo run --release -- --help
```

## CLI Usage

```
git-activity [OPTIONS]

Git Activity Dashboard - Analyze your git contributions across repositories

Options:
  -r, --repos <PATH>        Specific repository paths to analyze
  -s, --scan <DIR>          Scan directory for git repositories
  -d, --depth <N>           Maximum depth when scanning for repos [default: 3]
  -e, --email <EMAIL>       Filter commits by author email
  -a, --author <NAME>       Filter commits by author name
      --max-commits <N>     Maximum commits to analyze per repo
      --json <FILE>         Export to JSON file
      --markdown <FILE>     Export to Markdown file
      --linkedin <FILE>     Export LinkedIn-ready summary
      --portfolio <FILE>    Export project portfolio
      --badge <FILE>        Export README badge/widget
      --all-exports <DIR>   Export all formats to directory
  -q, --quiet               Suppress console output
      --include-ignored     Include files that match .gitignore patterns
  -h, --help                Print help
  -V, --version             Print version
```

### Examples

Analyze current repository:
```bash
git-activity
```

Analyze with author filter:
```bash
git-activity -e "your@email.com"
```

Scan a directory for all repos:
```bash
git-activity -s ~/projects -d 2
```

Export all formats:
```bash
git-activity --all-exports ./exports
```

## Library Usage (Rust)

```rust
use git_activity_dashboard::{GitAnalyzer, analyze_repo, AnalyzeOptions};

let options = AnalyzeOptions {
    author_email: Some("your@email.com".to_string()),
    respect_gitignore: true,
    ..Default::default()
};

let stats = analyze_repo(Path::new("."), &options)?;
let mut analyzer = GitAnalyzer::new(Some("your@email.com".to_string()), None);
analyzer.add_repo_data(stats);

// Get statistics
let total = analyzer.get_total_stats();
println!("Total commits: {}", total.total_commits);
println!("Lines added: {}", total.total_lines_added);

// Get activity
let weekly = analyzer.get_weekly_activity(4);
let monthly = analyzer.get_monthly_activity(6);
```

## WASM Usage (JavaScript)

```javascript
import init, { WasmAnalyzer, classifyFile } from 'git_activity_dashboard';

await init();

const analyzer = new WasmAnalyzer("your@email.com", null);

// Parse git log output (run in your git repo):
// git log --format='%H%x00%an%x00%ae%x00%aI%x00%s' --numstat
analyzer.parseGitLog("my-repo", "/path/to/repo", gitLogOutput);

// Get data
const stats = analyzer.getTotalStats();
const dashboard = analyzer.getDashboardData();

// Export
const markdown = analyzer.exportMarkdown();
const linkedin = analyzer.exportLinkedIn();
```

## Architecture

The codebase follows SOLID principles:

- **Single Responsibility** - Separate modules for analysis, classification, export, and git operations
- **Open/Closed** - Extensible via traits (`Classifier`, `Exporter`, `PeriodStrategy`)
- **Liskov Substitution** - All implementations are substitutable via trait interfaces
- **Interface Segregation** - Small, focused traits for each concern
- **Dependency Inversion** - Core analyzer depends on abstractions, not concrete implementations

### Module Structure
```
src/
├── lib.rs          # Public API and WASM bindings
├── analyzer.rs     # Core GitAnalyzer with statistics
├── classifier.rs   # File classification with SIMD patterns
├── exporters.rs    # Markdown, LinkedIn, Portfolio, Badge exporters
├── git.rs          # Native git operations using libgit2
├── parser.rs       # Git log parsing
├── periods.rs      # Time period strategies (daily/weekly/monthly)
├── traits.rs       # Trait definitions (Classifier, Exporter, etc.)
├── utils.rs        # Utility functions
└── tests/          # Dedicated test modules
```

## Supported Languages

Python, JavaScript, TypeScript, C#, Java, Go, Rust, Ruby, PHP, Swift, Kotlin, Scala, C, C++, Vue, Svelte, HTML, SQL, R, MATLAB, Perl, Lua, Dart, Elm, Elixir, Erlang, Haskell, Clojure, F#, Shell, PowerShell, and more.

## License

MIT

---

## About Me

I'm Fabian. I love solving business problems with technical solutions.

I'm a senior technology leader with 25 years of development experience, 15+ years building scalable applications and 7+ years leading an international development team. Recognized as the "go-to person for complicated code challenges that other team members can't fix."

Most recently at Reptune, transformed technology from startup-phase to enterprise-grade, solving critical stability issues while achieving top-3 global market position.

My approach focuses on removing bottlenecks and dependencies for sustainable growth—both in code architecture and team dynamics—ensuring operations run smoothly even as complexity increases.

I'm seeking Technology leadership roles (CTO, Head of Development, Fractional CTO) or Senior C# Developer positions where complex problems need pragmatic solutions. Open to both permanent positions and fractional engagements (2-4 days/week).

**Ideal Focus Areas:** Start-ups, FinTech, RegTech, Enterprise SaaS, Scale-ups needing technical transformation, AI.

**Location Preference:** Amsterdam, Weesp, Naarden, Almere, Bussum, Hilversum and surrounding areas (public transport-accessible).
