//! MCP (Model Context Protocol) integration module
//!
//! This module provides the foundation for integrating external tools
//! and services via the Model Context Protocol.
//!
//! # Overview
//!
//! The MCP integration allows Open-Jarvis to extend its capabilities
//! by connecting to external MCP servers that provide tools, resources,
//! and prompts.
//!
//! # Architecture
//!
//! - `client`: MCP client implementation for communicating with servers
//! - `tools`: Tool registry and management
//! - `config`: Configuration management for MCP servers
//!
//! # Example
//!
//! ```rust,no_run
//! use crate::mcp::{McpClient, McpConfig};
//!
//! async fn example() -> Result<(), String> {
//!     let config = McpConfig::load()?;
//!     let client = McpClient::new(&config).await?;
//!     
//!     // Call a tool
//!     let result = client.call_tool("github_list_repos", serde_json::json!({
//!         "username": "sunilkumarvalmiki"
//!     })).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod server;
pub mod tools;

// Re-exports for convenience
#[allow(unused_imports)]
pub use client::McpClient;
#[allow(unused_imports)]
pub use config::McpConfig;
#[allow(unused_imports)]
pub use tools::ToolRegistry;
