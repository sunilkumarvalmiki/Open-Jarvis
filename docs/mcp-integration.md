# MCP Integration Guide

This document provides a comprehensive guide to integrating and using Model Context Protocol (MCP) with Open-Jarvis.

## Table of Contents

- [What is MCP?](#what-is-mcp)
- [Architecture](#architecture)
- [Available Tools](#available-tools)
- [Configuration](#configuration)
- [Custom Tool Development](#custom-tool-development)
- [Security Considerations](#security-considerations)
- [Troubleshooting](#troubleshooting)

## What is MCP?

The Model Context Protocol (MCP) is an open protocol that standardizes how applications provide context to Large Language Models (LLMs). It enables:

- **Standardized Tool Integration**: Uniform interface for connecting tools and services
- **Extensibility**: Easy addition of new capabilities without modifying core code
- **Security**: Controlled access to system resources and external services
- **Interoperability**: Works with multiple AI models and platforms

### Key Concepts

- **MCP Server**: Provides tools and resources to clients
- **MCP Client**: Consumes tools and resources from servers (Open-Jarvis acts as a client)
- **Tools**: Callable functions exposed by MCP servers
- **Resources**: Read-only data sources (files, databases, APIs)
- **Prompts**: Reusable prompt templates

## Architecture

### MCP Integration in Open-Jarvis

```
┌─────────────────────────────────────────────────────────┐
│                    Open-Jarvis Core                     │
│                                                         │
│  ┌───────────────────────────────────────────────┐    │
│  │           MCP Client Manager                  │    │
│  │  ┌────────────┐  ┌─────────────┐             │    │
│  │  │   Client   │  │    Tools    │             │    │
│  │  │   Pool     │  │   Registry  │             │    │
│  │  └────────────┘  └─────────────┘             │    │
│  └───────────────────────────────────────────────┘    │
│                         │                              │
└─────────────────────────┼──────────────────────────────┘
                          │
              ┌───────────┼───────────┐
              │           │           │
    ┌─────────▼──┐  ┌─────▼─────┐  ┌─▼─────────┐
    │   GitHub   │  │   Files   │  │  Custom   │
    │MCP Server  │  │MCP Server │  │MCP Server │
    └────────────┘  └───────────┘  └───────────┘
```

### Communication Flow

```
User Command
    │
    ▼
Command Handler
    │
    ▼
MCP Client Manager
    │
    ├─► Select Tool
    │   │
    │   ▼
    │   Prepare Parameters
    │   │
    │   ▼
    │   Validate Permissions
    │
    ▼
MCP Server Communication (JSON-RPC)
    │
    ├─► Request: { tool, params }
    │   │
    │   ▼
    │   Tool Execution
    │   │
    │   ▼
    │   Response: { result }
    │
    ▼
Process Result
    │
    ▼
Return to User
```

## Available Tools

### Built-in MCP Tools

#### 1. GitHub Operations

**Tools Provided**:
- `github_list_repos`: List user repositories
- `github_create_issue`: Create issues
- `github_list_prs`: List pull requests
- `github_search_code`: Search code across repositories

**Example Usage**:
```rust
// List repositories
let result = mcp_client.call_tool("github_list_repos", json!({
    "username": "sunilkumarvalmiki",
    "per_page": 10
})).await?;

// Create an issue
let result = mcp_client.call_tool("github_create_issue", json!({
    "repo": "Open-Jarvis",
    "title": "New feature request",
    "body": "Description of the feature"
})).await?;
```

#### 2. File System Operations

**Tools Provided**:
- `fs_read_file`: Read file contents
- `fs_write_file`: Write to files
- `fs_list_directory`: List directory contents
- `fs_search_files`: Search for files

**Example Usage**:
```rust
// Read file
let result = mcp_client.call_tool("fs_read_file", json!({
    "path": "/home/user/document.txt"
})).await?;

// List directory
let result = mcp_client.call_tool("fs_list_directory", json!({
    "path": "/home/user/Documents",
    "recursive": false
})).await?;
```

#### 3. Database Operations

**Tools Provided**:
- `db_query`: Execute SQL queries
- `db_schema`: Get database schema
- `db_tables`: List tables

**Example Usage**:
```rust
// Query database
let result = mcp_client.call_tool("db_query", json!({
    "connection": "sqlite:./data.db",
    "query": "SELECT * FROM tasks WHERE status = 'pending'"
})).await?;
```

### Third-Party MCP Tools

Open-Jarvis can connect to any MCP-compatible server:

- **Slack MCP Server**: Slack integration
- **Google Drive MCP Server**: Drive file access
- **Jira MCP Server**: Project management
- **Custom Servers**: User-defined tools

## Configuration

### MCP Configuration File

Create `mcp-config.json` in the application config directory:

```json
{
  "servers": {
    "github": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      },
      "enabled": true
    },
    "filesystem": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user"],
      "enabled": true
    },
    "custom": {
      "type": "http",
      "url": "http://localhost:3000/mcp",
      "headers": {
        "Authorization": "Bearer ${API_TOKEN}"
      },
      "enabled": false
    }
  },
  "security": {
    "allowFileSystemAccess": true,
    "allowedPaths": [
      "/home/user/Documents",
      "/home/user/Downloads"
    ],
    "requireConfirmation": true,
    "confirmationThreshold": "medium"
  }
}
```

### Environment Variables

Set environment variables for sensitive data:

```bash
# GitHub
export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxx"

# Custom API
export API_TOKEN="your_api_token_here"

# MCP Configuration
export MCP_CONFIG_PATH="/path/to/mcp-config.json"
```

### Configuration in Rust

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct McpConfig {
    pub servers: HashMap<String, ServerConfig>,
    pub security: SecurityConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub r#type: ServerType,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    Stdio,
    Http,
    WebSocket,
}
```

## Custom Tool Development

### Creating a Custom MCP Server

#### 1. Set Up Project

```bash
mkdir my-custom-mcp-server
cd my-custom-mcp-server
npm init -y
npm install @modelcontextprotocol/sdk
```

#### 2. Implement Server

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";

const server = new Server({
  name: "my-custom-server",
  version: "1.0.0",
});

// Define tools
server.setRequestHandler("tools/list", async () => {
  return {
    tools: [
      {
        name: "my_custom_tool",
        description: "Does something useful",
        inputSchema: {
          type: "object",
          properties: {
            param1: { type: "string" },
            param2: { type: "number" }
          },
          required: ["param1"]
        }
      }
    ]
  };
});

// Implement tool
server.setRequestHandler("tools/call", async (request) => {
  if (request.params.name === "my_custom_tool") {
    const { param1, param2 } = request.params.arguments;
    // Tool logic here
    return {
      content: [
        { type: "text", text: `Processed ${param1} with ${param2}` }
      ]
    };
  }
  throw new Error("Unknown tool");
});

// Start server
const transport = new StdioServerTransport();
await server.connect(transport);
```

#### 3. Register in Open-Jarvis

Add to `mcp-config.json`:

```json
{
  "servers": {
    "my-custom-server": {
      "type": "stdio",
      "command": "node",
      "args": ["/path/to/my-custom-mcp-server/index.js"],
      "enabled": true
    }
  }
}
```

### MCP Client Implementation in Open-Jarvis

```rust
// src-tauri/src/mcp/client.rs

use serde_json::Value;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};

pub struct McpClient {
    process: Child,
    server_name: String,
}

impl McpClient {
    pub async fn new(config: &ServerConfig) -> Result<Self, String> {
        let mut command = Command::new(&config.command);
        command
            .args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Set environment variables
        if let Some(env) = &config.env {
            for (key, value) in env {
                command.env(key, value);
            }
        }

        let process = command.spawn().map_err(|e| e.to_string())?;

        Ok(Self {
            process,
            server_name: config.name.clone(),
        })
    }

    pub async fn call_tool(
        &mut self,
        tool_name: &str,
        params: Value,
    ) -> Result<Value, String> {
        // JSON-RPC request
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": params
            }
        });

        // Send request
        let stdin = self.process.stdin.as_mut().unwrap();
        let request_str = serde_json::to_string(&request).unwrap();
        stdin.write_all(request_str.as_bytes()).await.map_err(|e| e.to_string())?;
        stdin.write_all(b"\n").await.map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())?;

        // Read response
        let stdout = self.process.stdout.as_mut().unwrap();
        let mut reader = BufReader::new(stdout);
        let mut response_line = String::new();
        reader.read_line(&mut response_line).await.map_err(|e| e.to_string())?;

        // Parse response
        let response: Value = serde_json::from_str(&response_line).map_err(|e| e.to_string())?;
        
        if let Some(result) = response.get("result") {
            Ok(result.clone())
        } else if let Some(error) = response.get("error") {
            Err(error.to_string())
        } else {
            Err("Invalid response".to_string())
        }
    }
}
```

## Security Considerations

### Permission Model

1. **User Confirmation**: Sensitive operations require explicit approval
2. **Path Restrictions**: File access limited to allowed directories
3. **Credential Isolation**: API keys stored securely, not in code
4. **Audit Logging**: All MCP operations logged for review

### Security Best Practices

1. **Validate Inputs**: Always validate tool parameters
2. **Sanitize Outputs**: Clean data returned from tools
3. **Limit Scope**: Grant minimal necessary permissions
4. **Review Tools**: Audit third-party MCP servers before use
5. **Update Regularly**: Keep MCP servers and clients updated

### Example: Secure File Access

```rust
pub fn validate_file_access(path: &Path, config: &SecurityConfig) -> Result<(), String> {
    // Check if path is in allowed list
    let canonical = path.canonicalize().map_err(|e| e.to_string())?;
    
    for allowed_path in &config.allowed_paths {
        if canonical.starts_with(allowed_path) {
            return Ok(());
        }
    }
    
    Err(format!("Access denied: {:?} not in allowed paths", path))
}
```

## Troubleshooting

### Common Issues

#### 1. Server Not Starting

**Symptom**: MCP server process fails to start

**Solutions**:
- Check if command exists: `which npx` or `which node`
- Verify server package is installed
- Check environment variables are set
- Review server logs in stderr

#### 2. Permission Denied

**Symptom**: Tool calls return permission errors

**Solutions**:
- Check `allowed_paths` in config
- Verify user has OS-level permissions
- Review security settings

#### 3. Timeout Errors

**Symptom**: Tool calls timeout

**Solutions**:
- Increase timeout in configuration
- Check network connectivity (for HTTP servers)
- Verify server is responding

#### 4. Invalid Response

**Symptom**: Cannot parse server response

**Solutions**:
- Verify server is MCP-compatible
- Check JSON-RPC format
- Review server implementation

### Debug Mode

Enable debug logging:

```rust
env_logger::init();

log::debug!("MCP Request: {:?}", request);
log::debug!("MCP Response: {:?}", response);
```

### Testing MCP Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_client_connection() {
        let config = ServerConfig {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            // ... other config
        };
        
        let client = McpClient::new(&config).await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_tool_call() {
        let mut client = create_test_client().await;
        let result = client.call_tool("test_tool", json!({})).await;
        assert!(result.is_ok());
    }
}
```

## Resources

### Official Documentation

- [Model Context Protocol Spec](https://modelcontextprotocol.io/)
- [MCP SDK Documentation](https://github.com/modelcontextprotocol/sdk)
- [Official MCP Servers](https://github.com/modelcontextprotocol/servers)

### Community Resources

- [MCP Examples](https://github.com/modelcontextprotocol/examples)
- [Community Servers Registry](https://mcp-servers.com/)
- Open-Jarvis Discord: Discussion and support

### Related Projects

- [ai-context-manager](https://github.com/sunilkumarvalmiki/ai-context-manager) - RAG integration
- [polynote](https://github.com/sunilkumarvalmiki/polynote) - Knowledge management

---

For questions or issues with MCP integration, please open an issue on GitHub or join our community discussions.
