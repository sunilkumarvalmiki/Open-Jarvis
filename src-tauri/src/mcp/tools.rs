//! MCP Tool definitions
//!
//! This module contains reusable tool definitions that can be
//! used across different MCP servers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub server: String,
    pub tool: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub result: serde_json::Value,
}
