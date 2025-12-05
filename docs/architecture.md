# Open-Jarvis Architecture

## Overview

Open-Jarvis is a desktop AI assistant built with Tauri, combining a web-based frontend with a high-performance Rust backend. The architecture is designed for extensibility, privacy, and performance.

## System Architecture

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

## Components

### Frontend (Web View)

The frontend is built using web technologies (HTML, CSS, JavaScript) and rendered in a WebView.

**Responsibilities:**
- User interface rendering
- User input handling
- Display of AI responses
- Settings management

**Technologies:**
- HTML/CSS for UI
- JavaScript for interactivity
- Tauri API for backend communication

### Tauri Core (Rust)

The Tauri core provides the bridge between the frontend and backend functionality.

**Responsibilities:**
- Window management
- IPC (Inter-Process Communication) between frontend and backend
- Security boundaries
- System API access
- Command handling

**Key Files:**
- `src-tauri/src/main.rs`: Entry point and command registration
- `src-tauri/src/commands/`: Tauri command implementations
- `src-tauri/tauri.conf.json`: Tauri configuration

### MCP Client

The Model Context Protocol (MCP) client enables integration with external tools and services.

**Responsibilities:**
- Tool discovery and registration
- Tool execution
- Result handling
- Error management

**Architecture:**
```
McpClient
├── Server Registry (HashMap<String, Box<dyn McpServer>>)
└── call_tool(server, tool, params) -> Result
    └── Routes to appropriate server
        └── Executes tool with params
            └── Returns result
```

**Key Files:**
- `src-tauri/src/mcp/client.rs`: Core MCP client
- `src-tauri/src/mcp/server.rs`: Server implementations
- `src-tauri/src/mcp/tools.rs`: Tool definitions

### AI Backend

The AI backend handles all LLM interactions, supporting both local and cloud providers.

**Responsibilities:**
- LLM provider abstraction
- Prompt construction
- Response streaming
- Context management
- Model selection

**Supported Providers:**
- Local: Ollama, LLaMA.cpp
- Cloud: OpenAI, Anthropic, Google

## Data Flow

### User Query Flow

```
1. User enters query in Frontend
   ↓
2. Frontend sends IPC message to Tauri
   ↓
3. Tauri routes to appropriate command handler
   ↓
4. Command handler processes request
   ↓
5. AI Backend generates response
   ├─ Local LLM (Ollama)
   └─ Cloud LLM (OpenAI/Anthropic)
   ↓
6. Response sent back through IPC
   ↓
7. Frontend displays response
```

### Tool Execution Flow

```
1. AI determines tool is needed
   ↓
2. AI Backend calls MCP Client
   ↓
3. MCP Client routes to appropriate server
   ↓
4. Server executes tool
   ↓
5. Result returned to AI Backend
   ↓
6. AI incorporates result into response
   ↓
7. Final response sent to Frontend
```

## Security Model

### Sandboxing

- Frontend runs in a sandboxed WebView
- No direct file system access from frontend
- All system operations go through Tauri commands

### Permission Model

- Explicit permissions required in `tauri.conf.json`
- Capabilities defined per command
- CSP (Content Security Policy) enforced

### Data Privacy

- Local-first architecture
- User data stays on device
- Optional cloud sync with encryption
- No telemetry by default

## Configuration

Configuration is stored in `~/.config/open-jarvis/config.toml`:

```toml
[ai]
provider = "ollama"
model = "llama3.2"

[ai.ollama]
base_url = "http://localhost:11434"

[ai.openai]
api_key = "${OPENAI_API_KEY}"

[mcp]
servers = ["github", "filesystem"]
```

## Extension Points

### Adding a New MCP Server

1. Implement the `McpServer` trait
2. Define tools via `tools()` method
3. Implement tool execution in `call_tool()`
4. Register server with `McpClient`

### Adding a New AI Provider

1. Implement provider client
2. Add to AI backend router
3. Add configuration schema
4. Update settings UI

### Adding a New Command

1. Define command function with `#[tauri::command]`
2. Register in `invoke_handler`
3. Add frontend bindings
4. Update UI

## Performance Considerations

### Rust Backend

- Zero-cost abstractions
- Minimal runtime overhead
- Efficient memory usage
- Fast startup time

### Frontend

- Lazy loading
- Virtual scrolling for long conversations
- Efficient re-rendering
- Optimized bundle size

### AI Integration

- Request batching
- Response streaming
- Context caching
- Model warm-up

## Future Enhancements

- Plugin system for community extensions
- Cross-device sync
- Voice input/output
- Mobile companion app
- Advanced context management
- Multi-agent orchestration
