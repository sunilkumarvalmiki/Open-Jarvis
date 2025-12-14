//! Tool registry and management
//!
//! This module manages the registry of available MCP tools and their metadata.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry of available MCP tools
pub struct ToolRegistry {
    tools: HashMap<String, ToolMetadata>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Creates a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Registers a new tool
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the tool
    /// * `metadata` - Tool metadata
    pub fn register(&mut self, name: String, metadata: ToolMetadata) {
        self.tools.insert(name, metadata);
    }

    /// Gets metadata for a specific tool
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the tool
    ///
    /// # Returns
    ///
    /// Returns the tool metadata if found
    pub fn get(&self, name: &str) -> Option<&ToolMetadata> {
        self.tools.get(name)
    }

    /// Lists all registered tools
    ///
    /// # Returns
    ///
    /// Returns a vector of tool names
    pub fn list(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Removes a tool from the registry
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the tool to remove
    ///
    /// # Returns
    ///
    /// Returns true if the tool was removed, false if it didn't exist
    pub fn unregister(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    /// Clears all registered tools
    pub fn clear(&mut self) {
        self.tools.clear();
    }
}

/// Metadata for an MCP tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// Tool name
    pub name: String,

    /// Tool description
    pub description: String,

    /// Input schema (JSON Schema)
    pub input_schema: serde_json::Value,

    /// Server providing this tool
    pub server: String,

    /// Whether the tool requires confirmation
    #[serde(default)]
    pub requires_confirmation: bool,
}

impl ToolMetadata {
    /// Creates new tool metadata
    ///
    /// # Arguments
    ///
    /// * `name` - Tool name
    /// * `description` - Tool description
    /// * `server` - Server name
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        server: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: serde_json::json!({}),
            server: server.into(),
            requires_confirmation: false,
        }
    }

    /// Sets the input schema for the tool
    pub fn with_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = schema;
        self
    }

    /// Sets whether the tool requires confirmation
    pub fn with_confirmation(mut self, requires: bool) -> Self {
        self.requires_confirmation = requires;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_registry() {
        let mut registry = ToolRegistry::new();

        let metadata = ToolMetadata::new("test_tool", "A test tool", "test_server");

        registry.register("test_tool".to_string(), metadata.clone());

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("test_tool").is_some());

        let retrieved = registry.get("test_tool").unwrap();
        assert_eq!(retrieved.name, "test_tool");
        assert_eq!(retrieved.server, "test_server");
    }

    #[test]
    fn test_tool_metadata_builder() {
        let metadata = ToolMetadata::new("tool", "description", "server")
            .with_schema(serde_json::json!({"type": "object"}))
            .with_confirmation(true);

        assert_eq!(metadata.name, "tool");
        assert!(metadata.requires_confirmation);
    }
}
