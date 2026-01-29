in into# ![fab-pem](https://img.shields.io/badge/fab--pem-Privacy--First%20ME%2FCFS%20Assistant-blue?style=for-the-badge)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Deno](https://img.shields.io/badge/Deno-2.x-blue?logo=deno)](https://deno.com)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org)
[![WASM](https://img.shields.io/badge/WebAssembly-WASM-purple?logo=webassembly)](https://webassembly.org)

# 🩺 PEM Predictor - Privacy-First ME/CFS Assistant

**AI-powered Post-Exertional Malaise prediction and pattern discovery for ME/CFS patients**

Analyze your personal health data to discover crash patterns, predict PEM risk, and take control of your energy management. All processing happens locally in your browser - your health data never leaves your device.

## ✨ Features

- 📊 **CSV Data Import** - Parse Ultrahuman exports and wearable data
- 🔍 **Crash Pattern Detection** - Auto-identify your personal crash precursors
- 🤖 **ML-Powered Prediction** - LSTM + TFT models running entirely in your browser (WASM)
- 📈 **Personalized Thresholds** - Learn YOUR specific HRV/RHR warning signs
- 💡 **Activity Impact Forecasting** - Plan activities safely with risk projections
- 🔒 **100% Private** - All data processed locally, no cloud processing

## 🚀 Quick Start

```bash
# Clone and install
git clone https://github.com/fabiantax/fab-pem.git
cd fab-pem
npm install && npm run build:wasm

# Start web app
deno task dev
open http://localhost:3000
```

**Or use the CLI:**
```bash
# Build CLI
cargo build --release --bin pem-cli --features cli

# Forecast tomorrow's crash risk
./pem-cli forecast-crash --file data/your-data.csv
```

## 🛠️ Tech Stack

| Component | Technology |
|-----------|-----------|
| Runtime | ![Deno](https://img.shields.io/badge/Deno-2.x-blue?logo=deno) |
| ML Framework | ![Burn](https://img.shields.io/badge/Burn-Rust-orange?logo=rust) → WASM |
| Database | ![DuckDB-WASM](https://img.shields.io/badge/DuckDB-WASM-blue) |
| Framework | ![Hono](https://img.shields.io/badge/Hono-Ultra--Fast-orange) |

## 📚 Documentation

Full documentation is available in the **[GitHub Wiki](https://github.com/fabiantax/fab-pem/wiki)**.

### Quick Links

| Topic | Link |
|-------|------|
| 📘 **User Guide** | [Getting Started](https://github.com/fabiantax/fab-pem/wiki/Getting-Started) |
| 🤖 **Machine Learning** | [Model Architecture](https://github.com/fabiantax/fab-pem/wiki/Model-Architecture) |
| ⚡ **Performance** | [Optimization Guide](https://github.com/fabiantax/fab-pem/wiki/Performance-Optimization) |
| 🔌 **API Reference** | [API Documentation](https://github.com/fabiantax/fab-pem/wiki/API-Reference) |
| 🛠️ **Development** | [Contributing](https://github.com/fabiantax/fab-pem/wiki/Contributing) |
| 📅 **Changelog** | [Version History](https://github.com/fabiantax/fab-pem/wiki/Changelog) |

## 🎯 Roadmap

- [x] **Phase 1**: Personal webapp with CSV import
- [x] **Phase 2**: Plugin-ready architecture
- [ ] **Phase 3**: Ultrahuman PowerPlugs integration (Q2 2025)

## 📊 Project Stats

- **ML Models**: 2 (LSTM, TFT)
- **WASM Bundle Size**: ~2MB (compressed)
- **Inference Speed**: <100ms per prediction
- **Supported Data Sources**: Ultrahuman, CSV exports
- **Privacy**: 100% local processing

## 🔒 Privacy & Security

✅ **Your data never leaves your browser**
- Health metrics stored in DuckDB-WASM (browser storage)
- ML inference runs entirely in WASM
- No analytics, tracking, or cloud processing

## 🤝 Contributing

We welcome contributions! See **[Contributing Guide](https://github.com/fabiantax/fab-pem/wiki/Contributing)** for details.

**Focus areas:**
- Privacy-first ML (all client-side)
- Evidence-based features (backed by ME/CFS research)
- User-tested with real patient data

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

## 👨‍💻 Author

**Fabian Tax** - [@fabiantax](https://github.com/fabiantax)

---

<div align="center">

**⭐ Star us on GitHub** - it helps more ME/CFS patients find this tool!

**Made with ❤️ for the ME/CFS community**

**[Documentation](https://github.com/fabiantax/fab-pem/wiki)** • **[Report Issues](https://github.com/fabiantax/fab-pem/issues)** • **[Feature Requests](https://github.com/fabiantax/fab-pem/issues)**

</div>
