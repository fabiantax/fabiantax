pub mod analyzer;
pub mod classifier;
pub mod exporters;

pub use analyzer::{
    GitAnalyzer, RepoStats, CommitInfo, TotalStats, DashboardData, ActivitySummary,
    ParseOptions, ParseError, GIT_LOG_FORMAT, GIT_LOG_DELIMITER, git_log_command,
};
pub use classifier::{FileClassifier, FileClassification, ContributionType};
pub use exporters::{MarkdownExporter, LinkedInExporter, PortfolioExporter, BadgeExporter};

// WASM bindings
#[cfg(feature = "wasm")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn init() {
        #[cfg(feature = "wasm")]
        console_error_panic_hook::set_once();
    }

    /// WASM-compatible analyzer that can be used from JavaScript/TypeScript
    #[wasm_bindgen]
    pub struct WasmAnalyzer {
        inner: GitAnalyzer,
    }

    #[wasm_bindgen]
    impl WasmAnalyzer {
        /// Create a new analyzer instance
        #[wasm_bindgen(constructor)]
        pub fn new(author_email: Option<String>, author_name: Option<String>) -> Self {
            Self {
                inner: GitAnalyzer::new(author_email, author_name),
            }
        }

        /// Parse git log output from a repository
        ///
        /// The git log should be formatted as:
        /// git log --format='%H\x00%an\x00%ae\x00%aI\x00%s' --numstat
        /// (using null byte \x00 as delimiter for safety)
        #[wasm_bindgen(js_name = parseGitLog)]
        pub fn parse_git_log(&mut self, repo_name: &str, repo_path: &str, log_output: &str) -> Result<JsValue, JsValue> {
            match self.inner.parse_git_log(repo_name, repo_path, log_output) {
                Ok(stats) => serde_wasm_bindgen::to_value(&stats)
                    .map_err(|e| JsValue::from_str(&e.to_string())),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        }

        /// Add pre-parsed repository data
        #[wasm_bindgen(js_name = addRepoData)]
        pub fn add_repo_data(&mut self, data: JsValue) -> Result<(), JsValue> {
            let stats: RepoStats = serde_wasm_bindgen::from_value(data)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.inner.add_repo_data(stats);
            Ok(())
        }

        /// Get total statistics across all repositories
        #[wasm_bindgen(js_name = getTotalStats)]
        pub fn get_total_stats(&self) -> JsValue {
            serde_wasm_bindgen::to_value(&self.inner.get_total_stats()).unwrap_or(JsValue::NULL)
        }

        /// Get daily activity for the past N days
        #[wasm_bindgen(js_name = getDailyActivity)]
        pub fn get_daily_activity(&self, days: u32) -> JsValue {
            serde_wasm_bindgen::to_value(&self.inner.get_daily_activity(days)).unwrap_or(JsValue::NULL)
        }

        /// Get weekly activity for the past N weeks
        #[wasm_bindgen(js_name = getWeeklyActivity)]
        pub fn get_weekly_activity(&self, weeks: u32) -> JsValue {
            serde_wasm_bindgen::to_value(&self.inner.get_weekly_activity(weeks)).unwrap_or(JsValue::NULL)
        }

        /// Get all dashboard data as JSON
        #[wasm_bindgen(js_name = getDashboardData)]
        pub fn get_dashboard_data(&self) -> JsValue {
            serde_wasm_bindgen::to_value(&self.inner.get_dashboard_data()).unwrap_or(JsValue::NULL)
        }

        /// Get all repositories
        #[wasm_bindgen(js_name = getRepos)]
        pub fn get_repos(&self) -> JsValue {
            serde_wasm_bindgen::to_value(&self.inner.get_repos()).unwrap_or(JsValue::NULL)
        }

        /// Export as Markdown
        #[wasm_bindgen(js_name = exportMarkdown)]
        pub fn export_markdown(&self) -> String {
            MarkdownExporter::export(&self.inner)
        }

        /// Export as LinkedIn post
        #[wasm_bindgen(js_name = exportLinkedIn)]
        pub fn export_linkedin(&self) -> String {
            LinkedInExporter::export(&self.inner)
        }

        /// Export as Portfolio
        #[wasm_bindgen(js_name = exportPortfolio)]
        pub fn export_portfolio(&self) -> String {
            PortfolioExporter::export(&self.inner)
        }

        /// Export as README badge
        #[wasm_bindgen(js_name = exportBadge)]
        pub fn export_badge(&self) -> String {
            BadgeExporter::export(&self.inner)
        }

        /// Export as JSON string
        #[wasm_bindgen(js_name = exportJson)]
        pub fn export_json(&self) -> String {
            serde_json::to_string_pretty(&self.inner.get_dashboard_data()).unwrap_or_default()
        }
    }

    /// Classify a single file (utility function)
    #[wasm_bindgen(js_name = classifyFile)]
    pub fn classify_file(file_path: &str, lines_added: u32, lines_removed: u32) -> JsValue {
        let classifier = FileClassifier::new();
        let result = classifier.classify(file_path, lines_added, lines_removed);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    }
}

#[cfg(feature = "wasm")]
pub use wasm::*;
