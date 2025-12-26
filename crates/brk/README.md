# brk

Umbrella crate for the Bitcoin Research Kit.

## What It Enables

Single dependency to access any BRK component. Enable only what you need via feature flags.

## Usage

```toml
[dependencies]
brk = { version = "0.x", features = ["query", "types"] }
```

```rust,ignore
use brk::query::Query;
use brk::types::Height;
```

## Feature Flags

| Feature | Crate | Description |
|---------|-------|-------------|
| `bencher` | `brk_bencher` | Benchmarking utilities |
| `binder` | `brk_binder` | Client code generation |
| `bundler` | `brk_bundler` | JS bundling |
| `client` | `brk_client` | Generated Rust API client |
| `computer` | `brk_computer` | Metric computation |
| `error` | `brk_error` | Error types |
| `fetcher` | `brk_fetcher` | Price data fetching |
| `grouper` | `brk_grouper` | Cohort filtering |
| `indexer` | `brk_indexer` | Blockchain indexing |
| `iterator` | `brk_iterator` | Block iteration |
| `logger` | `brk_logger` | Logging setup |
| `mcp` | `brk_mcp` | MCP server |
| `mempool` | `brk_mempool` | Mempool monitoring |
| `query` | `brk_query` | Query interface |
| `reader` | `brk_reader` | Raw block reading |
| `rpc` | `brk_rpc` | Bitcoin RPC client |
| `server` | `brk_server` | HTTP API server |
| `store` | `brk_store` | Key-value storage |
| `traversable` | `brk_traversable` | Data traversal |
| `types` | `brk_types` | Domain types |

Use `full` to enable all features.
