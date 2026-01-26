---
name: git-activity
description: Analyze git contributions across repositories. Use when asked about git activity, code contributions, commit history, or "show me my activity"
argument-hint: [repos-or-scan-path]
allowed-tools: Bash(cargo:*)
---

# Git Activity Analysis

This skill uses the git_activity_dashboard Rust tool to analyze git contributions.

## Building

First ensure the tool is built:

```bash
cd git_activity_dashboard
cargo build --release
```

## Usage

### Analyze current repository:
```bash
cd git_activity_dashboard
cargo run --release
```

### Analyze with author filter:
```bash
cargo run --release -- -e "your@email.com"
```

### Scan directory for repos:
```bash
cargo run --release -- -s ~/projects -d 2
```

### Export all formats:
```bash
cargo run --release -- --all-exports ./exports
```

## What it shows

- Total commits, lines added/removed, files changed
- Contribution breakdown (code, tests, docs, infrastructure, etc.)
- Programming languages
- Weekly and monthly activity
- Per-repo breakdown
