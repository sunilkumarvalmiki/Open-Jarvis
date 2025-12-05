# Open-Jarvis

An intelligent, privacy-first desktop AI assistant built with Rust and Tauri.

## Features

- 🎤 **Voice-controlled AI interactions**: Control your computer with natural language
- 🔒 **Privacy-first**: All processing can run locally - your data stays on your machine
- ⚡ **Native performance**: Built with Rust for blazing-fast performance
- 🔌 **Extensible via MCP**: Model Context Protocol enables powerful integrations
- 🌐 **Cross-platform**: Works on Windows, macOS, and Linux

## Current Capabilities

Open-Jarvis currently provides the following automation features:

- **Web Navigation**: Open websites and URLs directly from voice commands
- **System Maintenance**: Empty recycle bin/trash with a single command
- **File Organization**: Automatically organize downloads folder by file type

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Open-Jarvis UI                       │
│                  (HTML/CSS/JavaScript)                  │
└───────────────────────┬─────────────────────────────────┘
                        │
                        │ Tauri IPC
                        │
┌───────────────────────▼─────────────────────────────────┐
│                  Tauri Runtime                          │
│  ┌─────────────┬─────────────┬──────────────────────┐  │
│  │   System    │    File     │   MCP Integration    │  │
│  │  Commands   │  Manager    │   (Coming Soon)      │  │
│  └─────────────┴─────────────┴──────────────────────┘  │
│                    Rust Backend                         │
└─────────────────────────────────────────────────────────┘
                        │
                        │
        ┌───────────────┼───────────────┐
        │               │               │
    ┌───▼───┐      ┌───▼───┐      ┌───▼────┐
    │  OS   │      │ File  │      │  MCP   │
    │ APIs  │      │System │      │ Tools  │
    └───────┘      └───────┘      └────────┘
```

## Quick Start

### Prerequisites

- **Rust** 1.75 or higher ([install from rustup.rs](https://rustup.rs/))
- **Node.js** 20 or higher ([install from nodejs.org](https://nodejs.org/))
- Platform-specific dependencies:
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual C++ Build Tools

### Installation

#### Linux (Ubuntu/Debian)

```bash
# Install system dependencies
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

# Clone the repository
git clone https://github.com/sunilkumarvalmiki/Open-Jarvis.git
cd Open-Jarvis

# Build and run
cargo tauri dev --manifest-path src-tauri/Cargo.toml
```

#### macOS

```bash
# Clone the repository
git clone https://github.com/sunilkumarvalmiki/Open-Jarvis.git
cd Open-Jarvis

# Build and run
cargo tauri dev --manifest-path src-tauri/Cargo.toml
```

#### Windows

```bash
# Clone the repository
git clone https://github.com/sunilkumarvalmiki/Open-Jarvis.git
cd Open-Jarvis

# Build and run
cargo tauri dev --manifest-path src-tauri/Cargo.toml
```

### Development

```bash
# Run in development mode with hot-reload
cargo tauri dev --manifest-path src-tauri/Cargo.toml

# Build for production
cargo tauri build --manifest-path src-tauri/Cargo.toml

# Run tests
cargo test --manifest-path src-tauri/Cargo.toml

# Format code
cargo fmt --manifest-path src-tauri/Cargo.toml

# Lint code
cargo clippy --manifest-path src-tauri/Cargo.toml
```

## MCP Integration

Open-Jarvis is designed to support the Model Context Protocol (MCP) for extensibility. MCP allows Open-Jarvis to:

- Connect to external AI services and models
- Access GitHub repositories and operations
- Interact with file systems securely
- Query databases and data sources
- Integrate custom tools and workflows

**Note**: MCP integration is currently under development. See [docs/mcp-integration.md](docs/mcp-integration.md) for the planned architecture.

## Cross-Project Integration

Open-Jarvis is part of a unified AI ecosystem designed to work seamlessly with:

- **[ai-context-manager](https://github.com/sunilkumarvalmiki/ai-context-manager)**: Retrieval-Augmented Generation (RAG) and knowledge retrieval system
- **[polynote](https://github.com/sunilkumarvalmiki/polynote)**: Knowledge management and note-taking application

Together, these projects create a comprehensive AI-powered productivity environment where:
- Open-Jarvis serves as the primary user interface and orchestrator
- ai-context-manager provides intelligent context and knowledge retrieval
- polynote manages and organizes your knowledge base

## Configuration

Configuration files will be stored in platform-specific directories:

- **Linux**: `~/.config/jarvis/config.json`
- **macOS**: `~/Library/Application Support/com.example.jarvis/config.json`
- **Windows**: `%APPDATA%\jarvis\config.json`

### Environment Variables

- `JARVIS_LOG_LEVEL`: Set logging level (trace, debug, info, warn, error)
- `JARVIS_CONFIG_PATH`: Override default configuration path

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Code style and conventions
- Development workflow
- Testing requirements
- Pull request process

## Architecture Documentation

For detailed architecture information, see:

- [Architecture Overview](docs/architecture.md)
- [MCP Integration Guide](docs/mcp-integration.md)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [x] Basic desktop application with system commands
- [x] File organization capabilities
- [ ] MCP integration framework
- [ ] Voice control interface
- [ ] Integration with ai-context-manager
- [ ] Integration with polynote
- [ ] Plugin system for custom commands
- [ ] Natural language command parsing
- [ ] Multi-language support

## Support

- 🐛 **Bug reports**: [Open an issue](https://github.com/sunilkumarvalmiki/Open-Jarvis/issues/new?template=bug_report.yml)
- 💡 **Feature requests**: [Open an issue](https://github.com/sunilkumarvalmiki/Open-Jarvis/issues/new?template=feature_request.yml)
- 💬 **Questions**: [Start a discussion](https://github.com/sunilkumarvalmiki/Open-Jarvis/discussions)

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop applications
- [Rust](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software
