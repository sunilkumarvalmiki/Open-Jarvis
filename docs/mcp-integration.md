# MCP Integration Guide

This guide explains how to integrate tools and services into Open-Jarvis using the Model Context Protocol (MCP).

## What is MCP?

The Model Context Protocol (MCP) is a standardized way for AI assistants to interact with external tools and services. It provides:

- **Standardized Interface**: Consistent API for all tools
- **Type Safety**: Strongly typed tool definitions
- **Error Handling**: Structured error responses
- **Extensibility**: Easy to add new tools

## Architecture

```
┌─────────────────────────────────────────────────┐
│              McpClient                          │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌──────────────┐  ┌──────────────┐             │
│  │   GitHub     │  │  Filesystem  │  ...        │
│  │   Server     │  │   Server     │             │
│  └──────────────┘  └──────────────┘             │
│        │                  │                     │
│  ┌─────▼──────┐     ┌─────▼──────┐              │
│  │   Tools    │     │   Tools    │              │
│  │ - create   │     │ - read     │              │
│  │ - search   │     │ - write    │              │
│  │ - list     │     │ - list     │              │
│  └────────────┘     └────────────┘              │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Creating a New MCP Server

### Step 1: Implement the McpServer Trait

```rust
use crate::mcp::client::{McpError, McpServer, ToolDefinition};

pub struct MyCustomServer {
    name: String,
    // Add any configuration or state needed
}

impl MyCustomServer {
    pub fn new() -> Self {
        Self {
            name: "my_custom_server".to_string(),
        }
    }
}

impl McpServer for MyCustomServer {
    fn name(&self) -> &str {
        &self.name
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "my_tool".to_string(),
                description: "Does something useful".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "param1": {
                            "type": "string",
                            "description": "First parameter"
                        },
                        "param2": {
                            "type": "number",
                            "description": "Second parameter"
                        }
                    },
                    "required": ["param1"]
                }),
            },
        ]
    }

    async fn call_tool(
        &self,
        tool: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, McpError> {
        match tool {
            "my_tool" => {
                // Extract parameters
                let param1 = params["param1"]
                    .as_str()
                    .ok_or_else(|| McpError::ExecutionError("Missing param1".to_string()))?;
                
                // Execute tool logic
                let result = self.execute_my_tool(param1).await?;
                
                // Return result
                Ok(serde_json::json!({
                    "success": true,
                    "data": result
                }))
            }
            _ => Err(McpError::ToolNotFound(tool.to_string())),
        }
    }
}

impl MyCustomServer {
    async fn execute_my_tool(&self, param: &str) -> Result<String, McpError> {
        // Implement your tool logic here
        Ok(format!("Processed: {}", param))
    }
}
```

### Step 2: Register the Server

In your initialization code (e.g., `main.rs` or a setup function):

```rust
use crate::mcp::McpClient;
use crate::mcp::server::MyCustomServer;

fn setup_mcp() -> McpClient {
    McpClient::new()
        .add_server(MyCustomServer::new())
        .add_server(FilesystemMcpServer::new())
        // Add more servers as needed
}
```

### Step 3: Use the MCP Client

```rust
let mcp_client = setup_mcp();

// Call a tool
let result = mcp_client
    .call_tool(
        "my_custom_server",
        "my_tool",
        serde_json::json!({
            "param1": "test value"
        })
    )
    .await?;

println!("Result: {}", result);
```

## Example: GitHub MCP Server

Here's a complete example of a GitHub MCP server:

```rust
use crate::mcp::client::{McpError, McpServer, ToolDefinition};
use serde_json::json;

pub struct GithubMcpServer {
    name: String,
    token: Option<String>,
}

impl GithubMcpServer {
    pub fn new() -> Self {
        Self {
            name: "github".to_string(),
            token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }
}

impl McpServer for GithubMcpServer {
    fn name(&self) -> &str {
        &self.name
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "create_issue".to_string(),
                description: "Create a GitHub issue".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "repo": {
                            "type": "string",
                            "description": "Repository (owner/repo)"
                        },
                        "title": {
                            "type": "string",
                            "description": "Issue title"
                        },
                        "body": {
                            "type": "string",
                            "description": "Issue body"
                        }
                    },
                    "required": ["repo", "title"]
                }),
            },
            ToolDefinition {
                name: "search_code".to_string(),
                description: "Search code in GitHub repositories".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "language": {
                            "type": "string",
                            "description": "Programming language filter"
                        }
                    },
                    "required": ["query"]
                }),
            },
        ]
    }

    async fn call_tool(
        &self,
        tool: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, McpError> {
        match tool {
            "create_issue" => self.create_issue(params).await,
            "search_code" => self.search_code(params).await,
            _ => Err(McpError::ToolNotFound(tool.to_string())),
        }
    }
}

impl GithubMcpServer {
    async fn create_issue(&self, params: serde_json::Value) -> Result<serde_json::Value, McpError> {
        let repo = params["repo"]
            .as_str()
            .ok_or_else(|| McpError::ExecutionError("Missing repo".to_string()))?;
        let title = params["title"]
            .as_str()
            .ok_or_else(|| McpError::ExecutionError("Missing title".to_string()))?;
        let body = params["body"].as_str().unwrap_or("");

        // Here you would make the actual API call to GitHub
        // For now, we'll return a mock response
        Ok(json!({
            "success": true,
            "issue_url": format!("https://github.com/{}/issues/1", repo),
            "issue_number": 1
        }))
    }

    async fn search_code(&self, params: serde_json::Value) -> Result<serde_json::Value, McpError> {
        let query = params["query"]
            .as_str()
            .ok_or_else(|| McpError::ExecutionError("Missing query".to_string()))?;

        // Here you would make the actual API call to GitHub
        // For now, we'll return a mock response
        Ok(json!({
            "success": true,
            "results": [],
            "total_count": 0
        }))
    }
}
```

## Tool Definition Schema

Tool parameters follow JSON Schema format:

```json
{
  "type": "object",
  "properties": {
    "param_name": {
      "type": "string|number|boolean|array|object",
      "description": "Parameter description",
      "enum": ["option1", "option2"],  // Optional: restrict values
      "default": "default_value"        // Optional: default value
    }
  },
  "required": ["param_name"]  // Required parameters
}
```

### Supported Types

- `string`: Text values
- `number`: Numeric values (int or float)
- `boolean`: true/false
- `array`: List of values
- `object`: Nested object

## Error Handling

MCP defines three error types:

```rust
pub enum McpError {
    ServerNotFound(String),   // Server doesn't exist
    ToolNotFound(String),     // Tool doesn't exist on server
    ExecutionError(String),   // Error during tool execution
}
```

Always provide meaningful error messages:

```rust
// Good
Err(McpError::ExecutionError(
    "Failed to connect to GitHub API: timeout after 30s".to_string()
))

// Bad
Err(McpError::ExecutionError("Error".to_string()))
```

## Best Practices

### 1. Tool Naming

- Use snake_case for tool names
- Be descriptive: `create_github_issue` vs `create`
- Group related tools by prefix: `github_create_issue`, `github_search_code`

### 2. Parameter Validation

```rust
async fn call_tool(&self, tool: &str, params: serde_json::Value) 
    -> Result<serde_json::Value, McpError> 
{
    // Validate required parameters
    let param = params["required_param"]
        .as_str()
        .ok_or_else(|| McpError::ExecutionError(
            "Missing required parameter: required_param".to_string()
        ))?;

    // Validate parameter format
    if !is_valid_format(param) {
        return Err(McpError::ExecutionError(
            "Invalid parameter format".to_string()
        ));
    }

    // Continue with execution
    self.execute_tool(param).await
}
```

### 3. Async Operations

Always use async for I/O operations:

```rust
async fn call_api(&self) -> Result<String, McpError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.example.com/data")
        .send()
        .await
        .map_err(|e| McpError::ExecutionError(e.to_string()))?;

    let data = response
        .text()
        .await
        .map_err(|e| McpError::ExecutionError(e.to_string()))?;

    Ok(data)
}
```

### 4. Configuration

Store credentials and configuration securely:

```rust
pub struct MyServer {
    name: String,
    api_key: Option<String>,
}

impl MyServer {
    pub fn new() -> Self {
        Self {
            name: "my_server".to_string(),
            // Read from environment or config file
            api_key: std::env::var("MY_API_KEY").ok(),
        }
    }

    fn check_auth(&self) -> Result<(), McpError> {
        if self.api_key.is_none() {
            return Err(McpError::ExecutionError(
                "API key not configured. Set MY_API_KEY environment variable".to_string()
            ));
        }
        Ok(())
    }
}
```

## Testing

Test your MCP server:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_execution() {
        let server = MyCustomServer::new();
        let result = server
            .call_tool(
                "my_tool",
                serde_json::json!({
                    "param1": "test"
                })
            )
            .await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["success"], true);
    }

    #[tokio::test]
    async fn test_missing_tool() {
        let server = MyCustomServer::new();
        let result = server
            .call_tool("nonexistent_tool", serde_json::json!({}))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            McpError::ToolNotFound(_) => (),
            _ => panic!("Expected ToolNotFound error"),
        }
    }
}
```

## Available Servers

### Filesystem Server

Tools for file system operations:
- `read_file`: Read file contents
- `write_file`: Write to file
- `list_directory`: List directory contents
- `create_directory`: Create a directory

### Future Servers

Planned MCP servers:
- **GitHub**: Repository operations, issues, PRs
- **Jira**: Issue tracking
- **Slack**: Team communication
- **Calendar**: Schedule management
- **Email**: Email integration
- **Browser**: Web automation

## Contributing

To contribute a new MCP server:

1. Create the server implementation in `src-tauri/src/mcp/server/`
2. Add comprehensive tool definitions
3. Include error handling
4. Write tests
5. Update this documentation
6. Submit a PR

See [CONTRIBUTING.md](../CONTRIBUTING.md) for more details.
