# brk_cli

Command-line interface for running a Bitcoin Research Kit instance.

## Preview

- https://bitview.space - web interface
- https://bitview.space/api - API docs

## Requirements

- Bitcoin Core running with RPC enabled
- Access to `blk*.dat` files
- ~400 GB disk space
- 12+ GB RAM

## Install

```bash
rustup update
RUSTFLAGS="-C target-cpu=native" cargo install --locked brk_cli --version "$(cargo search brk_cli | head -1 | awk -F'"' '{print $2}')"
```

The SIMD flags (`bmi1`, `bmi2`, `avx2`) significantly improve pcodec decompression performance.

Portable build (without native CPU optimizations):

```bash
cargo install --locked brk_cli
```

## Run

```bash
brk
```

Indexes the blockchain, computes datasets, starts the server on `localhost:3110`, and waits for new blocks.

**Note:** When more than 10,000 blocks behind, indexing completes before the server starts to free up memory from fragmentation that occurs during large syncs. The web interface at `localhost:3110` won't be available until sync finishes.

## Options

```bash
brk -h       # Show all options
brk -V       # Show version
```

Options are saved to `~/.brk/config.toml` after first use.

## Files

```
~/.brk/
├── config.toml   Configuration
└── log           Logs

<brkdir>/         Indexed data (default: ~/.brk)
```
