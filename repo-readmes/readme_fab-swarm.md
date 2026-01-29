# fab-swarm

**Minimal, fast multi-agent orchestration for Claude Code**

[![Crates.io](https://img.shields.io/crates/v/fab-swarm.svg)](https://crates.io/crates/fab-swarm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

fab-swarm enables Claude Code instances to coordinate as a swarm using stigmergy (environment-based coordination), eliminating context bloat while enabling massive parallelization.

## Quick Start

### 1. Install

```bash
# Option A: npm (recommended)
npm install -g @fabiantax/fab-swarm

# Option B: From source (Rust)
git clone https://github.com/fabiantax/fab-swarm
cd fab-swarm/fab-swarm
cargo build --profile release-fast
```

### 2. Initialize in Your Project

```bash
cd your-project
fab-swarm init --local           # Local SQLite (simple)
# or
fab-swarm init --all             # Full setup with hooks and agents
```

This creates:
- `fab-swarm.config.json` - Configuration
- `.fab-swarm/` - Data directory
- `.claude/mcp.json` - MCP server config (with --mcp or --all)
- `.claude/hooks/` - Event hooks (with --hooks or --all)

### 3. Verify Setup

```bash
fab-swarm doctor                 # Run diagnostics
fab-swarm status                 # Check swarm status
```

### 4. Start Using

```bash
# Start MCP server (for Claude Code)
fab-swarm mcp

# Or run tasks directly
fab-swarm tasks add "Implement user authentication"
fab-swarm status
```

## Getting Started Guide

### Level 1: Basic Local Setup (5 min)

Perfect for single-developer projects:

```bash
cd my-project
fab-swarm init --local
fab-swarm doctor                 # Verify setup

# Add your first task
fab-swarm tasks add "Review codebase for security issues"
fab-swarm tasks list
```

### Level 2: Claude Code Integration (10 min)

Add MCP tools to Claude Code:

```bash
fab-swarm init --mcp
# Restart Claude Code to load MCP server

# Now Claude can use stigmergy tools:
# - task_add, task_claim, task_complete
# - stigmergy_read, stigmergy_write
# - trace_read, trace_write
# - kg_add_entity, kg_query
```

### Level 3: Multi-Agent Coordination (15 min)

Enable hooks for event-driven coordination:

```bash
fab-swarm init --all             # MCP + hooks + agents

# Create a lead agent that decomposes tasks
fab-swarm tasks add --type orchestrate "Build REST API with tests"

# Subagents claim and complete subtasks automatically
fab-swarm watch                  # Monitor in real-time
```

### Level 4: Distributed Setup (20 min)

For multi-machine swarms:

```bash
# Get Turso credentials
# https://turso.tech (free tier available)
export TURSO_URL="libsql://your-db.turso.io"
export TURSO_TOKEN="your-token"

fab-swarm init                   # Auto-detects Turso
fab-swarm doctor                 # Verify connection
```

## Troubleshooting

### Common Issues

**"fab-swarm binary not found"**
```bash
# Build from source
cd fab-swarm/fab-swarm
cargo build --profile release-fast

# Or check PATH includes the binary location
fab-swarm doctor
```

**"Not initialized"**
```bash
fab-swarm init --local           # Create config
fab-swarm doctor                 # Verify
```

**"Turso connection failed"**
```bash
# Check credentials
echo $TURSO_URL
echo $TURSO_TOKEN

# Fall back to local
fab-swarm init --local
```

**"MCP server not configured"**
```bash
fab-swarm init --mcp             # Add MCP config
# Restart Claude Code
```

### Diagnostic Commands

```bash
fab-swarm doctor                 # Full diagnostics
fab-swarm doctor --verbose       # Detailed output
fab-swarm status                 # Quick status check
fab-swarm stats                  # SAFLA statistics
```

## Features

| Feature | Description |
|---------|-------------|
| **🚀 Self-Healing Swarm** | Automatic task recovery from agent failures with zero downtime |
| **Stigmergy Coordination** | Agents coordinate through environment traces, not direct messages |
| **Turso/SQLite Storage** | Default persistent storage via libSQL (local or distributed) |
| **Nano-Agents** | Deterministic tasks execute in ~50-500μs without LLM |
| **Smart Tier Routing** | Auto-selects nano/haiku/sonnet/opus based on task complexity |
| **Learning System** | Tracks outcomes, learns optimal tier routing over time |
| **DAG Execution** | Dependency-aware parallel execution with critical path analysis |
| **Knowledge Graph** | Track entities and relationships across the codebase |
| **MCP Server** | 40+ tools for Claude Code integration |

### 🛡️ Task Resilience Features

- **Automatic Recovery**: Detects and recovers from stuck/crashed agents
- **Heartbeat Monitoring**: Real-time agent health tracking
- **Periodic Cleanup**: Self-healing background processes
- **Concurrent Safety**: Race-condition-free task claiming
- **Production Ready**: Battle-tested with comprehensive stress tests

See [TASK_RESILIENCE.md](docs/TASK_RESILIENCE.md) for detailed resilience documentation.

## Architecture

```
Task → Nano-router → [Nano-agents] → Result (fast path, ~μs)
             ↓
       Coordinator → Workers (parallel) → Reviewer → Result

Stigmergy Coordination:
Lead → stig_add_tasks → [Subagents claim/complete] → stig_observe → Lead
```

## CLI Reference

### Commands

| Command | Description |
|---------|-------------|
| `fab-swarm init` | Setup fab-swarm in current directory |
| `fab-swarm status` | Show swarm status |
| `fab-swarm doctor` | Diagnose and troubleshoot setup |
| `fab-swarm mcp` | Start MCP server for Claude Code |
| `fab-swarm tasks` | Manage swarm tasks |
| `fab-swarm observe` | Record SAFLA observations |
| `fab-swarm suggest` | Get action suggestions |
| `fab-swarm stats` | Show SAFLA statistics |
| `fab-swarm watch` | Watch for file changes |
| `fab-swarm bench` | Run performance benchmark |
| `fab-swarm debt` | Scan for technical debt |

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-w, --workers <N>` | Maximum workers per task | 3 |
| `-s, --sequential` | Run workers sequentially | false |
| `-e, --evaluate` | Evaluate execution against plan | false |
| `--strict` | Strict evaluation mode | false |
| `--min-score <0-1>` | Minimum evaluation score | 0.7 |
| `-o, --output <DIR>` | Output directory | `.fab-swarm` |
| `--dry-run` | Show plan without executing | false |
| `-v, --verbose` | Verbose output | false |

## MCP Tools Reference

### Task Management
| Tool | Description |
|------|-------------|
| `task_add` | Add tasks to queue |
| `task_list` | List all tasks |
| `task_claim` | Claim a task for execution |
| `task_complete` | Mark task complete with result |
| `task_heartbeat` | Extend task claim lease (prevents expiry) |
| `task_release_expired` | Release expired claims from crashed agents |
| `task_recover` | Automatically recover stuck tasks |

### Resilience & Health
| Tool | Description |
|------|-------------|
| `health_heartbeat` | Record agent heartbeat for health monitoring |
| `health_check` | Check health status of specific agent |
| `health_status` | Get health dashboard for all agents |
| `health_stale_agents` | List stale agents (no heartbeat) |

### Stigmergy (Shared Memory)
| Tool | Description |
|------|-------------|
| `stigmergy_read` | Read from shared memory |
| `stigmergy_write` | Write to shared memory |
| `trace_read` | Read agent traces |
| `trace_write` | Write agent trace |

### Knowledge Graph
| Tool | Description |
|------|-------------|
| `kg_add_entity` | Add entity (file, function, etc.) |
| `kg_add_relation` | Link entities |
| `kg_query` | Query relationships |

### Quality Gates
| Tool | Description |
|------|-------------|
| `review_submit` | Submit work for review |
| `review_approve` | Approve reviewed work |
| `review_reject` | Reject with feedback |
| `review_pending` | List pending reviews |

### Decisions
| Tool | Description |
|------|-------------|
| `decision_request` | Request human/agent decision |
| `decision_pending` | List pending decisions |
| `decision_respond` | Respond to decision request |

### Search & RAG
| Tool | Description |
|------|-------------|
| `search_vector` | Semantic vector search |
| `search_bm25` | Keyword search |
| `search_hybrid` | Combined vector + BM25 |
| `rag_lightrag_query` | Entity-centric RAG |
| `rag_noderag_query` | Content cluster RAG |

## Model Tiers

| Tier | Use Case | Cost | Speed |
|------|----------|------|-------|
| **Nano** | grep, find, count, read file | $0 | ~50-500μs |
| **Haiku** | Simple formatting, linting | $ | Fast |
| **Sonnet** | Code generation, tests | $$ | Balanced |
| **Opus** | Architecture, security review | $$$ | Thorough |

## Storage Configuration

### Local SQLite (default, zero config)
```bash
fab-swarm mcp  # Uses .fab-swarm/swarm.db
```

### Remote Turso (distributed)
```bash
export TURSO_URL="libsql://your-db.turso.io"
export TURSO_TOKEN="your-token"
fab-swarm mcp
```

## Build Profiles

```bash
# Development (fast compile)
cargo build

# Fast release (recommended)
cargo build --profile release-fast

# Maximum optimization (slow compile)
cargo build --release
```

### Build Optimizations

The project is configured with:
- **mold linker** - 2-3x faster linking
- **Sparse registry** - faster crate downloads
- **argh** - lightweight CLI parser (faster than clap)

## Performance

| Operation | Time |
|-----------|------|
| Nano task (grep, find) | 50-500μs |
| Task claim | <1ms |
| DAG wave calculation | <5ms |
| Context summary lookup | <1ms |

## Optional Features

```bash
# Flash Attention innovations (12 TRIZ-inspired attention mechanisms)
cargo build --features flash-attention

# Semantic search with LanceDB
cargo build --features semantic

# OpenTelemetry tracing
cargo build --features telemetry

# GLiNER zero-shot NER
cargo build --features gliner

# WASM support
cargo build --features wasm
```

### Flash Attention Innovations

Enable 12 TRIZ-inspired attention mechanism optimizations:

| Innovation | Performance | Memory | Use Case |
|------------|-------------|--------|----------|
| Tiling & Recomputation | 2-4x faster | -40% | Large sequence processing |
| Sparse Attention | 3-10x faster | -60% | Long documents |
| Quantized Attention | 1.5-2x faster | -50% | Memory-constrained |
| Multi-Query Attention | 1.5-2x faster | -30% | Batch processing |
| Sliding Window | 2-5x faster | -70% | Streaming data |
| Micro-LoRA Adaptation | 2-4x faster | -90% | Fine-tuning |

See [FLASH_ATTENTION_MIGRATION.md](docs/FLASH_ATTENTION_MIGRATION.md) for:
- How to enable and use
- Performance expectations
- Migration guide
- Rollback procedures

## Testing & Coverage

### Running Tests

```bash
# Run all Rust tests (workspace)
make test
# or
cargo test --workspace --all-features

# Run npm package tests
make test-npm
# or
cd fab-swarm/npm && npm test
```

### Code Coverage

The project uses [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) for accurate, cross-platform code coverage tracking.

**Install cargo-llvm-cov** (if not already installed):
```bash
make install
# or
cargo install cargo-llvm-cov
```

**Generate coverage reports**:
```bash
# Full coverage report (HTML + LCOV)
make coverage

# HTML only (open in browser)
make coverage-open

# LCOV only (for CI integration)
make coverage-lcov

# Terminal summary
make coverage-summary

# Show uncovered lines
make coverage-uncovered
```

**Coverage reports are generated in**:
- **HTML**: `target/llvm-cov/html/index.html` - Interactive browsable report
- **LCOV**: `target/llvm-cov/lcov.info` - For CI integration (Codecov, Coveralls)
- **JSON**: `target/llvm-cov/coverage.json` - Programmatic analysis (use `make coverage-json`)
- **Cobertura**: `target/llvm-cov/cobertura.xml` - For GitLab/Jenkins (use `make coverage-cobertura`)

**Per-crate coverage** (for debugging specific crates):
```bash
make coverage-fab-swarm    # fab-swarm crate only
make coverage-fab-brain    # fab-brain crate only
make coverage-fab-learn    # fab-learn crate only
```

**Additional coverage commands**:
```bash
make coverage-unit         # Unit tests only (faster)
make coverage-no-fail-fast # Continue on test failures
make watch-coverage        # Watch mode (reruns on file changes)
```

### Current Coverage Status

**Overall**: ~44% (10,958/25,077 lines covered)

**Well-tested modules** (>80% coverage):
- `stigmergy.rs` - Task coordination and traces
- `messages.rs` - Inter-agent messaging
- `cost.rs` - Cost tracking and budgets
- MCP handlers - All handler modules have tests

**Modules needing coverage**:
- `turso.rs` - Database layer (only integration tests)
- `orchestrator.rs` - Swarm coordination logic
- `benchmark.rs` - Performance measurement
- `telemetry.rs` - OpenTelemetry integration

See [COVERAGE.md](COVERAGE.md) for detailed analysis.

### Why cargo-llvm-cov?

**Advantages over cargo-tarpaulin**:
- ✅ **More accurate** - Uses LLVM's native instrumentation
- ✅ **Faster** - No ptrace overhead, runs at near-native speed
- ✅ **Cross-platform** - Works on Linux, macOS, Windows
- ✅ **Better integration** - Maintained by Rust community

### Clean Up

```bash
# Remove all build artifacts and coverage reports
make clean

# Remove only coverage reports
make clean-coverage
```

## Project Structure

```
fab-swarm/
├── fab-swarm/           # Rust crate
│   ├── src/
│   │   ├── main.rs      # CLI entry point
│   │   ├── mcp.rs       # MCP server
│   │   ├── stigmergy.rs # Coordination
│   │   ├── nano.rs      # Nano-agents
│   │   └── turso.rs     # Storage backend
│   └── npm/             # Node.js package
│       ├── bin/cli.js   # npm CLI
│       └── lib/         # JS library
├── .claude/
│   └── settings.json    # MCP config
└── CLAUDE.md            # AI instructions
```

## Contributing

Contributions welcome! See [CLAUDE.md](CLAUDE.md) for development guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.
