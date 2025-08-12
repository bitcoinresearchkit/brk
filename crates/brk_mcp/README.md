# brk_mcp

Model Context Protocol (MCP) endpoint that provides LLMs with access to Bitcoin Research Kit data and functionality.

This crate implements a stateless MCP endpoint that integrates with `brk_server` to expose BRK's Bitcoin blockchain data through a standardized protocol, enabling LLMs like Claude to directly query blockchain metrics, transaction data, market prices, and time-series data.

The stateless design makes it compatible with load balancers by default.

## Tools Available

- `get_index_count` - Count of available data indexes
- `get_vecid_count` - Count of available vector identifiers
- `get_vec_count` - Total count of all vectors
- `get_indexes` - List all available indexes
- `get_accepted_indexes` - Index types and their variants
- `get_vecids` - Paginated list of vector identifiers
- `get_index_to_vecids` - Vectors supporting specific indexes
- `get_vecid_to_indexes` - Indexes supported by specific vectors
- `get_vecs` - Query vector data with flexible parameters
- `get_version` - BRK version information

## Usage

The MCP server is automatically exposed at `/mcp` when BRK's HTTP server is running with MCP enabled.

### With Claude Desktop

Add the MCP endpoint to Claude Desktop.

For example:

```
https://bitcoinresearchkit.org/mcp
```
