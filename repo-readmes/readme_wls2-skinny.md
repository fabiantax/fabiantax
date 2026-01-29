# WSL2 Manager

A .NET MAUI application for Windows to manage WSL2 virtual disk (VHDX) sizes. Easily view and shrink your WSL2 distributions' VHDX files to reclaim wasted disk space.

## Features

- **Automatic Discovery**: Detects all installed WSL2 distributions
- **Size Visualization**: Shows VHDX file sizes for each distribution
- **One-Click Optimization**: Shrink individual or all VHDX files
- **Progress Tracking**: Visual feedback during optimization
- **Smart Optimization**: Uses Hyper-V's Optimize-VHD when available, falls back to diskpart

## Requirements

- Windows 10 version 2004+ or Windows 11
- .NET 8.0 SDK
- MAUI workload installed
- WSL2 installed and configured

## Installation

### Prerequisites

1. Install .NET 8.0 SDK from [dotnet.microsoft.com](https://dotnet.microsoft.com/download)

2. Install MAUI workload:
   ```bash
   dotnet workload install maui
   ```

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd wls2-skinny

# Restore packages
dotnet restore

# Build
dotnet build

# Run
dotnet run --project src/Wsl2Manager/Wsl2Manager.csproj
```

### Publish Release Build

```bash
dotnet publish -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true
```

The executable will be in `src/Wsl2Manager/bin/Release/net8.0-windows10.0.19041.0/win-x64/publish/`

## Usage

1. Launch WSL2 Manager
2. Click **Refresh** to scan for WSL2 distributions
3. View the list of distributions with their VHDX sizes
4. Click **Shrink** next to a specific distribution to optimize it
5. Or click **Optimize All** to shrink all distributions at once

**Note**: Optimization requires shutting down WSL. The application will automatically shut down WSL before optimizing.

## How It Works

WSL2 uses dynamically expanding VHDX (Virtual Hard Disk) files to store Linux distributions. These files grow automatically as you add data but don't shrink when you delete files. This application:

1. Detects WSL2 distributions using `wsl --list --verbose`
2. Locates VHDX files in standard locations
3. Shuts down WSL using `wsl --shutdown`
4. Optimizes VHDX using:
   - `Optimize-VHD` PowerShell cmdlet (if Hyper-V is available)
   - `diskpart` compact command (fallback)

## Project Structure

```
wls2-skinny/
├── docs/                    # Documentation
│   ├── PRD.md              # Product Requirements
│   ├── ARCHITECTURE.md     # Architecture design
│   └── SPEC.md             # Technical specification
├── src/
│   └── Wsl2Manager/        # Main application
│       ├── Models/         # Data models
│       ├── Services/       # Business logic
│       ├── ViewModels/     # MVVM ViewModels
│       ├── Views/          # XAML pages
│       └── Helpers/        # Utility classes
├── tests/
│   └── Wsl2Manager.Tests/  # Unit tests
├── .editorconfig           # Code style rules
└── Directory.Build.props   # Build configuration
```

## Code Quality

This project uses:
- **StyleCop.Analyzers** for code style enforcement
- **Microsoft.CodeAnalysis.NetAnalyzers** for code quality
- **EditorConfig** for consistent formatting
- Nullable reference types enabled
- Warnings treated as errors

## Running Tests

```bash
dotnet test
```

## Troubleshooting

### "WSL not installed" error
- Ensure WSL2 is installed: `wsl --install`
- Update WSL: `wsl --update`

### Optimization fails
- Ensure no WSL processes are running
- Try running the application as administrator
- Check if another application is using the VHDX file

### VHDX not found
- The application searches standard WSL2 locations
- Custom VHDX locations (moved by user) may not be detected

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `dotnet test`
5. Submit a pull request

## License

This project is provided as-is for educational and personal use.

## Acknowledgments

- Built with [.NET MAUI](https://dotnet.microsoft.com/apps/maui)
- Uses [CommunityToolkit.Mvvm](https://github.com/CommunityToolkit/dotnet) for MVVM support
