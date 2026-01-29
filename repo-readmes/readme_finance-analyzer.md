# Finance Analyzer

**Privacy-first finance tracking with AI-powered categorization**

A modern web application for analyzing bank transactions with local WASM processing, self-improving AI categorization, and intelligent pattern learning. All data stays on your device - no backend required!

## ✨ Features

- 🔒 **100% Private** - All processing happens in your browser, no data sent to servers
- 🧠 **AI-Powered Categorization** - Self-improving AI (ruvLLM) learns from your corrections
- 📊 **Smart Recurring Detection** - Automatically detects subscriptions and recurring expenses
- 🔍 **Fuzzy Merchant Matching** - Groups "Netflix", "NETFLIX.COM", "Netflix Streaming" as one
- 💾 **Persistent Storage** - IndexedDB keeps your data across sessions with automatic deduplication
- ⚡ **WASM Performance** - Rust-powered CSV parsing for blazing-fast imports
- 📈 **Budget Insights** - Visualize spending patterns and track year-over-year changes
- 🔄 **Pattern Learning** - Confirm/reject detected patterns to improve accuracy over time
- 💸 **Tikkie Tracking** - Special support for Dutch inter-family payment tracking
- 🎯 **Outlier Detection** - Spot unusual transactions automatically using statistical methods

## Tech Stack

- **Frontend**: React 18 + TypeScript + Vite + TailwindCSS
- **Core Logic**: Rust compiled to WebAssembly (WASM)
- **AI**: ruvLLM (self-improving local AI) + optional Cerebras Cloud
- **Pattern Matching**: Levenshtein distance for fuzzy merchant names
- **Storage**: IndexedDB (browser-native) + AgentDB (vector database for learning)
- **Testing**: Vitest + React Testing Library (368 tests passing)
- **Icons**: Lucide React

## 🚀 Quick Start

### Prerequisites

1. **Node.js** (v18 or higher)
   ```bash
   node --version  # Should be v18+
   ```

2. **Rust Toolchain** (for WASM compilation)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

   # Verify installation
   cargo --version
   ```

3. **wasm-pack** (for building WebAssembly)
   ```bash
   cargo install wasm-pack
   wasm-pack --version
   ```

### Installation

```bash
# Clone the repository
git clone https://github.com/fabiantax/finance-analyzer.git
cd finance-analyzer

# Install frontend dependencies
cd web
npm install

# Build WASM modules (IMPORTANT!)
npm run wasm:build

# Start development server
npm run dev
```

The app will open at `http://localhost:5173`

## 📖 Usage

1. **Upload CSV Files**
   - Drag & drop or select your ING Bank CSV files
   - Multiple files supported (automatic deduplication)

2. **Automatic Processing**
   - WASM parser extracts transactions at native speed
   - AI categorizes expenses (groceries, utilities, subscriptions, etc.)
   - Recurring patterns detected automatically with confidence scores

3. **Review & Learn**
   - Correct any miscategorized transactions
   - Confirm or reject detected recurring patterns
   - AI learns from your feedback and improves over time

4. **Analyze**
   - View interactive dashboard with spending breakdown
   - Track recurring subscriptions and detect price changes
   - Spot outliers and unusual spending patterns
   - Compare year-over-year trends
   - Monitor Tikkie partner balances

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│         React Frontend (TypeScript)     │
│  ┌────────────────────────────────────┐ │
│  │  Components (12 main components)   │ │
│  │  - Dashboard, TransactionTable     │ │
│  │  - RecurringPatterns, Outliers     │ │
│  │  - PartnerBalance, Export, etc.    │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │  Business Logic (TypeScript)       │ │
│  │  - recurring.ts (pattern detection)│ │
│  │  - fuzzy-matcher.ts (Levenshtein)  │ │
│  │  - pattern-learner.ts (AI feedback)│ │
│  │  - ai-router.ts (categorization)   │ │
│  │  - outlier-detection.ts (Z-score)  │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │  Core (WASM - Rust)                │ │
│  │  - CSV parsing (fast, 442 lines)   │ │
│  │  - Pattern detection (native)      │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │  Storage (Browser)                 │ │
│  │  - IndexedDB (transactions)        │ │
│  │  - AgentDB (learning patterns)     │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

See [RUST_VS_TYPESCRIPT_ARCHITECTURE.md](docs/RUST_VS_TYPESCRIPT_ARCHITECTURE.md) for detailed architecture.

## 🧪 Development

```bash
# Run tests (368 tests)
npm test

# Run tests in watch mode
npm run test:watch

# Build for production
npm run build

# Preview production build
npm run preview

# Build WASM modules
npm run wasm:build
```

### Project Structure

```
finance-analyzer/
├── web/                      # React frontend
│   ├── src/
│   │   ├── components/       # React components (12+)
│   │   ├── lib/              # Business logic (recurring, fuzzy, AI)
│   │   ├── hooks/            # React hooks (useWasm, useDateRange)
│   │   ├── types/            # TypeScript types
│   │   ├── wasm/             # Generated WASM files (gitignored)
│   │   └── App.tsx           # Main app (955 lines)
│   ├── package.json
│   └── vite.config.ts
├── crates/                   # Rust WASM modules
│   ├── csv-parser/           # CSV parsing (442 lines)
│   └── finance-core/         # Core financial logic
├── docs/                     # Documentation
│   ├── INSTALLATION.md       # Claude Code setup
│   ├── RUST_VS_TYPESCRIPT_ARCHITECTURE.md
│   ├── EXISTING_CODEBASE_SUMMARY.md
│   ├── PATTERN_LEARNING.md
│   ├── BLINDSPOTS_ANALYSIS.md  # Known issues
│   └── QUICK_REFERENCE.md
└── README.md                 # This file
```

## 📚 Documentation

- **[Installation Guide](docs/INSTALLATION.md)** - Claude Code & hooks setup
- **[Architecture Overview](docs/RUST_VS_TYPESCRIPT_ARCHITECTURE.md)** - Rust vs TypeScript relationship
- **[Pattern Learning System](docs/PATTERN_LEARNING.md)** - AI learning details
- **[Codebase Summary](docs/EXISTING_CODEBASE_SUMMARY.md)** - Feature overview
- **[Blindspots Analysis](docs/BLINDSPOTS_ANALYSIS.md)** - Known issues & roadmap
- **[Quick Reference](docs/QUICK_REFERENCE.md)** - Development cheat sheet

## 🔒 Privacy & Security

- ✅ **No backend** - Everything runs in your browser
- ✅ **No analytics** - We don't track you
- ✅ **No network** - Data never leaves your device (except optional Cerebras API)
- ✅ **Open source** - Audit the code yourself
- ✅ **IndexedDB** - Browser-native encrypted storage
- ⚠️ **Optional Cloud AI** - Cerebras API key stored in localStorage (see [Security](docs/BLINDSPOTS_ANALYSIS.md#21-no-content-security-policy-csp))

Your financial data never leaves your device unless you explicitly enable cloud AI.

## 🧪 Testing

Current test coverage:

- ✅ **368 tests passing** (18 skipped)
- ✅ Fuzzy matcher: 93.38% coverage (61 tests)
- ✅ Pattern learner: ~85% coverage (36 tests)
- ✅ Recurring detection: ~80% coverage (45 tests)
- ✅ Outlier detection: 37 tests
- ✅ React components: 12 tests
- ⚠️ WASM tests: 1 suite failing (requires Rust build)
- ⚠️ E2E tests: Not yet implemented

Run tests:
```bash
cd web
npm test
```

## 🚧 Known Issues & Roadmap

See **[BLINDSPOTS_ANALYSIS.md](docs/BLINDSPOTS_ANALYSIS.md)** for comprehensive list of 36 identified blindspots.

### Critical (Fix Immediately)
1. ⚠️ Rust toolchain required but not auto-installed
2. ⚠️ RecurringPatternLearning component not integrated in UI
3. ✅ No root README (FIXED - you're reading it!)

### High Priority
- Enhanced recurring detection features exist but not fully integrated
- No React Error Boundary (app crashes on errors)
- IndexedDB quota management missing

### Planned Features
- Multi-currency support
- Transaction editing (description, amount, delete)
- Budget planning & forecasting
- PWA support (offline mode)
- Dark mode
- E2E tests (Playwright)
- CI/CD pipeline

## 🤝 Contributing

Contributions welcome!

**Development Workflow**:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`npm test`)
5. Build WASM if you changed Rust code (`npm run wasm:build`)
6. Commit with conventional commits (`git commit -m 'feat: add amazing feature'`)
7. Push to branch
8. Open a Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [WebAssembly](https://webassembly.org/)
- Powered by [React](https://react.dev/) and [Vite](https://vitejs.dev/)
- Uses [AgentDB](https://github.com/ruvnet/agentdb) for pattern learning
- Icons by [Lucide](https://lucide.dev/)

---

**Made with ❤️ for privacy-conscious financial tracking**
