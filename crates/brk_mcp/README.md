# brk_mcp

**Model Context Protocol (MCP) bridge for LLM integration with BRK**

`brk_mcp` provides a Model Context Protocol server that enables Large Language Models (LLMs) to access Bitcoin blockchain data through BRK's interface layer. It implements the MCP specification to expose BRK's analytics capabilities as tools that LLMs can call.

## What it provides

- **MCP Server Implementation**: Standards-compliant Model Context Protocol server
- **Bitcoin Data Tools**: LLM-accessible tools for querying blockchain analytics
- **Type-Safe Parameters**: Structured tool parameters with validation
- **Multi-Format Output**: JSON responses for LLM consumption
- **BRK Integration**: Direct access to all indexed and computed datasets

## Available MCP Tools

Based on the actual implementation, the following tools are available:

### Metadata Tools

#### `get_index_count`
Get the count of all existing indexes.

**Parameters:** None
**Returns:** Number of available time indices

#### `get_vecid_count`
Get the count of all existing vector IDs.

**Parameters:** None
**Returns:** Number of available dataset identifiers

#### `get_vec_count`
Get the count of all existing vectors (sum of supported indexes for each vector ID).

**Parameters:** None
**Returns:** Total number of vector/index combinations

#### `get_indexes`
Get the list of all existing indexes.

**Parameters:** None
**Returns:** Array of available index names (height, date, week, month, etc.)

#### `get_accepted_indexes`
Get an object with all existing indexes as keys and their accepted variants as values.

**Parameters:** None
**Returns:** Object mapping indexes to their accepted variant names

### Discovery Tools

#### `get_vecids`
Get a paginated list of all existing vector IDs.

**Parameters:**
- `page` (optional number): Page number (default: 0, up to 1,000 results per page)

**Returns:** Array of dataset identifiers

#### `get_index_to_vecids`
Get a paginated list of all vector IDs which support a given index.

**Parameters:**
- `index` (string): Index name to query
- `page` (optional number): Page number (default: 0)

**Returns:** Array of vector IDs that support the specified index

#### `get_vecid_to_indexes`
Get a list of all indexes supported by a given vector ID.

**Parameters:**
- `id` (string): Vector ID to query

**Returns:** Array of indexes supported by the vector ID (empty if ID doesn't exist)

### Data Query Tool

#### `get_vecs`
Get one or multiple vectors depending on given parameters.

**Parameters:**
- `index` (string): Time dimension (height, date, week, month, etc.)
- `ids` (string): Dataset identifiers (comma or space separated)
- `from` (optional i64): Start index (negative = from end)
- `to` (optional i64): End index (exclusive)
- `count` (optional usize): Maximum results
- `format` (optional string): Output format (json, csv, tsv, md)

**Response format depends on parameters:**
- **Single value**: One vector, one result (e.g., `from=-1`)
- **Array**: One vector, multiple results (e.g., `from=-100&count=100`)
- **Matrix**: Multiple vectors (always matrix, even for single results)

### System Tool

#### `get_version`
Get the running version of the Bitcoin Research Kit.

**Parameters:** None
**Returns:** Version string (e.g., "v0.0.88")

## Usage Examples

### Discovery Workflow

```
1. Call get_indexes to see available time dimensions
2. Call get_vecids to see available datasets (paginated)
3. Call get_index_to_vecids to find datasets for specific timeframes
4. Call get_vecs to query actual data
```

### Basic Data Query

```json
// Get latest Bitcoin price
{
  "tool": "get_vecs",
  "parameters": {
    "index": "date",
    "ids": "close",
    "from": -1
  }
}
```

### Multi-Dataset Query

```json
// Get last 30 days of OHLC data
{
  "tool": "get_vecs",
  "parameters": {
    "index": "date",
    "ids": "open,high,low,close",
    "from": -30,
    "format": "csv"
  }
}
```

### Metadata Exploration

```json
// Discover what's available
{
  "tool": "get_accepted_indexes"
}

// Find datasets for weekly analysis
{
  "tool": "get_index_to_vecids",
  "parameters": {
    "index": "week"
  }
}
```

## Server Configuration

The MCP server is stateless and integrates with BRK's HTTP server:

```rust
// In brk_server routes
router.add_mcp_routes(interface, true)  // Enable MCP at /mcp endpoint
```

**Server Info:**
- Protocol version: Latest MCP specification
- Capabilities: Tools enabled
- Instructions: Provides context about Bitcoin data access
- Stateless mode: Compatible with load balancers

## Error Handling

The MCP server provides structured error responses following MCP specification:
- Invalid parameters result in proper MCP error responses
- Tool failures are handled gracefully
- Parameter validation ensures type safety

## Integration

### With BRK Server
MCP is exposed at the `/mcp` endpoint when enabled:
```
POST /mcp
Content-Type: application/json

{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "get_vecs",
    "arguments": {
      "index": "height",
      "ids": "timestamp,size",
      "from": -1
    }
  }
}
```

### LLM Instructions
The server provides context to LLMs:
- Explains Bitcoin data terminology (vectors, indexes, vecids)
- Clarifies that vectors/vecs/arrays/datasets are interchangeable terms
- Describes indexes as timeframes and vecids as dataset names
- Encourages exploration before web browsing

## Dependencies

- `brk_interface` - Data access and formatting layer
- `brk_rmcp` - Rust MCP implementation
- `axum` - HTTP router integration
- `log` - Request logging

---

*This README was generated by Claude Code*
