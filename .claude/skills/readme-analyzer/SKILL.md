# README Analyzer

Analyzes README.md files against repository best practices and provides actionable feedback.

## Description

This skill evaluates README files for completeness and quality based on industry best practices for open-source and professional repositories. It checks for essential sections, formatting, and content quality.

## Usage

```bash
# Analyze current directory's README
readme-analyzer

# Analyze specific README file
readme-analyzer path/to/README.md

# Analyze with detailed output
readme-analyzer --verbose

# Generate a score (0-100)
readme-analyzer --score

# Check specific categories only
readme-analyzer --check sections,formatting,links
```

## Best Practices Checked

### Essential Sections (Critical)
- **Title/Project Name** - Clear project identification
- **Description** - Brief summary of what the project does
- **Installation** - How to install/setup the project
- **Usage** - Basic usage examples or commands
- **License** - License information (SPDX identifier recommended)
- **Badges** - Build status, version, coverage (optional but recommended)

### Important Sections (High Priority)
- **Prerequisites** - Required dependencies, tools, or environment
- **Configuration** - How to configure the project
- **Contributing** - Guidelines for contributors
- **Changelog/Version History** - Recent changes and version info
- **Authors/Maintainers** - Who maintains this project
- **Acknowledgments** - Credits to contributors or dependencies

### Nice-to-Have Sections (Medium Priority)
- **Table of Contents** - Navigation for long READMEs
- **Screenshots/Demos** - Visual examples (especially for UI projects)
- **Features** - List of key features/capabilities
- **Roadmap** - Planned features or future work
- **FAQ** - Common questions and answers
- **Support/Contact** - Where to get help

### Quality Checks
- **Formatting** - Proper Markdown formatting
- **Links** - All links are valid and working
- **Code Examples** - Code blocks are properly formatted with syntax highlighting
- **Spelling/Grammar** - Basic language quality checks
- **Consistency** - Consistent terminology and style

## Scoring

Scores are calculated as follows:
- **Essential Sections**: 50% of score
- **Important Sections**: 30% of score
- **Nice-to-Have Sections**: 15% of score
- **Quality Checks**: 5% of score

**Score Ranges:**
- 90-100: Excellent ⭐⭐⭐
- 75-89: Good ⭐⭐
- 60-74: Fair ⭐
- <60: Needs Improvement

## Examples

```bash
# Basic analysis
$ readme-analyzer
📊 README Analysis: myproject/README.md
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 72/100 (Fair)

✅ Present (6/10):
  • Title ✓
  • Description ✓
  • Installation ✓
  • Usage ✓
  • License ✓
  • Badges ✓

❌ Missing (4/10):
  • Prerequisites
  • Contributing
  • Changelog
  • Authors

⚠️  Quality Issues:
  • 2 broken links detected
  • No syntax highlighting on code blocks
  • Consider adding TOC for long content

💡 Recommendations:
  1. Add Prerequisites section before Installation
  2. Include CONTRIBUTING.md or add Contributing section
  3. Add table of contents for easier navigation
```

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `--verbose, -v` | Show detailed analysis | false |
| `--score` | Output only the numeric score | false |
| `--check` | Comma-separated categories to check | all |
| `--format` | Output format (text, json, markdown) | text |
| `--output, -o` | Save report to file | stdout |
| `--help, -h` | Show help message | - |

## Categories

- `sections` - Check for required sections
- `formatting` - Check Markdown formatting quality
- `links` - Validate all links
- `code` - Check code block formatting
- `spelling` - Basic spelling check
- `all` - All checks (default)

## Exit Codes

- `0` - All checks passed
- `1` - Critical issues found
- `2` - Usage error

## See Also

- [GitHub README Guide](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/readme)
- [Awesome README](https://github.com/matiassingers/awesome-readme)
- [Make a README](https://www.makeareadme.com/)
