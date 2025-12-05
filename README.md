# Open-Jarvis

An intelligent, privacy-first desktop AI assistant built with Rust and Tauri.

## Features

- 🎤 **Voice-controlled AI interactions** - Natural language command processing
- 🔒 **Privacy-first** - All processing can run locally on your machine
- ⚡ **Native performance** - Built with Rust backend for maximum efficiency
- 🔌 **Extensible via MCP** - Model Context Protocol for tool integration
- 🌐 **Cross-platform** - Runs on Windows, macOS, and Linux
- 🛠️ **System automation** - File organization, system cleanup, and more

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Open-Jarvis Desktop App                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────┐      ┌──────────────────┐            │
│  │   Frontend (UI)  │◄────►│  Tauri IPC Layer │            │
│  │   HTML/CSS/JS    │      │                  │            │
│  └──────────────────┘      └─────────┬────────┘            │
│                                       │                      │
│                                       ▼                      │
│                            ┌──────────────────┐             │
│                            │   Rust Backend   │             │
│                            │                  │             │
│                            │ ┌──────────────┐ │             │
│                            │ │ Command      │ │             │
│                            │ │ Handlers     │ │             │
│                            │ └──────────────┘ │             │
│                            │                  │             │
│                            │ ┌──────────────┐ │             │
│                            │ │ MCP Client   │ │             │
│                            │ │ Integration  │ │             │
│                            │ └──────────────┘ │             │
│                            └──────────────────┘             │
│                                       │                      │
└───────────────────────────────────────┼──────────────────────┘
                                        │
                                        ▼
                    ┌───────────────────────────────┐
                    │  External Tools & Services     │
                    ├───────────────────────────────┤
                    │  • ai-context-manager         │
                    │  • polynote                   │
                    │  • GitHub operations          │
                    │  • File system access         │
                    │  • Custom integrations        │
                    └───────────────────────────────┘
```

## Quick Start

### Prerequisites

- **Rust** 1.75 or higher - [Install Rust](https://rustup.rs/)
- **Node.js** 20 or higher - [Install Node.js](https://nodejs.org/)
- **pnpm** (recommended) or npm - `npm install -g pnpm`

#### Platform-Specific Requirements

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf
```

**Fedora/RHEL:**
```bash
sudo dnf install webkit2gtk4.1-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  patchelf
```

**macOS:**
```bash
# No additional dependencies required
# Xcode Command Line Tools are recommended
xcode-select --install
```

**Windows:**
```powershell
# Microsoft Visual Studio C++ Build Tools required
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sunilkumarvalmiki/Open-Jarvis.git
   cd Open-Jarvis
   ```

2. **Build the Tauri application:**
   ```bash
   cd src-tauri
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   cargo run --release
   ```

### Development

For active development with hot-reload:

```bash
# Terminal 1: Run Tauri in development mode
cd src-tauri
cargo run

# The application will launch with development features enabled
```

**Development workflow:**
- Make changes to Rust code in `src-tauri/src/`
- Make changes to frontend in `src/`
- Rebuild with `cargo build` or run with `cargo run`

**Code formatting:**
```bash
cd src-tauri
cargo fmt
```

**Linting:**
```bash
cd src-tauri
cargo clippy -- -D warnings
```

**Running tests:**
```bash
cd src-tauri
cargo test
```

## MCP Integration

Open-Jarvis supports the Model Context Protocol (MCP) for extensibility, allowing seamless integration with various tools and services.

### Supported MCP Tools

- **GitHub Operations** - Issue tracking, PR management, repository operations
- **File System Access** - Read, write, and organize files
- **Database Queries** - Connect to various data sources
- **Custom Tool Integration** - Add your own tools and services

### Configuration

MCP tools are configured via the application settings or configuration file:

```json
{
  "mcp": {
    "enabled": true,
    "tools": [
      {
        "name": "github",
        "config": {
          "token": "${GITHUB_TOKEN}"
        }
      }
    ]
  }
}
```

For detailed MCP integration documentation, see [docs/mcp-integration.md](docs/mcp-integration.md).

## Cross-Project Integration

Open-Jarvis is part of a unified AI ecosystem:

- **[ai-context-manager](https://github.com/sunilkumarvalmiki/ai-context-manager)** - RAG (Retrieval-Augmented Generation) and knowledge retrieval system
- **[polynote](https://github.com/sunilkumarvalmiki/polynote)** - Knowledge management and intelligent note-taking

These projects work together to provide:
- Context-aware AI responses
- Persistent knowledge storage
- Seamless information retrieval
- Unified workflow automation

## Configuration

### Environment Variables

- `GITHUB_TOKEN` - GitHub personal access token for repository operations
- `OPENAI_API_KEY` - OpenAI API key (if using cloud AI features)
- `JARVIS_CONFIG_PATH` - Custom configuration file path

### Configuration File

Create a `.jarvis.json` file in your home directory:

```json
{
  "theme": "dark",
  "voice": {
    "enabled": true,
    "engine": "local"
  },
  "ai": {
    "provider": "local",
    "model": "llama2"
  }
}
```

## Building for Production

### Create Release Build

```bash
cd src-tauri
cargo build --release
```

The compiled binary will be in `src-tauri/target/release/`.

### Create Distribution Package

```bash
cd src-tauri
cargo tauri build
```

This creates platform-specific installers in `src-tauri/target/release/bundle/`.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on:

- Code of Conduct
- Development setup
- Coding standards
- Pull request process
- Testing requirements

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Voice recognition and synthesis
- [ ] Multi-language support
- [ ] Plugin system
- [ ] Cloud sync (optional)
- [ ] Mobile companion app
- [ ] Enhanced MCP tool marketplace

## Support

- **Issues** - Report bugs via [GitHub Issues](https://github.com/sunilkumarvalmiki/Open-Jarvis/issues)
- **Discussions** - Join conversations in [GitHub Discussions](https://github.com/sunilkumarvalmiki/Open-Jarvis/discussions)
- **Documentation** - See the [docs](docs/) directory

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Desktop application framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [MCP](https://modelcontextprotocol.io/) - Model Context Protocol

---

**Made with ❤️ by the Open-Jarvis team**
