# Architecture Overview

This document provides a comprehensive overview of the Open-Jarvis architecture, including system design, component interactions, and security considerations.

## Table of Contents

- [System Overview](#system-overview)
- [Component Architecture](#component-architecture)
- [Data Flow](#data-flow)
- [MCP Integration Points](#mcp-integration-points)
- [Security Model](#security-model)
- [Technology Stack](#technology-stack)

## System Overview

Open-Jarvis is a desktop AI assistant built using the Tauri framework, combining a Rust backend with a web-based frontend. The application leverages the Model Context Protocol (MCP) for extensibility and integration with external tools and services.

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      User Interface Layer                   │
│                    (HTML/CSS/JavaScript)                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Voice Input │  │   Commands   │  │    Status    │      │
│  │   Interface  │  │   Dashboard  │  │   Display    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└────────────────────────────┬────────────────────────────────┘
                             │
                    Tauri IPC Bridge
                             │
┌────────────────────────────▼────────────────────────────────┐
│                    Application Core (Rust)                  │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │              Command Handler Layer                  │    │
│  │  ┌──────────────┐  ┌─────────────┐  ┌──────────┐  │    │
│  │  │    Browser   │  │    File     │  │  System  │  │    │
│  │  │   Commands   │  │ Organization│  │ Commands │  │    │
│  │  └──────────────┘  └─────────────┘  └──────────┘  │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │              MCP Integration Layer                  │    │
│  │  ┌──────────────┐  ┌─────────────┐  ┌──────────┐  │    │
│  │  │  MCP Client  │  │    Tools    │  │  Config  │  │    │
│  │  │   Manager    │  │   Registry  │  │  Manager │  │    │
│  │  └──────────────┘  └─────────────┘  └──────────┘  │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │              System Interface Layer                 │    │
│  │  ┌──────────────┐  ┌─────────────┐  ┌──────────┐  │    │
│  │  │ File System  │  │   Process   │  │ Network  │  │    │
│  │  │    APIs      │  │  Management │  │   APIs   │  │    │
│  │  └──────────────┘  └─────────────┘  └──────────┘  │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────┬───────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
┌───────▼────────┐  ┌─────▼──────┐  ┌──────▼─────────┐
│  External MCP  │  │ ai-context │  │   polynote     │
│     Tools      │  │  -manager  │  │  Knowledge DB  │
│ (GitHub, etc.) │  │   (RAG)    │  │                │
└────────────────┘  └────────────┘  └────────────────┘
```

## Component Architecture

### 1. User Interface Layer

**Technology**: HTML5, CSS3, JavaScript (Vanilla)

**Responsibilities**:
- Render user interface
- Capture user input (voice, clicks, keyboard)
- Display command results and status updates
- Provide visual feedback for operations

**Key Files**:
- `src/index.html` - Main UI structure
- `src/main.js` - Event handlers and Tauri IPC calls
- `src/style.css` - UI styling

### 2. Application Core (Rust)

**Technology**: Rust, Tauri Framework

**Responsibilities**:
- Handle IPC communication from frontend
- Execute system commands
- Manage application state
- Coordinate MCP integrations
- Handle errors and provide feedback

**Key Modules**:
- `main.rs` - Application entry point and Tauri setup
- Command handlers (`#[tauri::command]` functions)
- MCP integration module (future)

### 3. Command Handler Layer

Implements specific functionality exposed to the frontend:

#### Browser Commands
- `open_browser(url)` - Opens URLs in default browser
- Uses async execution to prevent UI blocking

#### File Organization
- `organize_files()` - Organizes downloads folder by file type
- Creates categorized folders (Documents, Music, Pictures)
- Moves files based on extensions

#### System Commands
- `empty_recycle_bin()` - Clears system trash/recycle bin
- Platform-specific implementations (Windows, macOS, Linux)

### 4. MCP Integration Layer

**Status**: In Development

**Purpose**: Enable extensibility through Model Context Protocol

**Components**:
- **MCP Client**: Manages connections to MCP servers
- **Tools Registry**: Registers and manages available tools
- **Config Manager**: Handles MCP configuration

See [MCP Integration](./mcp-integration.md) for details.

### 5. System Interface Layer

**Platform Abstraction**: Provides cross-platform system access

**Capabilities**:
- File system operations (read, write, organize)
- Process management (execute commands)
- Network operations (HTTP requests, WebSocket)
- OS-specific features (clipboard, notifications)

## Data Flow

### Command Execution Flow

```
User Action (Click/Voice)
    │
    ▼
Frontend Event Handler (main.js)
    │
    ▼
Tauri IPC Call (invoke)
    │
    ▼
Rust Command Handler (#[tauri::command])
    │
    ├─► Async Task Spawn
    │   │
    │   ▼
    │   Platform-Specific Implementation
    │   │
    │   ▼
    │   System API Call
    │   │
    │   ▼
    │   Result/Error
    │
    ▼
Response to Frontend
    │
    ▼
UI Update/Notification
```

### MCP Integration Flow (Planned)

```
Command Request
    │
    ▼
MCP Client
    │
    ├─► Tool Lookup
    │   │
    │   ▼
    │   Tool Selection
    │   │
    │   ▼
    │   Parameter Preparation
    │
    ▼
MCP Server Communication
    │
    ├─► Request Serialization
    │   │
    │   ▼
    │   Network Transport
    │   │
    │   ▼
    │   Response Deserialization
    │
    ▼
Result Processing
    │
    ▼
Return to Command Handler
```

## MCP Integration Points

### Current Integration Points

1. **File System Operations**
   - Directory organization
   - File management
   - Path resolution

2. **Browser Automation**
   - URL opening
   - Web navigation

3. **System Management**
   - Recycle bin operations
   - Process execution

### Planned Integration Points

1. **GitHub Operations**
   - Repository management
   - Issue tracking
   - Pull request automation

2. **AI Context Manager**
   - Knowledge retrieval (RAG)
   - Context injection
   - Semantic search

3. **Polynote Integration**
   - Note management
   - Knowledge graph
   - Cross-referencing

4. **Custom Tools**
   - User-defined MCP tools
   - Plugin system
   - Extension marketplace

## Security Model

### Security Principles

1. **Principle of Least Privilege**: Commands run with minimal required permissions
2. **User Confirmation**: Destructive operations require explicit user approval
3. **Sandboxing**: Tauri's security model provides process isolation
4. **Data Privacy**: No data transmitted without user consent

### Security Layers

#### 1. Tauri Security

- **CSP (Content Security Policy)**: Restricts resource loading
- **IPC Allowlist**: Only explicitly registered commands are callable
- **Process Isolation**: Frontend and backend run in separate processes

#### 2. Rust Memory Safety

- **No Buffer Overflows**: Rust's ownership system prevents memory corruption
- **Safe Concurrency**: Type system prevents data races
- **Error Handling**: Explicit error handling (Result types)

#### 3. Platform Security

- **Sandboxed File Access**: Limited to user-approved directories
- **System API Restrictions**: Uses platform security mechanisms
- **Credential Management**: Secure storage for API keys

### Threat Model

**Threats Mitigated**:
- ✅ Arbitrary code execution from frontend
- ✅ Unauthorized file system access
- ✅ Memory corruption vulnerabilities
- ✅ Cross-site scripting (XSS)

**Ongoing Considerations**:
- ⚠️ Supply chain security (dependency auditing)
- ⚠️ Network-based attacks (when MCP integration is active)
- ⚠️ Social engineering (user-initiated destructive commands)

### Security Best Practices

1. **Dependency Auditing**:
   ```bash
   cargo audit
   ```

2. **Code Scanning**:
   - GitHub Dependabot
   - Clippy security lints
   - Manual security reviews

3. **Update Policy**:
   - Regular dependency updates
   - Security patch priority
   - Version pinning for stability

## Technology Stack

### Backend (Rust)

- **Tauri 1.x**: Desktop application framework
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **dirs-next**: Platform-agnostic directory access

### Frontend

- **HTML5**: Markup
- **CSS3**: Styling
- **Vanilla JavaScript**: Logic (no framework dependencies)
- **Tauri API**: Bridge to Rust backend

### Development Tools

- **rustfmt**: Code formatting
- **clippy**: Linting and best practices
- **cargo**: Build system and package manager
- **GitHub Actions**: CI/CD

### Future Additions

- **MCP Client Library**: For protocol communication
- **Speech Recognition**: For voice commands
- **LLM Integration**: For natural language processing
- **Database**: For persistent state (SQLite or similar)

## Performance Considerations

### Optimization Strategies

1. **Async Execution**: Non-blocking operations for UI responsiveness
2. **Lazy Loading**: Load components on demand
3. **Caching**: Cache frequently accessed data
4. **Batch Operations**: Group similar operations

### Resource Management

- **Memory**: Rust's ownership system provides automatic memory management
- **CPU**: Background tasks for heavy computations
- **Disk I/O**: Async file operations
- **Network**: Connection pooling for MCP communications

## Scalability

### Current Limitations

- Single-user desktop application
- Local processing only
- Limited concurrent operations

### Future Scalability

- Multi-user configurations (enterprise)
- Distributed MCP server architecture
- Cloud-based AI model integration
- Plugin ecosystem

## Deployment

### Distribution Methods

1. **Platform-Specific Installers**:
   - Windows: .msi, .exe
   - macOS: .dmg, .app
   - Linux: .deb, .rpm, AppImage

2. **Auto-Update**:
   - Tauri's built-in updater
   - Version checking
   - Delta updates

3. **Package Managers**:
   - Homebrew (macOS)
   - Chocolatey (Windows)
   - apt/yum (Linux)

## Monitoring and Diagnostics

### Logging

- Structured logging with log levels
- Rotation and retention policies
- User-accessible log viewer

### Error Reporting

- Graceful error handling
- User-friendly error messages
- Optional error reporting (with user consent)

### Metrics

- Command execution times
- Success/failure rates
- Resource usage statistics

## Future Architecture Evolution

### Planned Enhancements

1. **Voice Interface**: Speech-to-text and text-to-speech
2. **AI Model Integration**: Local LLM support
3. **Plugin System**: Third-party extensions
4. **Cloud Sync**: Optional cloud backup and sync
5. **Multi-Device**: Cross-device coordination

### Research Areas

- Edge AI models for offline capabilities
- Federated learning for privacy-preserving model improvement
- Advanced MCP tool marketplace
- Integration with smart home systems

---

For questions or suggestions about the architecture, please open an issue or discussion on GitHub.
