use crate::traits::Classifier;
use serde::{Deserialize, Serialize};

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

pub struct FileClassifier;

impl FileClassifier {
    const TEST_PATTERNS: &'static [&'static str] = &[
        "test_", "_test.", ".test.", "tests/", "/test/", "spec_", "_spec.",
        ".spec.", "specs/", "/spec/", "__tests__/", ".tests.", "testing/",
        "unittest", "pytest", "jest", "mocha", "cypress/", "e2e/",
    ];

    const DOC_PATTERNS: &'static [&'static str] = &[
        "readme", "changelog", "contributing", "license", "authors",
        "docs/", "/doc/", "documentation/", "wiki/", "guide", "manual", "api-docs/",
    ];

    const DOC_EXTENSIONS: &'static [&'static str] = &[".md", ".rst", ".txt", ".adoc", ".wiki"];

    const SPEC_CONFIG_PATTERNS: &'static [&'static str] = &[
        "package.json", "tsconfig", "webpack", "babel", "eslint", "prettier",
        ".yaml", ".yml", ".json", ".toml", ".ini", ".cfg", ".conf",
        "openapi", "swagger", "schema", ".env", "config/", "/config",
        "settings", ".editorconfig", ".gitignore", ".dockerignore",
        "pyproject.toml", "setup.py", "setup.cfg", "requirements",
        "gemfile", "cargo.toml", "go.mod", "pom.xml", "build.gradle",
        ".github/", ".gitlab-ci", "azure-pipelines", "jenkinsfile",
        ".travis", "circle.yml", "bitbucket-pipelines",
    ];

    const INFRA_PATTERNS: &'static [&'static str] = &[
        "dockerfile", "docker-compose", "kubernetes/", "k8s/", "helm/",
        "terraform/", ".tf", "ansible/", "puppet/", "chef/",
        "cloudformation", "pulumi/", "vagrant", "makefile", "cmake",
        "deploy/", "deployment/", "infra/", "infrastructure/",
        "scripts/deploy", "scripts/build", "nginx", "apache",
    ];

    const STYLE_PATTERNS: &'static [&'static str] = &[
        ".css", ".scss", ".sass", ".less", ".styl", ".styled.",
        "styles/", "/style/", "theme", ".tailwind",
    ];

    // Build artifacts - compiled files, libraries, caches
    const BUILD_ARTIFACT_EXTENSIONS: &'static [&'static str] = &[
        ".o", ".obj", ".a", ".lib", ".so", ".dll", ".dylib", ".exe",
        ".rlib", ".rmeta", ".d", ".pdb", ".ilk", ".exp",
        ".class", ".jar", ".war", ".pyc", ".pyo", "__pycache__",
        ".wasm", ".wat",
        ".min.js", ".min.css", ".bundle.js", ".chunk.js",
    ];

    const BUILD_ARTIFACT_PATTERNS: &'static [&'static str] = &[
        "target/", "build/", "dist/", "out/", "bin/", "obj/",
        "node_modules/", ".cache/", "__pycache__/", ".tox/",
        "vendor/", "deps/", ".fingerprint/", "incremental/",
        ".timestamp", ".cargo-lock",
    ];

    // Assets - images, fonts, media, binary resources
    const ASSET_EXTENSIONS: &'static [&'static str] = &[
        ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".webp",
        ".ttf", ".otf", ".woff", ".woff2", ".eot",
        ".mp3", ".mp4", ".wav", ".ogg", ".webm", ".avi", ".mov",
        ".pdf", ".zip", ".tar", ".gz", ".rar", ".7z",
    ];

    const ASSET_PATTERNS: &'static [&'static str] = &[
        "assets/", "images/", "img/", "fonts/", "media/", "static/",
        "public/", "resources/",
    ];

    // Generated files - auto-generated code, lock files
    const GENERATED_PATTERNS: &'static [&'static str] = &[
        ".generated.", ".g.", "_generated", "generated/",
        ".lock", "package-lock.json", "yarn.lock", "cargo.lock",
        "poetry.lock", "pipfile.lock", "composer.lock", "gemfile.lock",
        ".min.", ".map", ".d.ts",
    ];

    // Data files - databases, datasets, logs
    const DATA_EXTENSIONS: &'static [&'static str] = &[
        ".csv", ".tsv", ".sqlite", ".db", ".sql",
        ".log", ".jsonl", ".ndjson", ".parquet", ".avro",
        ".xml", ".xls", ".xlsx",
    ];

    const DATA_PATTERNS: &'static [&'static str] = &[
        "data/", "datasets/", "fixtures/", "seeds/", "migrations/",
    ];

    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, file_path: &str, lines_added: u32, lines_removed: u32) -> FileClassification {
        let file_lower = file_path.to_lowercase();
        let ext = Self::get_extension(&file_lower);
        let language = Self::detect_language(&ext);

        // Check for tests first (high priority)
        if Self::TEST_PATTERNS.iter().any(|p| file_lower.contains(p)) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Tests,
                language,
                lines_added,
                lines_removed,
            };
        }

        // Check for documentation
        if Self::DOC_EXTENSIONS.iter().any(|e| file_lower.ends_with(e))
            || Self::DOC_PATTERNS.iter().any(|p| file_lower.contains(p))
        {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Documentation,
                language: Some("Documentation".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // Check for infrastructure
        if Self::INFRA_PATTERNS.iter().any(|p| file_lower.contains(p)) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Infrastructure,
                language: language.or(Some("Infrastructure".to_string())),
                lines_added,
                lines_removed,
            };
        }

        // Check for specs/config
        if Self::SPEC_CONFIG_PATTERNS.iter().any(|p| file_lower.contains(p)) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::SpecsConfig,
                language: Some("Configuration".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // Check for styling
        if Self::STYLE_PATTERNS.iter().any(|p| file_lower.contains(p)) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Styling,
                language: Some("CSS/Styling".to_string()),
                lines_added,
                lines_removed,
            };
        }

        // Check for build artifacts (should typically be gitignored)
        if Self::BUILD_ARTIFACT_EXTENSIONS.iter().any(|e| file_lower.ends_with(e))
            || Self::BUILD_ARTIFACT_PATTERNS.iter().any(|p| file_lower.contains(p))
        {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::BuildArtifacts,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Check for assets (images, fonts, media)
        if Self::ASSET_EXTENSIONS.iter().any(|e| file_lower.ends_with(e))
            || Self::ASSET_PATTERNS.iter().any(|p| file_lower.contains(p))
        {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Assets,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Check for generated files
        if Self::GENERATED_PATTERNS.iter().any(|p| file_lower.contains(p)) {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Generated,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Check for data files
        if Self::DATA_EXTENSIONS.iter().any(|e| file_lower.ends_with(e))
            || Self::DATA_PATTERNS.iter().any(|p| file_lower.contains(p))
        {
            return FileClassification {
                file_path: file_path.to_string(),
                contribution_type: ContributionType::Data,
                language: None,
                lines_added,
                lines_removed,
            };
        }

        // Default to production code if it has a known language
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

    fn get_extension(path: &str) -> String {
        path.rsplit('.')
            .next()
            .map(|s| format!(".{}", s))
            .unwrap_or_default()
    }

    fn detect_language(ext: &str) -> Option<String> {
        match ext {
            ".py" => Some("Python".to_string()),
            ".js" => Some("JavaScript".to_string()),
            ".ts" => Some("TypeScript".to_string()),
            ".tsx" => Some("TypeScript (React)".to_string()),
            ".jsx" => Some("JavaScript (React)".to_string()),
            ".cs" => Some("C#".to_string()),
            ".java" => Some("Java".to_string()),
            ".go" => Some("Go".to_string()),
            ".rs" => Some("Rust".to_string()),
            ".rb" => Some("Ruby".to_string()),
            ".php" => Some("PHP".to_string()),
            ".swift" => Some("Swift".to_string()),
            ".kt" => Some("Kotlin".to_string()),
            ".scala" => Some("Scala".to_string()),
            ".c" => Some("C".to_string()),
            ".cpp" | ".cc" | ".cxx" => Some("C++".to_string()),
            ".h" => Some("C/C++ Header".to_string()),
            ".hpp" => Some("C++ Header".to_string()),
            ".vue" => Some("Vue".to_string()),
            ".svelte" => Some("Svelte".to_string()),
            ".html" => Some("HTML".to_string()),
            ".sql" => Some("SQL".to_string()),
            ".r" => Some("R".to_string()),
            ".m" => Some("MATLAB/Objective-C".to_string()),
            ".pl" => Some("Perl".to_string()),
            ".lua" => Some("Lua".to_string()),
            ".dart" => Some("Dart".to_string()),
            ".elm" => Some("Elm".to_string()),
            ".ex" | ".exs" => Some("Elixir".to_string()),
            ".erl" => Some("Erlang".to_string()),
            ".hs" => Some("Haskell".to_string()),
            ".clj" => Some("Clojure".to_string()),
            ".fs" | ".fsx" => Some("F#".to_string()),
            ".sh" => Some("Shell".to_string()),
            ".ps1" => Some("PowerShell".to_string()),
            _ => None,
        }
    }
}

// Implement the Classifier trait for FileClassifier
impl Classifier for FileClassifier {
    fn classify(&self, file_path: &str, lines_added: u32, lines_removed: u32) -> FileClassification {
        // Delegate to the inherent method
        FileClassifier::classify(self, file_path, lines_added, lines_removed)
    }

    fn detect_language(&self, extension: &str) -> Option<String> {
        // Delegate to the inherent method (make it public via trait)
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
