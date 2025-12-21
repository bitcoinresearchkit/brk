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

| Machine | Time | Disk | Peak Disk | Memory | Peak Memory |
|---------|------|------|-----------|--------|-------------|
| MBP M3 Pro (36GB, internal SSD) | 5.2h | 341 GB | 415 GB | 6.4 GB | 12 GB |

Full benchmark data: [`https://github.com/bitcoinresearchkit/benches/tree/main/brk`](/benches/brk)

## Built On

- `brk_indexer` for blockchain indexing
- `brk_computer` for metric computation
- `brk_mempool` for mempool monitoring
- `brk_server` for HTTP API
- `brk_bundler` for web interface bundling
