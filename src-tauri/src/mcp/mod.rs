//! MCP (Model Context Protocol) integration module
//!
//! This module provides the foundation for integrating external tools
//! and services via the Model Context Protocol.
//!
//! # Overview
//!
//! The MCP integration allows Open-Jarvis to communicate with external
//! tool providers using a standardized protocol. This enables:
//! - Dynamic tool discovery
//! - Structured tool invocation
//! - Standardized error handling
//! - Security and permission management
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐
//! │ Open-Jarvis │
//! └──────┬──────┘
//!        │
//!        │ Uses
//!        │
//!        ▼
//! ┌──────────────┐       Communicates      ┌──────────────┐
//! │  McpClient   │◄────────────────────────►│  MCP Server  │
//! └──────────────┘      JSON-RPC 2.0        └──────────────┘
//!        │
//!        │ Manages
//!        │
//!        ▼
//! ┌──────────────┐
//! │    Tools     │
//! └──────────────┘
//! ```
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use mcp::{McpClient, McpConfig};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = McpConfig::default();
//!     let mut client = McpClient::new(config)?;
//!     
//!     // Connect to a server
//!     client.connect("github").await?;
//!     
//!     // Discover available tools
//!     let tools = client.list_tools("github").await?;
//!     
//!     // Invoke a tool
//!     let result = client.call_tool(
//!         "github",
//!         "create_issue",
//!         serde_json::json!({
//!             "owner": "user",
//!             "repo": "repo",
//!             "title": "Issue title"
//!         })
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod tools;

// Re-exports for convenience
pub use client::McpClient;
pub use config::McpConfig;
pub use tools::{Tool, ToolParameter};
