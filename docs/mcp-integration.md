# MCP Integration Guide

This guide explains how to integrate the Model Context Protocol (MCP) with Open-Jarvis, enabling powerful extensibility and tool integration.

## Table of Contents

1. [What is MCP?](#what-is-mcp)
2. [MCP in Open-Jarvis](#mcp-in-open-jarvis)
3. [Available Tools](#available-tools)
4. [Configuration](#configuration)
5. [Custom Tool Development](#custom-tool-development)
6. [Security Considerations](#security-considerations)
7. [Examples](#examples)

## What is MCP?

The Model Context Protocol (MCP) is an open protocol that enables seamless integration between AI applications and external data sources and tools. It provides:

- **Standardized Communication**: JSON-RPC based protocol
- **Tool Discovery**: Automatic discovery of available tools
- **Bidirectional Communication**: Request/response and streaming support
- **Security**: Built-in authentication and authorization
- **Extensibility**: Easy to add new tools and capabilities

### Key Concepts

- **MCP Server**: Provides tools and resources (e.g., GitHub MCP server)
- **MCP Client**: Consumes tools and resources (Open-Jarvis)
- **Tools**: Callable functions exposed by MCP servers
- **Resources**: Data sources accessible via MCP
- **Prompts**: Reusable prompt templates

## MCP in Open-Jarvis

Open-Jarvis will use MCP to:

1. **Extend Functionality**: Add new capabilities without modifying core code
2. **Connect Services**: Integrate with GitHub, databases, file systems, etc.
3. **Enable AI Features**: Connect to AI models and services
4. **Cross-Project Communication**: Communicate with ai-context-manager and polynote

### Architecture

```
┌─────────────────────────────────────────────────┐
│              Open-Jarvis UI                     │
└──────────────────────┬──────────────────────────┘
                       │
                       │ Tauri IPC
                       │
┌──────────────────────▼──────────────────────────┐
│           MCP Client (Rust)                     │
│  ┌──────────────────────────────────────────┐  │
│  │  Connection Manager                      │  │
│  │  - Server connections                    │  │
│  │  - Health monitoring                     │  │
│  │  - Auto-reconnect                        │  │
│  └──────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────┐  │
│  │  Tool Manager                            │  │
│  │  - Tool discovery                        │  │
│  │  - Tool execution                        │  │
│  │  - Result handling                       │  │
│  └──────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────┐  │
│  │  Configuration                           │  │
│  │  - Server registry                       │  │
│  │  - Authentication                        │  │
│  │  - Permissions                           │  │
│  └──────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────┘
                       │
                       │ JSON-RPC (stdio/HTTP/WebSocket)
                       │
         ┌─────────────┼─────────────┐
         │             │             │
    ┌────▼────┐   ┌───▼────┐   ┌───▼────┐
    │ GitHub  │   │  File  │   │ Custom │
    │   MCP   │   │ System │   │  Tool  │
    │ Server  │   │  MCP   │   │  MCP   │
    └─────────┘   └────────┘   └────────┘
```

## Available Tools

### Planned MCP Integrations

#### 1. GitHub MCP Server

**Capabilities**:
- Repository management
- Issue tracking
- Pull request operations
- Code search
- Workflow automation

**Example Tools**:
```rust
// List repositories
mcp.call_tool("github", "list_repositories", {
    "owner": "sunilkumarvalmiki",
    "visibility": "public"
})

// Create issue
mcp.call_tool("github", "create_issue", {
    "repo": "Open-Jarvis",
    "title": "Bug: Application crashes on startup",
    "body": "Detailed description...",
    "labels": ["bug", "priority-high"]
})
```

#### 2. File System MCP Server

**Capabilities**:
- File operations (read, write, delete)
- Directory navigation
- File search
- Permission management

**Example Tools**:
```rust
// Search files
mcp.call_tool("filesystem", "search", {
    "path": "/home/user/documents",
    "pattern": "*.pdf",
    "recursive": true
})

// Read file
mcp.call_tool("filesystem", "read_file", {
    "path": "/home/user/config.json"
})
```

#### 3. AI Context Manager MCP

**Capabilities**:
- RAG queries
- Vector search
- Context retrieval
- Knowledge base operations

**Example Tools**:
```rust
// Query knowledge base
mcp.call_tool("ai-context-manager", "rag_query", {
    "query": "How do I configure Open-Jarvis?",
    "top_k": 5
})

// Add to knowledge base
mcp.call_tool("ai-context-manager", "add_context", {
    "content": "Documentation content...",
    "metadata": {"source": "README.md"}
})
```

#### 4. Polynote MCP

**Capabilities**:
- Note creation
- Note search
- Tag management
- Note organization

**Example Tools**:
```rust
// Create note
mcp.call_tool("polynote", "create_note", {
    "title": "Meeting Notes",
    "content": "Discussion points...",
    "tags": ["meeting", "2024"]
})

// Search notes
mcp.call_tool("polynote", "search_notes", {
    "query": "machine learning",
    "tags": ["research"]
})
```

## Configuration

### MCP Configuration File

Location: `~/.config/jarvis/mcp-config.json` (Linux/macOS) or `%APPDATA%\jarvis\mcp-config.json` (Windows)

```json
{
  "version": "1.0",
  "servers": [
    {
      "name": "github",
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      },
      "enabled": true
    },
    {
      "name": "filesystem",
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user"],
      "enabled": true
    },
    {
      "name": "ai-context-manager",
      "transport": "http",
      "url": "http://localhost:8000/mcp",
      "auth": {
        "type": "bearer",
        "token": "${AI_CONTEXT_TOKEN}"
      },
      "enabled": true
    },
    {
      "name": "polynote",
      "transport": "http",
      "url": "http://localhost:8001/mcp",
      "enabled": true
    }
  ],
  "security": {
    "allow_dangerous_tools": false,
    "require_confirmation": true,
    "allowed_directories": [
      "/home/user/documents",
      "/home/user/projects"
    ]
  }
}
```

### Environment Variables

```bash
# GitHub authentication
export GITHUB_TOKEN="ghp_your_token_here"

# AI Context Manager authentication
export AI_CONTEXT_TOKEN="your_ai_context_token"

# MCP configuration path override
export MCP_CONFIG_PATH="/custom/path/to/mcp-config.json"
```

## Custom Tool Development

### Creating a Custom MCP Server

#### 1. Define Tool Interface

```rust
// src-tauri/src/mcp/tools/custom.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomToolInput {
    pub param1: String,
    pub param2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomToolOutput {
    pub result: String,
    pub status: String,
}

pub async fn execute_custom_tool(input: CustomToolInput) -> Result<CustomToolOutput, String> {
    // Tool implementation
    Ok(CustomToolOutput {
        result: format!("Processed {} with {}", input.param1, input.param2),
        status: "success".to_string(),
    })
}
```

#### 2. Register Tool with MCP Client

```rust
// src-tauri/src/mcp/client.rs

impl McpClient {
    pub fn register_custom_tools(&mut self) {
        self.register_tool("custom_tool", |input| {
            Box::pin(async move {
                let parsed: CustomToolInput = serde_json::from_value(input)?;
                let result = execute_custom_tool(parsed).await?;
                Ok(serde_json::to_value(result)?)
            })
        });
    }
}
```

#### 3. Call Custom Tool

```rust
// From Tauri command
#[tauri::command]
async fn call_custom_tool(
    state: State<'_, McpClient>,
    param1: String,
    param2: i32
) -> Result<String, String> {
    let result = state.call_tool("custom_tool", CustomToolInput {
        param1,
        param2,
    }).await?;
    
    Ok(result.result)
}
```

### MCP Server Example (Python)

```python
# custom_mcp_server.py

from mcp.server import Server
from mcp.types import Tool, TextContent

server = Server("custom-server")

@server.list_tools()
async def list_tools() -> list[Tool]:
    return [
        Tool(
            name="custom_tool",
            description="A custom tool for demonstration",
            inputSchema={
                "type": "object",
                "properties": {
                    "param1": {"type": "string"},
                    "param2": {"type": "integer"}
                },
                "required": ["param1", "param2"]
            }
        )
    ]

@server.call_tool()
async def call_tool(name: str, arguments: dict) -> list[TextContent]:
    if name == "custom_tool":
        result = f"Processed {arguments['param1']} with {arguments['param2']}"
        return [TextContent(type="text", text=result)]
    
    raise ValueError(f"Unknown tool: {name}")

if __name__ == "__main__":
    server.run()
```

## Security Considerations

### Authentication

- **Token-based Auth**: Use environment variables for sensitive tokens
- **OAuth Integration**: Support OAuth flows for services like GitHub
- **Key Rotation**: Regularly rotate authentication keys

### Authorization

- **Permission Model**: Explicitly define what each tool can access
- **User Confirmation**: Require user approval for sensitive operations
- **Audit Logging**: Log all MCP tool executions

### Sandboxing

- **Directory Restrictions**: Limit file system access to allowed directories
- **Network Policies**: Control which URLs/IPs MCP servers can access
- **Resource Limits**: Prevent DoS through resource exhaustion

### Best Practices

1. **Validate Inputs**: Always validate tool inputs before execution
2. **Sanitize Outputs**: Clean outputs before displaying to users
3. **Error Handling**: Never expose sensitive information in errors
4. **Rate Limiting**: Implement rate limits for tool calls
5. **Monitoring**: Monitor MCP usage for anomalies

### Configuration Security

```json
{
  "security": {
    "allow_dangerous_tools": false,
    "require_confirmation": true,
    "allowed_directories": ["/safe/path"],
    "blocked_tools": ["delete_all", "format_disk"],
    "max_tool_calls_per_minute": 60,
    "enable_audit_log": true,
    "audit_log_path": "~/.config/jarvis/audit.log"
  }
}
```

## Examples

### Example 1: GitHub Issue Management

```javascript
// Frontend: Create GitHub issue from UI
async function createGitHubIssue(title, description) {
  try {
    const result = await invoke('mcp_call_tool', {
      server: 'github',
      tool: 'create_issue',
      args: {
        repo: 'Open-Jarvis',
        title: title,
        body: description,
        labels: ['user-reported']
      }
    });
    
    console.log('Issue created:', result);
  } catch (error) {
    console.error('Failed to create issue:', error);
  }
}
```

### Example 2: Knowledge Base Query

```rust
// Backend: Query AI context manager
#[tauri::command]
async fn query_knowledge_base(
    mcp: State<'_, McpClient>,
    query: String
) -> Result<Vec<String>, String> {
    let result = mcp.call_tool("ai-context-manager", "rag_query", json!({
        "query": query,
        "top_k": 5
    })).await?;
    
    // Parse and return results
    let contexts: Vec<String> = serde_json::from_value(result)?;
    Ok(contexts)
}
```

### Example 3: File Organization with MCP

```rust
// Backend: Organize files using MCP filesystem server
#[tauri::command]
async fn organize_with_mcp(
    mcp: State<'_, McpClient>,
    source_dir: String
) -> Result<(), String> {
    // List files
    let files = mcp.call_tool("filesystem", "list_files", json!({
        "path": source_dir,
        "recursive": false
    })).await?;
    
    // Organize by type
    for file in files.as_array().unwrap() {
        let path = file["path"].as_str().unwrap();
        let extension = Path::new(path).extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let dest_dir = match extension {
            "pdf" | "doc" | "docx" => "Documents",
            "jpg" | "png" | "gif" => "Pictures",
            "mp3" | "wav" | "flac" => "Music",
            _ => continue
        };
        
        // Move file
        mcp.call_tool("filesystem", "move_file", json!({
            "source": path,
            "destination": format!("{}/{}", dest_dir, Path::new(path).file_name().unwrap().to_str().unwrap())
        })).await?;
    }
    
    Ok(())
}
```

## Roadmap

- [ ] **Phase 1**: MCP client infrastructure
  - Connection management
  - Tool discovery
  - Basic tool execution

- [ ] **Phase 2**: Core integrations
  - GitHub MCP server
  - File system MCP server
  - Configuration UI

- [ ] **Phase 3**: Cross-project integration
  - ai-context-manager MCP
  - polynote MCP
  - Bidirectional communication

- [ ] **Phase 4**: Advanced features
  - Custom tool development UI
  - Tool marketplace
  - Streaming support
  - WebSocket transport

## Resources

- [MCP Specification](https://modelcontextprotocol.io)
- [MCP SDK for Rust](https://github.com/modelcontextprotocol/rust-sdk)
- [Official MCP Servers](https://github.com/modelcontextprotocol/servers)
- [MCP TypeScript SDK](https://github.com/modelcontextprotocol/typescript-sdk)

## Support

For MCP-related questions:
- Open an issue with the `mcp` label
- Check existing MCP issues
- Join the discussion in GitHub Discussions

---

**Note**: MCP integration is currently under development. This guide describes the planned architecture and may be updated as implementation progresses.
