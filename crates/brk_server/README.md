# brk_server

HTTP API server for Bitcoin on-chain analytics.

## Features

- **OpenAPI spec**: Auto-generated docs at `/api` with full spec at `/openapi.json`
- **LLM-optimized**: Compact spec at `/api.json` for AI tools
- **Response caching**: ETag-based with LRU cache (5000 entries)
- **Compression**: Brotli, gzip, deflate, zstd
- **Static files**: Optional web interface hosting

## Usage

```rust,ignore
let server = Server::new(&async_query, data_path, WebsiteSource::Filesystem(files_path));
// Or WebsiteSource::Embedded, or WebsiteSource::Disabled
server.serve().await?;
```

## Endpoints

| Path | Description |
|------|-------------|
| `/api` | Interactive API documentation |
| `/openapi.json` | Full OpenAPI specification |
| `/api.json` | Compact OpenAPI for LLMs |
| `/api/address/{address}` | Address stats, transactions, UTXOs |
| `/api/block/{hash}` | Block info, transactions, status |
| `/api/block-height/{height}` | Block by height |
| `/api/tx/{txid}` | Transaction details, status, hex |
| `/api/mempool` | Fee estimates, mempool stats |
| `/api/metrics` | Metric catalog and data queries |
| `/api/v1/mining/...` | Hashrate, difficulty, pools |

## Caching

Uses ETag-based caching with `must-revalidate`:
- **Height-indexed**: Invalidates when new block arrives
- **Immutable**: 1-year cache for deeply-confirmed blocks/txs (6+ confirmations)
- **Mempool**: Short max-age, no ETag

## Configuration

Binds to port 3110, auto-incrementing up to 3210 if busy.

## Dependencies

- `brk_query` - data access
- `aide` + `axum` - HTTP routing and OpenAPI
- `tower-http` - compression and tracing
