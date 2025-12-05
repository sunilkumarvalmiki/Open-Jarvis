//! MCP tool definitions and utilities
//!
//! This module defines structures for representing MCP tools.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents an MCP tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Name of the tool
    pub name: String,

    /// Description of what the tool does
    pub description: String,

    /// JSON schema for input parameters
    pub input_schema: Value,

    /// Server this tool belongs to
    #[serde(skip)]
    pub server: String,
}

/// Represents a tool parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    /// Name of the parameter
    pub name: String,

    /// Type of the parameter
    #[serde(rename = "type")]
    pub param_type: ParameterType,

    /// Description of the parameter
    pub description: String,

    /// Whether this parameter is required
    #[serde(default)]
    pub required: bool,

    /// Default value for the parameter
    #[serde(default)]
    pub default: Option<Value>,
}

/// Types of tool parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

impl Tool {
    /// Validate parameters against the tool's input schema
    ///
    /// # Arguments
    ///
    /// * `parameters` - Parameters to validate
    ///
    /// # Returns
    ///
    /// Result indicating whether parameters are valid
    pub fn validate_parameters(&self, parameters: &Value) -> Result<(), String> {
        // TODO: Implement JSON schema validation
        // For now, just check if it's an object
        if !parameters.is_object() {
            return Err("Parameters must be an object".to_string());
        }
        Ok(())
    }

    /// Get required parameters for this tool
    ///
    /// # Returns
    ///
    /// List of required parameter names
    pub fn required_parameters(&self) -> Vec<String> {
        // TODO: Parse input_schema to extract required parameters
        Vec::new()
    }

    /// Get all parameters for this tool
    ///
    /// # Returns
    ///
    /// Map of parameter names to their definitions
    pub fn parameters(&self) -> HashMap<String, ToolParameter> {
        // TODO: Parse input_schema to extract all parameters
        HashMap::new()
    }
}

/// Registry for managing available tools
pub struct ToolRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    ///
    /// # Arguments
    ///
    /// * `tool` - Tool to register
    pub fn register(&mut self, tool: Tool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    /// Get a tool by name
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the tool
    ///
    /// # Returns
    ///
    /// Optional reference to the tool
    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }

    /// List all registered tools
    ///
    /// # Returns
    ///
    /// Vector of all tools
    pub fn list(&self) -> Vec<&Tool> {
        self.tools.values().collect()
    }

    /// Remove a tool
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the tool to remove
    ///
    /// # Returns
    ///
    /// The removed tool, if it existed
    pub fn remove(&mut self, name: &str) -> Option<Tool> {
        self.tools.remove(name)
    }

    /// Clear all tools
    pub fn clear(&mut self) {
        self.tools.clear();
    }

    /// Get tools by server
    ///
    /// # Arguments
    ///
    /// * `server` - Server name
    ///
    /// # Returns
    ///
    /// Vector of tools from the specified server
    pub fn get_by_server(&self, server: &str) -> Vec<&Tool> {
        self.tools
            .values()
            .filter(|t| t.server == server)
            .collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "param1": {"type": "string"}
                },
                "required": ["param1"]
            }),
            server: "test_server".to_string(),
        };

        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.server, "test_server");
    }

    #[test]
    fn test_tool_registry() {
        let mut registry = ToolRegistry::new();

        let tool = Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: serde_json::json!({}),
            server: "test_server".to_string(),
        };

        registry.register(tool.clone());
        assert!(registry.get("test_tool").is_some());
        assert_eq!(registry.list().len(), 1);

        registry.remove("test_tool");
        assert!(registry.get("test_tool").is_none());
    }
}
