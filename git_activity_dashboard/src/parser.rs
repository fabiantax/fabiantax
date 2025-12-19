//! Git log parsing module
//!
//! Separates parsing concerns from analytics (Single Responsibility Principle).

use crate::analyzer::{CommitInfo, ParseError, ParseOptions, RepoStats};
use crate::classifier::FileClassification;
use crate::traits::Classifier;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Git log parser - handles parsing of git log output
pub struct GitLogParser<'a> {
    classifier: &'a dyn Classifier,
    options: ParseOptions,
}

impl<'a> GitLogParser<'a> {
    /// Create a new parser with the given classifier and options
    pub fn new(classifier: &'a dyn Classifier, options: ParseOptions) -> Self {
        Self { classifier, options }
    }

    /// Create a parser with default options
    pub fn with_classifier(classifier: &'a dyn Classifier) -> Self {
        Self {
            classifier,
            options: ParseOptions::default(),
        }
    }

    /// Set whether to store individual commits
    pub fn store_commits(mut self, store: bool) -> Self {
        self.options.store_commits = store;
        self
    }

    /// Set whether to use legacy pipe delimiter
    pub fn legacy_delimiter(mut self, legacy: bool) -> Self {
        self.options.legacy_delimiter = legacy;
        self
    }

    /// Parse raw git log output into RepoStats
    /// Supports both null-byte delimiter (preferred) and legacy pipe delimiter
    pub fn parse(&self, repo_name: &str, repo_path: &str, log_output: &str) -> Result<RepoStats, ParseError> {
        if log_output.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let mut stats = RepoStats {
            name: repo_name.to_string(),
            path: repo_path.to_string(),
            ..Default::default()
        };

        let mut current_commit: Option<CommitInfo> = None;
        let mut first_hash: Option<String> = None;

        // Detect delimiter: if line contains null byte, use that; otherwise use pipe
        let delimiter = if log_output.contains('\x00') || !self.options.legacy_delimiter {
            '\x00'
        } else {
            '|'
        };

        for line in log_output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Check if this is a commit line
            let is_commit_line = if delimiter == '\x00' {
                line.contains('\x00')
            } else {
                line.contains('|') && line.matches('|').count() >= 4
            };

            if is_commit_line {
                let parts: Vec<&str> = line.splitn(5, delimiter).collect();
                if parts.len() == 5 {
                    // Save previous commit
                    if let Some(commit) = current_commit.take() {
                        if self.options.store_commits {
                            stats.commits.push(commit);
                        }
                    }

                    let date = DateTime::parse_from_rfc3339(parts[3])
                        .map(|d| d.with_timezone(&Utc))
                        .map_err(|_| ParseError::InvalidDate(parts[3].to_string()));

                    match date {
                        Ok(date) => {
                            if first_hash.is_none() {
                                first_hash = Some(parts[0].to_string());
                            }

                            current_commit = Some(CommitInfo {
                                hash: parts[0].to_string(),
                                author: parts[1].to_string(),
                                email: parts[2].to_string(),
                                date,
                                message: parts[4].to_string(),
                                files_changed: 0,
                                lines_added: 0,
                                lines_removed: 0,
                                file_classifications: Vec::new(),
                                contribution_types: HashMap::new(),
                                languages: HashMap::new(),
                            });
                        }
                        Err(_) => {
                            // Skip commits with invalid dates
                        }
                    }
                    continue;
                }
            }

            // Check if this is a numstat line (additions\tdeletions\tfilename)
            if let Some(ref mut commit) = current_commit {
                self.parse_numstat_line(line, commit, &mut stats);
            }
        }

        // Don't forget the last commit
        if let Some(commit) = current_commit {
            self.finalize_commit(commit, &mut stats);
        }

        // Count commits we processed
        stats.total_commits = if self.options.store_commits {
            stats.commits.len() as u32
        } else {
            // Count from log output
            log_output
                .lines()
                .filter(|l| {
                    l.contains(delimiter) || (delimiter == '|' && l.matches('|').count() >= 4)
                })
                .count() as u32
        };

        // Update date range from stored commits if available
        if self.options.store_commits && !stats.commits.is_empty() {
            let dates: Vec<_> = stats.commits.iter().map(|c| c.date).collect();
            stats.first_commit_date = dates.iter().min().copied();
            stats.last_commit_date = dates.iter().max().copied();
        }

        // Store the latest commit hash for incremental updates
        stats.last_commit_hash = first_hash;

        Ok(stats)
    }

    /// Parse a numstat line and update commit and stats
    fn parse_numstat_line(&self, line: &str, commit: &mut CommitInfo, stats: &mut RepoStats) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 3 {
            return;
        }

        // Binary files show "-" for added/removed
        let added: u32 = parts[0].parse().unwrap_or(0);
        let removed: u32 = parts[1].parse().unwrap_or(0);
        let filepath = parts[2];

        let classification = self.classifier.classify(filepath, added, removed);

        // Track language
        if let Some(ref lang) = classification.language {
            *stats.languages.entry(lang.clone()).or_insert(0) += added + removed;
            *commit.languages.entry(lang.clone()).or_insert(0) += added + removed;
        }

        // Track contribution type
        let type_key = format!("{:?}", classification.contribution_type).to_lowercase();
        *stats.contribution_types.entry(type_key.clone()).or_insert(0) += added + removed;
        *commit.contribution_types.entry(type_key).or_insert(0) += added + removed;

        // Track file extension
        let ext = Self::get_file_extension(filepath);
        *stats.file_extensions.entry(ext).or_insert(0) += added + removed;

        commit.lines_added += added;
        commit.lines_removed += removed;
        commit.files_changed += 1;

        // Update totals inline (avoid second iteration)
        stats.total_lines_added += added;
        stats.total_lines_removed += removed;
        stats.total_files_changed += 1;

        if self.options.store_commits {
            commit.file_classifications.push(classification);
        }
    }

    /// Finalize a commit and add it to stats
    fn finalize_commit(&self, commit: CommitInfo, stats: &mut RepoStats) {
        // Update date range from last commit
        if stats.first_commit_date.is_none() || commit.date < stats.first_commit_date.unwrap() {
            stats.first_commit_date = Some(commit.date);
        }
        if stats.last_commit_date.is_none() || commit.date > stats.last_commit_date.unwrap() {
            stats.last_commit_date = Some(commit.date);
        }

        stats.total_commits += 1;

        if self.options.store_commits {
            stats.commits.push(commit);
        }
    }

    /// Extract file extension from a path
    fn get_file_extension(filepath: &str) -> String {
        let path = std::path::Path::new(filepath);
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e.to_lowercase()))
            .unwrap_or_else(|| "(no ext)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classifier::FileClassifier;

    fn create_sample_log() -> String {
        "abc123\x00John Doe\x00john@example.com\x002024-01-15T10:30:00Z\x00Initial commit\n\
         10\t5\tsrc/main.rs\n\
         3\t2\tREADME.md\n"
            .to_string()
    }

    #[test]
    fn test_parser_basic() {
        let classifier = FileClassifier::new();
        let parser = GitLogParser::with_classifier(&classifier).store_commits(true);

        let result = parser.parse("test-repo", "/path/to/repo", &create_sample_log());
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.name, "test-repo");
        assert_eq!(stats.total_commits, 1);
        assert_eq!(stats.commits.len(), 1);
    }

    #[test]
    fn test_parser_empty_input() {
        let classifier = FileClassifier::new();
        let parser = GitLogParser::with_classifier(&classifier);

        let result = parser.parse("test-repo", "/path", "");
        assert!(matches!(result, Err(ParseError::EmptyInput)));
    }

    #[test]
    fn test_parser_without_storing_commits() {
        let classifier = FileClassifier::new();
        let parser = GitLogParser::with_classifier(&classifier).store_commits(false);

        let result = parser.parse("test-repo", "/path", &create_sample_log());
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.total_commits, 1);
        assert!(stats.commits.is_empty()); // Not stored
    }

    #[test]
    fn test_parser_tracks_languages() {
        let classifier = FileClassifier::new();
        let parser = GitLogParser::with_classifier(&classifier).store_commits(true);

        let result = parser.parse("test-repo", "/path", &create_sample_log());
        let stats = result.unwrap();

        assert!(stats.languages.contains_key("Rust"));
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(GitLogParser::get_file_extension("src/main.rs"), ".rs");
        assert_eq!(GitLogParser::get_file_extension("README.md"), ".md");
        assert_eq!(GitLogParser::get_file_extension("Makefile"), "(no ext)");
        assert_eq!(GitLogParser::get_file_extension("src/index.test.ts"), ".ts");
    }
}
