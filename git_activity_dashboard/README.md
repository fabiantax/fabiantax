# Git Activity Dashboard

A high-performance Rust library and CLI tool for analyzing git contributions across repositories. Generates detailed analytics, activity summaries, and exports for portfolios, LinkedIn, and README badges.

## Features

### Core Analytics
- **Multi-Repository Analysis** - Analyze single repos, scan directories, or fetch from GitHub
- **Author Filtering** - Filter commits by author email or name
- **Contribution Breakdown** - Categorize changes into 11 types (production code, tests, docs, infrastructure, config, styling, build artifacts, assets, generated, data, other)
- **Language Detection** - Automatically detect 30+ programming languages
- **Activity Tracking** - Daily, weekly, and monthly activity summaries
- **Gitignore Filtering** - Respects `.gitignore` patterns (toggle with `--include-ignored`)

### File Classification

| Category | Description |
|----------|-------------|
| Production Code | Application source code |
| Tests | Unit tests, integration tests, e2e tests |
| Documentation | README, docs, guides, API documentation |
| Specs & Config | Package configs, CI/CD, linting rules |
| Infrastructure | Docker, Kubernetes, Terraform, deployment scripts |
| Styling | CSS, SCSS, Sass, styling frameworks |
| Build Artifacts | Compiled outputs, binaries, cached files |
| Assets | Images, fonts, media files |
| Generated | Lock files, type definitions, minified code |
| Data | CSV, databases, fixtures, migrations |
| Other | Unclassified files |

### Export Formats
- **JSON** - Full structured data for custom integrations
- **Markdown** - Detailed report with tables and charts
- **LinkedIn** - Social media ready summary with hashtags
- **Portfolio** - Project showcase format for personal websites
- **Badge/Widget** - README-embeddable activity summary

## Installation

```bash
cd git_activity_dashboard
cargo build --release
```

## CLI Usage

### Basic Options

```
git-activity [OPTIONS]

Options:
  -r, --repos <PATH>        Specific repository paths to analyze
  -s, --scan <DIR>          Scan directory for git repositories
  -d, --depth <N>           Maximum scan depth [default: 3]
  -e, --email <EMAIL>       Filter by author email
  -a, --author <NAME>       Filter by author name
      --max-commits <N>     Maximum commits per repo
      --json <FILE>         Export to JSON
      --markdown <FILE>     Export to Markdown
      --linkedin <FILE>     Export LinkedIn summary
      --portfolio <FILE>    Export portfolio format
      --badge <FILE>        Export README badge
      --all-exports <DIR>   Export all formats
  -q, --quiet               Suppress console output
      --include-ignored     Include gitignored files
      --snapshot            Analyze current file state (LOC) vs commit history
```

### GitHub Integration (requires `GITHUB_TOKEN`)

```bash
# Fetch stats without cloning (fastest)
git-activity --github-stats USERNAME

# Clone and analyze all repos
git-activity --github-scan USERNAME

# GitHub options
  --github-clone-dir <DIR>     Clone directory [default: ./github_repos]
  --include-forks              Include forked repositories
  --include-archived           Include archived repositories
```

### Time-Based Filtering

```bash
# Date range
git-activity --since "2024-01-01" --until "2024-12-31"

# Relative dates
git-activity --since "1 month ago"

# Last month
git-activity --last-month

# Specific month
git-activity --month "2024-11"
git-activity --month "november"
```

### GitHub Stats Grouping

```bash
# Group by various dimensions
git-activity --github-stats USERNAME --group-by week
git-activity --github-stats USERNAME --group-by week-repo-category
git-activity --github-stats USERNAME --group-by month-lang

# Available groupings:
#   repo, category, lang, week, month
#   week-repo, week-category, week-lang, week-filetype
#   week-repo-category, week-repo-filetype
#   month-repo, month-category, month-lang, month-filetype
#   month-repo-category, month-repo-filetype

# Limit results
git-activity --github-stats USERNAME --group-by week --limit 10
```

## Examples

### Analyze current repository
```bash
git-activity
```

### Analyze with author filter
```bash
git-activity -e "your@email.com"
```

### Scan directory for repos
```bash
git-activity -s ~/projects -d 2
```

### Export all formats
```bash
git-activity --all-exports ./exports
```

### GitHub stats for last month
```bash
export GITHUB_TOKEN=your_token
git-activity --github-stats username --last-month
```

### Snapshot current file state
```bash
git-activity -s ~/projects --snapshot
```

## Console Output

The CLI displays:
- Total stats (commits, lines added/removed, files changed)
- Contribution breakdown (by type, with visual bars)
- Programming languages (top 8)
- File types/extensions (top 10)
- Weekly activity (past 4 weeks)
- Monthly activity (past 6 months)
- Per-repo breakdown with top languages/focus/file types

## Platform Support

- **Native CLI** - Fast command-line tool using libgit2 (no shell commands)
- **WebAssembly (WASM)** - Browser-compatible library for web applications
- **Library** - Embeddable Rust crate with full API

## Performance Optimizations

- **SIMD-Accelerated** - Aho-Corasick automaton for O(n) pattern matching
- **Perfect Hash Functions** - O(1) compile-time lookups for language detection
- **Lazy Initialization** - Pattern matchers built once on first use
- **Caching** - Computed stats cached for repeated access

## License

MIT
