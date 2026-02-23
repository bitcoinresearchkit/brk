# BRK CLI

Command-line interface for running a Bitcoin Research Kit instance.

## Demo

- [bitview.space](https://bitview.space) - web interface
- [bitview.space/api](https://bitview.space/api) - API docs

## Requirements

- Linux or macOS
- Bitcoin Core with `server=1` in `bitcoin.conf`
- Access to `blk*.dat` files
- [~400 GB disk space](https://bitview.space/api/server/disk)
- [12+ GB RAM](https://github.com/bitcoinresearchkit/benches#benchmarks)

## Install

```bash
rustup update
RUSTFLAGS="-C target-cpu=native" cargo install --locked brk_cli
```

Portable build (without native CPU optimizations):

```bash
cargo install --locked brk_cli
```

## Run

```bash
brk
```

Indexes the blockchain, computes datasets, starts the server on `localhost:3110`, and waits for new blocks.

## First sync

The initial sync processes the entire blockchain and can take several hours. During this time (more than 10,000 blocks behind), indexing completes before the server starts to free up memory. The web interface at `localhost:3110` won't be available until sync finishes.

## Options

```bash
brk -h       # Show all options
brk -V       # Show version
```

Command-line options override `~/.brk/config.toml` for that run only. Edit the file directly to persist settings:

```toml
brkdir = "/path/to/data"
bitcoindir = "/path/to/.bitcoin"
```

All fields are optional. See `brk -h` for the full list.

## Environment Variables

```bash
LOG=debug brk    # Enable debug logging (keeps noise filters)
RUST_LOG=... brk # Full control over log filtering (overrides all defaults)
```

## Files

```
~/.brk/
├── config.toml   Configuration
└── log           Logs

<brkdir>/         Indexed data (default: ~/.brk)
```
