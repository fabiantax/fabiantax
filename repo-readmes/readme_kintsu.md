# Kintsu (金継)

> Golden repair for scattered developer workflows

Named after **Kintsugi** (金継ぎ), the Japanese art of repairing broken pottery with gold, Kintsu helps developers repair scattered workflows and achieve focused productivity.

## What It Does

Kintsu helps developers who struggle with:

- **Analysis Paralysis** - Time-boxing and ratio tracking
- **Goal Drift** - Active drift detection and redirection
- **Completion Avoidance** - Commit discipline and triage
- **Decision Making** - Force-choice mechanisms
- **Perfectionism** - Good-enough detection
- **Context Switching** - Thread management

## Installation

```bash
# npm (recommended)
npm install -g kintsu

# From source
cargo install kintsu
```

## Quick Start

```bash
# Initialize session with a goal
kintsu init --goal "Implement authentication"

# Triage uncommitted files
kintsu triage --analyze

# Check for goal drift
kintsu check-drift

# Check if ready to ship
kintsu ship-readiness
```

## Configuration

Kintsu uses profiles to adjust intervention intensity:

| Profile | Research/Impl | Style |
|---------|---------------|-------|
| `gentle` | 50/50 | Soft reminders |
| `balanced` | 40/60 | Moderate nudges |
| `assertive` | 30/70 | Strong guidance |
| `forceful` | 20/80 | Hard deadlines |

```bash
# Use a profile
kintsu init --goal "Ship feature" --profile assertive

# Configure manually
kintsu config set time_ratios.research_target_percent 35
```

## Architecture

```
kintsu/
├── crates/
│   ├── kintsu-core/     # Rust core library
│   └── kintsu-wasm/     # WASM bindings
└── packages/
    └── kintsu/          # TypeScript CLI + npm package
```

## Development

```bash
# Build Rust
cargo build --release

# Build WASM
cd crates/kintsu-wasm && wasm-pack build --target nodejs

# Build TypeScript
cd packages/kintsu && npm install && npm run build

# Run CLI
npx kintsu --help
```

## License

MIT OR Apache-2.0
