# Git Activity Dashboard

A Rust-based tool to analyze git contributions across repositories.

## About

This tool provides insights into your coding activity:

- **Activity Metrics** - Lines added/removed, commits, files changed
- **GitHub Stats** - Fetch stats directly from GitHub API without cloning
- **Current State Snapshot** - Analyze actual LOC per category on disk
- **Multiple Groupings** - View by week, month, repo, category, or language
- **Export Formats** - JSON, Markdown, LinkedIn summary, Portfolio

### Contribution Categories

| Category | Description |
|----------|-------------|
| Code | Core application code |
| Tests | Unit, integration, e2e tests |
| Docs | README, documentation, markdown |
| Specs | OpenAPI, JSON schemas |
| Config | Package.json, tsconfig, etc. |
| CI | GitHub Actions, Jenkinsfile |
| Generated | Build outputs, minified files |
| Assets | Images, static files |

## License

MIT
