# brk_cli

Command-line interface for running the Bitcoin Research Kit.

## What It Enables

Run a full BRK instance: index the blockchain, compute metrics, serve the API, and optionally host a web interface. Continuously syncs with new blocks.

## Key Features

- **All-in-one**: Single binary runs indexer, computer, mempool monitor, and server
- **Auto-sync**: Waits for new blocks and processes them automatically
- **Web interface**: Downloads and bundles frontend from GitHub releases
- **Configurable**: TOML config for RPC, paths, and features
- **Collision checking**: Optional TXID collision validation mode
- **Memory optimized**: Uses mimalloc allocator, 512MB stack for deep recursion

## Install

```bash
cargo install --locked brk_cli
```

For pre-release versions (alpha, beta, rc), specify the version explicitly:

```bash
# Find the latest version
cargo search brk_cli

# Install a specific version
cargo install --locked brk_cli --version "VERSION"
```

See [crates.io/crates/brk_cli/versions](https://crates.io/crates/brk_cli/versions) for all available versions.

For better performance, build with native CPU optimizations:

```bash
RUSTFLAGS="-C target-cpu=native" cargo install --locked brk_cli
```

## Requirements

- Bitcoin Core with accessible `blk*.dat` files
- ~400 GB disk space
- 12+ GB RAM recommended

## Usage

```bash
# See all options
brk --help

# The CLI will:
# 1. Index new blocks
# 2. Compute derived metrics
# 3. Start mempool monitor
# 4. Launch API server (port 3110)
# 5. Wait for new blocks and repeat
```

## Components

1. **Indexer**: Processes blocks into queryable indexes
2. **Computer**: Derives 1000+ on-chain metrics
3. **Mempool**: Real-time fee estimation
4. **Server**: REST API + MCP endpoint
5. **Bundler**: JS bundling for web interface (if enabled)

## Performance

See [brk_computer](https://docs.rs/brk_computer) for full pipeline benchmarks.

## Built On

- `brk_indexer` for blockchain indexing
- `brk_computer` for metric computation
- `brk_mempool` for mempool monitoring
- `brk_server` for HTTP API
