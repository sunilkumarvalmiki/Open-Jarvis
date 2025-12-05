# Open-Jarvis Architecture

This document provides a comprehensive overview of the Open-Jarvis architecture, including system components, data flow, and integration points.

## Table of Contents

1. [System Overview](#system-overview)
2. [Component Architecture](#component-architecture)
3. [Data Flow](#data-flow)
4. [MCP Integration](#mcp-integration)
5. [Security Model](#security-model)
6. [Performance Considerations](#performance-considerations)

## System Overview

Open-Jarvis is built using the Tauri framework, combining a Rust backend with a web-based frontend. This architecture provides native performance while maintaining flexibility and ease of development.

### Key Design Principles

- **Privacy-First**: Local processing by default, optional cloud features
- **Modularity**: Plugin-based architecture for extensibility
- **Performance**: Native Rust backend for CPU-intensive tasks
- **Security**: Strict sandboxing and permission model
- **Cross-Platform**: Single codebase for Windows, macOS, and Linux

## Component Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Open-Jarvis Application                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                    Presentation Layer                          │ │
│  ├────────────────────────────────────────────────────────────────┤ │
│  │                                                                │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │ │
│  │  │   UI Views   │  │  Components  │  │   Assets     │        │ │
│  │  │   (HTML/CSS) │  │  (JavaScript)│  │  (Images)    │        │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘        │ │
│  │                                                                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                              ▲ │                                    │
│                              │ │                                    │
│                    Tauri IPC │ │ Events/Commands                   │
│                              │ ▼                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                      Application Layer                         │ │
│  ├────────────────────────────────────────────────────────────────┤ │
│  │                                                                │ │
│  │  ┌────────────────────────────────────────────────┐           │ │
│  │  │          Tauri Runtime                         │           │ │
│  │  │  • Window Management                           │           │ │
│  │  │  • Menu System                                 │           │ │
│  │  │  • Event Loop                                  │           │ │
│  │  │  • IPC Bridge                                  │           │ │
│  │  └────────────────────────────────────────────────┘           │ │
│  │                                                                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                              ▲ │                                    │
│                              │ │                                    │
│                        Calls │ │ Results                           │
│                              │ ▼                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                      Business Logic Layer                      │ │
│  ├────────────────────────────────────────────────────────────────┤ │
│  │                                                                │ │
│  │  ┌──────────────────┐  ┌──────────────────┐                  │ │
│  │  │ Command Handlers │  │  MCP Client      │                  │ │
│  │  │  • open_browser  │  │  • Tool Discovery│                  │ │
│  │  │  • empty_bin     │  │  • Tool Invoke   │                  │ │
│  │  │  • organize_files│  │  • Schema Parser │                  │ │
│  │  └──────────────────┘  └──────────────────┘                  │ │
│  │                                                                │ │
│  │  ┌──────────────────┐  ┌──────────────────┐                  │ │
│  │  │ Voice Processing │  │  AI Integration  │                  │ │
│  │  │  (Future)        │  │  (Future)        │                  │ │
│  │  └──────────────────┘  └──────────────────┘                  │ │
│  │                                                                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                              ▲ │                                    │
│                              │ │                                    │
│                              │ ▼                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                      Data/Service Layer                        │ │
│  ├────────────────────────────────────────────────────────────────┤ │
│  │                                                                │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │ │
│  │  │   Config     │  │   Cache      │  │   Logging    │        │ │
│  │  │   Manager    │  │   Manager    │  │   System     │        │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘        │ │
│  │                                                                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
                              ▲ │
                              │ │
                              │ ▼
        ┌─────────────────────────────────────────────────┐
        │        External Systems                         │
        ├─────────────────────────────────────────────────┤
        │  • File System                                  │
        │  • Operating System Services                    │
        │  • MCP Server Processes                         │
        │  • ai-context-manager                           │
        │  • polynote                                     │
        │  • GitHub API                                   │
        └─────────────────────────────────────────────────┘
```

### Component Descriptions

#### Presentation Layer
- **Responsibility**: User interface and user experience
- **Technology**: HTML, CSS, JavaScript
- **Communication**: Tauri IPC commands and events

#### Application Layer (Tauri Runtime)
- **Responsibility**: Window management, system integration, IPC bridge
- **Technology**: Tauri framework (Rust + WebView)
- **Features**:
  - Cross-platform window management
  - Native menu integration
  - System tray support
  - Secure IPC communication

#### Business Logic Layer
- **Responsibility**: Core application logic
- **Technology**: Rust
- **Components**:
  - **Command Handlers**: Process user actions and system commands
  - **MCP Client**: Communicate with MCP-compliant tools and services
  - **Voice Processing** (Future): Natural language understanding
  - **AI Integration** (Future): Local or cloud AI model integration

#### Data/Service Layer
- **Responsibility**: Data persistence, configuration, and cross-cutting concerns
- **Technology**: Rust with appropriate libraries
- **Components**:
  - **Config Manager**: Application and user settings
  - **Cache Manager**: Performance optimization
  - **Logging System**: Debugging and audit trails

## Data Flow

### User Command Flow

```
1. User Action (UI)
       │
       ▼
2. JavaScript Event Handler
       │
       ▼
3. Tauri Invoke Command
       │
       ▼
4. Rust Command Handler
       │
       ├─► 5a. Direct Execution (open_browser, empty_bin)
       │         │
       │         ▼
       │    6a. System Call / File Operation
       │
       └─► 5b. MCP Tool Invocation
                 │
                 ▼
            6b. MCP Server Communication
                 │
                 ▼
            7b. Tool Execution
       │
       ▼
8. Result Processing
       │
       ▼
9. Response to Frontend
       │
       ▼
10. UI Update
```

### MCP Integration Flow

```
1. MCP Tool Discovery
       │
       ▼
2. Schema Parsing
       │
       ▼
3. Tool Registration
       │
       ▼
4. User Invokes Functionality
       │
       ▼
5. Parameter Validation
       │
       ▼
6. MCP Request Construction
       │
       ▼
7. Send to MCP Server (JSON-RPC)
       │
       ▼
8. MCP Server Processes Request
       │
       ▼
9. Response Received
       │
       ▼
10. Result Parsing
       │
       ▼
11. Error Handling (if needed)
       │
       ▼
12. Return to User
```

## MCP Integration

### MCP Client Architecture

The MCP client is designed to be modular and extensible:

```rust
// Conceptual structure
pub struct McpClient {
    servers: HashMap<String, McpServer>,
    tools: HashMap<String, Tool>,
}

pub struct McpServer {
    name: String,
    transport: Transport,
    capabilities: Capabilities,
}

pub struct Tool {
    name: String,
    description: String,
    parameters: Schema,
    server: String,
}
```

### Communication Protocol

- **Protocol**: JSON-RPC 2.0 over stdio or SSE
- **Message Format**: Structured JSON messages
- **Error Handling**: Standardized error codes and messages
- **Versioning**: Protocol version negotiation

### Tool Discovery Process

1. **Server Connection**: Connect to MCP server process
2. **Initialize Handshake**: Exchange capabilities
3. **List Tools**: Request available tools
4. **Parse Schemas**: Validate and store tool schemas
5. **Register Tools**: Make tools available to application

## Security Model

### Threat Model

1. **Malicious Input**: User-provided data could contain malicious content
2. **External Tools**: MCP servers could be compromised or malicious
3. **File System Access**: Unauthorized file operations
4. **Network Requests**: Unintended data exfiltration
5. **Code Injection**: Script injection through UI

### Security Measures

#### Sandboxing
- Tauri's security features isolate web content
- CSP (Content Security Policy) prevents XSS attacks
- IPC validation ensures only authorized commands execute

#### Permission System
```json
{
  "allowlist": {
    "all": false,
    "shell": {
      "open": true  // Only specific shell operations allowed
    }
  }
}
```

#### Input Validation
- All user inputs sanitized before processing
- Path traversal prevention
- Command injection prevention

#### MCP Security
- Tool whitelisting
- Permission prompts for sensitive operations
- Audit logging of tool invocations

## Performance Considerations

### Optimization Strategies

1. **Lazy Loading**: Load MCP tools on-demand
2. **Caching**: Cache tool schemas and responses
3. **Async Operations**: Non-blocking I/O for all external calls
4. **Resource Pooling**: Reuse connections to MCP servers
5. **Minimal Rendering**: Efficient DOM updates

### Performance Metrics

- **Startup Time**: Target < 2 seconds cold start
- **Command Response**: Target < 100ms for local operations
- **Memory Usage**: Target < 100MB baseline
- **CPU Usage**: Target < 5% idle

### Profiling

Use Rust's built-in profiling tools:
```bash
cargo build --release
cargo flamegraph --bin jarvis-tauri
```

## Future Architecture Enhancements

1. **Plugin System**: Dynamic plugin loading
2. **Local AI Models**: Embedded ML inference
3. **Distributed Mode**: Multi-device synchronization
4. **Advanced Voice**: Real-time speech processing
5. **Smart Context**: Automated context gathering from ecosystem

## References

- [Tauri Architecture](https://tauri.app/v1/references/architecture/)
- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
