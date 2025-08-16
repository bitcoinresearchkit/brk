# brk_cli

**Command line interface for running complete BRK instances**

`brk_cli` provides the main command-line interface for operating the Bitcoin Research Kit. It orchestrates the complete data pipeline from Bitcoin Core block parsing through analytics computation to HTTP API serving, with automatic configuration management and graceful operation.

## What it provides

- **Complete Pipeline Orchestration**: Coordinates parser, indexer, computer, and server components
- **Automatic Configuration**: Saves settings to `~/.brk/config.toml` for consistent operation
- **Continuous Operation**: Handles blockchain updates and incremental processing
- **Web Interface Options**: Configurable website serving (none, default, custom)
- **Graceful Shutdown**: Ctrl+C handling with proper cleanup

## Key Features

### Pipeline Management
- **Automatic dependency handling**: Ensures Bitcoin Core sync before processing
- **Incremental updates**: Only processes new blocks since last run
- **Error recovery**: Automatic retry logic and graceful error handling
- **Resource management**: Optimized memory usage and disk I/O

### Configuration System
- **Auto-save configuration**: All CLI options saved to persistent config
- **Flexible paths**: Configurable Bitcoin directory, blocks directory, and output directory
- **RPC authentication**: Cookie file or username/password authentication
- **Data source options**: Configurable price fetching and exchange APIs

### Operation Modes
- **Initial sync**: Full blockchain processing from genesis
- **Continuous operation**: Real-time processing of new blocks
- **Update mode**: Resume from last processed block
- **Server mode**: HTTP API with optional web interface

## Installation

### Binary Release
```bash
# Download from GitHub releases
# https://github.com/bitcoinresearchkit/brk/releases/latest
```

### Via Cargo
```bash
cargo install brk --locked
```

### From Source
```bash
git clone https://github.com/bitcoinresearchkit/brk.git
cd brk && cargo build --release
```

## Usage

### First Run (Configuration Setup)

```bash
# Basic setup with default options
brk --brkdir ./my_brk_data

# Full configuration
brk --bitcoindir ~/.bitcoin \
    --brkdir ./brk_data \
    --fetch true \
    --exchanges true \
    --website default
```

### Subsequent Runs

```bash
# Uses saved configuration from ~/.brk/config.toml
brk

# Override specific options
brk --website none --fetch false
```

### Command Line Options

```bash
brk --help
```

## Configuration Reference

All options are automatically saved to `~/.brk/config.toml`:

### Core Paths
- `--bitcoindir <PATH>` - Bitcoin Core directory (default: `~/.bitcoin`)
- `--blocksdir <PATH>` - Block files directory (default: `bitcoindir/blocks`)
- `--brkdir <PATH>` - BRK output directory (default: `~/.brk`)

### Data Sources
- `--fetch <BOOL>` - Enable price data fetching (default: `true`)
- `--exchanges <BOOL>` - Use exchange APIs for prices (default: `true`)

### Web Interface
- `--website <OPTION>` - Web interface mode:
  - `none` - API only, no web interface
  - `default` - Built-in web interface from `websites/default/`
  - `custom` - Serve custom website from `websites/custom/`

### Bitcoin Core RPC
- `--rpcconnect <IP>` - RPC host (default: `localhost`)
- `--rpcport <PORT>` - RPC port (default: `8332`)
- `--rpccookiefile <PATH>` - Cookie authentication file
- `--rpcuser <USERNAME>` - Username authentication
- `--rpcpassword <PASSWORD>` - Password authentication

## Operation Flow

1. **Configuration Loading**: Loads saved config from `~/.brk/config.toml`
2. **Bitcoin Core Connection**: Establishes RPC connection and waits for sync
3. **Data Pipeline Initialization**: Sets up parser, indexer, computer, and interface
4. **Processing Loop**:
   - Index new blocks from Bitcoin Core
   - Compute analytics on new data
   - Update cached data
5. **Server Startup**: Launches HTTP API with optional web interface
6. **Continuous Operation**: Monitors for new blocks and processes incrementally

## System Requirements

- **Bitcoin Core**: Fully synced node with RPC enabled
- **Storage**: ~32% of blockchain size (~233GB as of 2025)
- **Memory**:
  - Peak: ~7-8GB during initial indexing
  - Steady state: ~4-5GB during operation
- **OS**: macOS or Linux
  - Ubuntu: `sudo apt install libssl-dev pkg-config`

## Performance Characteristics

### Initial Sync
- **Full blockchain processing**: ~13-15 hours total
- **Parser phase**: ~4 minutes for block parsing
- **Indexer phase**: ~7-8 hours for data indexing
- **Computer phase**: ~6-7 hours for analytics computation

### Continuous Operation
- **New block processing**: 3-5 seconds per block
- **API response times**: Typically <100ms with caching
- **Memory usage**: Stable ~4-5GB during normal operation

## Configuration File

Example `~/.brk/config.toml`:
```toml
bitcoindir = "/Users/username/.bitcoin"
blocksdir = "/Users/username/.bitcoin/blocks"
brkdir = "/Users/username/brk_data"
fetch = true
exchanges = true
website = "default"
rpcconnect = "localhost"
rpcport = 8332
rpccookiefile = "/Users/username/.bitcoin/.cookie"
```

## Error Handling

- **Bitcoin Core sync**: Waits for node sync before processing
- **RPC connection**: Automatic retry logic for connection issues
- **Processing errors**: Graceful error handling with detailed logging
- **Graceful shutdown**: Ctrl+C handling with proper cleanup and state saving

## Logging

Logs are written to `~/.brk/brk.log` with colored console output:
- Request/response logging with timing
- Processing progress indicators
- Error reporting and debugging information

## Dependencies

- `brk_parser` - Bitcoin block parsing
- `brk_indexer` - Blockchain data indexing
- `brk_computer` - Analytics computation
- `brk_interface` - Data query interface
- `brk_server` - HTTP API server
- `brk_logger` - Logging utilities
- `bitcoincore_rpc` - Bitcoin Core RPC client
- `color_eyre` - Enhanced error reporting

---

*This README was generated by Claude Code*
