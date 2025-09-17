# brk_mcp

Model Context Protocol bridge enabling LLM access to Bitcoin Research Kit data.

[![Crates.io](https://img.shields.io/crates/v/brk_mcp.svg)](https://crates.io/crates/brk_mcp)
[![Documentation](https://docs.rs/brk_mcp/badge.svg)](https://docs.rs/brk_mcp)

## Overview

This crate provides a Model Context Protocol (MCP) server implementation that exposes Bitcoin blockchain analytics as tools for Large Language Models. Built on the `brk_rmcp` library, it creates a standards-compliant bridge allowing LLMs to query Bitcoin data through structured tool calls with type-safe parameters and JSON responses.

**Key Features:**

- Model Context Protocol server with tools capability
- 10 specialized tools for Bitcoin data discovery and querying
- Type-safe parameter validation with structured error handling
- Stateless operation for scalability and load balancing
- Direct integration with BRK interface layer for real-time data access
- Paginated responses for large datasets (up to 1,000 entries per page)
- Multi-format output support through underlying interface layer

**Target Use Cases:**

- LLM-powered Bitcoin analytics and research applications
- Conversational interfaces for blockchain data exploration
- AI-driven financial analysis requiring Bitcoin metrics
- Automated research tools with natural language queries

## Claude

### Quick Start

Add a `bitview` custom connector with the following URL:

`https://bitview.space/mcp`

## Rust

### Installation

```bash
cargo add brk_mcp
```

### Quick Start

```rust
use brk_mcp::MCP;
use brk_interface::Interface;
use brk_rmcp::{ServerHandler, RoleServer};

// Initialize with static interface reference
let interface: &'static Interface = /* your interface */;
let mcp = MCP::new(interface);

// Use as MCP server handler
let server_info = mcp.get_info();
println!("MCP server ready with {} tools", server_info.capabilities.tools.unwrap().len());

// Individual tool usage
let index_count = mcp.get_index_count().await?;
let vec_count = mcp.get_vec_count().await?;
let version = mcp.get_version().await?;
```

## API Overview

### Core Structure

- **`MCP`**: Main server struct implementing ServerHandler trait
- **Tool Router**: Automatic tool discovery and routing with `#[tool_router]` macro
- **Parameters**: Type-safe tool parameters with validation
- **CallToolResult**: Structured tool responses with success/error handling

### Available Tools

**Discovery Tools:**

- `get_index_count()` - Total number of available indexes
- `get_vecid_count()` - Total number of vector identifiers
- `get_vec_count()` - Total vector/index combinations
- `get_indexes()` - List of all available indexes
- `get_accepted_indexes()` - Mapping of indexes to their variants

**Data Access Tools:**

- `get_vecids(pagination)` - Paginated list of vector IDs
- `get_index_to_vecids(index, pagination)` - Vector IDs supporting specific index
- `get_vecid_to_indexes(id)` - Indexes supported by vector ID
- `get_vecs(params)` - Main data query tool with flexible parameters

**System Tools:**

- `get_version()` - Bitcoin Research Kit version information

### Tool Parameters

**Core Query Parameters:**

- `index`: Time dimension (height, date, week, month, etc.)
- `ids`: Dataset identifiers (comma or space separated)
- `from`/`to`: Range filtering (supports negative indexing from end)
- `format`: Output format (json, csv, tsv, md)

**Pagination Parameters:**

- `page`: Page number for paginated results (up to 1,000 entries)

## Examples

### Basic Tool Usage

```rust
use brk_mcp::MCP;
use brk_interface::{Params, PaginationParam};

let mcp = MCP::new(interface);

// Get system information
let version = mcp.get_version().await?;
println!("BRK version: {:?}", version);

// Discover available data
let indexes = mcp.get_indexes().await?;
let vec_count = mcp.get_vec_count().await?;

// Get paginated vector IDs
let pagination = PaginationParam { page: Some(0) };
let vecids = mcp.get_vecids(pagination).await?;
```

### Data Querying

```rust
use brk_mcp::MCP;
use brk_interface::Params;

// Query latest Bitcoin price
let params = Params {
    index: "date".to_string(),
    ids: vec!["dateindex-to-price-close".to_string()].into(),
    from: Some(-1),
    ..Default::default()
};

let result = mcp.get_vecs(params).await?;
match result {
    CallToolResult::Success { content, .. } => {
        println!("Latest price: {:?}", content);
    },
    _ => println!("Query failed"),
}
```

### Multi-Vector Analysis

```rust
use brk_interface::Params;

// Get OHLC data for last 30 days
let params = Params {
    index: "date".to_string(),
    ids: vec![
        "dateindex-to-price-open".to_string(),
        "dateindex-to-price-high".to_string(),
        "dateindex-to-price-low".to_string(),
        "dateindex-to-price-close".to_string(),
    ].into(),
    from: Some(-30),
    format: Some("csv".to_string()),
    ..Default::default()
};

let ohlc_data = mcp.get_vecs(params).await?;
```

### Vector Discovery Workflow

```rust
use brk_interface::{PaginatedIndexParam, IdParam};

// 1. Get available indexes
let indexes = mcp.get_indexes().await?;

// 2. Find vectors for specific index
let paginated_index = PaginatedIndexParam {
    index: "height".to_string(),
    pagination: PaginationParam { page: Some(0) },
};
let height_vectors = mcp.get_index_to_vecids(paginated_index).await?;

// 3. Check what indexes a vector supports
let id_param = IdParam { id: "height-to-blockhash".to_string() };
let supported_indexes = mcp.get_vecid_to_indexes(id_param).await?;
```

## Architecture

### Server Implementation

**MCP Protocol Compliance:**

- Implements `ServerHandler` trait for MCP compatibility
- Provides `ServerInfo` with tools capability enabled
- Uses latest protocol version with proper initialization
- Includes instructional context for LLM understanding

**Tool Registration:**

- `#[tool_router]` macro for automatic tool discovery
- `#[tool]` attribute for individual tool definitions
- Structured parameter types with automatic validation
- Consistent error handling with `McpError` responses

### HTTP Integration

**Axum Router Extension:**

- `MCPRoutes` trait for router integration
- Conditional MCP endpoint mounting based on configuration
- Stateless HTTP service with `LocalSessionManager`
- Nested service mounting at `/mcp` path

**Transport Layer:**

- `StreamableHttpService` for MCP over HTTP
- Configurable server options with stateless mode
- Session management for concurrent connections
- Request context handling for proper MCP operation

### Data Integration

**Interface Layer Access:**

- Direct access to `brk_interface::Interface` for data queries
- Static lifetime requirements for server operation
- Unified access to indexer and computer data sources
- Consistent parameter types across tool boundaries

**Response Formatting:**

- JSON content serialization for all tool responses
- Success/error response wrapping with proper MCP structure
- Logging integration for request tracking and debugging
- Content type handling for different response formats

## Configuration

### Server Information

The MCP server provides comprehensive information to LLMs:

```rust
ServerInfo {
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
    server_info: Implementation::from_build_env(),
    instructions: "Context about Bitcoin data access and terminology",
}
```

### LLM Instructions

Built-in instructions explain Bitcoin data concepts:

- Vectors/vecs/arrays/datasets are interchangeable terms
- Indexes represent timeframes for data organization
- VecIds are dataset names describing their content
- Tool-based exploration before web browsing is encouraged

## Code Analysis Summary

**Main Structure**: `MCP` struct implementing `ServerHandler` with embedded `ToolRouter` for automatic tool discovery \
**Tool Implementation**: 10 specialized tools using `#[tool]` attribute with structured parameters and JSON responses \
**HTTP Integration**: `MCPRoutes` trait extending Axum routers with conditional MCP endpoint mounting \
**Parameter Types**: Type-safe parameter structs from `brk_interface` with automatic validation \
**Error Handling**: Consistent `McpError` responses with proper MCP protocol compliance \
**Transport Layer**: `StreamableHttpService` with stateless configuration for scalable deployment \
**Architecture**: Standards-compliant MCP bridge providing LLM access to comprehensive Bitcoin analytics

---

_This README was generated by Claude Code_
