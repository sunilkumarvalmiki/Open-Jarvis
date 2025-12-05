use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP Client for tool integration
pub struct McpClient {
    servers: HashMap<String, Box<dyn McpServer>>,
}

impl McpClient {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    pub fn add_server<S: McpServer + 'static>(mut self, name: &str, server: S) -> Self {
        self.servers.insert(name.to_string(), Box::new(server));
        self
    }

    pub async fn call_tool(
        &self,
        server: &str,
        tool: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, McpError> {
        let server = self
            .servers
            .get(server)
            .ok_or(McpError::ServerNotFound(server.to_string()))?;

        server.call_tool(tool, params).await
    }
}

impl Default for McpClient {
    fn default() -> Self {
        Self::new()
    }
}

pub trait McpServer: Send + Sync {
    fn name(&self) -> &str;
    fn tools(&self) -> Vec<ToolDefinition>;
    fn call_tool(
        &self,
        tool: &str,
        params: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<serde_json::Value, McpError>> + Send;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Server not found: {0}")]
    ServerNotFound(String),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
}
