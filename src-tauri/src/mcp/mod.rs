//! MCP (Model Context Protocol) integration module
//!
//! This module provides the foundation for integrating external tools
//! and services via the Model Context Protocol.
//!
//! # Architecture
//!
//! The MCP module is organized into three main components:
//!
//! - `client`: Handles communication with MCP servers
//! - `tools`: Defines and manages available MCP tools
//! - `config`: Configuration and settings for MCP connections
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::mcp::{McpClient, McpConfig};
//!
//! let config = McpConfig::load_from_file("config.json")?;
//! let client = McpClient::new(config);
//! client.connect().await?;
//! ```

pub mod client;
pub mod config;
pub mod tools;

// Re-exports for convenient access
pub use client::McpClient;
pub use config::McpConfig;
pub use tools::McpTool;
