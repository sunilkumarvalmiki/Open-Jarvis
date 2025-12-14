//! MCP configuration management
//!
//! This module handles loading and managing MCP server configurations.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpConfig {
    /// Map of server name to server configuration
    pub servers: HashMap<String, ServerConfig>,

    /// Security settings
    #[serde(default)]
    pub security: SecurityConfig,
}

impl McpConfig {
    /// Loads configuration from the default location
    ///
    /// # Returns
    ///
    /// Returns the loaded configuration or an error
    pub fn load() -> Result<Self, String> {
        // TODO: Implement actual config loading from file
        Ok(Self::default())
    }

    /// Saves configuration to the default location
    ///
    /// # Returns
    ///
    /// Returns Ok on success or an error message
    pub fn save(&self) -> Result<(), String> {
        // TODO: Implement actual config saving
        Ok(())
    }

    /// Gets a specific server configuration by name
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the server
    ///
    /// # Returns
    ///
    /// Returns the server configuration if found
    pub fn get_server(&self, name: &str) -> Option<&ServerConfig> {
        self.servers.get(name)
    }
}

/// Configuration for a specific MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Type of server transport
    #[serde(rename = "type")]
    pub server_type: ServerType,

    /// Command to run (for stdio servers)
    pub command: Option<String>,

    /// Arguments for the command
    pub args: Option<Vec<String>>,

    /// URL for HTTP/WebSocket servers
    pub url: Option<String>,

    /// Environment variables
    pub env: Option<HashMap<String, String>>,

    /// Whether the server is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

/// Type of MCP server transport
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    /// Standard I/O based communication
    Stdio,

    /// HTTP-based communication
    Http,

    /// WebSocket-based communication
    WebSocket,
}

/// Security configuration for MCP operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Whether to allow file system access
    #[serde(default = "default_false")]
    pub allow_file_system_access: bool,

    /// Allowed file system paths
    #[serde(default)]
    pub allowed_paths: Vec<String>,

    /// Whether to require user confirmation for operations
    #[serde(default = "default_true")]
    pub require_confirmation: bool,

    /// Confirmation threshold level
    #[serde(default)]
    pub confirmation_threshold: ConfirmationLevel,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allow_file_system_access: false,
            allowed_paths: Vec::new(),
            require_confirmation: true,
            confirmation_threshold: ConfirmationLevel::Medium,
        }
    }
}

fn default_false() -> bool {
    false
}

/// Level of confirmation required for operations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConfirmationLevel {
    /// No confirmation required
    None,

    /// Confirmation for low-risk operations
    Low,

    /// Confirmation for medium-risk operations (default)
    #[default]
    Medium,

    /// Confirmation for all operations
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = McpConfig::default();
        assert!(config.servers.is_empty());
        assert!(config.security.require_confirmation);
    }

    #[test]
    fn test_server_config_serialization() {
        let server = ServerConfig {
            server_type: ServerType::Stdio,
            command: Some("npx".to_string()),
            args: Some(vec!["-y".to_string(), "mcp-server".to_string()]),
            url: None,
            env: None,
            enabled: true,
        };

        let json = serde_json::to_string(&server).unwrap();
        let deserialized: ServerConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(server.server_type, deserialized.server_type);
        assert_eq!(server.command, deserialized.command);
    }
}
