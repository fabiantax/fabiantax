# Documentation Coordination Skill

Coordinates best practices across README, GitHub Wiki, and Docusaurus documentation.

## Purpose

Ensure consistency and quality across all documentation formats:
- **README.md** - Project overview, quick start, links to detailed docs
- **GitHub Wiki** - Comprehensive, searchable documentation
- **Docusaurus** - Static site with navigation, search, versions

## Best Practices

### README Structure

```markdown
# Project Title

[Badges]

Short description (one-liner)

## ✨ Features
- Feature 1
- Feature 2

## 🚀 Quick Start
Quick setup guide (3-5 commands)

## 🛠️ Tech Stack
| Component | Technology |
|-----------|-----------|

## 📚 Documentation
Full documentation in [Wiki](./.wiki/) or [docs/](./docs/)

## 🎯 Roadmap
- [x] Done
- [ ] In progress

## 📊 Project Stats
Key metrics

## 🔒 Privacy & Security (if applicable)
Privacy guarantees

## 🤝 Contributing
Link to CONTRIBUTING.md or wiki

## 📜 License
License info

## 👨‍💻 Author
Credits

---
<div align="center">
**⭐ Star us on GitHub**
**Made with ❤️ for community**
</div>
```

### Wiki Structure

Each wiki page should have:

```markdown
---
slug: page-name
title: Page Title
sidebar_position: 1
sidebar_label: 📘 Page Label
description: Meta description for SEO
keywords: [tag1, tag2, tag3]
---

import Mermaid from '@theme/Mermaid';

# Page Title

Content with Mermaid diagrams:

<Mermaid chart="graph LR
    A[A] --> B[B]" />

## Subsections

More content...

## Next Steps

- [Related Page 1](./Related-Page-1.md)
- [Related Page 2](./Related-Page-2.md)
```

### Docusaurus Compatibility

1. **Frontmatter** - Required for all pages
2. **Mermaid import** - `import Mermaid from '@theme/Mermaid';`
3. **Relative links** - Use `./Page-Name.md`
4. **Code blocks** - Use triple backticks with language
5. **Admonitions** - Use `:::tip`, `:::warning`, `:::danger`

## Coordination Rules

### Link Consistency

- **README → Wiki**: Link to `.wiki/Page-Name.md` files
- **README → Docusaurus**: Link to `/docs/page-name`
- **Wiki → Wiki**: Use `./Page-Name.md`
- **Wiki → README**: Use `../README.md`
- **External**: Use full URLs for GitHub pages

### Content Distribution

| Content Type | README | Wiki | Docusaurus |
|--------------|--------|------|------------|
| Overview | ✅ Full | ✅ Full | ✅ Full |
| Quick Start | ✅ Full | ✅ Full | ✅ Full |
| Detailed Docs | ❌ Link only | ✅ Full | ✅ Full |
| API Reference | ❌ Link only | ✅ Full | ✅ Full |
| Architecture | ❌ Summary only | ✅ Full | ✅ Full |
| Changelog | ❌ Link only | ✅ Full | ✅ Full |
| Contributing | ❌ Link only | ✅ Full | ✅ Full |

### Diagram Standards

**Always use Mermaid for diagrams:**

```markdown
<Mermaid chart="graph LR
    A[A] --> B[B]
    B --> C[C]" />
```

**Never use ASCII art:**

```markdown
❌ Don't do this:
A --> B
B --> C

✅ Do this:
<Mermaid chart="graph LR
    A[A] --> B[B]
    B --> C[C]" />
```

### Scoring System

**README Scoring (out of 100):**

| Section | Points | Criteria |
|---------|--------|----------|
| **Essential** | 50 | Title, description, installation, usage |
| **Important** | 30 | Features, docs link, contributing, license |
| **Nice-to-Have** | 15 | Badges, tech stack, roadmap, author |
| **Quality** | 5 | Formatting, links, spelling |

**Target: 90+ points**

## Usage Examples

### Creating a New Page

```bash
# Create wiki page
touch .wiki/New-Page.md

# Add frontmatter and content
cat > .wiki/New-Page.md << 'EOF'
---
slug: new-page
title: New Page
sidebar_position: N
---

import Mermaid from '@theme/Mermaid';

# New Page

Content here...
EOF

# Update Home.md to link to it
```

### Updating README

```bash
# Run this skill to check README score
docs-coord check-readme

# Auto-fix issues
docs-coord fix-readme
```

### Finding Broken Links

```bash
# Check all links
docs-coord check-links

# Fix broken links
docs-coord fix-links
```

## Command Reference

### `check-readme`

Analyzes README against best practices and outputs score.

**Usage:**
```bash
docs-coord check-readme [--path PATH]
```

**Output:**
```
README Score: 95/100

✅ Essential (50/50)
✅ Important (30/30)
✅ Nice-to-Have (15/15)
⚠️  Quality (0/5) - Fix formatting issues

Missing:
- Better description
- Improve code block formatting
```

### `check-wiki`

Validates wiki pages for Docusaurus compatibility.

**Usage:**
```bash
docs-coord check-wiki [--path PATH]
```

**Output:**
```
Wiki Pages: 12
✅ All pages have frontmatter
✅ All pages use Mermaid correctly
⚠️  3 pages have broken internal links
❌ 1 page missing description
```

### `sync-links`

Updates all links across README, wiki, and docs.

**Usage:**
```bash
docs-coord sync-links [--dry-run]
```

### `validate-mermaid`

Checks Mermaid diagram syntax.

**Usage:**
```bash
docs-coord validate-mermaid
```

## Troubleshooting

### Wiki Pages Not Rendering

**Issue:** Mermaid diagrams not showing

**Fix:**
```markdown
Add this import at the top:
import Mermaid from '@theme/Mermaid';
```

### Broken Links After Move

**Issue:** Links broke after moving files

**Fix:**
```bash
docs-coord fix-links
```

### Missing Frontmatter

**Issue:** Docusaurus not finding pages

**Fix:**
```markdown
Add to top of file:
---
slug: page-name
title: Page Title
sidebar_position: 1
---
```

## Templates

### Wiki Page Template

```markdown
---
slug: PAGE_NAME
title: PAGE_TITLE
sidebar_position: N
sidebar_label: 📘 LABEL
description: DESCRIPTION
keywords: [tag1, tag2, tag3]
---

import Mermaid from '@theme/Mermaid';

# TITLE

Overview...

## Section 1

<Mermaid chart="graph LR
    A[A] --> B[B]" />

Content...

## Next Steps

- [Related Page](./Related-Page.md)
```

### README Template

See README Structure section above.

## See Also

- [Docusaurus Docs](https://docusaurus.io/docs)
- [Mermaid Syntax](https://mermaid.js.org/syntax/flowchart.html)
- [GitHub Wiki Guide](https://docs.github.com/en/wiki)
