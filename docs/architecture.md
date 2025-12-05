# Open-Jarvis Architecture

This document provides a comprehensive overview of the Open-Jarvis architecture, system components, data flow, and integration points.

## Table of Contents

1. [System Overview](#system-overview)
2. [Component Architecture](#component-architecture)
3. [Data Flow](#data-flow)
4. [MCP Integration Points](#mcp-integration-points)
5. [Security Model](#security-model)
6. [Cross-Project Integration](#cross-project-integration)
7. [Future Architecture](#future-architecture)

## System Overview

Open-Jarvis is built as a desktop application using the Tauri framework, which combines a Rust backend with a web-based frontend. This architecture provides:

- **Performance**: Rust's zero-cost abstractions and memory safety
- **Security**: Tauri's security-first approach with minimal attack surface
- **Cross-platform**: Single codebase for Windows, macOS, and Linux
- **Small footprint**: Significantly smaller than Electron-based alternatives

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface                          │
│                     (HTML/CSS/JavaScript)                       │
│  ┌─────────────┬─────────────┬──────────────────────────────┐  │
│  │   Action    │   Status    │      Configuration UI        │  │
│  │   Cards     │  Display    │                              │  │
│  └─────────────┴─────────────┴──────────────────────────────┘  │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               │ Tauri IPC (JSON-RPC)
                               │
┌──────────────────────────────▼──────────────────────────────────┐
│                         Tauri Runtime                           │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Command Handlers                      │  │
│  │  - open_browser()  - empty_recycle_bin()               │  │
│  │  - organize_files() - [future commands]                  │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                   Business Logic Layer                   │  │
│  │  ┌────────────┬──────────────┬─────────────────────────┐ │  │
│  │  │  System    │     File     │    MCP Integration      │ │  │
│  │  │  Commands  │   Manager    │    (Coming Soon)        │ │  │
│  │  └────────────┴──────────────┴─────────────────────────┘ │  │
│  └──────────────────────────────────────────────────────────┘  │
│                        Rust Backend                             │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               │ Platform APIs
                               │
┌──────────────────────────────▼──────────────────────────────────┐
│                      Operating System                           │
│  ┌──────────────┬──────────────┬────────────────────────────┐  │
│  │   Shell      │  File System │    Process Management      │  │
│  │   APIs       │              │                            │  │
│  └──────────────┴──────────────┴────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Component Architecture

### Frontend Layer

**Technology**: HTML, CSS, JavaScript

**Responsibilities**:
- User interface rendering
- Event handling
- Communication with Rust backend via Tauri IPC
- State management for UI components

**Key Files**:
- `src/index.html` - Main application HTML
- `src/style.css` - Application styling
- `src/main.js` - Frontend logic and IPC calls

### Tauri Runtime

**Technology**: Rust + Tauri framework

**Responsibilities**:
- Bridge between frontend and backend
- Window management
- Security boundary enforcement
- IPC message routing

**Key Features**:
- Type-safe communication using `#[tauri::command]` macro
- Async/await support for non-blocking operations
- Scoped shell execution for security

### Backend Layer

**Technology**: Rust

**Responsibilities**:
- Business logic implementation
- System integration
- File operations
- Command execution
- Future: MCP protocol handling

**Current Modules**:

#### System Commands Module
```rust
// Functions for system-level operations
- open_browser() - Opens URLs in default browser
- empty_recycle_bin() - Clears system trash/recycle bin
- organize_files() - Organizes files by type
```

**Platforms Supported**:
- Windows: PowerShell commands
- macOS: Unix commands
- Linux: Unix commands

#### File Manager Module
```rust
// File system operations
- organize_impl() - Core file organization logic
- File type detection
- Directory creation
- File movement operations
```

#### MCP Module (Planned)
```rust
// Model Context Protocol integration
- mcp/client.rs - MCP client implementation
- mcp/tools.rs - MCP tool definitions
- mcp/config.rs - MCP configuration
```

## Data Flow

### Command Execution Flow

```
User Action → Frontend Event → Tauri IPC → Command Handler → 
Business Logic → System API → Result → IPC Response → UI Update
```

### Detailed Example: File Organization

1. **User Interaction**
   - User clicks "Organize Downloads" button in UI
   
2. **Frontend Event**
   ```javascript
   document.getElementById('organize-files').addEventListener('click', async () => {
     await invoke('organize_files');
   });
   ```

3. **IPC Communication**
   - Tauri serializes the command to JSON-RPC
   - Message sent to Rust backend

4. **Command Handler**
   ```rust
   #[tauri::command]
   fn organize_files() -> Result<(), String> {
     // Async spawn to avoid blocking
     tauri::async_runtime::spawn(async move {
       if let Some(home) = dirs_next::download_dir() {
         organize_impl(&home).ok();
       }
     });
     Ok(())
   }
   ```

5. **Business Logic**
   - Retrieve downloads directory path
   - Create target directories (Documents, Music, Pictures)
   - Scan files in downloads
   - Move files based on extension

6. **System Interaction**
   - File system APIs (`fs::read_dir`, `fs::rename`)
   - Directory creation (`fs::create_dir_all`)

7. **Response**
   - Success/error propagated back through IPC
   - UI updated accordingly

## MCP Integration Points

### Planned MCP Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Open-Jarvis                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │              MCP Client                          │  │
│  │  - Connection management                         │  │
│  │  - Request/response handling                     │  │
│  │  - Tool discovery                                │  │
│  └────────────────────┬─────────────────────────────┘  │
└────────────────────────┼────────────────────────────────┘
                         │
                         │ JSON-RPC over stdio/HTTP
                         │
         ┌───────────────┼───────────────┐
         │               │               │
    ┌────▼────┐    ┌────▼────┐    ┌────▼────┐
    │ GitHub  │    │  File   │    │ Custom  │
    │  MCP    │    │ System  │    │  Tools  │
    │ Server  │    │  MCP    │    │   MCP   │
    └─────────┘    └─────────┘    └─────────┘
```

### Integration Capabilities (Planned)

1. **GitHub Integration**
   - Repository operations
   - Issue management
   - Pull request workflows

2. **File System Integration**
   - Enhanced file operations
   - Search capabilities
   - Version control integration

3. **Database Integration**
   - Query execution
   - Data retrieval
   - Schema inspection

4. **Custom Tool Integration**
   - User-defined tools
   - Plugin system
   - Extensible command set

## Security Model

### Security Principles

1. **Least Privilege**: Commands execute with user permissions only
2. **Explicit Consent**: No automatic execution without user action
3. **Scope Limitation**: Shell commands use Tauri's scoped execution
4. **Input Validation**: All user inputs are validated
5. **Memory Safety**: Rust's ownership system prevents common vulnerabilities

### Security Boundaries

```
┌─────────────────────────────────────────┐
│         Untrusted Zone                  │
│      (Frontend JavaScript)              │
│  - User input                           │
│  - UI interactions                      │
└──────────────────┬──────────────────────┘
                   │
                   │ Tauri IPC (Validated)
                   │
┌──────────────────▼──────────────────────┐
│          Trusted Zone                   │
│       (Rust Backend)                    │
│  - Type-safe command handlers           │
│  - Validated business logic             │
│  - Controlled system access             │
└──────────────────┬──────────────────────┘
                   │
                   │ OS APIs (Scoped)
                   │
┌──────────────────▼──────────────────────┐
│      Operating System                   │
└─────────────────────────────────────────┘
```

### Security Features

- **Command Allowlisting**: Only explicitly defined commands can be invoked
- **Type Safety**: Rust's type system prevents injection attacks
- **Async Execution**: Commands run in isolated async tasks
- **Error Handling**: All errors are caught and logged safely
- **No Eval**: No dynamic code execution in frontend or backend

## Cross-Project Integration

### Ecosystem Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    User Layer                           │
└───────────────────────┬─────────────────────────────────┘
                        │
                        │
┌───────────────────────▼─────────────────────────────────┐
│                   Open-Jarvis                           │
│              (Orchestration Layer)                      │
│  - User interface                                       │
│  - Command processing                                   │
│  - Cross-project coordination                           │
└────────┬─────────────────────────────┬──────────────────┘
         │                             │
         │ MCP                         │ MCP
         │                             │
┌────────▼──────────────┐    ┌────────▼─────────────────┐
│  ai-context-manager   │    │      polynote            │
│                       │    │                          │
│  - RAG system         │    │  - Knowledge base        │
│  - Vector search      │    │  - Note management       │
│  - Context retrieval  │    │  - Document storage      │
└───────────────────────┘    └──────────────────────────┘
```

### Integration Workflows

#### Knowledge Retrieval Workflow
1. User asks question via Open-Jarvis
2. Open-Jarvis sends query to ai-context-manager via MCP
3. ai-context-manager performs RAG search
4. Results returned and displayed in Open-Jarvis UI

#### Note Management Workflow
1. User requests note creation via Open-Jarvis
2. Open-Jarvis communicates with polynote via MCP
3. Note created and indexed
4. Confirmation returned to user

## Future Architecture

### Planned Enhancements

1. **Voice Interface**
   - Speech-to-text integration
   - Natural language processing
   - Voice command routing

2. **Plugin System**
   - Dynamic plugin loading
   - Plugin marketplace
   - User-developed extensions

3. **AI Model Integration**
   - Local LLM support
   - Cloud API integration
   - Model switching capability

4. **Advanced Automation**
   - Workflow builder
   - Scheduled tasks
   - Event-driven automation

### Scalability Considerations

- **Multi-threading**: Leverage Rust's concurrency for parallel operations
- **Caching**: Implement intelligent caching for frequently accessed data
- **Resource Management**: Monitor and limit resource usage
- **Modular Design**: Keep components loosely coupled for easy extension

## Technical Decisions

### Why Tauri?

- **Size**: ~600KB vs 50MB+ for Electron
- **Performance**: Native Rust backend
- **Security**: Smaller attack surface
- **Memory**: More efficient than Chromium-based alternatives

### Why Rust?

- **Safety**: Memory safety without garbage collection
- **Performance**: Zero-cost abstractions
- **Concurrency**: Fearless concurrency model
- **Ecosystem**: Rich ecosystem of libraries

### Why MCP?

- **Standardization**: Industry-standard protocol
- **Flexibility**: Works with any MCP-compatible server
- **Extensibility**: Easy to add new capabilities
- **Interoperability**: Cross-project communication

## Conclusion

Open-Jarvis's architecture is designed to be secure, performant, and extensible. The combination of Tauri, Rust, and MCP provides a solid foundation for building a powerful AI assistant that can grow and adapt to user needs while maintaining privacy and security.

For implementation details on MCP integration, see [MCP Integration Guide](mcp-integration.md).
