# Fabian Tax

**Senior Technology Leader | 25 Years Experience | CTO / Fractional CTO / Senior C# Developer**

*Building scalable solutions and leading high-performance teams*

---

## 👋 About Me

I'm a senior technology leader with **25 years of development experience**, 15+ years building scalable applications and 7+ years leading international development teams. I transform technology organizations from startup-phase to enterprise-grade while achieving market leadership positions.

**My approach:** Remove bottlenecks and dependencies for sustainable growth—both in code architecture and team dynamics.

---

## 🎯 What I'm Looking For

- **Senior C# Developer** positions
- **Technical Architecture** consulting

**Focus Areas:** FinTech, RegTech, Enterprise SaaS, Start-ups needing technical foundation, AI/ML applications

**Location:** Amsterdam area (Weesp, Naarden, Almere, Bussum, Hilversum) or remote/hybrid

---

## 💼 Featured Projects

### PEM Pal
**AI-powered energy management for ME/CFS patients and cancer survivors with mitochondrial dysfunction**

*(**P**ersonal **E**nergy **M**onitor, **P**redictor & **A**nalytics **L**ogger)*

**Tech Stack:** Rust, Deno, Burn (ML), DuckDB-WASM, LSTM/TFT models

**What it does:** Privacy-first health analytics that discovers personal crash patterns from wearable data. Predicts Post-Exertional Malaise (PEM) risk using HRV, heart rate, sleep, and activity metrics—helping users avoid energy crashes that can last days or weeks.

**Who it helps:**
- **ME/CFS patients** - Manage energy envelopes and prevent debilitating crashes
- **Cancer survivors** - Navigate recovery with mitochondrial damage from chemotherapy/radiation
- **Long COVID patients** - Understand post-viral energy patterns and pacing

**Business Value:** Personalized ML models identify individual warning signs and safe activity levels. All data processed locally in the browser for complete privacy—no health data leaves the user's device.

---

### fab-swarm
**Self-improving multi-agent orchestration system with NER and knowledge graph capabilities**

**Tech Stack:** Rust, libSQL/Turso, GLiNER, Tree-sitter, MCP Server

**What it does:** Enables AI agents to coordinate as a swarm using stigmergy (environment-based coordination), eliminating context bloat while enabling massive parallelization. Self-improving system that learns optimal execution strategies over time.

**Key Crates:**
- **fab-swarm** - Core CLI and MCP server with 40+ tools, self-healing swarm orchestration
- **fab-brain** - Personal knowledge graph with semantic search, "second brain" for AI-assisted knowledge curation
- **fab-learn** - Learning system that tracks outcomes, learns optimal tier routing over time
- **fab-entity** - High-performance graph-based NER with GLiNER (NAACL 2024 SOTA zero-shot entity extraction)
- **fab-codebox** - Tree-sitter AST cache with query REPL for code intelligence (40+ languages)
- **fab-lint** - Fast technical debt linter with jj integration

**What Makes It Different:**
- **Nano-agents**: Deterministic tasks execute in ~50-500μs without LLM calls (vs. seconds for traditional agents)
- **Stigmergy over messaging**: Agents coordinate through shared environment, not message passing (avoids N² message explosion)
- **Auto-improving**: fab-learn tracks which model tiers work best for each task type, continuously optimizing
- **Self-healing**: Automatic recovery from crashed agents with zero downtime
- **Production-ready**: Follows hexagonal architecture, comprehensive testing with cargo-llvm-cov (44% coverage)

**Practices Followed:**
- Hexagonal architecture with clean separation of concerns
- Dependency injection for testability
- Circuit breaker pattern for external API resilience
- Rate limit pooling for distributed systems
- Graceful shutdown with LIFO cleanup ordering

---

### repository-pattern-analyzers
**Performance analyzers for C# 11-14 and .NET 9-10**

**Tech Stack:** C#, Roslyn Analyzers, .NET 9-10 RC1

**What it does:** 23 Roslyn analyzers providing 10-200x performance improvements for C# codebases

**Business Value:** Catches performance anti-patterns at compile-time before they reach production, eliminating costly refactoring cycles

---

### agentdb-net
**Complete C# 14 implementation of Google's ReasoningBank with self-learning capabilities**

**Tech Stack:** C# 14, .NET 10, TensorPrimitives, SIMD (AVX-512/ARM SVE), Microsoft Agent Framework

**What it does:** Production-ready AI memory engine built on Google's ReasoningBank architecture, fully implemented in C# 14 with .NET 10's native TensorPrimitives and SIMD vectorization for 30-250x performance improvements.

**Self-Learning Capabilities:**
- **9 Modern RL Algorithms**: MCTS, DQN, Dueling DQN, Rainbow, A2C, PPO, SAC, Q-Learning, Multi-Agent systems
- **Reflexion Memory**: Self-critique and learning from failures
- **Skill Library**: Pattern consolidation with k-means clustering
- **Causal Memory Graph**: Pearl's do-calculus for understanding cause-and-effect
- **Propensity Score Methods**: IPW (Inverse Probability Weighting) for causal inference

**Key Features:**
- Hardware-accelerated vector operations with TensorPrimitives (AVX-512/ARM SVE)
- Zero-allocation hot paths with Span<T> for maximum performance
- Microsoft Agent Framework integration for AI workflow orchestration
- 340+ tests with 90-95% coverage
- 20 NuGet packages ready for production
- Docker support with multi-stage builds

**Performance:**
- 30-250x faster than JavaScript implementation
- Batch operations: 50,000 vector inserts/sec, 100,000 deletes/sec
- 75% memory reduction compared to Node.js (200MB vs 800MB for 1M vectors)

**Status:** 100% complete, production-ready, fully documented with 25+ guides

---

### Atlas
**Real-time code graph visualization for developers building with AI assistants**

**Tech Stack:** Rust, Tree-sitter, Graph Algorithms, Matryoshka Embeddings, RocksDB

**What it does:** Enables developers to visually watch their codebase architecture evolve in real-time. Generates C4 diagrams instantly as you code, with incremental scanning that updates only changed files. Multi-language support (Rust, Python, TypeScript, C#) with Claude Code integration for automatic architecture updates.

**Key Features:**
- **Real-time C4 visualization** - Watch mode updates diagrams on every file change
- **Incremental scanning** - Content-addressable caching, only re-parses modified files
- **Complexity heatmaps** - Color-coded diagrams showing technical debt hotspots
- **Multi-language parsing** - Tree-sitter based support for 4+ languages
- **AI assistant integration** - Hooks into Claude Code for automatic updates

**Impact:** Fixed critical SIGSEGV issues with Matryoshka embeddings, enabling stable large-scale code analysis. Phase 1A complete with 20+ integrated crates covering parsing, storage, metrics, distributed coordination, and ML capabilities.

---

## 🔥 Core Competencies

| **Area** | **Technologies** |
|----------|------------------|
| **Languages** | C#, Rust, TypeScript, Python, SQL |
| **Frameworks** | .NET, ASP.NET Core, React, Node.js |
| **Databases** | PostgreSQL, SQL Server, Redis, Elasticsearch |
| **Cloud** | Azure, Cloudflare, Fly.io, Docker |
| **Architecture** | Modular Monoliths, Microservices, Event-Driven, DDD, SOLID |
| **Leadership** | Team Building, Technical Strategy, Agile/Scrum |

---

## 📊 Quick Stats

| **Metric** | **Value** |
|------------|-----------|
| **Years Experience** | 25+ |
| **Leadership Experience** | 7+ years leading teams |
| **Active Repositories** | 35+ |
| **Primary Focus** | C#, Rust, TypeScript, Python |
| **Recent Achievement** | Transformed Reptune tech from startup to enterprise-grade, achieving top-3 global market position |

---

## 🏆 Leadership Highlights

- **Fractional CTO** - Helped startups scale from MVP to enterprise-grade architecture, led international teams through critical transformation phases
- **Technical Lead** - Architected FinTech, RegTech, and SaaS solutions

---

## 📫 Get In Touch

- **Email:** [Available upon request]
- **Location:** Amsterdam area, Netherlands
- **Open To:** New opportunities, connections, and collaborations
