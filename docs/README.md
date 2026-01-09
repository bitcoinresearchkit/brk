# Bitcoin Research Kit

<div align="center">

  <p>
    <strong>A suite of Rust crates for working with Bitcoin data.</strong>
  </p>

  <p>
    <a href="https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md"><img alt="MIT Licensed" src="https://img.shields.io/badge/license-MIT-blue.svg"/></a>
    <a href="https://crates.io/crates/brk"><img alt="Crates.io" src="https://img.shields.io/crates/v/brk.svg"/></a>
    <a href="https://docs.rs/brk"><img alt="docs.rs" src="https://img.shields.io/docsrs/brk"/></a>
    <a href="https://discord.gg/WACpShCB7M"><img alt="Discord" src="https://img.shields.io/discord/1350431684562124850?logo=discord"/></a>
  </p>

</div>

[Homepage](https://bitcoinresearchkit.org) · [API Docs](https://bitcoinresearchkit.org/api) · [Charts](https://bitview.space) · [Changelog](https://github.com/bitcoinresearchkit/brk/blob/main/docs/CHANGELOG.md)

## About

BRK is a toolkit for parsing, indexing, and analyzing Bitcoin blockchain data. It combines functionality similar to [Glassnode](https://glassnode.com) (on-chain analytics), [mempool.space](https://mempool.space) (block explorer), and [electrs](https://github.com/romanz/electrs) (address indexing) into a single self-hostable package.

- **Parse** blocks directly from Bitcoin Core's data files
- **Index** transactions, addresses, and UTXOs
- **Compute** derived metrics across multiple time resolutions
- **Monitor** mempool with fee estimation and projected block building
- **Serve** data via REST API and web interface
- **Query** addresses, transactions, blocks, and analytics

The crates can be used together as a complete solution, or independently for specific needs.

Built on [`rust-bitcoin`], [`vecdb`], and [`fjall`].

## Crates

**Entry Points**

| Crate | Purpose |
|-------|---------|
| [`brk`](./crates/brk) | Umbrella crate, re-exports all components via feature flags |
| [`brk_cli`](./crates/brk_cli) | CLI binary (`cargo install --locked brk_cli`) |

**Core**

| Crate | Purpose |
|-------|---------|
| [`brk_reader`](./crates/brk_reader) | Read blocks from `blk*.dat` with parallel parsing and XOR decoding |
| [`brk_indexer`](./crates/brk_indexer) | Index transactions, addresses, and UTXOs |
| [`brk_computer`](./crates/brk_computer) | Compute derived metrics (realized cap, MVRV, SOPR, cohorts, etc.) |
| [`brk_mempool`](./crates/brk_mempool) | Monitor mempool, estimate fees, project upcoming blocks |
| [`brk_query`](./crates/brk_query) | Query interface for indexed and computed data |
| [`brk_server`](./crates/brk_server) | REST API with OpenAPI docs |

**Data & Storage**

| Crate | Purpose |
|-------|---------|
| [`brk_types`](./crates/brk_types) | Domain types: `Height`, `Sats`, `Txid`, addresses, etc. |
| [`brk_store`](./crates/brk_store) | Key-value storage (fjall wrapper) |
| [`brk_fetcher`](./crates/brk_fetcher) | Fetch price data from exchanges |
| [`brk_rpc`](./crates/brk_rpc) | Bitcoin Core RPC client |
| [`brk_iterator`](./crates/brk_iterator) | Unified block iteration with automatic source selection |
| [`brk_grouper`](./crates/brk_grouper) | UTXO and address cohort filtering |
| [`brk_traversable`](./crates/brk_traversable) | Navigate hierarchical data structures |

**Clients & Integration**

| Crate | Purpose |
|-------|---------|
| [`brk_mcp`](./crates/brk_mcp) | Model Context Protocol server for LLM integration |
| [`brk_binder`](./crates/brk_binder) | Generate typed clients (Rust, JavaScript, Python) |
| [`brk_client`](./crates/brk_client) | Generated Rust API client |

**Internal**

| Crate | Purpose |
|-------|---------|
| [`brk_error`](./crates/brk_error) | Error types |
| [`brk_logger`](./crates/brk_logger) | Logging infrastructure |
| [`brk_bencher`](./crates/brk_bencher) | Benchmarking utilities |

## Architecture

```
blk*.dat ──▶ Reader ──┐
                      ├──▶ Indexer ──▶ Computer ──┐
         RPC Client ──┤                           ├──▶ Query ──▶ Server
                      └──▶ Mempool ───────────────┘
```

- `Reader` parses `blk*.dat` files directly
- `RPC Client` connects to Bitcoin Core
- `Indexer` builds lookup tables from blocks
- `Computer` derives metrics from indexed data
- `Mempool` tracks unconfirmed transactions
- `Query` provides unified access to all data
- `Server` exposes `Query` as REST API

## Usage

**As a library:**

```rust
use brk::{reader::Reader, indexer::Indexer, computer::Computer};

let reader = Reader::new(blocks_dir, &rpc);
let indexer = Indexer::forced_import(&brk_dir)?;
let computer = Computer::forced_import(&brk_dir, &indexer, None)?;
```

**As a CLI:** See [`brk_cli`](./crates/brk_cli)

**Public API:** [bitcoinresearchkit.org/api](https://bitcoinresearchkit.org/api)

## Documentation

- [Changelog](./docs/CHANGELOG.md)
- [TODO](./docs/TODO.md)
- [Hosting](./docs/HOSTING.md)
- [Support](./docs/SUPPORT.md)

## Contributing

Contributions are welcome. See [open issues](https://github.com/bitcoinresearchkit/brk/issues).

Join the discussion on [Discord](https://discord.gg/WACpShCB7M) or [Nostr](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6).

## Acknowledgments

Development supported by [OpenSats](https://opensats.org/).

## License

[MIT](https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md)

[`rust-bitcoin`]: https://github.com/rust-bitcoin/rust-bitcoin
[`vecdb`]: https://github.com/anydb-rs/anydb
[`fjall`]: https://github.com/fjall-rs/fjall
