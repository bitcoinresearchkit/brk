# brk_mcp

Model Context Protocol (MCP) server for Bitcoin on-chain data.

## What It Enables

Expose BRK's REST API to AI assistants via MCP. The LLM reads the OpenAPI spec and calls any endpoint through a generic fetch tool.

## Available Tools

| Tool | Description |
|------|-------------|
| `get_openapi` | Get the OpenAPI specification for all REST endpoints |
| `fetch` | Call any REST API endpoint by path and query |

## Workflow

1. LLM calls `get_openapi` to discover available endpoints
2. LLM calls `fetch` with the desired path and query parameters

## Usage

```rust,ignore
let mcp = MCP::new("http://127.0.0.1:3110", openapi_json);
```

## Integration

The MCP server is integrated into `brk_server` and exposed at `/mcp` endpoint.

## Built On

- `brk_rmcp` for MCP protocol implementation
- `minreq` for HTTP requests
