//! MCP (Model Context Protocol) client implementation
//!
//! Provides integration with external tools via the MCP protocol.

pub mod client;
pub mod server;
pub mod tools;

pub use client::McpClient;
