# brk

Umbrella crate for the Bitcoin Research Kit.

[crates.io](https://crates.io/crates/brk) | [docs.rs](https://docs.rs/brk)

## Usage

Single dependency to access any BRK component. Enable only what you need via feature flags.

```toml
[dependencies]
brk = { version = "0.1", features = ["query", "types"] }
```

```rust
use brk::query::Query;
use brk::types::Height;
```

Feature flags match crate names without the `brk_` prefix. Use `full` to enable all.

## Crates

**Core Pipeline**

| Crate | Description |
|-------|-------------|
| [brk_reader](https://docs.rs/brk_reader) | Read blocks from `blk*.dat` with parallel parsing and XOR decoding |
| [brk_indexer](https://docs.rs/brk_indexer) | Index transactions, addresses, and UTXOs |
| [brk_computer](https://docs.rs/brk_computer) | Compute derived metrics (realized cap, MVRV, SOPR, cohorts, etc.) |
| [brk_mempool](https://docs.rs/brk_mempool) | Monitor mempool, estimate fees, project upcoming blocks |
| [brk_query](https://docs.rs/brk_query) | Query interface for indexed and computed data |
| [brk_server](https://docs.rs/brk_server) | REST API with OpenAPI docs |

**Data & Storage**

| Crate | Description |
|-------|-------------|
| [brk_types](https://docs.rs/brk_types) | Domain types: `Height`, `Sats`, `Txid`, addresses, etc. |
| [brk_store](https://docs.rs/brk_store) | Key-value storage (fjall wrapper) |
| [brk_fetcher](https://docs.rs/brk_fetcher) | Fetch price data from exchanges |
| [brk_rpc](https://docs.rs/brk_rpc) | Bitcoin Core RPC client |
| [brk_iterator](https://docs.rs/brk_iterator) | Unified block iteration with automatic source selection |
| [brk_cohort](https://docs.rs/brk_cohort) | UTXO and address cohort filtering |
| [brk_traversable](https://docs.rs/brk_traversable) | Navigate hierarchical data structures |

**Clients & Integration**

| Crate | Description |
|-------|-------------|
| [brk_client](https://docs.rs/brk_client) | Generated Rust API client |
| [brk_bindgen](https://docs.rs/brk_bindgen) | Generate typed clients (Rust, JavaScript, Python) |
| [brk_mcp](https://docs.rs/brk_mcp) | Model Context Protocol server for LLM integration |

**Internal**

| Crate | Description |
|-------|-------------|
| [brk_cli](https://docs.rs/brk_cli) | CLI binary (`cargo install --locked brk_cli`) |
| [brk_error](https://docs.rs/brk_error) | Error types |
| [brk_logger](https://docs.rs/brk_logger) | Logging infrastructure |
| [brk_bencher](https://docs.rs/brk_bencher) | Benchmarking utilities |

## License

MIT
