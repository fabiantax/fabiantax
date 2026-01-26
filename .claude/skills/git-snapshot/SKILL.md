---
name: git-snapshot
description: Analyze current file state (LOC) instead of commit history. Use when asked about current code state, file counts, or LOC analysis
argument-hint: [scan-path]
allowed-tools: Bash(cargo:*)
---

# Git Snapshot

Analyze the current state of files on disk, counting lines of code per category. This is different from commit history analysis.

## Building

```bash
cd git_activity_dashboard
cargo build --release
```

## Usage

### Snapshot current repo:
```bash
cargo run --release -- --snapshot
```

### Snapshot with scan:
```bash
cargo run --release -- -s ~/projects -d 2 --snapshot
```

### GitHub scan with snapshot:
```bash
export GITHUB_TOKEN=your_token
cargo run --release -- --github-scan USERNAME --snapshot
```

## What it shows

- Current LOC (lines of code) per category
- Production code vs tests vs documentation
- File counts by type
- Repository breakdown

The snapshot mode ignores git history and analyzes what's actually on disk right now.
