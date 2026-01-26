---
name: github-stats
description: Fetch GitHub user stats without cloning. Use when asked about GitHub activity, user stats, or repository stats
argument-hint: [username]
allowed-tools: Bash(cargo:*)
---

# GitHub Stats

Fetch GitHub repository statistics via API without cloning. Requires `GITHUB_TOKEN` environment variable.

## Setup

```bash
export GITHUB_TOKEN=your_token
```

## Usage

### Basic stats for user:
```bash
cd git_activity_dashboard
cargo run --release -- --github-stats USERNAME
```

### Stats for last month:
```bash
cargo run --release -- --github-stats USERNAME --last-month
```

### Date range:
```bash
cargo run --release -- --github-stats USERNAME --since "2024-01-01" --until "2024-12-31"
```

### With grouping:
```bash
cargo run --release -- --github-stats USERNAME --group-by week --limit 10
```

## Grouping options

repo, category, lang, week, month, week-repo, week-category, week-lang, week-filetype, week-repo-category, week-repo-filetype, month-repo, month-category, month-lang, month-filetype, month-repo-category, month-repo-filetype

## Export to JSON

```bash
cargo run --release -- --github-stats USERNAME --json stats.json
```
