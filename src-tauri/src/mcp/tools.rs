//! MCP Tools
//!
//! This module defines and manages MCP tools.

use serde::{Deserialize, Serialize};

/// Represents an MCP tool
///
/// Tools are callable functions exposed by MCP servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema (JSON Schema)
    pub input_schema: serde_json::Value,
}

impl McpTool {
    /// Create a new MCP tool
    ///
    /// # Arguments
    ///
    /// * `name` - Tool name
    /// * `description` - Tool description
    /// * `input_schema` - JSON Schema for tool inputs
    pub fn new(name: String, description: String, input_schema: serde_json::Value) -> Self {
        Self {
            name,
            description,
            input_schema,
        }
    }
}

/// Tool execution result
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether the tool execution was successful
    pub success: bool,
    /// Result data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolResult {
    /// Create a successful tool result
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create a failed tool result
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
