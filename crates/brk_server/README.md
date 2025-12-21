# brk_server

HTTP API server for Bitcoin on-chain analytics.

## What It Enables

Serve BRK data via REST API with OpenAPI documentation, response caching, MCP endpoint, and optional static file hosting for web interfaces.

## Key Features

- **OpenAPI spec**: Auto-generated documentation at `/api/openapi.json`
- **Response caching**: LRU cache with 5000 entries for repeated queries
- **Compression**: Brotli, gzip, deflate, zstd support
- **Static files**: Optional web interface hosting
- **Request logging**: Colorized status/latency logging

## Core API

```rust,ignore
let server = Server::new(&async_query, Some(files_path));
server.serve(true).await?;  // true enables MCP endpoint
```

## API Endpoints

| Path | Description |
|------|-------------|
| `/api/blocks/{height}` | Block info, transactions, status |
| `/api/txs/{txid}` | Transaction details, status, merkle proof |
| `/api/addresses/{addr}` | Address stats, transactions, UTXOs |
| `/api/metrics` | Metric catalog and data queries |
| `/api/mining/*` | Hashrate, difficulty, pools, epochs |
| `/api/mempool/*` | Fee estimates, projected blocks |
| `/mcp` | MCP endpoint (if enabled) |

## Caching

Uses ETag-based caching with stale-while-revalidate semantics:
- Height-indexed data: Cache until height changes
- Date-indexed data: Cache with longer TTL
- Mempool data: Short TTL, frequent updates

## Configuration

Server binds to port 3110 by default, auto-incrementing if busy (up to 3210).

## Recommended: mimalloc v3

Use [mimalloc v3](https://crates.io/crates/mimalloc) as the global allocator to reduce memory usage.

## Built On

- `brk_query` for data access
- `brk_mcp` for MCP protocol
- `aide` + `axum` for HTTP routing and OpenAPI
- `tower-http` for compression and tracing
