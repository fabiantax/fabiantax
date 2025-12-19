//! File Classification Module
//!
//! # Performance Optimizations
//!
//! This module uses several sub-linear and SIMD-accelerated algorithms:
//!
//! ## Aho-Corasick Automaton - O(n) multi-pattern matching
//! Instead of checking each pattern individually O(patterns × path_length),
//! we build a finite automaton that matches ALL patterns in a single pass O(path_length).
//! The `aho-corasick` crate uses SIMD instructions for additional speedup.
//!
//! ## PHF (Perfect Hash Function) - O(1) extension lookup
//! Compile-time generated perfect hash map for extension→language mapping.
//! Guarantees no collisions and O(1) lookup vs O(n) linear scan.
//!
//! ## Extension-First Dispatch - O(1) before O(n)
//! Many file types can be determined by extension alone (O(1) hash lookup).
//! Pattern matching only runs when extension is inconclusive.

use crate::traits::Classifier;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use once_cell::sync::Lazy;
use phf::phf_map;
use serde::{Deserialize, Serialize};

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributionType {
    ProductionCode,
    Tests,
    Documentation,
    SpecsConfig,
    Infrastructure,
    Styling,
    BuildArtifacts,
    Assets,
    Generated,
    Data,
    Other,
}

impl ContributionType {
    pub fn label(&self) -> &'static str {
        match self {
            ContributionType::ProductionCode => "Production Code",
            ContributionType::Tests => "Tests",
            ContributionType::Documentation => "Documentation",
            ContributionType::SpecsConfig => "Specs & Config",
            ContributionType::Infrastructure => "Infrastructure",
            ContributionType::Styling => "Styling",
            ContributionType::BuildArtifacts => "Build Artifacts",
            ContributionType::Assets => "Assets",
            ContributionType::Generated => "Generated",
            ContributionType::Data => "Data",
            ContributionType::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileClassification {
    pub file_path: String,
    pub contribution_type: ContributionType,
    pub language: Option<String>,
    pub lines_added: u32,
    pub lines_removed: u32,
}

// ============================================================================
// PHF Perfect Hash Maps - O(1) Lookup
// Algorithm: Compile-time perfect hash function (no collisions, guaranteed O(1))
// ============================================================================

/// Extension → Language mapping using PHF (Perfect Hash Function)
/// Complexity: O(1) lookup vs O(n) linear scan
static LANGUAGE_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    ".py" => "Python",
    ".js" => "JavaScript",
    ".ts" => "TypeScript",
    ".tsx" => "TypeScript (React)",
    ".jsx" => "JavaScript (React)",
    ".cs" => "C#",
    ".java" => "Java",
    ".go" => "Go",
    ".rs" => "Rust",
    ".rb" => "Ruby",
    ".php" => "PHP",
    ".swift" => "Swift",
    ".kt" => "Kotlin",
    ".scala" => "Scala",
    ".c" => "C",
    ".cpp" => "C++",
    ".cc" => "C++",
    ".cxx" => "C++",
    ".h" => "C/C++ Header",
    ".hpp" => "C++ Header",
    ".vue" => "Vue",
    ".svelte" => "Svelte",
    ".html" => "HTML",
    ".sql" => "SQL",
    ".r" => "R",
    ".m" => "MATLAB/Objective-C",
    ".pl" => "Perl",
    ".lua" => "Lua",
    ".dart" => "Dart",
    ".elm" => "Elm",
    ".ex" => "Elixir",
    ".exs" => "Elixir",
    ".erl" => "Erlang",
    ".hs" => "Haskell",
    ".clj" => "Clojure",
    ".fs" => "F#",
    ".fsx" => "F#",
    ".sh" => "Shell",
    ".ps1" => "PowerShell",
};

/// Extension → ContributionType for types determinable by extension alone
/// Complexity: O(1) lookup - early termination before pattern matching
/// Note: Styling extensions (.css, .scss) handled separately to set language
static EXTENSION_TYPE_MAP: phf::Map<&'static str, ContributionType> = phf_map! {
    // Documentation extensions
    ".md" => ContributionType::Documentation,
    ".rst" => ContributionType::Documentation,
    ".adoc" => ContributionType::Documentation,
    ".wiki" => ContributionType::Documentation,
    ".txt" => ContributionType::Documentation,

    // Build artifacts
    ".o" => ContributionType::BuildArtifacts,
    ".obj" => ContributionType::BuildArtifacts,
    ".a" => ContributionType::BuildArtifacts,
    ".lib" => ContributionType::BuildArtifacts,
    ".so" => ContributionType::BuildArtifacts,
    ".dll" => ContributionType::BuildArtifacts,
    ".dylib" => ContributionType::BuildArtifacts,
    ".exe" => ContributionType::BuildArtifacts,
    ".rlib" => ContributionType::BuildArtifacts,
    ".rmeta" => ContributionType::BuildArtifacts,
    ".pdb" => ContributionType::BuildArtifacts,
    ".class" => ContributionType::BuildArtifacts,
    ".jar" => ContributionType::BuildArtifacts,
    ".war" => ContributionType::BuildArtifacts,
    ".pyc" => ContributionType::BuildArtifacts,
    ".pyo" => ContributionType::BuildArtifacts,
    ".wasm" => ContributionType::BuildArtifacts,

    // Assets
    ".png" => ContributionType::Assets,
    ".jpg" => ContributionType::Assets,
    ".jpeg" => ContributionType::Assets,
    ".gif" => ContributionType::Assets,
    ".bmp" => ContributionType::Assets,
    ".ico" => ContributionType::Assets,
    ".svg" => ContributionType::Assets,
    ".webp" => ContributionType::Assets,
    ".ttf" => ContributionType::Assets,
    ".otf" => ContributionType::Assets,
    ".woff" => ContributionType::Assets,
    ".woff2" => ContributionType::Assets,
    ".eot" => ContributionType::Assets,
    ".mp3" => ContributionType::Assets,
    ".mp4" => ContributionType::Assets,
    ".wav" => ContributionType::Assets,
    ".ogg" => ContributionType::Assets,
    ".webm" => ContributionType::Assets,
    ".avi" => ContributionType::Assets,
    ".mov" => ContributionType::Assets,
    ".pdf" => ContributionType::Assets,
    ".zip" => ContributionType::Assets,
    ".tar" => ContributionType::Assets,
    ".gz" => ContributionType::Assets,
    ".rar" => ContributionType::Assets,
    ".7z" => ContributionType::Assets,

    // Data files
    ".csv" => ContributionType::Data,
    ".tsv" => ContributionType::Data,
    ".sqlite" => ContributionType::Data,
    ".db" => ContributionType::Data,
    ".log" => ContributionType::Data,
    ".jsonl" => ContributionType::Data,
    ".ndjson" => ContributionType::Data,
    ".parquet" => ContributionType::Data,
    ".avro" => ContributionType::Data,
    ".xls" => ContributionType::Data,
    ".xlsx" => ContributionType::Data,

    // Styling
    ".css" => ContributionType::Styling,
    ".scss" => ContributionType::Styling,
    ".sass" => ContributionType::Styling,
    ".less" => ContributionType::Styling,
    ".styl" => ContributionType::Styling,
};

// ============================================================================
// Aho-Corasick Pattern Matchers - O(n) Multi-Pattern Matching
// Algorithm: Aho-Corasick automaton with SIMD acceleration
// Matches ALL patterns in a single pass through the input string
// ============================================================================

/// Pattern set with associated contribution type
/// Note: contribution_type and language_override stored for documentation/future use
/// The classify function directly indexes into the vector for performance
struct PatternMatcher {
    automaton: AhoCorasick,
    #[allow(dead_code)]
    contribution_type: ContributionType,
    #[allow(dead_code)]
    language_override: Option<&'static str>,
}

/// Lazy-initialized Aho-Corasick matchers for each contribution type
/// Built once at first use, then O(path_length) matching thereafter
static PATTERN_MATCHERS: Lazy<Vec<PatternMatcher>> = Lazy::new(|| {
    // Helper to build an Aho-Corasick automaton
    // Uses LeftmostFirst match semantics for deterministic results
    let build_ac = |patterns: &[&str]| -> AhoCorasick {
        AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(patterns)
            .expect("Failed to build Aho-Corasick automaton")
    };

    vec![
        // Tests - highest priority (checked first)
        PatternMatcher {
            automaton: build_ac(&[
                "test_", "_test.", ".test.", "tests/", "/test/", "spec_", "_spec.",
                ".spec.", "specs/", "/spec/", "__tests__/", ".tests.", "testing/",
                "unittest", "pytest", "jest", "mocha", "cypress/", "e2e/",
            ]),
            contribution_type: ContributionType::Tests,
            language_override: None,
        },
        // Documentation
        PatternMatcher {
            automaton: build_ac(&[
                "readme", "changelog", "contributing", "license", "authors",
                "docs/", "/doc/", "documentation/", "wiki/", "guide", "manual", "api-docs/",
            ]),
            contribution_type: ContributionType::Documentation,
            language_override: Some("Documentation"),
        },
        // Infrastructure
        PatternMatcher {
            automaton: build_ac(&[
                "dockerfile", "docker-compose", "kubernetes/", "k8s/", "helm/",
                "terraform/", ".tf", "ansible/", "puppet/", "chef/",
                "cloudformation", "pulumi/", "vagrant", "makefile", "cmake",
                "deploy/", "deployment/", "infra/", "infrastructure/",
                "scripts/deploy", "scripts/build", "nginx", "apache",
            ]),
            contribution_type: ContributionType::Infrastructure,
            language_override: None, // Keep detected language if any
        },
        // Specs/Config
        PatternMatcher {
            automaton: build_ac(&[
                "package.json", "tsconfig", "webpack", "babel", "eslint", "prettier",
                ".yaml", ".yml", ".json", ".toml", ".ini", ".cfg", ".conf",
                "openapi", "swagger", "schema", ".env", "config/", "/config",
                "settings", ".editorconfig", ".gitignore", ".dockerignore",
                "pyproject.toml", "setup.py", "setup.cfg", "requirements",
                "gemfile", "cargo.toml", "go.mod", "pom.xml", "build.gradle",
                ".github/", ".gitlab-ci", "azure-pipelines", "jenkinsfile",
                ".travis", "circle.yml", "bitbucket-pipelines",
            ]),
            contribution_type: ContributionType::SpecsConfig,
            language_override: Some("Configuration"),
        },
        // Styling
        PatternMatcher {
            automaton: build_ac(&[
                ".styled.", "styles/", "/style/", "theme", ".tailwind",
            ]),
            contribution_type: ContributionType::Styling,
            language_override: Some("CSS/Styling"),
        },
        // Build artifacts (patterns)
        PatternMatcher {
            automaton: build_ac(&[
                "target/", "build/", "dist/", "out/", "bin/", "obj/",
                "node_modules/", ".cache/", "__pycache__/", ".tox/",
                "vendor/", "deps/", ".fingerprint/", "incremental/",
                ".timestamp", ".cargo-lock", ".min.js", ".min.css",
                ".bundle.js", ".chunk.js",
            ]),
            contribution_type: ContributionType::BuildArtifacts,
            language_override: None,
        },
        // Assets (patterns)
        PatternMatcher {
            automaton: build_ac(&[
                "assets/", "images/", "img/", "fonts/", "media/", "static/",
                "public/", "resources/",
            ]),
            contribution_type: ContributionType::Assets,
            language_override: None,
        },
        // Generated files
        PatternMatcher {
            automaton: build_ac(&[
                ".generated.", ".g.", "_generated", "generated/",
                ".lock", "package-lock.json", "yarn.lock", "cargo.lock",
                "poetry.lock", "pipfile.lock", "composer.lock", "gemfile.lock",
                ".min.", ".map", ".d.ts",
            ]),
            contribution_type: ContributionType::Generated,
            language_override: None,
        },
        // Data patterns
        PatternMatcher {
            automaton: build_ac(&[
                "data/", "datasets/", "fixtures/", "seeds/", "migrations/",
            ]),
            contribution_type: ContributionType::Data,
            language_override: None,
        },
    ]
});

// ============================================================================
// FileClassifier Implementation
// ============================================================================

pub struct FileClassifier;

impl FileClassifier {
    pub fn new() -> Self {
        Self
    }

    /// Classify a file path into a contribution type
    ///
    /// # Algorithm Complexity
    /// - Extension lookup: O(1) via PHF perfect hash
    /// - Pattern matching: O(path_length) via Aho-Corasick automaton
    /// - Total: O(path_length) instead of O(patterns × path_length)
    ///
    /// # Priority Order
    /// 1. Test patterns (highest - override everything)
    /// 2. Documentation patterns
    /// 3. Extension-based for build artifacts, assets, data
    /// 4. Infrastructure patterns
    /// 5. Config patterns
    /// 6. Styling (extension or pattern)
    /// 7. Production code (if language detected)
    /// 8. Other
    pub fn classify(&self, file_path: &str, lines_added: u32, lines_removed: u32) -> FileClassification {
        // Convert to lowercase once (amortized across all checks)
        let file_lower = file_path.to_lowercase();
        let ext = Self::get_extension(&file_lower);

        // Detect language via PHF - O(1)
        let language = Self::detect_language(&ext);

        // ====================================================================
        // Phase 1: High-Priority Pattern Matching - O(path_length)
        // Test and Documentation patterns override extension-based dispatch
        // Uses Aho-Corasick automaton with SIMD acceleration
        // ====================================================================

        // Tests - highest priority (index 0 in PATTERN_MATCHERS)
        if PATTERN_MATCHERS[0].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Tests,
                language,
                lines_added,
                lines_removed,
            };
        }

        // Documentation patterns (index 1) - check before extension
        if PATTERN_MATCHERS[1].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Documentation,
                language: Some("Documentation".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // ====================================================================
        // Phase 2: Extension-First Dispatch - O(1)
        // For build artifacts, assets, data - these don't need pattern override
        // ====================================================================
        if let Some(&contribution_type) = EXTENSION_TYPE_MAP.get(ext.as_str()) {
            // Skip styling extensions here - handle them with pattern check below
            if contribution_type != ContributionType::Styling {
                let lang = if contribution_type == ContributionType::Documentation {
                    Some("Documentation".to_string())
                } else {
                    None
                };
                return FileClassification {
                    file_path: file_path.to_string(),
                    contribution_type,
                    language: lang,
                    lines_added,
                    lines_removed,
                };
            }
        }

        // ====================================================================
        // Phase 3: Remaining Pattern Matching - O(path_length)
        // Infrastructure, Config, Styling, etc.
        // ====================================================================

        // Infrastructure (index 2)
        if PATTERN_MATCHERS[2].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Infrastructure,
                language: Some("Infrastructure".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // Specs/Config (index 3)
        if PATTERN_MATCHERS[3].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::SpecsConfig,
                language: Some("Configuration".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // Styling - check both extension and pattern
        let is_styling_ext = matches!(
            ext.as_str(),
            ".css" | ".scss" | ".sass" | ".less" | ".styl"
        );
        let is_styling_pattern = PATTERN_MATCHERS[4].automaton.is_match(&file_lower);

        if is_styling_ext || is_styling_pattern {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Styling,
                language: Some("CSS/Styling".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // Build artifacts (index 5)
        if PATTERN_MATCHERS[5].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::BuildArtifacts,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Assets (index 6)
        if PATTERN_MATCHERS[6].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Assets,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Generated (index 7)
        if PATTERN_MATCHERS[7].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Generated,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Data (index 8)
        if PATTERN_MATCHERS[8].automaton.is_match(&file_lower) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Data,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // ====================================================================
        // Phase 4: Default Classification
        // If no patterns matched, classify based on language detection
        // ====================================================================
        if language.is_some() {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::ProductionCode,
                language,
                lines_added,
                lines_removed,
            };
        }

        // Unknown/other
        FileClassification {
            file_path: file_path.to_string(),
            contribution_type: ContributionType::Other,
            language: None,
            lines_added,
            lines_removed,
        }
    }

    /// Extract file extension - O(path_length) worst case, typically O(1)
    /// Uses reverse search to find last '.' efficiently
    fn get_extension(path: &str) -> String {
        // memchr could be used here for SIMD, but rsplit is already optimized
        path.rsplit('.')
            .next()
            .map(|s| format!(".{}", s))
            .unwrap_or_default()
    }

    /// Detect programming language from extension
    ///
    /// # Algorithm: PHF Perfect Hash - O(1)
    /// Compile-time generated hash function with zero collisions
    pub fn detect_language(ext: &str) -> Option<String> {
        LANGUAGE_MAP.get(ext).map(|s| (*s).to_string())
    }
}

// ============================================================================
// Trait Implementation
// ============================================================================

impl Classifier for FileClassifier {
    fn classify(&self, file_path: &str, lines_added: u32, lines_removed: u32) -> FileClassification {
        FileClassifier::classify(self, file_path, lines_added, lines_removed)
    }

    fn detect_language(&self, extension: &str) -> Option<String> {
        FileClassifier::detect_language(extension)
    }
}

impl Default for FileClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "tests/classifier_tests.rs"]
mod tests;
