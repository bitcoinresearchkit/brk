# brk

Bitcoin Research Kit (BRK) is a high-performance toolchain for parsing, indexing, computing, and serving Bitcoin blockchain data. It provides an alternative to services like Glassnode and mempool.space with a focus on self-hosting and open-source transparency.

This is the main wrapper crate that re-exports all workspace crates through feature flags.

## Crates

- [`brk`](https://crates.io/crates/brk): A wrapper around all other `brk-*` crates
- [`brk_bundler`](https://crates.io/crates/brk_bundler): A thin wrapper around [`rolldown`](https://rolldown.rs/)
- [`brk_cli`](https://crates.io/crates/brk_cli): A command line interface to run a BRK instance
- [`brk_computer`](https://crates.io/crates/brk_computer): A Bitcoin dataset computer built on top of [`brk_indexer`](https://crates.io/crates/brk_indexer)
- [`brk_error`](https://crates.io/crates/brk_error): Errors used throughout BRK
- [`brk_fetcher`](https://crates.io/crates/brk_fetcher): A Bitcoin price fetcher
- [`brk_indexer`](https://crates.io/crates/brk_indexer): A Bitcoin indexer built on top of [`brk_parser`](https://crates.io/crates/brk_parser)
- [`brk_interface`](https://crates.io/crates/brk_interface): An interface to find and format data from BRK
- [`brk_logger`](https://crates.io/crates/brk_logger): A thin wrapper around [`env_logger`](https://crates.io/crates/env_logger)
- [`brk_mcp`](https://crates.io/crates/brk_mcp): A bridge for LLMs to access BRK
- [`brk_parser`](https://crates.io/crates/brk_parser): A very fast Bitcoin block parser and iterator built on top of [`bitcoin-rust`](https://crates.io/crates/bitcoin)
- [`brk_server`](https://crates.io/crates/brk_server): A server with an API for anything from BRK
- [`brk_store`](https://crates.io/crates/brk_store): A thin wrapper around [`fjall`](https://crates.io/crates/fjall)
- [`brk_structs`](https://crates.io/crates/brk_structs): Structs used throughout BRK

## Features

- `full` - Enable all workspace crates
- `bundler` - Re-export `brk_bundler`
- `cli` - Re-export `brk_cli` (always enabled)
- `computer` - Re-export `brk_computer`
- `error` - Re-export `brk_error`
- `fetcher` - Re-export `brk_fetcher`
- `indexer` - Re-export `brk_indexer`
- `interface` - Re-export `brk_interface`
- `logger` - Re-export `brk_logger`
- `mcp` - Re-export `brk_mcp`
- `parser` - Re-export `brk_parser`
- `server` - Re-export `brk_server`
- `store` - Re-export `brk_store`
- `structs` - Re-export `brk_structs`

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
brk = { version = "0.1", features = ["full"] }
```

Or enable specific components:

```toml
[dependencies]
brk = { version = "0.1", features = ["parser", "indexer", "computer"] }
```

## Example

```rust
use brk::{cli, parser, indexer, computer};

// Use individual crates as needed
let config = cli::Config::load()?;
let blocks = parser::BlockIterator::new(&config.bitcoin_path)?;
// ...
```
