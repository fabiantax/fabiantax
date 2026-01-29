# WSL2 Manager - Tauri Edition

A modern, lightweight desktop application for managing Windows Subsystem for Linux 2 (WSL2) distributions on Windows. Built with Tauri, Rust, and React for maximum performance and user experience.

## Overview

WSL2 Manager provides a comprehensive graphical interface to manage your WSL2 distributions without needing command-line knowledge. Whether you're a developer managing multiple Linux environments or a system administrator overseeing WSL deployments, WSL2 Manager simplifies common tasks and provides advanced features like VHDX optimization and crash recovery.

## Features

### Core Features (Phase 1 - Complete)

- **Distribution Management**
  - View all installed WSL distributions with detailed status
  - Start/Stop distributions with a single click
  - Set default distribution
  - Unregister (delete) distributions
  - View WSL version and state information

- **VHDX Optimization**
  - Automatic VHDX discovery and analysis
  - Preflight safety checks before optimization
  - Multi-method optimization (Hyper-V, Diskpart, Auto)
  - Real-time progress tracking (5-phase process)
  - Automatic backup creation option
  - Crash recovery with rollback capability
  - Space savings verification

- **Terminal Integration**
  - Auto-detection of running terminals
  - Windows Terminal support (with ConHost fallback)
  - One-click terminal launch for any distribution
  - Terminal process tracking

- **Backup and Recovery**
  - Create full distribution backups
  - Scheduled backup support
  - One-click restore functionality
  - Crash recovery after failed operations
  - Point-in-time recovery options

- **Space Analysis**
  - Detailed disk usage scanning per distribution
  - Directory-level breakdown
  - Smart cleanup recommendations
  - Safe cleanup execution with verification

- **Container Management**
  - Docker/Podman container detection
  - Runtime status monitoring
  - Container process analysis
  - Resource usage tracking

- **Virtual Desktop Management**
  - Save and restore virtual desktop views
  - Multi-view workspace management
  - Quick view switching
  - View persistence across sessions

- **Advanced Features**
  - Comprehensive audit logging (all operations)
  - Caching system for performance
  - Multi-level error recovery
  - Administrator privilege management
  - System tray integration
  - Real-time event system
  - Database-backed configuration

## Tech Stack

### Frontend
- **React 18** - Modern UI library with hooks
- **TypeScript** - Full type safety
- **Vite** - Lightning-fast build tool
- **shadcn/ui** - Beautiful component library
- **Tailwind CSS** - Utility-first styling

### Backend
- **Rust 1.70+** - Safe, fast, concurrent systems language
- **Tauri 2.0** - Lightweight desktop framework
- **Tokio** - Async runtime for concurrent operations
- **Windows API** - Direct Windows integration via windows-rs
- **SQLite** - Embedded database for persistence
- **Hyper-V / Diskpart** - VHDX optimization

## Quick Start

### Prerequisites

- **OS**: Windows 10 (Build 19041+) or Windows 11
- **WSL2**: Must be installed and working
- **Node.js**: 18+ with npm
- **Rust**: 1.70+ (for development)
- **Disk Space**: 500 MB minimum for installation
- **RAM**: 256 MB minimum (1 GB recommended)

### Installation

#### For End Users

1. Download the latest installer from [GitHub Releases](https://github.com/fabiantax/wsl2-manager-tauri/releases)
2. Run `WSL2Manager_0.1.0_x64_setup.exe` as Administrator
3. Follow the installation wizard
4. Launch WSL2 Manager from Start menu

#### For Developers

See [DEVELOPER_GUIDE.md](docs/DEVELOPER_GUIDE.md#development-setup) for detailed setup.

```bash
# Clone repository
git clone https://github.com/fabiantax/wsl2-manager-tauri.git
cd wsl2-manager-tauri

# Install dependencies
npm install

# Start development server
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Status

**Version:** 0.1.0 - Phase 1 Complete

- ✅ WSL distribution management (start/stop/list)
- ✅ VHDX optimization with safety checks
- ✅ Terminal integration and detection
- ✅ Backup and recovery system
- ✅ Space analysis and cleanup
- ✅ Audit logging and recovery
- ✅ Comprehensive documentation
- 🚀 Production-ready, Phase 2 planned

## Architecture

```
User Interface Layer (React + TypeScript)
    ↓ Tauri IPC (JSON Serialization)
Command Layer (Request Handlers)
    ↓ Dependency Injection
Service Layer (Business Logic)
    ↓ Coordination & Validation
Infrastructure Layer (Database, Cache, Logging)
    ↓ System APIs
Backend Integrations (Windows API, WSL, Hyper-V)
```

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed architecture documentation.

## Documentation

📚 **[Complete Documentation](docs/README.md)** - Organized documentation index with all guides and references

### For End Users
- **[User Guide](docs/user-guide/)** - Features overview and user stories
- **[USER_GUIDE.md](docs/USER_GUIDE.md)** - Complete user manual with features, tasks, and FAQ
- **[TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** - Common issues and solutions

### For Developers
- **[Developer Guide](docs/developer-guide/)** - Frontend implementation and technical user stories
- **[DEVELOPER_GUIDE.md](docs/DEVELOPER_GUIDE.md)** - Development setup, architecture, and workflow
- **[API_REFERENCE.md](docs/API_REFERENCE.md)** - Complete API documentation for all 50+ commands
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - System design patterns and decisions
- **[CONTRIBUTING.md](docs/CONTRIBUTING.md)** - Contribution guidelines and process

### Implementation Details
- **[Core Features](docs/implementation/core-features/)** - VHDX optimization, backup/recovery, space analysis, virtual desktop
- **[Infrastructure](docs/implementation/infrastructure/)** - Cache system, crash recovery, confirmation dialogs
- **[Frontend](docs/implementation/frontend/)** - Component structure and testing guides
- **[Testing](docs/implementation/testing/)** - Integration tests and completion summaries

### Project Management
- **[Project Status](docs/project/)** - Implementation plans, status reports, and delivery checklists
- **[Setup & Configuration](docs/setup/)** - Development setup, spec kit usage, and deployment guides
- **[Guides & Best Practices](docs/guides/)** - Tauri best practices and optimization guides
- **[Tools & AI](docs/tools/)** - AI assistance and automation documentation

### Reports & Analytics
- **[Reports](docs/reports/)** - Benchmarks, completion summaries, and analytics

### Analysis & Specifications
- **[Analysis](docs/analysis/)** - Requirements analysis, specifications, and design documents for all features

### Reports & Quick Reference
- **[Reports](docs/reports/)** - Benchmarks, completion summaries, and analytics
- **[Quick Reference](docs/quick-reference/)** - Start here guide and implementation references

## Development

### Available Commands

```bash
# Development
npm run tauri dev        # Start dev server with hot reload

# Building
npm run tauri build      # Build production bundle

# Testing
cargo test --all         # Run Rust tests
npm test                 # Run TypeScript tests

# Code Quality
cargo fmt                # Format Rust code
npm run lint             # Lint TypeScript
npm run type-check       # Check TypeScript types
cargo clippy             # Lint Rust code
```

### Project Structure

```
wsl2-manager-tauri/
├── src/                      # React Frontend (TypeScript)
│   ├── components/           # React components
│   ├── services/             # API client services
│   ├── types/                # TypeScript types
│   └── App.tsx               # Root component
├── src-tauri/                # Rust Backend
│   ├── src/
│   │   ├── commands/         # Tauri command handlers
│   │   ├── services/         # Business logic services
│   │   ├── models/           # Data structures
│   │   ├── infrastructure/   # Database, cache, logging
│   │   └── main.rs           # Application entry point
│   └── Cargo.toml            # Rust dependencies
├── docs/                     # Documentation
│   ├── USER_GUIDE.md         # End-user manual
│   ├── DEVELOPER_GUIDE.md    # Developer guide
│   ├── API_REFERENCE.md      # API documentation
│   ├── ARCHITECTURE.md       # System design
│   ├── TROUBLESHOOTING.md    # Troubleshooting
│   └── CONTRIBUTING.md       # Contribution guidelines
└── package.json              # Node.js dependencies
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for:

- Code of conduct
- Development setup
- Coding standards
- Commit guidelines
- Pull request process
- Testing requirements

Quick start for contributors:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Follow coding standards and commit guidelines
4. Push to branch and create a Pull Request

## Security

- All operations are logged to audit trail for security and troubleshooting
- Administrator privileges required only for sensitive operations
- No data leaves your computer (all processing is local)
- VHDX optimization is read-only until final phase
- Automatic crash recovery prevents data loss

## Performance

- **Lightweight**: Tauri app is ~50MB (vs 150MB+ for Electron)
- **Fast Startup**: Starts in <2 seconds
- **Memory Efficient**: Uses 30-50MB RAM at idle
- **Responsive UI**: Async operations prevent UI blocking
- **Smart Caching**: Frequently accessed data cached automatically

## System Requirements

| Component | Requirement |
|-----------|------------|
| OS | Windows 10 Build 19041+ or Windows 11 |
| Processor | 1 GHz or faster |
| RAM | 256 MB minimum (1 GB recommended) |
| Disk | 500 MB for installation |
| WSL2 | Must be installed and working |

## Known Limitations

- Windows only (can be extended to macOS/Linux with future updates)
- Requires administrator privileges for certain operations
- Some features require Hyper-V on Windows 10 Pro/Enterprise

## License

MIT License - See [LICENSE](LICENSE) file for details

## Changelog

See [CHANGELOG.md](docs/CHANGELOG.md) for version history and updates.

## Support

### Getting Help

1. Check [USER_GUIDE.md](docs/USER_GUIDE.md#faq) FAQ
2. See [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for common issues
3. Search [GitHub Issues](https://github.com/fabiantax/wsl2-manager-tauri/issues)
4. Open a new issue with detailed information

### Reporting Bugs

Report bugs at: https://github.com/fabiantax/wsl2-manager-tauri/issues

Include:
- Exact error message
- Steps to reproduce
- Windows and WSL version
- Application version

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Desktop application framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [React](https://react.dev/) - UI framework
- [Vite](https://vitejs.dev/) - Build tool
- Open source community

## Roadmap

### Phase 2 (Planned)
- Import/Export distribution snapshots
- Advanced scheduling and automation
- Distribution templates
- Performance monitoring dashboard
- Multi-user support

### Phase 3+ (Future)
- Cross-platform support (macOS/Linux)
- Web dashboard for remote management
- Integration with cloud providers
- Advanced analytics and reporting

---

**Project Owner:** [Fabian Taxis](https://github.com/fabiantax)

**Last Updated:** 2025-12-06

**Note:** This project requires Windows 10/11 with WSL2 installed. WSL2 can be installed via Windows Update or manually from Microsoft Store.
