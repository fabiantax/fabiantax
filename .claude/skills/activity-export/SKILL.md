---
name: activity-export
description: Export git activity to all formats (JSON, Markdown, LinkedIn, Portfolio, Badge). Use when asked to export or generate reports
argument-hint: [output-directory]
allowed-tools: Bash(cargo:*), Write
---

# Activity Export

Generate all export formats from git activity analysis.

## Building

```bash
cd git_activity_dashboard
cargo build --release
```

## Export All Formats

```bash
cargo run --release -- --all-exports ./exports
```

This creates:
- `activity.json` - Full structured data
- `report.md` - Detailed markdown report
- `linkedin.txt` - LinkedIn-ready summary
- `portfolio.md` - Project portfolio format
- `badge.md` - README widget

## Individual Exports

### JSON:
```bash
cargo run --release -- --json output.json
```

### Markdown:
```bash
cargo run --release -- --markdown report.md
```

### LinkedIn:
```bash
cargo run --release -- --linkedin linkedin.txt
```

### Portfolio:
```bash
cargo run --release -- --portfolio portfolio.md
```

### Badge:
```bash
cargo run --release -- --badge badge.md
```

## With Author Filter

```bash
cargo run --release -- -e "your@email.com" --all-exports ./exports
```
