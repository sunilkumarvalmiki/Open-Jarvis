# Open-Jarvis

An intelligent, privacy-first desktop AI assistant built with Rust and Tauri.

## 🌟 Features

- 🎤 **Voice-controlled AI interactions** - Natural language commands for seamless interaction
- 🔒 **Privacy-first** - All processing can run locally, your data stays with you
- ⚡ **Native performance** - Built with Rust for blazing-fast execution
- 🔌 **Extensible via MCP** - Model Context Protocol support for unlimited capabilities
- 🌐 **Cross-platform** - Works on Windows, macOS, and Linux
- 🤖 **Smart automation** - File organization, system management, and more

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Open-Jarvis Desktop UI                │
│                   (HTML/CSS/JavaScript)                 │
└───────────────────────┬─────────────────────────────────┘
                        │
                        │ Tauri IPC
                        │
┌───────────────────────▼─────────────────────────────────┐
│                  Rust Backend (Tauri)                   │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │   Commands  │  │  MCP Client  │  │  System APIs  │  │
│  │   Handler   │  │              │  │               │  │
│  └─────────────┘  └──────────────┘  └───────────────┘  │
└───────────────────────┬─────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼────────┐ ┌───▼────────┐ ┌───▼──────────┐
│  MCP Tools     │ │  ai-context│ │   polynote   │
│  (GitHub, FS)  │ │  -manager  │ │              │
└────────────────┘ └────────────┘ └──────────────┘
```

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.75+ ([Install Rust](https://rustup.rs/))
- **Node.js** 20+ ([Install Node.js](https://nodejs.org/))
- **npm** or **pnpm** (comes with Node.js)

#### Platform-specific dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

**macOS:**
```bash
# Xcode Command Line Tools required
xcode-select --install
```

**Windows:**
```bash
# No additional dependencies required
# Windows 10/11 with WebView2 (usually pre-installed)
```

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sunilkumarvalmiki/Open-Jarvis.git
   cd Open-Jarvis
   ```

2. **Install frontend dependencies** (if package.json exists):
   ```bash
   npm install
   # or
   pnpm install
   ```

3. **Build the application:**
   ```bash
   # Development mode
   npm run tauri dev
   
   # Production build
   npm run tauri build
   ```

### Development

#### Running in development mode

```bash
cd src-tauri
cargo tauri dev
```

This will start the Tauri application with hot-reload enabled.

#### Building for production

```bash
cd src-tauri
cargo tauri build
```

The compiled application will be available in `src-tauri/target/release`.

#### Code formatting

```bash
cd src-tauri
cargo fmt
```

#### Linting

```bash
cd src-tauri
cargo clippy -- -D warnings
```

#### Testing

```bash
cd src-tauri
cargo test
```

## 🔌 MCP Integration

Open-Jarvis supports the Model Context Protocol (MCP) for extensibility, allowing you to connect various tools and services:

- **GitHub operations** - Repository management, issue tracking, pull requests
- **File system access** - Smart file organization and management
- **Database queries** - Connect to databases for data retrieval
- **Custom tool integration** - Build your own MCP tools

See [docs/mcp-integration.md](docs/mcp-integration.md) for detailed integration guide.

## 🌐 Cross-Project Integration

Open-Jarvis is part of a unified AI ecosystem:

- **[ai-context-manager](https://github.com/sunilkumarvalmiki/ai-context-manager)** - RAG and knowledge retrieval system
- **[polynote](https://github.com/sunilkumarvalmiki/polynote)** - Knowledge management and note-taking

These projects work together to provide a comprehensive AI-assisted workflow.

## ⚙️ Configuration

Configuration is managed through environment variables and config files:

```bash
# Example .env file
JARVIS_MODEL=gpt-4
JARVIS_API_KEY=your_api_key_here
MCP_SERVER_URL=http://localhost:3000
```

See [docs/architecture.md](docs/architecture.md) for detailed configuration options.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Code style and conventions
- Git workflow
- Testing requirements
- Pull request process

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Powered by [Rust](https://www.rust-lang.org/)
- MCP support via [Model Context Protocol](https://modelcontextprotocol.io/)

## 📚 Documentation

- [Architecture Overview](docs/architecture.md)
- [MCP Integration Guide](docs/mcp-integration.md)
- [API Reference](docs/api-reference.md)

## 🐛 Issues & Support

Found a bug or need help? Please [open an issue](https://github.com/sunilkumarvalmiki/Open-Jarvis/issues).

---

Made with ❤️ by the Open-Jarvis team
