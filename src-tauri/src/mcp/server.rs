//! MCP Server implementations
//!
//! This module contains concrete implementations of MCP servers
//! that can be used with the MCP client.

#![allow(dead_code)]

use super::client::{McpServer, ToolDefinition};

/// Example filesystem MCP server
pub struct FilesystemMcpServer {
    name: String,
}

impl FilesystemMcpServer {
    pub fn new() -> Self {
        Self {
            name: "filesystem".to_string(),
        }
    }
}

impl Default for FilesystemMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer for FilesystemMcpServer {
    fn name(&self) -> &str {
        &self.name
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "read_file".to_string(),
                description: "Read contents of a file".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "list_directory".to_string(),
                description: "List contents of a directory".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the directory"
                        }
                    },
                    "required": ["path"]
                }),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_server_name() {
        let server = FilesystemMcpServer::new();
        assert_eq!(server.name(), "filesystem");
    }

    #[test]
    fn test_filesystem_server_tools() {
        let server = FilesystemMcpServer::new();
        let tools = server.tools();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0].name, "read_file");
        assert_eq!(tools[1].name, "list_directory");
    }
}
