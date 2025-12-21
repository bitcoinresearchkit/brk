# brk_mcp

Model Context Protocol (MCP) server for Bitcoin on-chain data.

## What It Enables

Expose BRK's query capabilities to AI assistants via the MCP standard. LLMs can browse metrics, fetch datasets, and analyze on-chain data through structured tool calls.

## Key Features

- **Tool-based API**: 8 tools for metric discovery and data retrieval
- **Pagination support**: Browse large metric catalogs in chunks
- **Self-documenting**: Built-in instructions explain available capabilities
- **Async**: Full tokio integration via `AsyncQuery`

## Available Tools

| Tool | Description |
|------|-------------|
| `get_metric_count` | Count of unique metrics |
| `get_vec_count` | Total metric × index combinations |
| `get_indexes` | List all index types and variants |
| `get_vecids` | Paginated list of metric IDs |
| `get_index_to_vecids` | Metrics supporting a given index |
| `get_vecid_to_indexes` | Indexes supported by a metric |
| `get_vecs` | Fetch metric data with range selection |
| `get_version` | BRK version string |

## Usage

```rust,ignore
let mcp = MCP::new(&async_query);

// The MCP server implements ServerHandler for use with rmcp
// Tools are auto-registered via the #[tool_router] macro
```

## Integration

The MCP server is integrated into `brk_server` and exposed at `/mcp` endpoint for MCP transport.

## Built On

- `brk_query` for data access
- `brk_rmcp` for MCP protocol implementation
