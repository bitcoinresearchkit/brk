# brk_server

HTTP server providing REST API access to Bitcoin analytics data

[![Crates.io](https://img.shields.io/crates/v/brk_server.svg)](https://crates.io/crates/brk_server)
[![Documentation](https://docs.rs/brk_server/badge.svg)](https://docs.rs/brk_server)

## Overview

This crate provides a high-performance HTTP server built on `axum` that exposes Bitcoin blockchain analytics data through a comprehensive REST API. It integrates with the entire BRK ecosystem, serving data from indexers, computers, and parsers with intelligent caching, compression, and multiple output formats.

**Key Features:**

- RESTful API for blockchain data queries with flexible filtering
- Multiple output formats: JSON, CSV
- Intelligent caching system with ETags and conditional requests
- HTTP compression (Gzip, Brotli, Deflate, Zstd) for bandwidth efficiency
- Static file serving for web interfaces and documentation
- Bitcoin address and transaction lookup endpoints
- Vector database query interface with pagination
- Health monitoring and status endpoints

**Target Use Cases:**

- Bitcoin data APIs for applications and research
- Web-based blockchain explorers and analytics dashboards
- Research data export and analysis tools
- Integration with external systems requiring Bitcoin data

## Installation

```bash
cargo add brk_server
```

## Quick Start

```rust
use brk_server::Server;
use brk_query::Interface;
use std::path::PathBuf;

// Initialize interface with your data sources
let interface = Interface::new(/* your config */);

// Optional static file serving directory
let files_path = Some(PathBuf::from("./web"));

// Create and start server
let server = Server::new(interface, files_path);

// Start server with optional MCP (Model Context Protocol) support
server.serve(true).await?;
```

## API Overview

### Core Endpoints

**Blockchain Queries:**

- `GET /api/address/{address}` - Address information, balance, transaction counts
- `GET /api/tx/{txid}` - Transaction details including version, locktime
- `GET /api/vecs/{variant}` - Vector database queries with filtering

**System Information:**

- `GET /api/vecs/index-count` - Total number of indexes available
- `GET /api/vecs/id-count` - Vector ID statistics
- `GET /api/vecs/indexes` - List of available data indexes
- `GET /health` - Service health status
- `GET /version` - Server version information

### Vector Database API

**Query Interface:**

- `GET /api/vecs/query` - Generic vector query with parameters
- `GET /api/vecs/{variant}?from={start}&to={end}&format={format}` - Range queries

**Supported Parameters:**

- `from` / `to`: Range filtering (height, timestamp, date-based)
- `format`: Output format (json, csv)
- Pagination parameters for large datasets

### Address API Response Format

```json
{
  "address": "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
  "type": "p2pkh",
  "index": 12345,
  "chain_stats": {
    "funded_txo_sum": 500000000,
    "spent_txo_sum": 400000000,
    "utxo_count": 5,
    "balance": 100000000,
    "balance_usd": 4200.5,
    "realized_value": 450000000,
    "avg_cost_basis": 45000.0
  }
}
```

## Examples

### Basic Server Setup

```rust
use brk_server::Server;
use brk_query::Interface;

// Initialize with BRK interface
let interface = Interface::builder()
    .with_indexer_path("./data/indexer")
    .with_computer_path("./data/computer")
    .build()?;

let server = Server::new(interface, None);

// Server automatically finds available port starting from 3110
server.serve(false).await?;
```

### Address Balance Lookup

```bash
# Get address information
curl http://localhost:3110/api/address/1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2

# Response includes balance, transaction counts, USD value
{
  "address": "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
  "type": "p2pkh",
  "chain_stats": {
    "balance": 100000000,
    "balance_usd": 4200.50,
    "utxo_count": 5
  }
}
```

### Data Export Queries

```bash
# Export height-to-price data as CSV
curl "http://localhost:3110/api/vecs/height-to-price?from=800000&to=800100&format=csv" \
  -H "Accept-Encoding: gzip"

# Query with caching - subsequent requests return 304 Not Modified
curl "http://localhost:3110/api/vecs/dateindex-to-price-ohlc?from=0&to=1000" \
  -H "If-None-Match: \"etag-hash\""
```

### Transaction Details

```bash
# Get transaction information
curl http://localhost:3110/api/tx/abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890

# Response includes version, locktime, and internal indexing
{
  "txid": "abcdef...",
  "index": 98765,
  "version": 2,
  "locktime": 0
}
```

## Architecture

### Server Stack

- **HTTP Framework**: `axum` with async/await for high concurrency
- **Compression**: Multi-algorithm support (Gzip, Brotli, Deflate, Zstd)
- **Caching**: `quick_cache` with LRU eviction and ETag validation
- **Tracing**: Request/response logging with latency tracking
- **Static Files**: Optional web interface serving

### Caching Strategy

The server implements intelligent caching:

- **ETags**: Generated from data version and query parameters
- **Conditional Requests**: 304 Not Modified responses for unchanged data
- **Memory Cache**: LRU cache with configurable capacity (5,000 entries)
- **Cache Control**: `must-revalidate` headers for data consistency

### Request Processing

1. **Route Matching**: Path-based routing to appropriate handlers
2. **Parameter Validation**: Query parameter parsing and validation
3. **Data Retrieval**: Interface calls to indexer/computer components
4. **Caching Logic**: ETag generation and cache lookup
5. **Format Conversion**: JSON/CSV output formatting
6. **Compression**: Response compression based on Accept-Encoding
7. **Response**: HTTP response with appropriate headers

### Static File Serving

Optional static file serving supports:

- Web interface hosting for blockchain explorers
- Documentation and API reference serving
- Asset serving (CSS, JS, images) with proper MIME types
- Directory browsing with index.html fallback

## Configuration

### Environment Variables

The server automatically configures itself but respects:

- Port selection: Starts at 3110, increments if unavailable
- Compression: Enabled by default for all supported algorithms
- CORS: Permissive headers for cross-origin requests

### Memory Management

- Cache size: 5,000 entries by default
- Request weight limits: 65MB maximum per query
- Timeout handling: 50ms cache guard timeout
- Compression: Adaptive based on content type and size

## Code Analysis Summary

**Main Components**: `Server` struct with `AppState` containing interface, cache, and file paths \
**HTTP Framework**: Built on `axum` with middleware for compression, tracing, and CORS \
**API Routes**: Address lookup, transaction details, vector queries, and system information \
**Caching Layer**: `quick_cache` integration with ETag-based conditional requests \
**Data Integration**: Direct interface to BRK indexer, computer, parser, and fetcher components \
**Static Serving**: Optional file serving for web interfaces and documentation \
**Architecture**: Async HTTP server with intelligent caching and multi-format data export capabilities

---

_This README was generated by Claude Code_
