# Git Activity Dashboard - Comprehensive Test Plan & Risk Analysis

**Generated:** 2025-12-18
**Purpose:** Risk analysis by persona with BDD scenarios and test coverage plan

---

## Executive Summary

This document identifies risks, edge cases, and required test coverage for the Git Activity Dashboard across four key user personas. Current test coverage is **minimal** (only 1 test in `git.rs`). To achieve 80%+ coverage and production readiness, we need comprehensive unit, integration, and edge-case testing.

**Critical Risk Areas:**
1. **Git log parsing** - Delimiter confusion, malformed input, injection attacks
2. **File classification** - Pattern ambiguity, false positives/negatives
3. **Statistics calculation** - Division by zero, overflow, empty datasets
4. **Native git operations** - Corrupt repos, permissions, memory issues with large repos

---

## Table of Contents

1. [Persona Risk Matrix](#persona-risk-matrix)
2. [Persona Analysis & User Stories](#persona-analysis--user-stories)
   - [Freelance Developer](#1-freelance-developer-primary-user)
   - [Hiring Manager](#2-hiring-manager)
   - [Open Source Contributor](#3-open-source-contributor)
   - [Enterprise Developer](#4-enterprise-developer)
3. [Module-by-Module Risk Analysis](#module-by-module-risk-analysis)
4. [BDD Scenarios (Given/When/Then)](#bdd-scenarios-givenwhenthenprioritized)
5. [Test Cases for 80%+ Coverage](#test-cases-for-80-coverage)
6. [Edge Cases Requiring Special Attention](#edge-cases-requiring-special-attention)

---

## Persona Risk Matrix

| Risk Category | Freelance Dev | Hiring Mgr | OSS Contrib | Enterprise Dev | Severity |
|---------------|--------------|------------|-------------|----------------|----------|
| **Incorrect stats** (inflated/deflated numbers) | HIGH | CRITICAL | MEDIUM | HIGH | CRITICAL |
| **Portfolio misrepresentation** (wrong language %) | HIGH | CRITICAL | LOW | MEDIUM | HIGH |
| **Privacy leak** (corporate code patterns) | LOW | LOW | LOW | CRITICAL | CRITICAL |
| **Performance** (hangs on large repos) | MEDIUM | LOW | HIGH | HIGH | HIGH |
| **Data corruption** (crash loses work) | MEDIUM | LOW | HIGH | MEDIUM | MEDIUM |
| **False classification** (test code as prod) | HIGH | HIGH | MEDIUM | MEDIUM | HIGH |
| **Export formatting** (broken markdown) | MEDIUM | HIGH | LOW | LOW | MEDIUM |
| **Missing author attribution** (filters wrong commits) | HIGH | HIGH | CRITICAL | MEDIUM | CRITICAL |
| **Injection vulnerability** (malicious commit msgs) | LOW | LOW | MEDIUM | HIGH | HIGH |
| **Cross-platform issues** (Windows path issues) | MEDIUM | LOW | MEDIUM | MEDIUM | MEDIUM |

**Legend:**
- **CRITICAL**: Can cause job loss, legal issues, or data breach
- **HIGH**: Major user-facing problems, incorrect results
- **MEDIUM**: Annoyances, workarounds available
- **LOW**: Minor cosmetic issues

---

## Persona Analysis & User Stories

### 1. Freelance Developer (Primary User)

**Profile:** Self-employed developer creating portfolio materials for client pitches and job applications.

**Goals:**
- Generate professional portfolio documents showing expertise
- Demonstrate code quality practices (testing, documentation)
- Quickly export stats for LinkedIn posts
- Showcase diverse technical skills

**User Stories:**

#### US-1.1: Portfolio Generation
```gherkin
Given I am a freelance developer with 10 client projects
When I run the tool with --portfolio flag
Then I should see accurate stats for each project
And languages should be correctly identified
And contribution types should highlight code quality
```

#### US-1.2: LinkedIn Weekly Summary
```gherkin
Given I want to post my weekly progress on LinkedIn
When I run the tool with --linkedin flag
Then I should get a concise, shareable summary
And it should include current week's metrics
And it should highlight quality indicators (tests, docs)
```

#### US-1.3: Author Attribution
```gherkin
Given I work on shared repositories with my team
When I filter by my email address
Then I should only see MY contributions
And team members' commits should be excluded
And the stats should accurately reflect my work
```

**Risk Areas:**
1. **Stat Inflation Risk** - If tool overreports, could damage credibility with employers
2. **Classification Errors** - Test code counted as production inflates perceived productivity
3. **Language Detection Gaps** - Missing languages makes skills seem narrower
4. **Export Formatting** - Broken markdown in portfolio makes them look unprofessional

**Failure Modes:**
- Tool crashes on certain repo structure → Can't meet client deadline
- Stats show 0% tests → Appears to not follow best practices
- Author filter fails → Credits others' work or vice versa
- Memory overflow on large project → Can't analyze important portfolio piece

---

### 2. Hiring Manager

**Profile:** Technical recruiter or engineering manager reviewing candidate portfolios.

**Goals:**
- Quickly assess candidate's technical breadth
- Verify code quality practices (testing, documentation)
- Understand contribution patterns
- Validate claims on resume

**User Stories:**

#### US-2.1: Code Quality Assessment
```gherkin
Given I receive a portfolio markdown from a candidate
When I review the contribution breakdown
Then I should see what % is production vs tests vs docs
And I should be able to verify these match the repos
And the stats should be verifiable against actual git history
```

#### US-2.2: Skill Verification
```gherkin
Given a candidate claims expertise in "Rust, TypeScript, Python"
When I look at the language breakdown
Then I should see accurate percentages for each language
And the percentages should match the actual codebase
And no languages should be missing or misclassified
```

#### US-2.3: Contribution Pattern Analysis
```gherkin
Given I want to see the candidate's consistency
When I review the monthly activity section
Then I should see accurate commit counts per month
And date ranges should be correct
And there should be no phantom contributions
```

**Risk Areas:**
1. **Misrepresentation Risk** - False stats could lead to bad hires
2. **Verification Difficulty** - Can't easily validate claimed stats
3. **Classification Confusion** - Config files counted as "infrastructure expertise" misleading
4. **Time Period Errors** - Date bugs make experience seem longer/shorter

**Failure Modes:**
- Candidate inflates test coverage → Hire someone who doesn't test
- Language percentages wrong → Hire Python dev for Rust role
- Commit counts inflated → Pay premium for inflated productivity
- Date ranges incorrect → Experience level misrepresented

---

### 3. Open Source Contributor

**Profile:** Developer contributing to multiple OSS projects, wants to track impact across many repos.

**Goals:**
- Analyze contributions across dozens/hundreds of repos
- Track activity over long time periods
- Generate badges for GitHub profile
- Monitor contribution diversity

**User Stories:**

#### US-3.1: Multi-Repo Analysis
```gherkin
Given I contribute to 50+ open source repositories
When I scan my ~/projects directory recursively
Then the tool should find all git repos
And analyze them without running out of memory
And complete in reasonable time (< 5 minutes)
```

#### US-3.2: Incremental Updates
```gherkin
Given I analyzed all repos yesterday
When I want to update with today's commits
Then I should be able to do incremental analysis
And it should only process new commits since last run
And it should merge with previous stats correctly
```

#### US-3.3: Large Repository Handling
```gherkin
Given I contribute to Linux kernel (1M+ commits)
When I run the analyzer with my email filter
Then it should handle the massive history gracefully
And not crash with out-of-memory errors
And should provide accurate stats for my commits only
```

**Risk Areas:**
1. **Performance/Scalability** - Hangs or crashes on large/many repos
2. **Memory Issues** - OOM on repos with massive history
3. **Incorrect Merging** - Incremental updates corrupt existing data
4. **Author Filtering** - Misses contributions with different email/name variants

**Failure Modes:**
- Crash on 100th repo → Can't get complete picture
- Takes hours to run → Unusable for regular updates
- OOM on large repo → Can't include major contribution
- Misses commits → Underreports contribution impact
- Incremental update corrupts data → Have to restart analysis

---

### 4. Enterprise Developer

**Profile:** Software engineer at a corporation using tool to track internal contributions. Sensitive about data exposure.

**Goals:**
- Track personal productivity internally
- Share safe metrics with management
- Ensure no proprietary code patterns leak
- Comply with security policies

**User Stories:**

#### US-4.1: Privacy Protection
```gherkin
Given I work on proprietary codebases
When I generate any export format
Then no commit messages should be exposed
And no file paths should reveal architecture
And no sensitive patterns should leak
```

#### US-4.2: Corporate Network Usage
```gherkin
Given I'm behind a corporate firewall
When I run the analyzer
Then it should work with no internet connectivity
And should work with corporate proxy settings
And should handle network drives properly
```

#### US-4.3: Compliance Validation
```gherkin
Given my company requires security audits
When I use the tool
Then it should not store data in cloud
And all operations should be local-only
And exports should be sanitizable for sharing
```

**Risk Areas:**
1. **Data Leak Risk** - Commit messages expose proprietary info
2. **Path Disclosure** - File paths reveal internal architecture
3. **Network Dependencies** - Tool requires internet (none currently, good!)
4. **Compliance** - Exports violate corporate sharing policies

**Failure Modes:**
- Commit messages in exports → Leak proprietary info
- File paths in reports → Expose architecture
- Cloud storage requirement → Policy violation
- Requires external dependencies → Can't use behind firewall

---

## Module-by-Module Risk Analysis

### Module: `classifier.rs` (File Classification)

**Purpose:** Classify files into Production, Tests, Docs, Config, Infra, Styling

**Current Implementation:**
- Pattern-based matching on file paths/names
- Priority order: Tests > Docs > Infra > Config > Styling > Production
- Language detection by extension

**Risk Assessment:**

| Risk | Severity | Example | Impact |
|------|----------|---------|--------|
| **Pattern ambiguity** | HIGH | `test_utils.py` (util, not test) | False positive inflates test coverage |
| **Multiple pattern match** | MEDIUM | `docker-compose.test.yml` | Could match infra OR test |
| **Missing patterns** | MEDIUM | Vitest, Playwright files | Modern test frameworks missed |
| **Case sensitivity** | LOW | `README.MD` vs `readme.md` | Uses `.to_lowercase()`, likely safe |
| **Path traversal** | LOW | `src/test/data/model.py` | `test/` pattern matches data files |
| **Extension ambiguity** | MEDIUM | `.m` (MATLAB or Objective-C?) | Language detection unclear |
| **No extension handling** | LOW | Makefiles, Dockerfiles | Falls to `(no ext)` |

**Edge Cases:**
1. Files matching multiple patterns: `kubernetes/tests/test_config.yaml`
2. Unconventional naming: `main.test.backup.ts`
3. Language variants: `test_foo.py` vs `foo_test.py` vs `test/foo.py`
4. Binary files: Images in docs, compiled assets
5. Generated files: `.d.ts`, `.map.js`, build artifacts
6. Vendor/node_modules: Should these count?
7. Empty files: Zero lines added/removed

**Test Coverage Needed:**
- [ ] Each pattern individually
- [ ] Pattern priority (test overrides config)
- [ ] Edge cases (multiple extensions: `.test.tsx`)
- [ ] Language detection for all supported extensions
- [ ] Files with no extension
- [ ] Case-insensitive matching

---

### Module: `analyzer.rs` (Git Log Parsing & Stats)

**Purpose:** Parse git logs, aggregate statistics, compute percentages

**Current Implementation:**
- Delimiter-based parsing (null byte `\x00` or pipe `|`)
- Line-by-line state machine (commit lines vs numstat lines)
- Date parsing with RFC3339
- Author filtering
- Incremental updates via `last_commit_hash`

**Risk Assessment:**

| Risk | Severity | Example | Impact |
|------|----------|---------|--------|
| **Delimiter confusion** | HIGH | Mixed pipe in commit message | Parsing fails, wrong stats |
| **Date parsing failure** | MEDIUM | Invalid timezone format | Commit dropped or crash |
| **Empty input** | MEDIUM | `""` as log output | Should error cleanly |
| **Injection attack** | MEDIUM | Commit msg with `\x00` | Could inject fake stats |
| **Division by zero** | HIGH | Empty repo → 0/0 for % | Panic or NaN |
| **Integer overflow** | LOW | Repo with 5B lines | u32 overflow |
| **Numstat binary files** | LOW | `-` for binary changes | Parses to 0 (correct) |
| **Merge commits** | MEDIUM | No parent or multiple parents | May double-count |
| **Author filtering** | HIGH | Email substring match too broad | Wrong commits included |
| **Date timezone issues** | MEDIUM | UTC vs local time | Activity wrong day |

**Edge Cases:**
1. **Empty repository**: No commits at all
2. **Single commit**: No parent for diff
3. **Bare repository**: No working tree
4. **Shallow clone**: `--depth=1`, missing history
5. **Orphan branches**: Multiple root commits
6. **Commit with no files**: Empty commit (e.g., revert of revert)
7. **Binary files**: Show as `-` in numstat
8. **Renamed files**: May show as delete + add
9. **Very long commit message**: Contains newlines with tabs
10. **Malformed git log**: Truncated output, git error messages
11. **Legacy delimiter mode**: Pipe character in commit message
12. **Concurrent modification**: Repo updated during analysis

**Test Coverage Needed:**
- [ ] Parse valid null-byte delimited log
- [ ] Parse legacy pipe-delimited log
- [ ] Handle empty input
- [ ] Handle single commit
- [ ] Handle commit with no files
- [ ] Handle binary file changes (`-` in numstat)
- [ ] Handle malformed dates
- [ ] Handle injection attempts (delimiters in messages)
- [ ] Division by zero protection (empty repo)
- [ ] Integer overflow (massive repos)
- [ ] Author filter edge cases (exact match, substring, missing email)
- [ ] Date range calculations
- [ ] Percentage calculations (0 total, 100% single category)
- [ ] Merge statistics from multiple repos

---

### Module: `exporters.rs` (Output Generation)

**Purpose:** Generate Markdown, LinkedIn, Portfolio, Badge outputs

**Current Implementation:**
- String building with `Vec<String>` and `join("\n")`
- Number formatting with thousands separator
- Percentage display
- Sorting by counts
- Progress bars using Unicode characters

**Risk Assessment:**

| Risk | Severity | Example | Impact |
|------|----------|---------|--------|
| **Empty dataset** | HIGH | No repos analyzed | Empty tables, divide by zero |
| **Missing fields** | MEDIUM | Repo with no description | String interpolation issues |
| **Number formatting** | LOW | Very large numbers | Display truncation |
| **Percentage display** | LOW | Very small values | 0.0% vs 0.05% |
| **Unicode rendering** | LOW | Progress bars `█` | May break in some terminals |
| **Date formatting** | LOW | Edge dates (year 1970) | Display issues |
| **Markdown injection** | MEDIUM | Repo name with `**` | Breaks formatting |
| **Division by zero** | HIGH | No contributions of a type | NaN or panic |
| **Empty sorting** | LOW | No languages | Empty loop, no crash |

**Edge Cases:**
1. **Zero repos**: All exports should handle gracefully
2. **Zero commits**: No activity data
3. **All contributions one type**: 100% production, 0% tests
4. **Huge numbers**: 1,000,000,000+ lines
5. **Special characters in names**: Repo name with markdown chars
6. **Missing optional fields**: No description, technologies
7. **Empty weekly/monthly data**: No commits in period
8. **Future dates**: Commits with date in future (clock skew)

**Test Coverage Needed:**
- [ ] Export with zero repos
- [ ] Export with empty stats (all zeros)
- [ ] Export with one repo
- [ ] Export with missing optional fields
- [ ] Number formatting for large values
- [ ] Percentage calculation for edge values
- [ ] Markdown escaping for special characters
- [ ] Unicode progress bar rendering
- [ ] Each export format (Markdown, LinkedIn, Portfolio, Badge)

---

### Module: `git.rs` (Native Git Operations)

**Purpose:** Direct git repository access via libgit2, no shell commands

**Current Implementation:**
- Opens repo with `Repository::open()`
- Walks commits with `revwalk`
- Computes diffs between commits
- Filters by author
- Supports incremental analysis (`since_commit`)
- Finds repos recursively with `walkdir`

**Risk Assessment:**

| Risk | Severity | Example | Impact |
|------|----------|---------|--------|
| **Not a git repo** | HIGH | `.git` folder missing | Panic or unclear error |
| **Corrupt repository** | MEDIUM | Broken HEAD, missing objects | Crash or wrong stats |
| **Permission denied** | MEDIUM | No read access to `.git` | Crash or skip silently |
| **Bare repository** | MEDIUM | Server-side clone | Different structure |
| **Shallow clone** | MEDIUM | `--depth=1` | Missing history |
| **Detached HEAD** | LOW | HEAD not on branch | May miss commits |
| **Empty repository** | MEDIUM | No commits | Should handle gracefully |
| **Submodules** | LOW | Nested repos | May double-count |
| **Large files** | HIGH | Huge binary in history | OOM during diff |
| **Long history** | HIGH | 1M+ commits | Takes hours, OOM |
| **Broken refs** | MEDIUM | Corrupted reference | Walk fails |
| **Merge commits** | LOW | Multiple parents | Which parent to diff? |
| **Initial commit** | MEDIUM | No parent | Special case |

**Edge Cases:**
1. **Non-existent path**: Path doesn't exist
2. **Not a directory**: Path is a file
3. **Symbolic link**: Link to repo elsewhere
4. **Network drive**: Slow I/O, different permissions
5. **Git worktree**: Multiple working directories
6. **Nested repos**: Repo inside another repo
7. **`.git` as file**: Submodule pointer
8. **Concurrent modification**: Repo updated during analysis
9. **Pack files**: Large pack file handling
10. **Ref log**: Should we use it or HEAD only?

**Test Coverage Needed:**
- [ ] Open valid repository
- [ ] Handle non-existent path
- [ ] Handle not a git repository
- [ ] Handle empty repository (no commits)
- [ ] Handle single commit (no parent)
- [ ] Handle merge commits (multiple parents)
- [ ] Author filtering (exact match, substring)
- [ ] Incremental analysis (since_commit)
- [ ] Max commits limit
- [ ] Diff stats computation
- [ ] File classification integration
- [ ] Find repos recursively
- [ ] Handle permission errors
- [ ] Handle corrupt repository

---

## BDD Scenarios (Given/When/Then) - Prioritized

### P0: Critical Path Scenarios (Must Work)

#### Scenario: Basic single repository analysis
```gherkin
Feature: Single repository analysis
  As a developer
  I want to analyze my git repository
  So that I can see my contribution statistics

Scenario: Analyze single repository with valid commits
  Given a git repository at "/tmp/test-repo"
  And the repository has 10 commits by "dev@example.com"
  And each commit modifies 5 files with 100 lines added
  When I run the analyzer on the repository
  Then the total commits should be 10
  And the total lines added should be 1000
  And the total files changed should be 50
```

#### Scenario: Author filtering
```gherkin
Feature: Author filtering
  As a freelancer working on shared projects
  I want to filter commits by my email
  So that only my contributions are counted

Scenario: Filter commits by author email
  Given a repository with commits from multiple authors
  And 5 commits by "alice@example.com"
  And 5 commits by "bob@example.com"
  When I analyze with author filter "alice@example.com"
  Then the total commits should be 5
  And all commits should be by "alice@example.com"
```

#### Scenario: File classification accuracy
```gherkin
Feature: File type classification
  As a hiring manager
  I want to see accurate contribution breakdown
  So that I can assess code quality practices

Scenario: Classify test files correctly
  Given a commit that modifies:
    | File | Lines Added |
    | src/main.rs | 100 |
    | tests/test_main.rs | 50 |
    | README.md | 20 |
  When I classify the contributions
  Then "src/main.rs" should be "ProductionCode"
  And "tests/test_main.rs" should be "Tests"
  And "README.md" should be "Documentation"
  And the contribution breakdown should show:
    | Type | Lines |
    | Production Code | 100 |
    | Tests | 50 |
    | Documentation | 20 |
```

#### Scenario: Multiple repository aggregation
```gherkin
Feature: Multi-repository analysis
  As an open source contributor
  I want to analyze multiple repositories together
  So that I can see my total impact

Scenario: Aggregate stats from multiple repositories
  Given repository "project-a" with 10 commits and 1000 lines
  And repository "project-b" with 15 commits and 1500 lines
  When I analyze both repositories
  Then the total commits should be 25
  And the total lines added should be 2500
  And the repository count should be 2
```

---

### P1: Important Scenarios (Should Work)

#### Scenario: Empty repository handling
```gherkin
Feature: Empty repository handling
  As a developer
  I want graceful handling of empty repositories
  So that the tool doesn't crash

Scenario: Analyze repository with no commits
  Given an empty git repository with no commits
  When I run the analyzer
  Then it should return an error "No commits found"
  And it should not crash
  And the error message should be user-friendly
```

#### Scenario: Division by zero protection
```gherkin
Feature: Percentage calculation safety
  As a developer
  I want safe percentage calculations
  So that edge cases don't crash the tool

Scenario: Calculate percentages with zero total
  Given a repository with no lines of code changed
  When I calculate contribution percentages
  Then all percentages should be 0.0
  And there should be no NaN or Infinity values
  And the tool should not panic
```

#### Scenario: Incremental analysis
```gherkin
Feature: Incremental repository updates
  As an OSS contributor
  I want to update analysis without re-analyzing everything
  So that updates are fast

Scenario: Incremental update with new commits
  Given a repository analyzed yesterday with 10 commits
  And the last commit hash was "abc123"
  And today there are 3 new commits after "abc123"
  When I run incremental analysis since "abc123"
  Then only 3 new commits should be analyzed
  And the total commits should be 13
  And previous stats should be preserved
```

#### Scenario: Binary file handling
```gherkin
Feature: Binary file handling
  As a developer
  I want binary files to be handled gracefully
  So that stats are accurate

Scenario: Commit with binary files
  Given a commit that adds:
    | File | Type | Size |
    | image.png | binary | 1MB |
    | code.js | text | 100 lines |
  When I analyze the commit
  Then the binary file should show 0 lines added
  And the text file should show 100 lines added
  And files changed should be 2
```

---

### P2: Edge Case Scenarios (Nice to Have)

#### Scenario: Malicious input protection
```gherkin
Feature: Injection attack prevention
  As a security-conscious user
  I want protection from malicious git logs
  So that the tool is safe to use

Scenario: Commit message with delimiter injection
  Given a commit with message "Fake\x00hacker@evil.com\x00\x00\x002099-01-01\x00Injected"
  When I parse the git log
  Then the commit should be handled safely
  And no fake commits should be created
  And the author should be the real author
```

#### Scenario: Large repository performance
```gherkin
Feature: Large repository handling
  As an OSS contributor
  I want to analyze large repositories efficiently
  So that I can include major projects

Scenario: Analyze repository with 100k commits
  Given a repository with 100,000 commits
  When I analyze with max_commits limit of 10,000
  Then analysis should complete in under 60 seconds
  And memory usage should stay under 500MB
  And the most recent 10,000 commits should be analyzed
```

#### Scenario: Cross-platform path handling
```gherkin
Feature: Cross-platform compatibility
  As a Windows user
  I want the tool to work on Windows
  So that I can use it regardless of OS

Scenario: Windows path handling
  Given a repository on Windows at "C:\Users\dev\project"
  When I analyze the repository
  Then paths should be handled correctly
  And file classification should work
  And exports should generate valid output
```

---

## Test Cases for 80%+ Coverage

### Unit Tests: `classifier.rs`

**Test Suite: FileClassifier**

```rust
// Basic classification
#[test] test_classify_production_code() // src/main.rs
#[test] test_classify_test_file() // tests/test_main.rs
#[test] test_classify_documentation() // README.md
#[test] test_classify_config() // package.json
#[test] test_classify_infrastructure() // Dockerfile
#[test] test_classify_styling() // styles.css

// Pattern priority
#[test] test_test_pattern_overrides_config() // test.json → Tests, not Config
#[test] test_doc_pattern_overrides_production() // src/docs/api.rs → Docs

// Edge cases
#[test] test_multiple_extensions() // main.test.tsx
#[test] test_no_extension() // Makefile
#[test] test_uppercase_extension() // README.MD
#[test] test_hidden_file() // .gitignore
#[test] test_path_with_test_in_name() // test-utils.js (not a test)
#[test] test_binary_file() // image.png
#[test] test_empty_path() // ""
#[test] test_very_long_path() // 500 char path

// Language detection
#[test] test_detect_all_languages() // Loop through all extensions
#[test] test_unknown_extension() // .xyz
#[test] test_ambiguous_extension() // .m (MATLAB vs Obj-C)
```

**Coverage Target:** 95%+ (simple pattern matching logic)

---

### Unit Tests: `analyzer.rs`

**Test Suite: GitAnalyzer**

```rust
// Parsing
#[test] test_parse_null_delimited_log() // Valid \x00 delimited
#[test] test_parse_pipe_delimited_log() // Legacy | delimited
#[test] test_parse_empty_log() // "" → Error
#[test] test_parse_single_commit() // One commit
#[test] test_parse_commit_with_no_files() // Empty commit
#[test] test_parse_binary_files() // - - path
#[test] test_parse_invalid_date() // Malformed timestamp
#[test] test_parse_delimiter_in_message() // Pipe in commit msg
#[test] test_parse_multiline_message() // Commit msg with \n

// Stats aggregation
#[test] test_aggregate_multiple_repos() // Sum across repos
#[test] test_calculate_percentages() // Correct math
#[test] test_calculate_percentages_zero_total() // 0/0 case
#[test] test_merge_repo_stats() // Incremental update
#[test] test_find_repo_by_name() // Lookup

// Author filtering
#[test] test_filter_by_email_exact() // Exact match
#[test] test_filter_by_email_substring() // Partial match
#[test] test_filter_by_name() // Name match
#[test] test_filter_no_matches() // No commits match

// Activity summaries
#[test] test_daily_activity() // Last 7 days
#[test] test_weekly_activity() // Last 4 weeks
#[test] test_monthly_activity() // Last 6 months
#[test] test_activity_date_ranges() // Correct start/end
#[test] test_activity_empty_period() // No commits in range

// Edge cases
#[test] test_integer_overflow_protection() // Very large numbers
#[test] test_empty_analyzer() // No repos added
#[test] test_cache_invalidation() // Cache cleared on change
```

**Coverage Target:** 85%+ (complex parsing logic)

---

### Unit Tests: `exporters.rs`

**Test Suite: Exporters**

```rust
// Markdown exporter
#[test] test_markdown_export_basic() // Standard case
#[test] test_markdown_export_empty() // No repos
#[test] test_markdown_export_special_chars() // Repo name with `**`
#[test] test_markdown_number_formatting() // 1,234,567

// LinkedIn exporter
#[test] test_linkedin_export_with_activity() // Normal case
#[test] test_linkedin_export_no_activity() // Zero commits this week
#[test] test_linkedin_quality_metrics() // Test/doc percentages

// Portfolio exporter
#[test] test_portfolio_export_complete() // All fields present
#[test] test_portfolio_export_minimal() // Missing descriptions
#[test] test_portfolio_skill_bars() // Progress bar rendering

// Badge exporter
#[test] test_badge_export_html() // HTML generation
#[test] test_badge_current_week() // This week stats

// Number formatting
#[test] test_fmt_num_thousands() // 1234 → "1,234"
#[test] test_fmt_num_millions() // 1234567 → "1,234,567"
#[test] test_fmt_num_zero() // 0 → "0"
```

**Coverage Target:** 90%+ (mostly string formatting)

---

### Unit Tests: `git.rs`

**Test Suite: Git Operations**

```rust
// Repository operations
#[test] test_open_valid_repo() // Open real repo
#[test] test_open_invalid_path() // Path doesn't exist → Error
#[test] test_open_not_a_repo() // Path exists but no .git → Error
#[test] test_is_git_repo_true() // Valid repo
#[test] test_is_git_repo_false() // Not a repo

// Analysis
#[test] test_analyze_repo_basic() // Normal case
#[test] test_analyze_empty_repo() // No commits
#[test] test_analyze_single_commit() // One commit, no parent
#[test] test_analyze_with_author_filter() // Email filter
#[test] test_analyze_max_commits_limit() // Stop at limit
#[test] test_analyze_since_commit() // Incremental

// Commit walking
#[test] test_walk_commits_chronological() // Order correct
#[test] test_walk_merge_commits() // Handle multiple parents
#[test] test_walk_initial_commit() // No parent

// Diff stats
#[test] test_diff_stats_additions() // Lines added
#[test] test_diff_stats_deletions() // Lines removed
#[test] test_diff_stats_binary_files() // Binary changes
#[test] test_diff_stats_renamed_files() // File rename

// Repository discovery
#[test] test_find_repos_recursive() // Find nested repos
#[test] test_find_repos_max_depth() // Depth limit
#[test] test_find_repos_none_found() // Empty directory

// Error handling
#[test] test_permission_denied() // No read access
#[test] test_corrupt_repo() // Broken .git
```

**Coverage Target:** 80%+ (uses real git operations, harder to test)

---

### Integration Tests

**Test Suite: End-to-End Workflows**

```rust
// CLI integration
#[test] integration_cli_single_repo() // CLI analyze one repo
#[test] integration_cli_scan_directory() // CLI scan with -s
#[test] integration_cli_author_filter() // CLI with --email
#[test] integration_cli_export_all() // CLI --all-exports

// WASM integration (if feature enabled)
#[cfg(feature = "wasm")]
#[test] integration_wasm_parse_log() // JS calls parseGitLog
#[cfg(feature = "wasm")]
#[test] integration_wasm_get_stats() // JS calls getTotalStats

// Workflow scenarios
#[test] integration_freelancer_portfolio() // Full portfolio gen
#[test] integration_oss_multi_repo() // Scan 20 repos
#[test] integration_incremental_update() // Analyze, add commits, re-analyze
```

**Coverage Target:** 70%+ (covers main workflows)

---

### Property-Based Tests (QuickCheck/Proptest)

**Fuzzing for edge cases:**

```rust
#[quickcheck]
fn prop_parse_never_panics(random_input: String) // Fuzz parser
#[quickcheck]
fn prop_percentages_sum_to_100(data: Vec<u32>) // Math checks
#[quickcheck]
fn prop_merge_is_commutative(stats1: RepoStats, stats2: RepoStats) // Order doesn't matter
```

---

## Edge Cases Requiring Special Attention

### Category: Data Integrity

**EC-1: Integer Overflow**
```
Issue: u32 max = 4,294,967,295 lines
Scenario: Linux kernel has 30M+ lines
Risk: Overflow causes incorrect stats
Test: Create repo with lines > u32::MAX
Mitigation: Use u64 or checked arithmetic
```

**EC-2: Division by Zero**
```
Issue: percentage = count / total, total could be 0
Scenario: Empty repo, no contributions of a type
Risk: Panic or NaN
Test: Calculate percentages with total=0
Mitigation: Check total > 0 before division
```

**EC-3: Date Edge Cases**
```
Issue: Dates can be invalid, future, or epoch
Scenario: Commit with timestamp=0, or year 2099
Risk: Activity in wrong time bucket
Test: Parse dates from 1970, 2099, invalid timezone
Mitigation: Validate dates, handle parse errors
```

**EC-4: Empty Collections**
```
Issue: Empty repos, languages, contribution types
Scenario: Brand new repo, filter matches nothing
Risk: Empty loops, division by zero, empty exports
Test: Every function with empty input
Mitigation: Check .is_empty() before processing
```

---

### Category: Security

**EC-5: Commit Message Injection**
```
Issue: Commit message contains delimiter
Scenario: Message = "Fix\x00fake@evil.com\x00\x00..."
Risk: Parser creates fake commits
Test: Inject delimiters in commit message
Mitigation: Use null byte (safer than pipe), validate format
```

**EC-6: Path Traversal**
```
Issue: File paths with ../.. or absolute paths
Scenario: Commit modifies "../../../../etc/passwd"
Risk: None currently (not executing paths), but classification could be fooled
Test: Classify paths with traversal
Mitigation: Normalize paths before classification
```

**EC-7: Data Privacy**
```
Issue: Commit messages, file paths expose proprietary info
Scenario: Enterprise user exports portfolio
Risk: Leak trade secrets, internal architecture
Test: Ensure exports don't include commit messages (currently safe for CLI)
Mitigation: Add --sanitize flag to strip sensitive data
```

---

### Category: Performance

**EC-8: Memory Exhaustion**
```
Issue: Large repo loaded into memory
Scenario: Linux kernel with 1M commits, store_commits=true
Risk: OOM crash
Test: Analyze large repo with commit storage
Mitigation: stream processing, max_commits limit
```

**EC-9: Infinite Loop**
```
Issue: Circular references in git
Scenario: Corrupt repo with circular refs
Risk: Hang forever
Test: Create repo with circular ref (hard to do)
Mitigation: Timeout, max iteration limit
```

**EC-10: Slow I/O**
```
Issue: Network drives, slow disks
Scenario: Analyze 100 repos on NFS mount
Risk: Takes hours
Test: Benchmark on slow filesystem
Mitigation: Parallel processing, progress indicators
```

---

### Category: Compatibility

**EC-11: Windows Path Handling**
```
Issue: Windows uses backslash, different separators
Scenario: Path = "C:\Users\dev\project\src\main.rs"
Risk: Pattern matching fails
Test: Run on Windows, verify classification works
Mitigation: Use Path::new() for normalization
```

**EC-12: Unicode Handling**
```
Issue: Non-ASCII characters in paths/names
Scenario: File = "src/файл.rs" (Cyrillic)
Risk: UTF-8 decode errors
Test: Repo with Unicode filenames
Mitigation: Use String (UTF-8) throughout, handle errors
```

**EC-13: Git Worktrees**
```
Issue: Worktrees have .git as file not directory
Scenario: Repo has multiple worktrees
Risk: is_git_repo() returns false
Test: Analyze worktree
Mitigation: Check for .git file pointing to main repo
```

---

### Category: Git Edge Cases

**EC-14: Bare Repository**
```
Issue: No working directory, only .git contents
Scenario: Server-side clone
Risk: Structure different, analysis fails
Test: Clone --bare, analyze
Mitigation: Handle bare repos explicitly
```

**EC-15: Shallow Clone**
```
Issue: --depth=1 means missing history
Scenario: CI/CD clones with depth=1
Risk: Incomplete stats
Test: Shallow clone, analyze
Mitigation: Warn user about incomplete history
```

**EC-16: Submodules**
```
Issue: Nested repos
Scenario: Repo has submodules
Risk: Double-count, or miss contributions
Test: Repo with submodules
Mitigation: Handle .gitmodules, skip submodule dirs
```

**EC-17: Initial Commit**
```
Issue: No parent to diff against
Scenario: First commit in repo
Risk: No diff available
Test: Analyze repo with 1 commit
Mitigation: Diff against empty tree
```

**EC-18: Merge Commits**
```
Issue: Multiple parents
Scenario: Merge commit has 2+ parents
Risk: Which parent to diff? May count changes twice
Test: Analyze merge-heavy repo
Mitigation: Diff against first parent only
```

---

## Test Implementation Priority

### Phase 1: Critical Path (Week 1)
- [ ] Unit tests for `classifier.rs` (all main patterns)
- [ ] Unit tests for `analyzer.rs` (parsing, stats)
- [ ] Integration test: Single repo analysis
- [ ] Integration test: Author filtering
- [ ] Edge case: Empty repo handling
- [ ] Edge case: Division by zero

**Goal:** 60% coverage, no crashes on basic usage

---

### Phase 2: Robustness (Week 2)
- [ ] Unit tests for `exporters.rs` (all formats)
- [ ] Unit tests for `git.rs` (repo operations)
- [ ] Edge cases: Binary files, malformed dates
- [ ] Edge cases: Large numbers, overflow protection
- [ ] Integration test: Multi-repo analysis
- [ ] Integration test: Incremental updates

**Goal:** 75% coverage, handles edge cases

---

### Phase 3: Production Ready (Week 3)
- [ ] Security tests: Injection attempts
- [ ] Performance tests: Large repos, many repos
- [ ] Cross-platform tests: Windows, macOS, Linux
- [ ] Git edge cases: Bare repos, shallow clones, submodules
- [ ] Property-based tests: Fuzzing
- [ ] Documentation: Test README, coverage reports

**Goal:** 85%+ coverage, ready for release

---

## Recommended Testing Tools

### Rust Testing Ecosystem
- **cargo test** - Built-in test runner
- **proptest** - Property-based testing
- **criterion** - Benchmarking
- **mockall** - Mocking (for git operations)
- **tempfile** - Temporary test repos
- **git2** - Create test repos programmatically

### Coverage Tools
- **cargo-tarpaulin** - Coverage reports
- **cargo-llvm-cov** - LLVM-based coverage

### CI Integration
- GitHub Actions workflow
- Run tests on: Linux, macOS, Windows
- Run on: Rust stable, beta
- Coverage badge in README

---

## Test Data Setup

### Creating Test Repositories

**Helper functions needed:**

```rust
// tests/common.rs
pub fn create_test_repo(path: &Path) -> Repository { ... }
pub fn add_commit(repo: &Repository, author: &str, files: Vec<(&str, &str)>) { ... }
pub fn create_repo_with_commits(commits: Vec<CommitData>) -> Repository { ... }
```

**Example test repos:**

1. **simple-repo**: 10 commits, 1 author, basic files
2. **multi-author-repo**: 20 commits, 3 authors
3. **complex-repo**: Mix of file types, tests, docs, config
4. **empty-repo**: No commits
5. **large-repo**: 10,000 commits (for performance testing)
6. **unicode-repo**: Non-ASCII filenames
7. **merge-heavy-repo**: Lots of merge commits

---

## Success Metrics

**Definition of 80%+ Coverage:**
- Line coverage: >80% of code lines executed
- Branch coverage: >75% of conditional branches tested
- Function coverage: >90% of functions called
- Edge case coverage: All identified edge cases have tests

**Quality Gates:**
- All tests pass on Linux, macOS, Windows
- No panics on invalid input
- Performance: <1s for typical repo, <60s for large repo
- Memory: <100MB for typical usage, <500MB for large repos
- Documentation: All public APIs documented with examples

---

## Appendix: Current Test Coverage

**Baseline (as of 2025-12-18):**
```
Total tests: 1 (in git.rs)
Coverage: ~5% (estimated)
CI: Not set up
```

**Coverage by module:**
- `classifier.rs`: 0%
- `analyzer.rs`: 0%
- `exporters.rs`: 0%
- `git.rs`: ~10% (one basic test)

**Next Steps:**
1. Set up cargo-tarpaulin for coverage tracking
2. Add test helpers for creating test repos
3. Implement Phase 1 tests (critical path)
4. Set up GitHub Actions CI
5. Generate coverage badge

---

## Conclusion

This Git Activity Dashboard has significant value for developers, but **requires comprehensive testing** before production use. The current ~5% test coverage is insufficient for the critical use cases (portfolio generation, hiring decisions).

**Key Recommendations:**

1. **Prioritize data integrity tests** - Incorrect stats can damage careers
2. **Add security tests** - Protect enterprise users from data leaks
3. **Performance test with real repos** - OSS contributors need scalability
4. **Cross-platform validation** - Windows support is critical
5. **Property-based testing** - Catch unexpected edge cases

With the test plan outlined above, the tool can reach **production quality** suitable for all four personas.

**Estimated Effort:**
- Phase 1 (Critical): 40 hours
- Phase 2 (Robustness): 30 hours
- Phase 3 (Production): 30 hours
- **Total: ~100 hours (2.5 weeks full-time)**

---

*End of Test Plan*
