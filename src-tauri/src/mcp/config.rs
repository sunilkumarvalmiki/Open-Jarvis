//! MCP Configuration
//!
//! This module provides configuration structures and loading for MCP.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// MCP configuration
///
/// Contains settings for MCP servers, security, and general configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Configuration version
    pub version: String,
    /// List of configured MCP servers
    pub servers: Vec<ServerConfig>,
    /// Security settings
    pub security: SecurityConfig,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            servers: Vec::new(),
            security: SecurityConfig::default(),
        }
    }
}

impl McpConfig {
    /// Load configuration from default location
    ///
    /// Returns the default configuration if file doesn't exist.
    pub fn load() -> Result<Self, String> {
        // TODO: Implement loading from platform-specific config directory
        Ok(Self::default())
    }

    /// Get configuration file path for the current platform
    pub fn get_config_path() -> Result<PathBuf, String> {
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = dirs_next::home_dir() {
                return Ok(home.join(".config/jarvis/mcp-config.json"));
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(home) = dirs_next::home_dir() {
                return Ok(
                    home.join("Library/Application Support/com.example.jarvis/mcp-config.json")
                );
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(app_data) = dirs_next::data_dir() {
                return Ok(app_data.join("jarvis/mcp-config.json"));
            }
        }

        Err("Could not determine config path".to_string())
    }
}

/// Configuration for a single MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server name (identifier)
    pub name: String,
    /// Transport type (stdio, http, websocket)
    pub transport: TransportType,
    /// Whether the server is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Server-specific settings
    #[serde(flatten)]
    pub settings: ServerSettings,
}

fn default_enabled() -> bool {
    true
}

/// Transport type for MCP communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    /// Standard input/output
    Stdio,
    /// HTTP/HTTPS
    Http,
    /// WebSocket
    WebSocket,
}

/// Server-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerSettings {
    /// Settings for stdio transport
    Stdio {
        /// Command to execute
        command: String,
        /// Command arguments
        #[serde(default)]
        args: Vec<String>,
        /// Environment variables
        #[serde(default)]
        env: std::collections::HashMap<String, String>,
    },
    /// Settings for HTTP transport
    Http {
        /// Server URL
        url: String,
        /// Authentication settings
        #[serde(skip_serializing_if = "Option::is_none")]
        auth: Option<AuthConfig>,
    },
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication type (bearer, basic, etc.)
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Authentication token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Allow dangerous tools
    #[serde(default)]
    pub allow_dangerous_tools: bool,
    /// Require user confirmation for tool execution
    #[serde(default = "default_require_confirmation")]
    pub require_confirmation: bool,
    /// Allowed directories for file operations
    #[serde(default)]
    pub allowed_directories: Vec<PathBuf>,
}

fn default_require_confirmation() -> bool {
    true
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allow_dangerous_tools: false,
            require_confirmation: true,
            allowed_directories: Vec::new(),
        }
    }
}
