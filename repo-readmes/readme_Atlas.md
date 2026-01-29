# 📚 Atlas Documentation

Welcome to the Atlas documentation hub. This directory contains comprehensive guides, reference materials, and project information for understanding and developing the Atlas code graph analysis tool.

## 🚀 Quick Start

**New to Atlas?** Start here:
1. 📖 Read [MVP Scope](./active/project-status/MVP-SCOPE.md) - Understand the project vision
2. 📊 Check [Project Status](./active/project-status/STATUS.md) - See where we are now
3. 🛣️ View [Roadmap](./active/project-status/ROADMAP.md) - See the development roadmap

---

## 📑 Documentation Structure

### 🎯 **[Project Status & Roadmap](./active/project-status/)**
Find current project information, status updates, and strategic direction.

- **[STATUS.md](./active/project-status/STATUS.md)** - Latest project status report
  - Phase progress and milestones
  - Component completion status
  - Critical updates and blockers

- **[MVP-SCOPE.md](./active/project-status/MVP-SCOPE.md)** - Project vision and MVP definition
  - User personas and pain points
  - Core features and success metrics
  - Timeline and phases

- **[ROADMAP.md](./active/project-status/ROADMAP.md)** - Development roadmap
  - Phase breakdown and timeline
  - Upcoming priorities
  - Technical direction

- **[ATLAS-SYSTEM-STATUS-OCT-28-2025.md](./active/project-status/ATLAS-SYSTEM-STATUS-OCT-28-2025.md)** - System status snapshot
  - Component overview
  - Integration status
  - Performance metrics

**Navigation**: [→ Project Status Directory](./active/project-status/README.md)

### 📋 **[Reference Materials](./active/reference/)**
Technical reference documentation, performance analysis, and research findings.

- **[QUICK-PERFORMANCE-REFERENCE.md](./active/reference/QUICK-PERFORMANCE-REFERENCE.md)** - Quick performance reference
  - Performance metrics at a glance
  - Optimization guidelines
  - Benchmarking reference

- **[PERFORMANCE-ANALYSIS-SUMMARY.md](./active/reference/PERFORMANCE-ANALYSIS-SUMMARY.md)** - Comprehensive performance analysis
  - Detailed performance metrics
  - Bottleneck analysis
  - Optimization recommendations

- **[performance-analysis-vector-search.md](./active/reference/performance-analysis-vector-search.md)** - Vector search performance
  - Vector search optimization
  - Embedding performance analysis
  - Benchmark results

- **[DOCUMENTATION-SYSTEM-SUMMARY.md](./active/reference/DOCUMENTATION-SYSTEM-SUMMARY.md)** - Documentation system overview
  - Documentation architecture
  - File organization principles
  - Governance standards

**Navigation**: [→ Reference Materials Directory](./active/reference/README.md)

### 🏗️ **Architecture & Design**

- **[ARCHITECTURE/](./ARCHITECTURE/)** - C4 model and system architecture
  - System context diagrams
  - Container diagrams
  - Component diagrams
  - Integration points

- **[ARCHITECTURE/architecture-decision-records/](./ARCHITECTURE/architecture-decision-records/)** - ADRs
  - Documented design decisions
  - Rationale and trade-offs
  - Implementation guides

### 📂 **Other Documentation Areas**

- **[RESEARCH/](./RESEARCH/)** - Research archive
  - Phase-specific investigations
  - Technical deep-dives
  - Feature research

- **[PHASE-GUIDES/](./PHASE-GUIDES/)** - Phase-specific documentation
  - Phase objectives and scope
  - Detailed implementation guides
  - Success criteria

- **[in-progress/](./in-progress/)** - Work-in-progress documentation
  - Research and investigation documents
  - Draft specifications
  - Pending review materials

- **[archive/](./archive/)** - Historical documentation
  - Completed research and findings
  - Historical decisions
  - Reference materials

---

## 🗺️ How to Navigate

### By Role

**Developers**:
1. Start with [STATUS.md](./active/project-status/STATUS.md) for current state
2. Review [ARCHITECTURE/](./ARCHITECTURE/) for system design
3. Check [PHASE-GUIDES/](./PHASE-GUIDES/) for implementation details
4. Reference [active/reference/](./active/reference/) for APIs and specs

**Project Managers**:
1. Review [MVP-SCOPE.md](./active/project-status/MVP-SCOPE.md) for vision
2. Check [STATUS.md](./active/project-status/STATUS.md) for progress
3. See [ROADMAP.md](./active/project-status/ROADMAP.md) for timeline

**Researchers**:
1. Browse [RESEARCH/](./RESEARCH/) for investigations
2. Check [in-progress/](./in-progress/) for current work
3. Reference [archive/](./archive/) for historical findings

### By Topic

**System Architecture**: [ARCHITECTURE/](./ARCHITECTURE/) → [architecture-decision-records/](./ARCHITECTURE/architecture-decision-records/)

**Performance**: [QUICK-PERFORMANCE-REFERENCE.md](./active/reference/QUICK-PERFORMANCE-REFERENCE.md) → [PERFORMANCE-ANALYSIS-SUMMARY.md](./active/reference/PERFORMANCE-ANALYSIS-SUMMARY.md)

**Vector Search**: [performance-analysis-vector-search.md](./active/reference/performance-analysis-vector-search.md)

**Phase Information**: [ROADMAP.md](./active/project-status/ROADMAP.md) → [PHASE-GUIDES/](./PHASE-GUIDES/)

---

## 📊 Key Metrics

| Metric | Status | Details |
|--------|--------|---------|
| **Phase** | Phase 1A ✅ Complete, Phase 1B Ready | 80% progress |
| **Core Components** | ✅ Operational | Parser, Storage, Metrics |
| **Critical Issues** | ✅ Resolved | SIGSEGV fixed with Matryoshka embeddings |
| **Test Suite** | ✅ Passing | Integration tests operational |
| **Documentation** | 📖 Current | Updated through 2025-10-31 |

---

## 🔄 Documentation Standards

All documentation in this repository follows the governance standards defined in:
- [docs/.governance/FILE-NAMING-CONVENTION.md](./docs/.governance/FILE-NAMING-CONVENTION.md)
- [docs/.governance/YAML-FRONTMATTER-SCHEMA.md](./docs/.governance/YAML-FRONTMATTER-SCHEMA.md)

### File Organization

**Active Documentation** (`docs/active/`):
- Published, approved materials
- Reference documents
- Current best practices
- Filename: `*.md` (no timestamp required)

**In-Progress Documentation** (`docs/in-progress/`):
- Work-in-progress materials
- Research and investigations
- Draft specifications
- Filename: `YYYYMMDD-HHMMSS-{author}-{slug}.{phase}.md`

**Archived Documentation** (`docs/archive/`):
- Historical materials
- Completed research
- Reference for decisions made
- Filename: `*.md` (no timestamp required)

---

## 🔗 Related Resources

- **Main Project**: [README.md](../README.md)
- **Crates**: [crates/](../crates/) - Rust implementation
- **CLI Tool**: [crates/atlas-cli/](../crates/atlas-cli/)
- **Agents & Skills**: [.claude/](../.claude/) - Claude Code agents

---

## 📝 Last Updated

- **Date**: 2025-10-31
- **Status**: Documentation system reorganized with improved navigation
- **Phase**: Phase 1A Complete, Phase 1B Ready to Start

---

## ❓ Questions or Feedback?

- 📖 Browse the documentation structure above
- 🔍 Search for specific topics using the "By Topic" section
- 📧 Check [.governance/](./docs/.governance/) for standards and templates
