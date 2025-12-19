//! Trait abstractions for dependency inversion
//!
//! These traits enable loose coupling and testability throughout the codebase.

use crate::analyzer::{ActivitySummary, RepoStats, TotalStats};
use crate::classifier::{ContributionType, FileClassification};
use std::collections::HashMap;

/// Trait for file classification - enables dependency injection and mocking
pub trait Classifier: Send + Sync {
    /// Classify a file based on its path and change statistics
    fn classify(&self, file_path: &str, lines_added: u32, lines_removed: u32) -> FileClassification;

    /// Detect programming language from file extension
    fn detect_language(&self, extension: &str) -> Option<String>;
}

/// Trait for analytics data access - decouples exporters from GitAnalyzer
pub trait Analytics: Send + Sync {
    /// Get aggregated statistics across all repositories
    fn total_stats(&self) -> TotalStats;

    /// Get list of analyzed repositories
    fn repos(&self) -> &[RepoStats];

    /// Get daily activity summaries
    fn daily_activity(&self, days: u32) -> Vec<ActivitySummary>;

    /// Get weekly activity summaries
    fn weekly_activity(&self, weeks: u32) -> Vec<ActivitySummary>;

    /// Get monthly activity summaries
    fn monthly_activity(&self, months: u32) -> Vec<ActivitySummary>;
}

/// Trait for exporters - enables plugin architecture
pub trait Exporter: Send + Sync {
    /// Export analytics data to a string format
    fn export(&self, analytics: &dyn Analytics) -> String;

    /// Get the name/identifier of this exporter
    fn name(&self) -> &'static str;

    /// Get the default file extension for this format
    fn extension(&self) -> &'static str;
}

/// Strategy trait for time period calculations
pub trait PeriodStrategy: Send + Sync {
    /// Calculate the start and end boundaries for a period at the given index
    /// Index 0 is the current/most recent period
    fn boundaries(&self, index: u32) -> (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>);

    /// Generate a human-readable label for the period at the given index
    fn label(&self, index: u32) -> String;
}

/// Trait for classification rules - enables extensible rule-based classification
pub trait ClassificationRule: Send + Sync {
    /// Priority of this rule (higher = checked first)
    fn priority(&self) -> u8;

    /// Check if this rule matches the given file path
    fn matches(&self, file_path: &str, lines_added: u32, lines_removed: u32) -> bool;

    /// Get the classification result if this rule matches
    fn contribution_type(&self) -> ContributionType;

    /// Get the detected language if applicable
    fn language(&self) -> Option<String> {
        None
    }
}

/// Configuration for classification behavior
#[derive(Debug, Clone, Default)]
pub struct ClassifierConfig {
    /// Custom patterns for test files
    pub test_patterns: Vec<String>,
    /// Custom patterns for documentation files
    pub doc_patterns: Vec<String>,
    /// Custom patterns for infrastructure files
    pub infra_patterns: Vec<String>,
    /// Custom language mappings (extension -> language name)
    pub language_map: HashMap<String, String>,
}

/// Configuration for export behavior
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Maximum number of items to show in lists
    pub max_items: usize,
    /// Maximum number of languages to show
    pub max_languages: usize,
    /// Maximum number of repositories to show
    pub max_repos: usize,
    /// Include detailed breakdowns
    pub include_details: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            max_items: 10,
            max_languages: 8,
            max_repos: 10,
            include_details: true,
        }
    }
}
