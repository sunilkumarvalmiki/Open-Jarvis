//! MCP configuration structures
//!
//! This module defines configuration structures for MCP integration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main MCP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Whether MCP integration is enabled
    pub enabled: bool,

    /// Configuration for each MCP server
    pub servers: HashMap<String, ServerConfig>,

    /// Tool-specific configuration
    #[serde(default)]
    pub tools: HashMap<String, ToolConfig>,
}

/// Configuration for a single MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Command to execute to start the server
    pub command: String,

    /// Arguments to pass to the command
    #[serde(default)]
    pub args: Vec<String>,

    /// Environment variables for the server process
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Whether this server is disabled
    #[serde(default)]
    pub disabled: bool,

    /// Server-specific settings
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
}

/// Configuration for a specific tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    /// Whether this tool is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Whether to require user confirmation before executing
    #[serde(default)]
    pub require_confirmation: bool,

    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limit: Option<RateLimitConfig>,

    /// Default parameter values
    #[serde(default)]
    pub defaults: HashMap<String, serde_json::Value>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum number of calls allowed
    pub max_calls: u32,

    /// Time window in milliseconds
    pub window_ms: u64,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            servers: HashMap::new(),
            tools: HashMap::new(),
        }
    }
}

impl McpConfig {
    /// Load configuration from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// Loaded configuration or error
    pub fn from_file(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Self = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(config)
    }

    /// Save configuration to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the configuration
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Get the default configuration file path
    ///
    /// # Returns
    ///
    /// Path to the default configuration file
    pub fn default_path() -> Result<std::path::PathBuf, String> {
        let config_dir = dirs_next::config_dir()
            .ok_or_else(|| "Could not determine config directory".to_string())?;

        Ok(config_dir.join("jarvis").join("mcp-config.json"))
    }

    /// Load configuration from the default location
    ///
    /// # Returns
    ///
    /// Loaded configuration or default if file doesn't exist
    pub fn load_or_default() -> Self {
        let path = match Self::default_path() {
            Ok(p) => p,
            Err(_) => return Self::default(),
        };

        if path.exists() {
            Self::from_file(&path).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = McpConfig::default();
        assert!(config.enabled);
        assert!(config.servers.is_empty());
        assert!(config.tools.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = McpConfig::default();
        config.servers.insert(
            "test".to_string(),
            ServerConfig {
                command: "test-command".to_string(),
                args: vec!["--arg".to_string()],
                env: HashMap::new(),
                disabled: false,
                settings: HashMap::new(),
            },
        );

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: McpConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.servers.len(), deserialized.servers.len());
    }
}
