# ![fab-pem](https://img.shields.io/badge/fab--pem-Privacy--First%20ME%2FCFS%20Assistant-blue?style=for-the-badge)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Deno](https://img.shields.io/badge/Deno-2.x-blue?logo=deno)](https://deno.com)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org)
[![WASM](https://img.shields.io/badge/WebAssembly-WASM-purple?logo=webassembly)](https://webassembly.org)

# 🩺 PEM Predictor - Privacy-First ME/CFS Assistant

**AI-powered Post-Exertional Malaise prediction and pattern discovery for ME/CFS patients**

Analyze your personal health data to discover crash patterns, predict PEM risk, and take control of your energy management. All processing happens locally in your browser - your health data never leaves your device.

## 🌟 Features

### 🚀 Phase 1: Personal Webapp (Available Now)

**Analyze your personal health data to discover crash patterns and warning signs.**

- 📊 **CSV Data Import** - Parse Ultrahuman exports and wearable data
- 🔍 **Crash Period Detection** - Auto-identify crashes from missing data or user reports
- 📈 **Baseline Metrics** - Calculate your personal HRV/RHR percentiles and sleep averages
- 🎯 **Pattern Discovery** - Find your specific crash precursors:
  - HRV drops (e.g., "HRV <31ms precedes 92% of my crashes")
  - RHR elevation (e.g., "RHR >66 bpm is a warning sign")
  - Activity mismatch (e.g., "High activity after low recovery is risky")
  - Recovery patterns (e.g., "I need 5-7 days to recover")
- 💡 **Personalized Recommendations** - Evidence-based thresholds and actions:
  - Your safe activity limit (steps/day and weekly)
  - Your HRV target (based on research + your data)
  - Your recovery window (how long you need between exertion)
  - Specific actions to prevent crashes

### 🔌 Phase 2: Plugin Foundation (Week 3-4)

**Prepare architecture for seamless transition to Ultrahuman plugin.**

- 🔄 **DataSource Abstraction** - Same analysis works with CSV, APIs, or future data sources
- 🔌 **Plugin-Ready Architecture** - UI swaps from full dashboard to embedded gauge
- 💾 **Storage Abstraction** - Switches from DuckDB-WASM to Ultrahuman storage
- 📋 **Plugin Manifest** - Ready for submission when PowerPlugs API available

### 📅 Phase 3: Ultrahuman Plugin (Q2 2025)

**When Ultrahuman PowerPlugs API available, deploy as plugin with zero code changes.**

### 🔬 Advanced ML Features (Legacy, Lower Priority)

Phase 1 focuses on validating patterns with YOUR data. These work but Phase 1 takes priority:

- 🤖 **Real-time PEM Risk Prediction** - Burn LSTM model (Rust/WASM) analyzes biometrics
- ⚡ **Tired but Wired Detection** - Autonomic dysregulation alerts
- 📊 **Activity Impact Forecasting** - Predict activity impact on risk
- 🔮 **Multi-Horizon Forecasting** - TFT predicts 4h-72h windows
- 🗄️ **DuckDB-WASM Storage** - All data stays in browser via OPFS

## 📋 Table of Contents

- [Features](#-features)
- [Tech Stack](#-tech-stack)
- [Architecture](#-architecture)
- [Prerequisites](#-prerequisites)
- [Getting Started](#-getting-started)
- [Usage](#-usage)
- [API Endpoints](#api-endpoints)
- [How It Works](#-how-it-works)
- [Data Privacy](#-data-privacy)
- [Development](#-development)
- [Contributing](#-contributing)
- [Changelog](#-changelog)
- [License](#-license)
- [Authors](#-authors)

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Runtime** | ![Deno](https://img.shields.io/badge/Deno-2.x-blue?logo=deno) | Secure TypeScript runtime |
| **Server** | ![Hono](https://img.shields.io/badge/Hono-Fast-orange) | Ultra-fast web framework |
| **ML Engine** | ![Burn](https://img.shields.io/badge/Burn-Rust-orange?logo=rust) → ![WASM](https://img.shields.io/badge/WebAssembly-WASM-purple?logo=webassembly) | LSTM + TFT models in browser |
| **Database** | ![DuckDB-WASM](https://img.shields.io/badge/DuckDB-WASM-blue) | OPFS persistent storage |
| **Styling** | ![Tailwind CSS](https://img.shields.io/badge/Tailwind-3.x-38B2AC) | Utility-first CSS |

## 🏗️ Architecture

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

## 📦 Prerequisites

- **![Deno](https://img.shields.io/badge/Deno-2.x-blue?logo=deno)** - JavaScript runtime
  ```bash
  curl -fsSL https://deno.land/install.sh | sh
  ```

- **![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)** - For building WASM
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **wasm-pack** - WASM build tool
  ```bash
  cargo install wasm-pack
  ```

## 🚀 Getting Started

### Phase 1: Analyze Your Personal Data

**1. Clone the repository:**
```bash
git clone https://github.com/fabiantax/fab-pem.git
cd fab-pem
```

**2. Install dependencies and build WASM models:**
```bash
npm install              # Install dependencies
npm run build:wasm      # Compile Rust → WASM (first time only, ~2 min)
```

**3. Start the development server:**
```bash
deno task dev           # Start local server at http://localhost:3000
```

**4. Upload your health data:**
- Export your data from Ultrahuman or any wearable (CSV format)
- Use the CSV import feature in the web app
- Discover YOUR personal crash patterns

### CLI Quick Start (Alternative)

Prefer command-line? The PEM CLI has you covered:

```bash
# Build CLI tool
cargo build --release --bin pem-cli --features cli

# Forecast crash probability for tomorrow
cargo run --release --bin pem-cli --features cli -- \
  forecast-crash --file data/your-data.csv --detailed

# Learn your personal thresholds
cargo run --release --bin pem-cli --features cli -- \
  learn-thresholds --file data/your-data.csv

# Analyze a historical date (validation)
cargo run --release --bin pem-cli --features cli -- \
  forecast-crash --file data/your-data.csv --date 2025-12-27
```

See **[CLI_REFERENCE.md](docs/CLI_REFERENCE.md)** for all 17 CLI commands.

## 💻 Usage

### Web Application

```bash
# Development mode with hot reload
deno task dev

# Production mode
deno task start

# Access the app
open http://localhost:3000
```

### CLI Tool

```bash
# Show all commands
pem-cli --help

# Get detailed help for a specific command
pem-cli forecast-crash --help

# Analyze with custom date range
pem-cli forecast-crash --file data.csv --start 2025-01-01 --end 2025-01-31
```

## 🔌 API Endpoints

| Endpoint | Method | Description | Query Params |
|----------|--------|-------------|--------------|
| `/api/health` | GET | Fetch health metrics | `days=30` |
| `/api/health/today` | GET | Fetch today's metrics | - |
| `/api/activity/presets` | GET | Get activity presets | - |
| `/api/activity/analyze` | POST | Analyze activity impact | Body: `{ activity, duration }` |
| `/health` | GET | Server health check | - |

**Example:**
```bash
# Get last 30 days of health data
curl http://localhost:3000/api/health?days=30

# Analyze activity impact
curl -X POST http://localhost:3000/api/activity/analyze \
  -H "Content-Type: application/json" \
  -d '{"activity": "walking", "duration": 45}'
```

## 🧠 How It Works

### PEM Prediction Model

The **Burn LSTM** model processes 7 days of health data with 8 features per day:

| Feature | Description | Example Threshold |
|---------|-------------|-------------------|
| `avg_heart_rate` | Average resting heart rate | RHR >66 bpm = warning |
| `avg_hrv` | HRV (RMSSD) average | HRV <31ms = crash risk |
| `min_hrv` / `max_hrv` | HRV range | High variability = good |
| `avg_temperature` | Skin temperature | Elevated = autonomic stress |
| `total_steps` | Daily activity | Steps > baseline = risk |
| `sleep_score` | Sleep quality (0-100) | <60 = poor recovery |
| `recovery_index` | Recovery readiness (0-100) | <40 = not recovered |

**Model Architecture:**
```
Input [batch, 7, 8] → LSTM(64) → LSTM(64) → Dense(32) → Dense(16) → Sigmoid(1)
                              ↓
                         Risk Score (0-100%)
```

**Risk Classification:**
- 🟢 **Low**: <25% - Safe to proceed with planned activities
- 🟡 **Medium**: 25-50% - Consider reducing intensity
- 🟠 **High**: 50-75% - Rest recommended
- 🔴 **Critical**: >75% - Avoid exertion

### Activity Impact Forecasting

Plan activities safely with risk projections:

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
│  💡 Suggestions:                                            │
│  • Wait 4 more hours: 31%                                   │
│  • Reduce to 22 min: 32%                                    │
│  • Take rest breaks: 35%                                    │
└─────────────────────────────────────────────────────────────┘
```

### Tired but Wired Detection

Flags autonomic dysregulation when 2+ of these are true:
1. Evening HR is >10% above 30-day baseline
2. Skin temperature fails to drop (elevated vs baseline)
3. HRV is suppressed >15% below 7-day average

### ACWR (Acute:Chronic Workload Ratio)

- **Acute load**: 7-day average activity
- **Chronic load**: 28-day average activity
- **Risk zones**:
  - 🟢 Optimal: 0.8-1.3
  - 🟡 Caution: <0.8 (deconditioned) or >1.3 (overreaching)
  - 🔴 Danger: >1.5 (high injury/PEM risk)

## 🔒 Data Privacy

**Your health data never leaves your browser.**

- ✅ Health metrics stored in **DuckDB-WASM** (browser storage via OPFS)
- ✅ ML inference runs entirely in **WASM** (client-side)
- ✅ Server only used for **Ultrahuman API proxy** (token never exposed to client)
- ✅ No analytics, no tracking, no cloud processing

**Security:**
- API tokens stored server-side only
- All computation happens in your browser
- CSV data processed locally
- No external service dependencies for ML

## 💻 Development

### Project Structure

```
├── server/
│   ├── main.ts            # Hono server entry point
│   ├── routes/
│   │   ├── health.ts      # Health data API endpoints
│   │   └── activity.ts    # Activity impact API endpoints
│   └── views/
│       └── dashboard.tsx  # Server-rendered dashboard
├── crates/
│   └── pem-engine/        # Rust/Burn ML models
│       ├── src/
│       │   ├── lib.rs     # WASM bindings
│       │   ├── model.rs   # LSTM model definition
│       │   ├── activity.rs # Activity simulator
│       │   └── tft/       # Temporal Fusion Transformer
│       └── Cargo.toml
├── static/
│   └── wasm/              # Compiled WASM output
├── docs/                  # Documentation
├── deno.json             # Deno config + tasks
└── Cargo.toml            # Workspace config
```

### Build Commands

```bash
# Development server with hot reload
deno task dev

# Build WASM module
deno task build:wasm

# Run production server
deno task start

# Run tests
deno task test

# Lint code
deno lint
```

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `ULTRAHUMAN_TOKEN` | API token from Ultrahuman app | Phase 3 only | - |
| `PORT` | Server port | No | `3000` |

**Setup:**
```bash
cp .env.example .env
# Edit .env with your credentials
```

## 🤝 Contributing

Contributions welcome! We focus on:

1. **Privacy-first** - All ML must run client-side
2. **Evidence-based** - Features backed by ME/CFS research
3. **User-tested** - Validated with real patient data

**How to contribute:**

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

**Development guidelines:**
- See **[claude.md](claude.md)** for architecture decisions
- See **[FEATURES.md](docs/FEATURES.md)** for feature roadmap
- See **[Architectural Decision Records](docs/adr/)** for design rationale

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
  - ADR-0005: ML Algorithms and Signals

## 📅 Changelog

### [Unreleased](https://github.com/fabiantax/fab-pem/compare/v0.1.0...HEAD)

### [v0.1.0] - 2025-01-29

**Added:**
- 🎉 Initial release
- 📊 CSV data import from Ultrahuman and wearables
- 🔍 Crash pattern detection
- 🤖 LSTM PEM risk prediction model
- 💡 Personalized threshold learning
- 📡 Activity impact forecasting
- 🔌 Plugin-ready architecture foundation

**Documentation:**
- Complete feature specification
- CLI reference (17 commands)
- Architecture decision records

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Authors

**Fabian Tax** - [@fabiantax](https://github.com/fabiantax)

## 🙏 Acknowledgments

- **Ultrahuman** - For the amazing health data platform
- **Burn Framework** - For the excellent Rust ML framework
- **ME/CFS Community** - For feedback, testing, and courage

---

<div align="center">

**⭐ Star this repo if it helps you manage your energy!**

**Made with ❤️ for the ME/CFS community**

</div>
