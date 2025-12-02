"""
File classifier for categorizing contributions by type.
"""

import os
from dataclasses import dataclass
from enum import Enum
from typing import Dict, List, Optional


class ContributionType(Enum):
    """Types of code contributions."""
    PRODUCTION_CODE = "production_code"
    TESTS = "tests"
    DOCUMENTATION = "documentation"
    SPECS_CONFIG = "specs_config"
    INFRASTRUCTURE = "infrastructure"
    STYLING = "styling"
    OTHER = "other"


@dataclass
class FileClassification:
    """Classification result for a file."""
    file_path: str
    contribution_type: ContributionType
    language: Optional[str]
    lines_added: int = 0
    lines_removed: int = 0


class FileClassifier:
    """Classifies files into contribution types."""

    # Test patterns
    TEST_PATTERNS = [
        'test_', '_test.', '.test.', 'tests/', '/test/', 'spec_', '_spec.',
        '.spec.', 'specs/', '/spec/', '__tests__/', '.tests.', 'testing/',
        'unittest', 'pytest', 'jest', 'mocha', 'cypress/', 'e2e/'
    ]

    # Documentation patterns
    DOC_PATTERNS = [
        'readme', 'changelog', 'contributing', 'license', 'authors',
        'docs/', '/doc/', 'documentation/', '.md', '.rst', '.txt',
        'wiki/', 'guide', 'manual', 'api-docs/'
    ]

    DOC_EXTENSIONS = ['.md', '.rst', '.txt', '.adoc', '.wiki']

    # Specs and config patterns
    SPEC_CONFIG_PATTERNS = [
        'package.json', 'tsconfig', 'webpack', 'babel', 'eslint', 'prettier',
        '.yaml', '.yml', '.json', '.toml', '.ini', '.cfg', '.conf',
        'openapi', 'swagger', 'schema', '.env', 'config/', '/config',
        'settings', '.editorconfig', '.gitignore', '.dockerignore',
        'pyproject.toml', 'setup.py', 'setup.cfg', 'requirements',
        'Gemfile', 'Cargo.toml', 'go.mod', 'pom.xml', 'build.gradle',
        '.github/', '.gitlab-ci', 'azure-pipelines', 'Jenkinsfile',
        '.travis', 'circle.yml', 'bitbucket-pipelines'
    ]

    # Infrastructure patterns
    INFRA_PATTERNS = [
        'dockerfile', 'docker-compose', 'kubernetes/', 'k8s/', 'helm/',
        'terraform/', '.tf', 'ansible/', 'puppet/', 'chef/',
        'cloudformation', 'pulumi/', 'vagrant', 'makefile', 'cmake',
        'deploy/', 'deployment/', 'infra/', 'infrastructure/',
        'scripts/deploy', 'scripts/build', '.sh', 'nginx', 'apache'
    ]

    # Styling patterns
    STYLE_PATTERNS = [
        '.css', '.scss', '.sass', '.less', '.styl', '.styled.',
        'styles/', '/style/', 'theme', '.tailwind'
    ]

    # Language detection by extension
    LANGUAGE_MAP = {
        '.py': 'Python',
        '.js': 'JavaScript',
        '.ts': 'TypeScript',
        '.tsx': 'TypeScript (React)',
        '.jsx': 'JavaScript (React)',
        '.cs': 'C#',
        '.java': 'Java',
        '.go': 'Go',
        '.rs': 'Rust',
        '.rb': 'Ruby',
        '.php': 'PHP',
        '.swift': 'Swift',
        '.kt': 'Kotlin',
        '.scala': 'Scala',
        '.c': 'C',
        '.cpp': 'C++',
        '.h': 'C/C++ Header',
        '.hpp': 'C++ Header',
        '.vue': 'Vue',
        '.svelte': 'Svelte',
        '.html': 'HTML',
        '.sql': 'SQL',
        '.r': 'R',
        '.m': 'MATLAB/Objective-C',
        '.pl': 'Perl',
        '.lua': 'Lua',
        '.dart': 'Dart',
        '.elm': 'Elm',
        '.ex': 'Elixir',
        '.exs': 'Elixir',
        '.erl': 'Erlang',
        '.hs': 'Haskell',
        '.clj': 'Clojure',
        '.fs': 'F#',
        '.fsx': 'F#',
    }

    def classify(self, file_path: str, lines_added: int = 0, lines_removed: int = 0) -> FileClassification:
        """Classify a file based on its path and extension."""
        file_lower = file_path.lower()
        ext = os.path.splitext(file_path)[1].lower()

        # Determine language
        language = self.LANGUAGE_MAP.get(ext)

        # Check for tests first (high priority)
        if any(pattern in file_lower for pattern in self.TEST_PATTERNS):
            return FileClassification(
                file_path=file_path,
                contribution_type=ContributionType.TESTS,
                language=language,
                lines_added=lines_added,
                lines_removed=lines_removed
            )

        # Check for documentation
        if ext in self.DOC_EXTENSIONS or any(pattern in file_lower for pattern in self.DOC_PATTERNS):
            return FileClassification(
                file_path=file_path,
                contribution_type=ContributionType.DOCUMENTATION,
                language='Documentation',
                lines_added=lines_added,
                lines_removed=lines_removed
            )

        # Check for infrastructure
        if any(pattern in file_lower for pattern in self.INFRA_PATTERNS):
            return FileClassification(
                file_path=file_path,
                contribution_type=ContributionType.INFRASTRUCTURE,
                language=language or 'Infrastructure',
                lines_added=lines_added,
                lines_removed=lines_removed
            )

        # Check for specs/config
        if any(pattern in file_lower for pattern in self.SPEC_CONFIG_PATTERNS):
            return FileClassification(
                file_path=file_path,
                contribution_type=ContributionType.SPECS_CONFIG,
                language='Configuration',
                lines_added=lines_added,
                lines_removed=lines_removed
            )

        # Check for styling
        if any(pattern in file_lower for pattern in self.STYLE_PATTERNS):
            return FileClassification(
                file_path=file_path,
                contribution_type=ContributionType.STYLING,
                language='CSS/Styling',
                lines_added=lines_added,
                lines_removed=lines_removed
            )

        # Default to production code if it has a known language
        if language:
            return FileClassification(
                file_path=file_path,
                contribution_type=ContributionType.PRODUCTION_CODE,
                language=language,
                lines_added=lines_added,
                lines_removed=lines_removed
            )

        # Unknown/other
        return FileClassification(
            file_path=file_path,
            contribution_type=ContributionType.OTHER,
            language=None,
            lines_added=lines_added,
            lines_removed=lines_removed
        )

    def get_contribution_type_label(self, contribution_type: ContributionType) -> str:
        """Get human-readable label for contribution type."""
        labels = {
            ContributionType.PRODUCTION_CODE: "Production Code",
            ContributionType.TESTS: "Tests",
            ContributionType.DOCUMENTATION: "Documentation",
            ContributionType.SPECS_CONFIG: "Specs & Config",
            ContributionType.INFRASTRUCTURE: "Infrastructure",
            ContributionType.STYLING: "Styling",
            ContributionType.OTHER: "Other",
        }
        return labels.get(contribution_type, "Unknown")
