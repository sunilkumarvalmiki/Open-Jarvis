# MCP Integration Guide

This guide explains how to integrate and use the Model Context Protocol (MCP) with Open-Jarvis.

## Table of Contents

1. [Introduction](#introduction)
2. [MCP Overview](#mcp-overview)
3. [Available Tools](#available-tools)
4. [Configuration](#configuration)
5. [Custom Tool Development](#custom-tool-development)
6. [Security Considerations](#security-considerations)
7. [Troubleshooting](#troubleshooting)

## Introduction

The Model Context Protocol (MCP) is an open protocol that enables seamless integration between AI applications and external tools and data sources. Open-Jarvis uses MCP to extend its capabilities beyond built-in features.

### Benefits of MCP Integration

- **Extensibility**: Add new capabilities without modifying core code
- **Standardization**: Use a common protocol for tool integration
- **Security**: Controlled access to external resources
- **Flexibility**: Switch between tool providers easily
- **Ecosystem**: Access to a growing ecosystem of MCP tools

## MCP Overview

### What is MCP?

MCP defines a standard way for AI applications to:
- Discover available tools and their capabilities
- Invoke tools with structured parameters
- Receive structured responses
- Handle errors consistently

### Architecture

```
┌─────────────────┐         MCP Protocol        ┌──────────────────┐
│   Open-Jarvis   │◄────────────────────────────►│   MCP Server     │
│   (MCP Client)  │      JSON-RPC 2.0           │   (Tool Provider)│
└─────────────────┘                              └──────────────────┘
         │                                                 │
         │                                                 │
         ▼                                                 ▼
   User Interface                               External Resources
   - Command Input                              - GitHub API
   - Voice Control                              - File System
   - Settings                                   - Databases
                                                - Web Services
```

### Communication Flow

1. **Initialization**: Client connects to MCP server
2. **Capability Exchange**: Server advertises supported features
3. **Tool Discovery**: Client requests list of available tools
4. **Schema Validation**: Client validates tool schemas
5. **Tool Invocation**: Client calls tools with parameters
6. **Response Handling**: Client processes tool responses

## Available Tools

### GitHub Operations

Interact with GitHub repositories, issues, and pull requests.

**Configuration:**
```json
{
  "mcp": {
    "servers": {
      "github": {
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-github"],
        "env": {
          "GITHUB_TOKEN": "${GITHUB_TOKEN}"
        }
      }
    }
  }
}
```

**Available Operations:**
- Create, read, update issues
- Manage pull requests
- Search repositories
- File operations
- Commit and branch management

**Example Usage:**
```javascript
// In JavaScript/UI
invoke('mcp_call', {
  server: 'github',
  tool: 'create_issue',
  params: {
    owner: 'sunilkumarvalmiki',
    repo: 'Open-Jarvis',
    title: 'New Feature Request',
    body: 'Description of the feature...'
  }
});
```

### File System Access

Read, write, and manage files on the local system.

**Configuration:**
```json
{
  "mcp": {
    "servers": {
      "filesystem": {
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-filesystem"],
        "env": {
          "ALLOWED_DIRECTORIES": "/home/user/documents,/home/user/projects"
        }
      }
    }
  }
}
```

**Available Operations:**
- Read file contents
- Write files
- List directory contents
- Create/delete directories
- File metadata operations

**Security Notes:**
- Access is restricted to configured directories
- Path traversal is prevented
- User confirmation required for write operations

### Database Queries

Execute queries against various databases.

**Configuration:**
```json
{
  "mcp": {
    "servers": {
      "postgres": {
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-postgres"],
        "env": {
          "POSTGRES_CONNECTION_STRING": "${POSTGRES_URL}"
        }
      }
    }
  }
}
```

**Available Operations:**
- Execute SELECT queries
- List tables and schemas
- Describe table structure

**Security Notes:**
- Read-only access recommended
- Connection strings stored securely
- Query sanitization required

### Web Search

Perform web searches and retrieve information.

**Configuration:**
```json
{
  "mcp": {
    "servers": {
      "brave-search": {
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-brave-search"],
        "env": {
          "BRAVE_API_KEY": "${BRAVE_API_KEY}"
        }
      }
    }
  }
}
```

## Configuration

### Configuration File Location

**Linux/macOS:**
```
~/.config/jarvis/mcp-config.json
```

**Windows:**
```
%APPDATA%\jarvis\mcp-config.json
```

### Configuration Format

```json
{
  "mcp": {
    "enabled": true,
    "servers": {
      "server-name": {
        "command": "executable",
        "args": ["arg1", "arg2"],
        "env": {
          "VAR_NAME": "value"
        },
        "disabled": false,
        "settings": {
          // Server-specific settings
        }
      }
    },
    "tools": {
      "tool-name": {
        "enabled": true,
        "requireConfirmation": true,
        "rateLimit": {
          "maxCalls": 100,
          "windowMs": 60000
        }
      }
    }
  }
}
```

### Environment Variables

Use environment variable substitution in configuration:

```json
{
  "env": {
    "API_KEY": "${MY_API_KEY}"
  }
}
```

The actual values are loaded from:
- System environment variables
- `.env` file in config directory
- User settings in the application

### Tool-Specific Configuration

Some tools require additional configuration:

```json
{
  "mcp": {
    "tools": {
      "github_create_issue": {
        "enabled": true,
        "requireConfirmation": true,
        "defaults": {
          "labels": ["from-jarvis"]
        }
      }
    }
  }
}
```

## Custom Tool Development

### Creating an MCP Server

You can create your own MCP server to add custom functionality.

#### 1. Choose a Transport

MCP supports two transports:
- **stdio**: Communication via standard input/output
- **SSE**: Server-Sent Events over HTTP

#### 2. Implement the Protocol

**Example in Node.js:**

```javascript
const { Server } = require('@modelcontextprotocol/sdk/server');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio');

class MyCustomServer {
  constructor() {
    this.server = new Server({
      name: 'my-custom-server',
      version: '1.0.0'
    }, {
      capabilities: {
        tools: {}
      }
    });
    
    this.setupTools();
  }
  
  setupTools() {
    this.server.setRequestHandler('tools/list', async () => ({
      tools: [
        {
          name: 'my_custom_tool',
          description: 'Does something useful',
          inputSchema: {
            type: 'object',
            properties: {
              input: {
                type: 'string',
                description: 'Input parameter'
              }
            },
            required: ['input']
          }
        }
      ]
    }));
    
    this.server.setRequestHandler('tools/call', async (request) => {
      const { name, arguments: args } = request.params;
      
      if (name === 'my_custom_tool') {
        // Implement tool logic
        const result = await this.doCustomWork(args.input);
        
        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify(result)
            }
          ]
        };
      }
      
      throw new Error(`Unknown tool: ${name}`);
    });
  }
  
  async doCustomWork(input) {
    // Your custom logic here
    return { result: `Processed: ${input}` };
  }
  
  async run() {
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
  }
}

const server = new MyCustomServer();
server.run().catch(console.error);
```

#### 3. Package Your Server

Create a `package.json`:

```json
{
  "name": "my-custom-mcp-server",
  "version": "1.0.0",
  "bin": {
    "my-custom-server": "./index.js"
  },
  "dependencies": {
    "@modelcontextprotocol/sdk": "^0.1.0"
  }
}
```

#### 4. Configure in Open-Jarvis

```json
{
  "mcp": {
    "servers": {
      "my-custom": {
        "command": "npx",
        "args": ["-y", "my-custom-mcp-server"]
      }
    }
  }
}
```

### Best Practices

1. **Error Handling**: Always handle errors gracefully
2. **Input Validation**: Validate all parameters
3. **Security**: Never execute untrusted code
4. **Performance**: Keep operations fast (< 5 seconds)
5. **Documentation**: Provide clear tool descriptions
6. **Versioning**: Use semantic versioning
7. **Testing**: Write comprehensive tests

## Security Considerations

### Authentication

- Store credentials securely (system keychain)
- Use environment variables for sensitive data
- Never commit credentials to version control
- Rotate API keys regularly

### Authorization

- Implement least privilege access
- Require user confirmation for destructive operations
- Audit all tool invocations
- Whitelist allowed tools

### Data Protection

- Encrypt sensitive data at rest
- Use HTTPS for network communication
- Sanitize all inputs
- Validate all outputs

### Sandboxing

- Run MCP servers in isolated processes
- Limit file system access
- Network access controls
- Resource limits (CPU, memory, time)

### Audit Logging

Log all MCP operations:

```json
{
  "timestamp": "2024-12-05T16:00:00Z",
  "server": "github",
  "tool": "create_issue",
  "user": "user@example.com",
  "parameters": {
    "repo": "Open-Jarvis",
    "title": "..."
  },
  "result": "success"
}
```

## Troubleshooting

### Server Connection Issues

**Problem**: MCP server fails to start

**Solutions**:
1. Check if the command is in PATH
2. Verify environment variables are set
3. Check server logs in `~/.config/jarvis/logs/mcp/`
4. Ensure required dependencies are installed

### Tool Discovery Failures

**Problem**: Tools not appearing in Open-Jarvis

**Solutions**:
1. Verify server is running: check process list
2. Check server capabilities response
3. Validate tool schemas
4. Restart Open-Jarvis

### Tool Execution Errors

**Problem**: Tool calls fail with errors

**Solutions**:
1. Validate parameters match schema
2. Check API credentials
3. Review rate limits
4. Check network connectivity
5. Review detailed error messages

### Performance Issues

**Problem**: Tool calls are slow

**Solutions**:
1. Enable caching for repeated calls
2. Reduce payload sizes
3. Check network latency
4. Optimize server implementation
5. Consider local alternatives

### Debug Mode

Enable debug logging:

```json
{
  "mcp": {
    "debug": true,
    "logLevel": "trace"
  }
}
```

View logs:
```bash
tail -f ~/.config/jarvis/logs/mcp.log
```

## Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [MCP SDK Documentation](https://github.com/modelcontextprotocol/sdk)
- [Example MCP Servers](https://github.com/modelcontextprotocol/servers)
- [Open-Jarvis MCP Examples](https://github.com/sunilkumarvalmiki/Open-Jarvis/tree/main/examples/mcp)

## Contributing

Have ideas for new MCP integrations? See our [Contributing Guide](../CONTRIBUTING.md) to get started!
