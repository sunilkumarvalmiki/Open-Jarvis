# Open-Jarvis

An open-source, privacy-first AI assistant for desktop.

[![CI](https://github.com/sunilkumarvalmiki/Open-Jarvis/workflows/CI/badge.svg)](https://github.com/sunilkumarvalmiki/Open-Jarvis/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🖥️ **Desktop Native**: Built with Tauri for minimal footprint
- 🔒 **Privacy First**: Local-first architecture, your data stays yours
- 🧠 **AI Powered**: Integrates with local and cloud LLMs
- 🔧 **Extensible**: MCP (Model Context Protocol) for tool integration
- ⚡ **Fast**: Rust backend for maximum performance

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Open-Jarvis                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   │
│  │   Frontend   │    │  Tauri Core  │    │  MCP Client  │   │
│  │  (Web View)  │◄──►│    (Rust)    │◄──►│   (Tools)    │   │
│  └──────────────┘    └──────┬───────┘    └──────────────┘   │
│                             │                                │
│                    ┌────────▼────────┐                      │
│                    │   AI Backend    │                      │
│                    │                 │                      │
│                    │ ┌─────────────┐ │                      │
│                    │ │ Local LLM   │ │                      │
│                    │ │ (Ollama)    │ │                      │
│                    │ └─────────────┘ │                      │
│                    │ ┌─────────────┐ │                      │
│                    │ │ Cloud LLM   │ │                      │
│                    │ │ (OpenAI/    │ │                      │
│                    │ │  Anthropic) │ │                      │
│                    │ └─────────────┘ │                      │
│                    └─────────────────┘                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.75+
- Node.js 20+
- pnpm 9+

### Development

```bash
# Clone the repository
git clone https://github.com/sunilkumarvalmiki/Open-Jarvis.git
cd Open-Jarvis

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Build

```bash
# Build for production
pnpm tauri build
```

## Project Structure

```
Open-Jarvis/
├── src/                    # Frontend source
│   ├── components/         # UI components
│   ├── hooks/              # React hooks
│   ├── stores/             # State management
│   └── styles/             # CSS/styling
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── commands/       # Tauri commands
│   │   ├── ai/             # AI integration
│   │   ├── mcp/            # MCP client
│   │   └── config/         # Configuration
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                   # Documentation
└── tests/                  # Test suites
```

## MCP Integration

Open-Jarvis uses the Model Context Protocol for extensible tool integration:

```rust
// Example MCP tool registration
let mcp_client = McpClient::new()
    .add_server("github", GithubMcpServer::new())
    .add_server("filesystem", FilesystemMcpServer::new())
    .build();
```

## Configuration

Create `~/.config/open-jarvis/config.toml`:

```toml
[ai]
provider = "ollama"  # or "openai", "anthropic"
model = "llama3.2"

[ai.ollama]
base_url = "http://localhost:11434"

[ai.openai]
api_key = "${OPENAI_API_KEY}"

[mcp]
servers = ["github", "filesystem"]
```

## Roadmap

- [ ] Voice input/output (Whisper + TTS)
- [ ] Plugin system
- [ ] Cross-device sync
- [ ] Mobile companion app
- [ ] Integration with ai-context-manager
- [ ] Integration with polynote

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE)
