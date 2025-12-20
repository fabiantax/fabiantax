# Git Activity Dashboard

A Rust-based tool to analyze git contributions across repositories. Works as both a **CLI tool** and a **WASM module** for JavaScript/TypeScript.

## Features

### Activity Metrics
- Lines added/removed (total and per repo)
- Number of commits (total and per repo)
- Files changed
- Languages/technologies used (breakdown by percentage)

### GitHub Stats (No Cloning Required)
- Fetch stats directly from GitHub API
- Multiple grouping options for analysis
- Per-file additions/deletions with pagination support

### Current State Snapshot
- Analyze actual LOC per category on disk
- Respects .gitignore patterns
- Parallel processing with rayon

### Contribution Type Breakdown
| Type | Description |
|------|-------------|
| **Code** | Core application code |
| **Tests** | Unit, integration, e2e tests |
| **Docs** | README, docs, markdown |
| **Specs** | OpenAPI, JSON schemas |
| **Config** | Package.json, tsconfig, etc. |
| **CI** | GitHub Actions, Jenkinsfile |
| **Generated** | Build outputs, minified files |
| **Assets** | Images, static files |

### Time-Based Views
- Daily activity (last 7 days)
- Weekly activity (last 4 weeks)
- Monthly activity breakdown

### Export Formats
- **JSON** - Raw data for custom integrations
- **Markdown** - Full activity report
- **LinkedIn** - Post-ready summary
- **Portfolio** - Professional project list for employers
- **README Badge** - Embeddable widget

## Installation

### CLI (Rust)

```bash
# Build the CLI
cd git_activity_dashboard
cargo build --release --features cli

# The binary will be at target/release/git-activity
```

### WASM (for TypeScript/JavaScript)

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM module
cd git_activity_dashboard
wasm-pack build --target web --features wasm

# Or for Node.js
wasm-pack build --target nodejs --features wasm
```

## CLI Usage

### Local Repository Analysis

```bash
# Analyze current repository
git-activity

# Analyze specific repositories
git-activity -r ~/projects/repo1 ~/projects/repo2

# Scan a directory for all git repos
git-activity -s ~/projects

# Filter by your email (useful for shared repos)
git-activity -s ~/projects -e your@email.com

# Export to different formats
git-activity -s ~/projects --json activity.json
git-activity -s ~/projects --markdown report.md
git-activity -s ~/projects --portfolio portfolio.md
git-activity -s ~/projects --linkedin linkedin.txt

# Export all formats at once
git-activity -s ~/projects --all-exports ./exports/
```

### GitHub Stats (No Cloning)

```bash
# Set your GitHub token
export GITHUB_TOKEN=your_token

# Weekly overview
git-activity --github-stats --group-by week --limit 12

# Monthly by repository
git-activity --github-stats --group-by month-repo --limit 6

# Total LOC per category (all time)
git-activity --github-stats --group-by category

# Total LOC per repository
git-activity --github-stats --group-by repo --limit 20

# Weekly breakdown by category
git-activity --github-stats --group-by week-category

# Monthly breakdown by language
git-activity --github-stats --group-by month-lang
```

### Current State Snapshot

```bash
# Analyze actual LOC on disk (respects .gitignore)
git-activity -r ~/projects/myrepo --snapshot

# Scan multiple repos and get current state
git-activity -s ~/projects --snapshot

# With GitHub scan (clones repos first)
git-activity --github-scan --snapshot
```

### CLI Options

| Option | Description |
|--------|-------------|
| `-r, --repos` | Specific repository paths |
| `-s, --scan` | Scan directory for repos |
| `-d, --depth` | Max scan depth (default: 3) |
| `-e, --email` | Filter by author email |
| `-a, --author` | Filter by author name |
| `--github-stats` | Fetch stats from GitHub API (no cloning) |
| `--github-scan` | Clone repos and analyze |
| `--group-by` | Grouping for stats (see below) |
| `--limit` | Limit results (default: 20) |
| `--snapshot` | Analyze current file state instead of commits |
| `--json FILE` | Export to JSON |
| `--markdown FILE` | Export to Markdown |
| `--linkedin FILE` | Export LinkedIn summary |
| `--portfolio FILE` | Export portfolio |
| `--badge FILE` | Export README badge |
| `--all-exports DIR` | Export all formats |
| `-q, --quiet` | Suppress output |

### Grouping Options

| Grouping | Description |
|----------|-------------|
| `repo` | Total LOC per repository (all time) |
| `category` | Total LOC per category (code, tests, docs, etc.) |
| `lang` | Total LOC per language |
| `week` | Weekly totals |
| `week-repo` | Weekly breakdown by repository |
| `week-category` | Weekly breakdown by category |
| `week-repo-category` | Weekly per repo with category breakdown |
| `week-lang` | Weekly breakdown by language |
| `month` | Monthly totals |
| `month-repo` | Monthly breakdown by repository |
| `month-category` | Monthly breakdown by category |
| `month-repo-category` | Monthly per repo with category breakdown |
| `month-lang` | Monthly breakdown by language |

## TypeScript/JavaScript Usage

```typescript
import { WasmAnalyzer } from 'git-activity-dashboard';
import { execSync } from 'child_process';

// Create analyzer
const analyzer = new WasmAnalyzer('your@email.com', null);

// Get git log from a repo
const gitLog = execSync(
  "git log --format='%H|%an|%ae|%aI|%s' --numstat",
  { cwd: '/path/to/repo', encoding: 'utf-8' }
);

// Parse the log
analyzer.parseGitLog('repo-name', '/path/to/repo', gitLog);

// Get stats
const stats = analyzer.getTotalStats();
console.log(`Commits: ${stats.total_commits}`);
console.log(`Test coverage: ${stats.contribution_percentages.tests}%`);

// Get activity
const weekly = analyzer.getWeeklyActivity(4);
weekly.forEach(w => console.log(`${w.period_label}: ${w.commits} commits`));

// Export
const markdown = analyzer.exportMarkdown();
const linkedin = analyzer.exportLinkedIn();
const portfolio = analyzer.exportPortfolio();
const json = analyzer.exportJson();
```

## Example Output

### Console Summary
```
============================================================
GIT ACTIVITY DASHBOARD
============================================================

Repositories analyzed: 5
Total commits: 1,234
Lines added: 45,678
Lines removed: 12,345
Files changed: 890

----------------------------------------
CONTRIBUTION BREAKDOWN
----------------------------------------
  Production Code      65.2% ████████████████
  Tests                18.5% █████████
  Documentation         8.3% ████
  Specs & Config        5.0% ██
  Infrastructure        3.0% █
```

### LinkedIn Export
```
🚀 My Developer Activity This Week

📊 42 commits
💻 3,456 lines of code
📁 3 active repos

Code Quality:
  ✅ Tests: 18.5%
  📝 Documentation: 8.3%

🔧 Top Languages: TypeScript, Rust, Python

#coding #developer #programming #softwareengineering
```

### GitHub Stats Output
```
================================================================================
ACTIVITY BREAKDOWN
================================================================================

Repository                      Commits    Additions    Deletions      Net LOC
--------------------------------------------------------------------------------
my-awesome-app                       99      +537899         6157      +531742
api-service                          42      +360298       177905      +182393
frontend-dashboard                   87      +336652       203818      +132834

Category                  Files    Additions    Deletions      Net LOC
----------------------------------------------------------------------
code                      18745     +3167051       563619     +2603432  ████████████████████
docs                       5762     +2025016       195885     +1829131  ████████████████████
tests                      2886      +486513       250849      +235664  █████████
```

### Snapshot Output
```
================================================================================
CURRENT STATE SNAPSHOT (Main Branch LOC)
================================================================================

Total: 156 files, 45.2K lines of code

BY CATEGORY
--------------------------------------------------------------------------------
Category                    Files           Lines
code                           98           32.1K  ███████████████████
tests                          32            8.5K  ████████
docs                           18            3.2K  ███
config                          8            1.4K  █

BY LANGUAGE
--------------------------------------------------------------------------------
Language                    Files           Lines
Rust                           45           25.3K  ████████████████████
TypeScript                     38           12.8K  ██████████
Markdown                       18            3.2K  ██
```

## Use Cases

1. **Freelancers** - Generate portfolio documents for client pitches
2. **Job Seekers** - Show code quality practices to employers
3. **Personal Tracking** - Monitor productivity and habits
4. **LinkedIn Content** - Share weekly activity summaries
5. **GitHub Profile** - Embed activity widgets in README

## Architecture

```
git_activity_dashboard/
├── Cargo.toml           # Rust config with cli/wasm features
├── src/
│   ├── lib.rs           # Main library + WASM bindings
│   ├── analyzer.rs      # Git analysis logic
│   ├── classifier.rs    # File type classification
│   ├── exporters.rs     # Export formatters
│   └── bin/
│       └── cli.rs       # CLI binary
├── pkg/                 # npm package config
│   ├── package.json
│   └── *.d.ts          # TypeScript definitions
└── typescript/
    └── example.ts       # Usage example
```

## License

MIT
