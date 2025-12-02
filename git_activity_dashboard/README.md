# Git Activity Dashboard & Portfolio Generator

A Python tool to analyze your git contributions across repositories, providing insights and shareable content for professional networking and freelance opportunities.

## Features

### Activity Metrics
- Lines added/removed (total and per repo)
- Number of commits (total and per repo)
- Files changed
- Languages/technologies used (breakdown by percentage)
- Most active repositories

### Contribution Type Breakdown
- **Production Code**: Core application code (%)
- **Tests**: Unit tests, integration tests, e2e tests (%)
- **Documentation**: README, docs/, markdown files (%)
- **Specs/Configs**: OpenAPI, JSON schemas, CI/CD configs (%)
- **Infrastructure**: Dockerfiles, Terraform, deployment scripts (%)
- **Styling**: CSS, SCSS, design-related files (%)

### Time-Based Views
- Daily activity (last 7 days)
- Weekly activity (last 4 weeks)
- Custom date ranges

### Export Formats
- **JSON**: Raw data for custom integrations
- **Markdown**: Full activity report
- **LinkedIn**: Post-ready summary
- **Portfolio**: Professional project list for employers/clients
- **README Badge**: Embeddable widget for GitHub profile

## Installation

No external dependencies required - uses only Python standard library.

```bash
# Clone or copy the git_activity_dashboard folder to your project
```

## Usage

### Basic Usage

```bash
# Analyze current repository
python -m git_activity_dashboard

# Analyze specific repositories
python -m git_activity_dashboard -r ~/projects/repo1 ~/projects/repo2

# Scan a directory for all git repos
python -m git_activity_dashboard -s ~/projects

# Filter by your email (useful for shared repos)
python -m git_activity_dashboard -s ~/projects -e your@email.com
```

### Export Options

```bash
# Export to JSON
python -m git_activity_dashboard -s ~/projects --json activity.json

# Export markdown report
python -m git_activity_dashboard -s ~/projects --markdown report.md

# Export portfolio for employers
python -m git_activity_dashboard -s ~/projects --portfolio portfolio.md

# Export LinkedIn-ready summary
python -m git_activity_dashboard -s ~/projects --linkedin linkedin.txt

# Export README badge/widget
python -m git_activity_dashboard -s ~/projects --badge badge.md

# Export all formats at once
python -m git_activity_dashboard -s ~/projects --all-exports ./exports/
```

### Options

| Option | Description |
|--------|-------------|
| `-r, --repos` | Specific repository paths to analyze |
| `-s, --scan` | Scan directory for git repositories |
| `-d, --depth` | Maximum depth when scanning (default: 3) |
| `-e, --email` | Filter commits by author email |
| `-a, --author` | Filter commits by author name |
| `--json FILE` | Export to JSON file |
| `--markdown FILE` | Export to Markdown file |
| `--linkedin FILE` | Export LinkedIn-ready summary |
| `--portfolio FILE` | Export project portfolio |
| `--badge FILE` | Export README badge/widget |
| `--all-exports DIR` | Export all formats to directory |
| `-q, --quiet` | Suppress console output |

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

### Portfolio Export

The portfolio export generates a professional document listing:
- All your projects with descriptions
- Technologies used in each project
- Your contribution metrics per project
- Duration of involvement
- Quality indicators (test coverage, documentation ratio)

Perfect for sharing with potential clients or employers!

## Use Cases

1. **Freelancers**: Generate portfolio documents for client pitches
2. **Job Seekers**: Show code quality practices to potential employers
3. **Personal Tracking**: Monitor your own productivity and habits
4. **LinkedIn Content**: Share weekly activity summaries
5. **GitHub Profile**: Embed activity widgets in your README

## License

MIT License
