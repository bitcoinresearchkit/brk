# brk_cli

Command-line interface orchestrating complete Bitcoin Research Kit instances with automatic configuration and continuous blockchain processing.

[![Crates.io](https://img.shields.io/crates/v/brk_cli.svg)](https://crates.io/crates/brk_cli)
[![Documentation](https://docs.rs/brk_cli/badge.svg)](https://docs.rs/brk_cli)

## Overview

This crate provides the primary command-line interface for running Bitcoin Research Kit instances. It orchestrates the entire data processing pipeline from Bitcoin Core block parsing through analytics computation to HTTP API serving, with persistent configuration management, automatic error recovery, and continuous blockchain synchronization.

**Key Features:**

- Complete BRK pipeline orchestration with parser, indexer, computer, and server coordination
- Persistent configuration system with TOML-based auto-save functionality
- Continuous blockchain processing with new block detection and incremental updates
- Flexible Bitcoin Core RPC authentication with cookie file and user/password support
- Configurable web interface options including auto-downloading from GitHub releases
- Large stack allocation (512MB) for handling complex blockchain processing workloads
- Graceful shutdown handling with proper cleanup and state preservation

**Target Use Cases:**

- Production Bitcoin analytics deployments requiring full pipeline operation
- Development environments for Bitcoin research and analysis
- Continuous blockchain monitoring with real-time data updates
- Academic research requiring comprehensive historical blockchain datasets

## Installation

```bash
cargo install brk # or cargo install brk_cli
```

## Quick Start

```bash
# First run - configure and start processing
brk --brkdir ./data --bitcoindir ~/.bitcoin --fetch true

# Subsequent runs use saved configuration
brk

# Override specific options
brk --website none --fetch false
```

## API Overview

### Core Structure

- **`Config`**: Persistent configuration with clap-based CLI parsing and TOML serialization
- **`Bridge`**: Interface trait for generating JavaScript bridge files for web interfaces
- **`Website`**: Enum for web interface options (None, Bitview, Custom)
- **Path Functions**: Cross-platform default path resolution for Bitcoin and BRK directories

### Main Operations

**`main() -> color_eyre::Result<()>`**
Entry point with error handling setup, directory creation, logging initialization, and high-stack thread spawning.

**`run() -> color_eyre::Result<()>`**
Core processing loop handling configuration, RPC connection, component initialization, and continuous blockchain monitoring.

### Configuration Management

**Persistent Settings:**

- All CLI arguments automatically saved to `~/.brk/config.toml`
- Argument overrides update saved configuration on each run
- Cross-platform path resolution with tilde and $HOME expansion
- Validation of Bitcoin directory, blocks directory, and RPC authentication

**CLI Parameters:**

- `--bitcoindir`, `--blocksdir`, `--brkdir`: Directory configuration
- `--fetch`, `--exchanges`: Data source configuration
- `--website`: Web interface selection
- `--rpcconnect`, `--rpcport`, `--rpccookiefile`, `--rpcuser`, `--rpcpassword`: RPC settings

## Examples

### Basic Usage

```bash
# Initialize with custom directories
brk --bitcoindir /data/bitcoin --brkdir /data/brk

# Enable all features with custom RPC
brk --fetch true --exchanges true --website bitview \
    --rpcuser myuser --rpcpassword mypass

# Minimal setup with API only
brk --website none --fetch false
```

### Configuration File Example

After first run, settings are saved to `~/.brk/config.toml`:

```toml
bitcoindir = "/home/user/.bitcoin"
blocksdir = "/home/user/.bitcoin/blocks"
brkdir = "/home/user/brk_data"
fetch = true
exchanges = true
website = "bitview"
rpcconnect = "localhost"
rpcport = 8332
rpccookiefile = "/home/user/.bitcoin/.cookie"
```

### Web Interface Configuration

```bash
# Use built-in Bitview interface
brk --website bitview

# Use custom web interface
brk --website custom

# API only, no web interface
brk --website none
```

### Development Mode

```bash
# Development with local website directory
# Looks for ../../websites directory first
brk --website bitview

# Production with auto-download from GitHub
# Downloads websites from release artifacts
brk --website bitview
```

## Architecture

### Startup Sequence

1. **Environment Setup**: Color eyre error handling, directory creation, logging initialization
2. **High-Stack Thread**: 512MB stack for complex blockchain processing operations
3. **Configuration Loading**: CLI parsing, TOML reading, argument merging, validation
4. **Component Initialization**: Parser, indexer, computer, interface creation with proper dependencies

### Processing Pipeline

**Continuous Operation Loop:**

1. **Bitcoin Core Sync Wait**: Monitors `headers == blocks` for full node synchronization
2. **Block Count Detection**: Compares current and previous block counts for new block detection
3. **Indexing Phase**: Processes new blocks through parser with collision detection option
4. **Computing Phase**: Runs analytics computations on newly indexed data
5. **Server Operation**: Serves HTTP API with optional web interface throughout processing

### Web Interface Integration

**Website Handling:**

- **Development Mode**: Uses local `../../websites` directory if available
- **Production Mode**: Downloads release artifacts from GitHub using semantic versioning
- **Bundle Generation**: Creates optimized JavaScript bundles using `brk_bundler`
- **Bridge Files**: Generates JavaScript bridge files for vector IDs and pool data

**Download and Bundle Process:**

```rust
// Automatic website download and bundling
let url = format!("https://github.com/bitcoinresearchkit/brk/archive/refs/tags/v{VERSION}.zip");
let response = minreq::get(url).send()?;
zip::ZipArchive::new(cursor).extract(downloads_path)?;
bundle(&websites_path, website.to_folder_name(), true).await?
```

### RPC Authentication

**Flexible Authentication Methods:**

- **Cookie File**: Automatic detection at `--bitcoindir/.cookie`
- **User/Password**: Manual configuration with `--rpcuser` and `--rpcpassword`
- **Connection Validation**: Startup checks ensure proper Bitcoin Core connectivity

### Configuration System

**TOML Persistence:**

- Automatic serialization/deserialization with `serde` and `toml`
- Error-tolerant parsing with `default_on_error` deserializer
- Argument consumption validation ensuring all CLI options are processed
- Path expansion supporting `~` and `$HOME` environment variables

## Configuration

### Default Paths

**Cross-Platform Path Resolution:**

- **Linux**: `~/.bitcoin` for Bitcoin Core, `~/.brk` for BRK data
- **macOS**: `~/Library/Application Support/Bitcoin` for Bitcoin Core
- **Logs**: `~/.brk/log` for application logging
- **Downloads**: `~/.brk/downloads` for temporary website artifacts

### Performance Settings

**Memory Management:**

- 512MB stack size for main processing thread
- Multi-threaded tokio runtime with all features enabled
- Persistent configuration caching to minimize I/O operations

### Error Handling

**Comprehensive Validation:**

- Directory existence checks with user-friendly error messages
- RPC authentication verification before processing begins
- Graceful exit with help suggestions for configuration issues

## Code Analysis Summary

**Main Structure**: `Config` struct with clap-derived CLI parsing and persistent TOML configuration management \
**Processing Loop**: Continuous Bitcoin Core monitoring with sync detection and incremental block processing \
**Web Integration**: Automatic website download from GitHub releases with JavaScript bundle generation \
**Component Orchestration**: Coordination of parser, indexer, computer, and server with proper dependency management \
**Error Handling**: `color_eyre` integration with comprehensive validation and user-friendly error messages \
**Threading**: High-stack thread allocation (512MB) with tokio multi-threaded runtime for complex operations \
**Architecture**: Complete BRK pipeline orchestration with persistent configuration and continuous blockchain synchronization

---

_This README was generated by Claude Code_
