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

Portable build (without native CPU optimizations):

```bash
cargo install --locked brk_cli
```

## Run

```bash
brk
```

Indexes the blockchain, computes datasets, starts the server on `localhost:3110`, and waits for new blocks.

## Help

```
brk -h       Show all options
brk -V       Show version
```

## Options

Options are saved to `~/.brk/config.toml` after first use.

```
--bitcoindir <PATH>       Bitcoin data directory
--blocksdir <PATH>        Blocks directory (default: bitcoindir/blocks)
--brkdir <PATH>           Output directory (default: ~/.brk)

--rpcconnect <IP>         RPC host (default: localhost)
--rpcport <PORT>          RPC port (default: 8332)
--rpccookiefile <PATH>    RPC cookie file (default: bitcoindir/.cookie)
--rpcuser <USERNAME>      RPC username
--rpcpassword <PASSWORD>  RPC password

-F, --fetch <BOOL>        Fetch price data (default: true)
--exchanges <BOOL>        Fetch from exchange APIs (default: true)
-w, --website <BOOL|PATH> Serve web interface (default: true)
```

## Files

```
~/.brk/
├── config.toml   Configuration
└── log/          Logs

<brkdir>/         Indexed data (default: ~/.brk)
```
