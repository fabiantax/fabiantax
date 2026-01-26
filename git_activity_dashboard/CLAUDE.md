# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Claude Code Skills

Project-specific skills are available in `.claude/skills/`:

- `/git-activity` - Analyze git contributions across repositories
- `/github-stats` - Fetch GitHub stats via API (no cloning)
- `/git-snapshot` - Analyze current file state (LOC) vs history
- `/activity-export` - Export all formats (JSON, MD, LinkedIn, etc.)

These skills provide quick access to common git_activity_dashboard workflows.

## Overview

Git Activity Dashboard is a high-performance Rust library and CLI tool for analyzing git contributions across repositories. It supports both native CLI execution and WebAssembly (WASM) for browser integration. The codebase follows SOLID principles with a trait-based architecture for extensibility.

## Development Commands

### Building

```bash
# Build CLI (native)
cargo build --release

# Run CLI without installing
cargo run --release -- --help

# Build WASM module
wasm-pack build --target web --features wasm
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test analyzer_tests
cargo test classifier_tests
cargo test exporters_tests
cargo test git_tests

# Run tests with output
cargo test -- --nocapture

# Run a single test by name
cargo test test_name
```

### Running Examples

```bash
# Fetch GitHub README summaries
cargo run --example fetch_readmes

# Analyze current repo
cargo run --release

# Analyze with author filter
cargo run --release -- -e "your@email.com"

# Scan directory for repos
cargo run --release -- -s ~/projects -d 2

# Export all formats
cargo run --release -- --all-exports ./exports
```

## Exporting Data and Creating Visualizations

### Export Console Output to File

The `--output` flag saves all console output to a file for documentation or sharing:

```bash
# Save full activity breakdown to text file
export GITHUB_TOKEN=$(gh auth token)
cargo run -- --github-stats --since 2025-11-01 --group-by week --output examples/full-activity.txt

# Example: Creating historical activity record
# This creates examples/full-activity-since-nov2025.txt with:
# - Repository list with checkmarks
# - Weekly breakdown table (Week | Commits | Additions | Deletions | Net LOC)
# - Visual bar indicators
#
# Usage for documentation:
# 1. Track activity over time periods
# 2. Create reports for presentations
# 3. Share activity summaries with team
# 4. Archive milestones and achievements
```

### Export JSON for Visualizations

Export data in JSON format for use with external visualization tools:

```bash
# Export weekly stats as JSON
cargo run -- --github-stats --since 2025-11-01 --group-by week --json activity.json

# Export monthly stats
cargo run -- --github-stats --since 2025-11-01 --group-by month --json activity-monthly.json

# Export with multiple groupings
cargo run -- --github-stats --group-by week --json weekly.json
cargo run -- --github-stats --group-by week-repo --json weekly-by-repo.json
cargo run -- --github-stats --group-by lang --json by-language.json
```

### Creating Visual Charts

#### Web-Based Interactive Charts

The `web/` directory contains ready-to-use visualizations:

**1. Bar Charts (web/barcharts.html)**
- Interactive weekly/monthly commit charts
- Moving averages (4-period)
- Color-coded net LOC (green/red)
- Pre-loaded with sample data
- Drag & drop your own JSON files

```bash
# View in browser
firefox web/barcharts.html
# or
chrome web/barcharts.html

# Use your data
# 1. Export JSON: cargo run -- --github-stats --json activity.json
# 2. Open web/barcharts.html
# 3. Drag & drop activity.json into the page
```

**2. Constellation Graph (web/index.html)**
- Network visualization of repos, languages, and contribution types
- Animated floating nodes with glow effects
- Interactive hover for details

```bash
# Generate constellation data
cargo run -- -s ~/projects --json constellation-data.json

# View
firefox web/index.html
```

#### Generating Animated Videos

Create professional animated videos from git activity data for LinkedIn, portfolios, or presentations.

**Quick Prototype with canvas2video (5 minutes)**

```bash
# 1. Create project directory
mkdir git-video && cd git-video
npm init -y
npm install canvas2video canvas

# 2. Export your git data
cd git_activity_dashboard
export GITHUB_TOKEN=$(gh auth token)
cargo run -- --github-stats --since 2025-11-01 --group-by week --json ../git-video/activity.json

# 3. Generate video
cd ../git-video
# Create generate.js script (see below)
node generate.js activity.json git-activity.mp4
```

Example `generate.js`:
```javascript
const { createCanvas } = require('canvas');
const { Canvas2Video } = require('canvas2video');

const data = require('./activity.json');
const canvas = createCanvas(1920, 1080);
const ctx = canvas.getContext('2d');
const video = new Canvas2Video(canvas, { fps: 60 });

// Animate bars growing
for (let frame = 0; frame < 360; frame++) {
  // Clear and draw background
  ctx.fillStyle = '#0a0a0f';
  ctx.fillRect(0, 0, 1920, 1080);

  // Draw animated bars
  data.commits.forEach((commits, i) => {
    const height = (commits / max) * 600 * Math.min(1, frame / 240);
    ctx.fillStyle = '#7dd3fc';
    ctx.fillRect(100 + i * 80, 900 - height, 50, height);
  });

  video.addFrame();
}

video.save('git-activity.mp4');
```

**Advanced Options:**

For more control and professional output, use:
- **Remotion** - React-based video generation
- **Motion Canvas** - TypeScript animations with math
- **Puppeteer** - Screen record web charts

See `.claude/skills/video-charts` for complete examples and `.claude/skills/motion-canvas` for Motion Canvas workflows.

### Example: Complete Visualization Workflow

This workflow created `examples/full-activity-since-nov2025.txt`:

```bash
# Step 1: Export console output to file
export GITHUB_TOKEN=$(gh auth token)
cargo run -- \
  --github-stats \
  --since 2025-11-01 \
  --group-by week \
  --output examples/full-activity-since-nov2025.txt

# Step 2: Export JSON for visualizations
cargo run -- \
  --github-stats \
  --since 2025-11-01 \
  --group-by week \
  --json examples/activity-nov2025.json

# Step 3: Create interactive charts
# Open web/barcharts.html and drag & drop activity-nov2025.json

# Step 4: Generate animated video (optional)
# Use canvas2video, Remotion, or Motion Canvas with activity-nov2025.json
```

**Video Generation Quick Start:**

```bash
# Prototype setup (2 minutes)
cd /tmp
mkdir git-video && cd git-video
npm install canvas2video canvas

# Copy activity data
cp /path/to/git_activity_dashboard/examples/activity-nov2025.json ./activity.json

# Generate video
# Create generate.js (use template above)
node generate.js

# Output: git-activity.mp4 (1920x1080, 60fps, 6 seconds)
```

### LinkedIn-Ready Visualizations

**Optimal Settings:**
- Resolution: 1920x1080 (16:9) or 1080x1920 (9:16 for mobile)
- Duration: 15-30 seconds
- Text size: Minimum 48px
- Colors: Dark background (#0a0a0f), bright accents (cyan #7dd3fc, green #00ff88)

**Export Formats:**
- MP4 - Best for LinkedIn uploads
- PNG sequence - Max quality for post-processing
- GIF - Quick previews (lower quality)

## Architecture

### Core Design Principles

The codebase follows **SOLID principles** with dependency inversion via traits:

- **Single Responsibility** - Each module handles one concern (analysis, classification, export, git ops)
- **Open/Closed** - Extend via traits (`Classifier`, `Exporter`, `PeriodStrategy`)
- **Liskov Substitution** - All trait implementations are fully substitutable
- **Interface Segregation** - Small, focused traits (`Analytics`, `ClassificationRule`)
- **Dependency Inversion** - Core analyzer depends on trait abstractions, not concrete types

### Module Structure

```
src/
├── lib.rs          # Public API, WASM bindings, feature gates
├── analyzer.rs     # GitAnalyzer - core statistics engine
├── classifier.rs   # FileClassifier - SIMD pattern matching for file types
├── exporters.rs    # Markdown, LinkedIn, Portfolio, Badge exporters
├── git.rs          # Native libgit2 operations (non-WASM only)
├── github.rs       # GitHub API integration (non-WASM only)
├── parser.rs       # GitLogParser - parse git log output
├── periods.rs      # Time period strategies (daily/weekly/monthly)
├── traits.rs       # Trait definitions for dependency injection
├── utils.rs        # Utility functions (formatting, sorting)
├── bin/cli.rs      # CLI entry point using clap
└── tests/          # Unit tests for each module
```

### Key Architectural Patterns

**Trait-Based Plugin Architecture:**
- `Classifier` trait enables custom file classification rules
- `Exporter` trait allows new export formats without modifying core
- `PeriodStrategy` trait supports different time aggregations
- `Analytics` trait decouples exporters from `GitAnalyzer`

**Platform Compilation:**
- Native features (`git2`, `walkdir`, `ignore`) are gated with `#[cfg(not(target_arch = "wasm32"))]`
- WASM features (`wasm-bindgen`, `js-sys`) are behind feature flag `wasm`
- GitHub API integration only available in native builds

**Performance Optimizations:**
- **SIMD Pattern Matching** - Aho-Corasick automaton for O(n) multi-pattern file classification
- **Perfect Hash Functions (PHF)** - O(1) compile-time hash lookups for language detection
- **Lazy Initialization** - Pattern matchers built once on first use via `once_cell`
- **Parallel Processing** - `rayon` for concurrent snapshot analysis
- **Gitignore Filtering** - Respects `.gitignore` to skip build artifacts and dependencies

### Data Flow

1. **Input**: Git log parsing (`parser.rs`) or native libgit2 analysis (`git.rs`)
2. **Classification**: Files classified by type and language (`classifier.rs`)
3. **Aggregation**: Statistics computed and cached (`analyzer.rs`)
4. **Export**: Data formatted for different outputs (`exporters.rs`)

### Critical Implementation Details

**Git Log Format Safety:**
- Uses null byte (`\x00`) delimiter to prevent commit message injection
- Format: `%H\x00%an\x00%ae\x00%aI\x00%s` (hash, author, email, date, subject)
- Delimiter constant: `GIT_LOG_DELIMITER = '\x00'`

**File Classification Rules:**
- Production Code, Tests, Documentation, Specs/Config, Infrastructure, Styling, Build Artifacts, Assets, Generated, Data, Other
- Classification logic in `classifier.rs` uses pattern matching (test paths, file extensions, directory names)
- Language detection via perfect hash map (PHF) for 30+ languages

**Incremental Analysis:**
- `last_commit_hash` stored in `RepoStats` for incremental updates
- Use `git log <last_hash>..HEAD` to analyze only new commits

## Features and Capabilities

### CLI Options

- `-r, --repos <PATH>` - Analyze specific repository paths
- `-s, --scan <DIR>` - Scan directory for git repositories
- `-d, --depth <N>` - Maximum scan depth (default: 3)
- `-e, --email <EMAIL>` - Filter by author email
- `-a, --author <NAME>` - Filter by author name
- `--json <FILE>` - Export to JSON
- `--markdown <FILE>` - Export to Markdown report
- `--linkedin <FILE>` - Export LinkedIn summary
- `--portfolio <FILE>` - Export portfolio format
- `--badge <FILE>` - Export README widget
- `--all-exports <DIR>` - Export all formats
- `--include-ignored` - Include gitignored files (disabled by default)
- `--github-stats <USERNAME>` - Fetch GitHub stats without cloning
- `--snapshot` - Analyze current state on disk (not just git history)

### GitHub Integration

Requires `GITHUB_TOKEN` environment variable for API access.

- Fetch repository stats without cloning
- Aggregate by week, month, repo, category, or language
- Combine with local analysis for comprehensive view

## Testing Strategy

Current test coverage is minimal. See `TESTS.md` for comprehensive risk analysis and test scenarios.

**Key Test Areas:**
- Git log parsing (delimiter confusion, malformed input, injection attacks)
- File classification (pattern ambiguity, false positives/negatives)
- Statistics calculation (division by zero, overflow, empty datasets)
- Native git operations (corrupt repos, permissions, large repositories)

## Benchmarking and Performance Tools

Use state-of-the-art tools to measure and optimize performance:

```bash
# Run all benchmarks (uses Criterion)
cargo bench

# Run specific benchmark
cargo bench benchmark_name

# Run benchmarks and save baseline for comparison
cargo bench -- --save-baseline main

# Compare performance against baseline
cargo bench -- --baseline main

# Generate benchmark comparison plots (output in target/criterion)
cargo bench
```

**Key Benchmark Areas:**
- File classification performance (SIMD vs non-SIMD)
- Git log parsing throughput
- Statistics aggregation speed
- Parallel processing scaling (rayon)

**Note**: Uses `criterion` crate for advanced statistical analysis, automatic benchmark detection, and performance regression detection.

## Development Tools

```bash
# Lint code for potential issues
cargo clippy -- -D warnings

# Format code according to Rust style guidelines
cargo fmt

# Check code without building
cargo check

# Generate and open documentation
cargo doc --open

# Run tests with coverage (faster than tarpaulin)
cargo llvm-cov --html

# Generate coverage report for specific tests
cargo llvm-cov --html -- --test analyzer_tests

# Show coverage in terminal
cargo llvm-cov --text

# Audit dependencies for security vulnerabilities
cargo audit
```

**Note**: `cargo-llvm-cov` uses LLVM's native coverage instrumentation and is significantly faster than `cargo-tarpaulin`.

## Common Development Patterns

### Adding a New Export Format

1. Implement `Exporter` trait in `exporters.rs`
2. Add format to CLI options in `bin/cli.rs`
3. Export the type from `lib.rs`

### Adding a New Contribution Type

1. Add variant to `ContributionType` enum in `classifier.rs`
2. Update `FileClassifier::classify()` pattern matching
3. Update `contribution_type_label()` in `utils.rs`

### Adding a New Language

1. Add language to `LANGUAGE_MAP` PHF in `classifier.rs`
2. Ensure file extensions are covered in detection logic

## Repository Context

This is a personal portfolio project by Fabian, a senior technology leader seeking CTO/Head of Development roles. The tool is designed to:
- Generate professional portfolio materials from git history
- Demonstrate code quality practices (testing, documentation ratio)
- Create LinkedIn-ready summaries of technical contributions
- Showcase technical skills across multiple languages and projects

The tool analyzes the parent repository (`fabiantax/fabiantax`) which contains multiple sub-projects demonstrating different technical capabilities.
