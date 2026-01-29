# VSCode AI Diagnostics Extension

Adds code actions to the VSCode problems pane that allow opening diagnostic information in various AI tools via terminal commands.

## Features

- **Open in Claude Bash**: Opens terminal with `dsp "{code},{message},{file}"`
- **Open with Gemini**: Opens terminal with `gemini --yolo "{code},{message},{file}"`
- **Open with Qwen**: Opens terminal with `qwen --yolo "{code},{message},{file}"`

## Installation

### From GitHub Releases

1. Go to the [Releases](https://github.com/your-username/vscode-ai-diagnostics/releases) page
2. Download the latest `.vsix` file
3. In VSCode, run:
   ```bash
   code --install-extension vscode-claude-extension-X.X.X.vsix
   ```
4. Reload VSCode

### Manual Installation

```bash
npm install -g @vscode/vsce
npm run package
code --install-extension vscode-claude-extension-0.0.1.vsix
```

## Usage

1. Open a file with diagnostics (errors/warnings)
2. Go to Problems pane (Ctrl+Shift+M)
3. Hover over any diagnostic
4. Click "Show fixes"
5. Select your preferred AI tool option
6. A terminal opens with the formatted command

## Development

### Prerequisites

- Node.js 18+
- VSCode

### Setup

```bash
npm install
npm run compile
```

### Testing

Press F5 in VSCode to launch extension development host.

### Packaging

```bash
npm run package
```

## Requirements

- VSCode 1.74.0 or later
- Terminal commands (`dsp`, `gemini`, `qwen`) must be available in PATH

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details
