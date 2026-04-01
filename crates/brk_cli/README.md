# BRK CLI

Run your own Bitcoin Research Kit instance. One binary, one command. Full sync in ~4-7h depending on hardware. ~44% disk overhead vs 250% for mempool/electrs.

[bitview.space](https://bitview.space) is the official free hosted instance.

## Requirements

- Linux or macOS
- Bitcoin Core with `server=1` in `bitcoin.conf`
- Access to `blk*.dat` files
- [~400 GB disk space](https://bitview.space/api/server/disk) (see [Disk usage](#disk-usage))
- [12+ GB RAM](https://github.com/bitcoinresearchkit/benches#benchmarks)

## Disk usage

BRK uses [sparse files](https://en.wikipedia.org/wiki/Sparse_file). Tools like `ls -l` or Finder report the logical file size (>1 TB), not actual disk usage (~350 GB). Use `du -sh` to see real usage.

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
