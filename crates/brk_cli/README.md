# brk_cli

Command line interface for running BRK (Bitcoin Research Kit) instances. Orchestrates the complete pipeline: parsing Bitcoin Core blocks, indexing data, computing analytics, and serving via HTTP API.

## Overview

**Core Operation**: Continuous loop that waits for Bitcoin node sync, indexes new blocks, computes analytics, and serves data via HTTP.

**Key Components**:
- **Parser**: Reads Bitcoin Core block files
- **Indexer**: Processes and stores blockchain data in vecs/stores
- **Computer**: Computes analytics across 9 specialized domains
- **Server**: HTTP API with multiple output formats (JSON, CSV, TSV, Markdown)
- **Website**: Optional web interface (none/default/custom)

## Requirements

- **Bitcoin Core**: Fully synced node with RPC enabled
- **Storage**: ~32% of blockchain size (~233GB currently)
- **Memory**: ~7-8GB peak during indexing, ~4-5GB steady state
- **OS**: macOS or Linux (Ubuntu: `sudo apt install libssl-dev pkg-config`)

## Installation

```bash
# Binary
# https://github.com/bitcoinresearchkit/brk/releases/latest

# Via cargo
cargo install brk --locked

# From source
git clone https://github.com/bitcoinresearchkit/brk.git
cd brk && cargo build --release
```

## Usage

```bash
# First run (set configuration)
brk --brkdir ./my_data --fetch true --website default

# Subsequent runs (uses saved config)
brk

# View all options
brk --help
```

## Configuration

All options auto-save to `~/.brk/config.toml` for subsequent runs:

```bash
# Core paths
--bitcoindir <PATH>      # Bitcoin directory (default: ~/.bitcoin)
--blocksdir <PATH>       # Block files (default: bitcoindir/blocks)
--brkdir <PATH>          # BRK output directory (default: ~/.brk)

# Data sources
-F, --fetch <BOOL>       # Enable price data fetching (default: true)
--exchanges <BOOL>       # Use exchange APIs for prices (default: true)

# Server
-w, --website <WEBSITE>  # Web interface: none|default|custom

# Bitcoin RPC
--rpcconnect <IP>        # RPC host (default: localhost)
--rpcport <PORT>         # RPC port (default: 8332)
--rpccookiefile <PATH>   # Cookie auth (default: bitcoindir/.cookie)
--rpcuser <USERNAME>     # Username auth (alternative to cookie)
--rpcpassword <PASSWORD> # Password auth (alternative to cookie)
```
