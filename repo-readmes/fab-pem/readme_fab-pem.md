# PEM Predictor - Privacy-First ME/CFS Assistant

AI-powered Post-Exertional Malaise prediction and pattern discovery for ME/CFS patients, with a clear path to becoming an Ultrahuman plugin.

## 🚀 Phase 1: Personal Webapp (NOW)

Analyze your personal health data to discover crash patterns and warning signs.

### Phase 1 Features

- **CSV Data Import** - Parse Ultrahuman exports and other wearable data
- **Crash Period Detection** - Auto-identify crashes from missing data or user reports
- **Baseline Metrics** - Calculate your personal HRV/RHR percentiles, sleep averages
- **Pattern Discovery** - Find your specific crash precursors:
  - HRV drops (e.g., "HRV <31ms precedes 92% of my crashes")
  - RHR elevation (e.g., "RHR >66 bpm is a warning sign")
  - Activity mismatch (e.g., "High activity after low recovery is risky")
  - Recovery patterns (e.g., "I need 5-7 days to recover")
- **Personalized Recommendations** - Evidence-based thresholds and actions:
  - Your safe activity limit (steps/day and weekly)
  - Your HRV target (based on research + your data)
  - Your recovery window (how long you need between exertion)
  - Specific actions to prevent crashes

## 🔌 Phase 2: Plugin Foundation (Week 3-4)

Prepare architecture for seamless transition to Ultrahuman plugin.

- **DataSource Abstraction** - Same analysis works with CSV, APIs, or future data sources
- **Plugin-Ready Architecture** - UI swaps from full dashboard to embedded gauge
- **Storage Abstraction** - Switches from DuckDB-WASM to Ultrahuman storage
- **Plugin Manifest** - Ready for submission when PowerPlugs API available

## 📅 Phase 3: Ultrahuman Plugin (Q2 2025)

When Ultrahuman PowerPlugs API available, deploy as plugin with zero code changes.

## Legacy Features (Existing, Lower Priority)

Phase 1 focuses on validating patterns with YOUR data. These work but Phase 1 takes priority:
- **Real-time PEM Risk Prediction** - Burn LSTM model (Rust/WASM) analyzes biometrics
- **Tired but Wired Detection** - Autonomic dysregulation alerts
- **Activity Impact Forecasting** - Predict activity impact on risk
- **Multi-Horizon Forecasting** - TFT predicts 4h-72h windows
- **DuckDB-WASM Storage** - All data stays in browser via OPFS

## Tech Stack

| Layer | Technology |
|-------|------------|
| **Runtime** | Deno 2.x |
| **Server** | Hono |
| **ML Engine** | Burn (Rust) → WASM (LSTM + TFT) |
| **Database** | DuckDB-WASM with OPFS persistence |
| **Styling** | Tailwind CSS |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Browser                                                     │
│  ┌────────────────────┐  ┌─────────────────────────────────┐│
│  │ Burn WASM          │  │ Dashboard (Server-rendered HTML)││
│  │ LSTM + TFT Models  │  │ + Vanilla JS hydration          ││
│  │ Activity Simulator │  └─────────────────────────────────┘│
│  └────────────────────┘                                      │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ DuckDB-WASM (OPFS - persistent storage)                 ││
│  └─────────────────────────────────────────────────────────┘│
└────────────────────────────────┬────────────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  Deno + Hono Server     │
                    │  /api/health            │
                    │  /api/activity          │
                    │  Ultrahuman API proxy   │
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  Ultrahuman API         │
                    │  (Ring biometrics)      │
                    └─────────────────────────┘
```

## 📚 Documentation

### Start Here
- **[FEATURES.md](docs/FEATURES.md)** - Complete feature overview by phase
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and recent changes
- **[claude.md](claude.md)** - Developer guidelines and architecture
- **[ARCHITECTURE_SYNTHESIS.md](docs/ARCHITECTURE_SYNTHESIS.md)** - 10-part architecture guide

### CLI Tool
- **[CLI_REFERENCE.md](docs/CLI_REFERENCE.md)** - Complete CLI command reference (17 commands)
- **[QUICK_REFERENCE.md](docs/QUICK_REFERENCE.md)** - One-page printable cheat sheet
- **[TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[DATA_QUALITY.md](docs/DATA_QUALITY.md)** - Getting the best predictions
- **[GLOSSARY.md](docs/GLOSSARY.md)** - Term definitions (PEM, HRV, etc.)

### For Implementation
- **[Specifications](/.specify/specs/)** - Executable blueprints for each feature
  - [001: CSV Data Import](/.specify/specs/001-csv-data-import/)
  - [002: Pattern Discovery](/.specify/specs/002-crash-pattern-discovery/)
  - [003: Data Abstraction](/.specify/specs/003-data-abstraction-integration/)
  - [004: Plugin Architecture](/.specify/specs/004-plugin-ready-architecture/)

### For Architecture Decisions
- **[Architectural Decision Records](docs/adr/)** - Why we built things this way
  - ADR-0001: Data Abstraction Layer
  - ADR-0002: Browser-Side Analysis
  - ADR-0003: TypeScript + Rust Split
  - ADR-0004: Defer CLI Tool
  - ADR-0005: ML Algorithms and Signals (models, features, why/why not)

### For Spec-Driven Development
- **[Spec Constitution](.specify/constitution.md)** - Standards for specs
- **[Spec Template](/.specify/specs/README.md)** - How to write specs

---

## Getting Started

### Prerequisites

- Deno 2.x (`curl -fsSL https://deno.land/install.sh | sh`)
- Rust 1.75+ (for building WASM)
- wasm-pack (`cargo install wasm-pack`)

### Phase 1: Analyze Your Personal Data

1. **Clone the repository:**
```bash
git clone https://github.com/fabiantax/fab-pem.git
cd fab-pem
```

2. **Build WASM models (first time only):**
```bash
npm install              # Install dependencies
npm run build:wasm      # Compile Rust → WASM
```

3. **Run development server:**
```bash
deno task dev           # Start local server
```

4. **Open in browser:**
```
http://localhost:3000
```

5. **Upload your CSV:**
- Export your data from Ultrahuman or any wearable
- Use the CSV import feature in the app
- Discover YOUR crash patterns

### CLI Quick Start (Alternative to Web App)

```bash
# Build CLI
cargo build --release --bin pem-cli --features cli

# Forecast crash probability for tomorrow
cargo run --release --bin pem-cli --features cli -- forecast-crash --file data/your-data.csv --detailed

# Learn your personal thresholds
cargo run --release --bin pem-cli --features cli -- learn-thresholds --file data/your-data.csv

# Analyze a historical date (validation)
cargo run --release --bin pem-cli --features cli -- forecast-crash --file data/your-data.csv --date 2025-12-27
```

See [CLI_REFERENCE.md](docs/CLI_REFERENCE.md) for full command documentation.

### Phase 2 & 3: Ultrahuman Integration (Later)

When Phase 2 is complete and PowerPlugs API available:
```bash
# Set your Ultrahuman token (optional, for Phase 3)
cp .env.example .env
# Edit .env with your ULTRAHUMAN_TOKEN
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Fetch health metrics (days=30 param) |
| `/api/health/today` | GET | Fetch today's metrics |
| `/api/activity/presets` | GET | Get activity presets |
| `/api/activity/analyze` | POST | Analyze activity impact |
| `/health` | GET | Server health check |

## How It Works

### PEM Prediction Model

The Burn LSTM model processes 7 days of health data with 8 features per day:

| Feature | Description |
|---------|-------------|
| `avg_heart_rate` | Average resting heart rate |
| `avg_hrv` | HRV (RMSSD) average |
| `min_hrv` / `max_hrv` | HRV range |
| `avg_temperature` | Skin temperature |
| `total_steps` | Daily activity |
| `sleep_score` | Sleep quality (0-100) |
| `recovery_index` | Recovery readiness (0-100) |

**Model Architecture:**
```
Input [batch, 7, 8] → LSTM(64) → LSTM(64) → Dense(32) → Dense(16) → Sigmoid(1)
```

Risk is classified into: **Low** (<25%), **Medium** (25-50%), **High** (50-75%), **Critical** (>75%)

### Activity Impact Forecasting

The Activity Impact module helps you plan activities safely:

```
┌─────────────────────────────────────────────────────────────┐
│  Activity Risk Calculator                                    │
├─────────────────────────────────────────────────────────────┤
│  Planned: Grocery Shopping (45 min)                         │
│                                                             │
│  Base risk:      25%                                        │
│  Activity impact: +13%                                      │
│  Projected risk:  38% (Medium)                              │
│                                                             │
│  Suggestions:                                               │
│  • Wait 4 more hours: 31%                                   │
│  • Reduce to 22 min: 32%                                    │
└─────────────────────────────────────────────────────────────┘
```

### Tired but Wired Detection

Flags autonomic dysregulation when 2+ of these are true:
1. Evening HR is >10% above 30-day baseline
2. Skin temperature fails to drop (elevated vs baseline)
3. HRV is suppressed >15% below 7-day average

### ACWR (Acute:Chronic Workload Ratio)

- **Acute load**: 7-day average
- **Chronic load**: 28-day average
- **Risk zones**: Optimal (0.8-1.3), Caution (<0.8 or >1.3), Danger (>1.5)

## Data Privacy

All data is processed locally in your browser:
- Health metrics stored in DuckDB-WASM (browser storage)
- ML inference runs entirely in WASM
- Server only used for Ultrahuman API proxy (token never exposed to client)

## Development

### Project Structure
```
├── server/
│   ├── main.ts            # Hono server entry
│   ├── routes/
│   │   ├── health.ts      # Health data API
│   │   └── activity.ts    # Activity impact API
│   └── views/
│       └── dashboard.tsx  # Server-rendered dashboard
├── crates/
│   └── pem-engine/        # Rust/Burn ML models
│       ├── src/
│       │   ├── lib.rs     # WASM bindings
│       │   ├── model.rs   # LSTM model
│       │   ├── activity.rs # Activity simulator
│       │   └── tft/       # Temporal Fusion Transformer
│       └── Cargo.toml
├── static/
│   └── wasm/              # Compiled WASM output
├── app/                   # (Legacy React components)
│   ├── ml/                # ML engine wrappers
│   └── lib/               # Shared utilities
├── docs/
│   └── TFT_IMPLEMENTATION_PLAN.md
├── deno.json              # Deno config + tasks
└── Cargo.toml             # Workspace config
```

### Build Commands

```bash
# Development server with hot reload
deno task dev

# Build WASM module
deno task build:wasm

# Run production server
deno task start
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `ULTRAHUMAN_TOKEN` | API token from Ultrahuman app |
| `PORT` | Server port (default: 3000) |

## License

MIT
