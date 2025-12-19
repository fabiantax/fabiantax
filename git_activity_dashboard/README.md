# Git Activity Dashboard

A Rust-based tool to analyze git contributions across repositories. Works as both a **CLI tool** and a **WASM module** for JavaScript/TypeScript.

## Features

### Activity Metrics
- Lines added/removed (total and per repo)
- Number of commits (total and per repo)
- Files changed
- Languages/technologies used (breakdown by percentage)

### Contribution Type Breakdown
| Type | Description |
|------|-------------|
| **Production Code** | Core application code |
| **Tests** | Unit, integration, e2e tests |
| **Documentation** | README, docs, markdown |
| **Specs & Config** | OpenAPI, JSON schemas, CI/CD |
| **Infrastructure** | Docker, Terraform, scripts |
| **Styling** | CSS, SCSS, design files |

### Time-Based Views
- Daily activity (last 7 days)
- Weekly activity (last 4 weeks)

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

### CLI Options

| Option | Description |
|--------|-------------|
| `-r, --repos` | Specific repository paths |
| `-s, --scan` | Scan directory for repos |
| `-d, --depth` | Max scan depth (default: 3) |
| `-e, --email` | Filter by author email |
| `-a, --author` | Filter by author name |
| `--json FILE` | Export to JSON |
| `--markdown FILE` | Export to Markdown |
| `--linkedin FILE` | Export LinkedIn summary |
| `--portfolio FILE` | Export portfolio |
| `--badge FILE` | Export README badge |
| `--all-exports DIR` | Export all formats |
| `-q, --quiet` | Suppress output |

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
