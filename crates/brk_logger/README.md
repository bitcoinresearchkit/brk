# brk_logger

Colored console logging with optional file output for Bitcoin Research Kit applications.

[![Crates.io](https://img.shields.io/crates/v/brk_logger.svg)](https://crates.io/crates/brk_logger)
[![Documentation](https://docs.rs/brk_logger/badge.svg)](https://docs.rs/brk_logger)

## Overview

This crate provides a thin wrapper around `env_logger` with enhanced formatting, colored output, and optional file logging. Designed specifically for BRK applications, it offers structured logging with timestamps, level-based coloring, and automatic filtering of noisy dependencies.

**Key Features:**

- Level-based color coding for console output (error=red, warn=yellow, info=green, etc.)
- Dual output: colored console logs and optional plain-text file logging
- System timezone-aware timestamps with customizable formatting
- Automatic filtering of verbose dependencies (bitcoin, fjall, tracing, etc.)
- Environment variable configuration support via `RUST_LOG`

**Target Use Cases:**

- Bitcoin blockchain analysis applications requiring structured logging
- Development and debugging of BRK components
- Production deployments with file-based log archival
- Applications needing both human-readable and machine-parseable logs

## Installation

```bash
cargo add brk_logger
```

## Quick Start

```rust
use brk_logger;

// Initialize with console output only
brk_logger::init(None)?;

// Initialize with file logging
let log_path = std::path::Path::new("application.log");
brk_logger::init(Some(log_path))?;

// Use standard log macros
log::info!("Application started");
log::warn!("Configuration issue detected");
log::error!("Failed to process block");
```

## API Overview

### Core Functions

**`init(path: Option<&Path>) -> io::Result<()>`**
Initializes the logging system with optional file output. Console logging is always enabled with color formatting.

**Re-exported Types:**

- **`OwoColorize`**: Color formatting trait for custom colored output

### Log Formatting

**Console Output Format:**

```
2024-09-16 14:23:45 - INFO  Application started successfully
2024-09-16 14:23:46 - WARN  Bitcoin RPC connection slow
2024-09-16 14:23:47 - ERROR Failed to parse block data
```

**File Output Format (Plain Text):**

```
2024-09-16 14:23:45 - info  Application started successfully
2024-09-16 14:23:46 - warn  Bitcoin RPC connection slow
2024-09-16 14:23:47 - error Failed to parse block data
```

### Default Filtering

The logger automatically filters out verbose logs from common dependencies:

- `bitcoin=off` - Bitcoin library internals
- `bitcoincore-rpc=off` - RPC client details
- `fjall=off` - Database engine logs
- `lsm_tree=off` - LSM tree operations
- `tracing=off` - Tracing framework internals

## Examples

### Basic Logging Setup

```rust
use brk_logger;
use log::{info, warn, error};

fn main() -> std::io::Result<()> {
    // Initialize logging to console only
    brk_logger::init(None)?;

    info!("Starting Bitcoin analysis");
    warn!("Price feed temporarily unavailable");
    error!("Block parsing failed at height 750000");

    Ok(())
}
```

### File Logging with Custom Path

```rust
use brk_logger;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let log_file = Path::new("/var/log/brk/application.log");

    // Initialize with dual output: console + file
    brk_logger::init(Some(log_file))?;

    log::info!("Application configured with file logging");

    // Both console (colored) and file (plain) will receive this log
    log::error!("Critical system error occurred");

    Ok(())
}
```

### Environment Variable Configuration

```bash
# Set custom log level and targets
export RUST_LOG="debug,brk_indexer=trace,bitcoin=warn"

# Run application - logger respects RUST_LOG
./my_brk_app
```

```rust
use brk_logger;

fn main() -> std::io::Result<()> {
    // Respects RUST_LOG environment variable
    brk_logger::init(None)?;

    log::debug!("Debug information (only shown if RUST_LOG=debug)");
    log::trace!("Trace information (only for specific modules)");

    Ok(())
}
```

### Colored Output Usage

```rust
use brk_logger::OwoColorize;

fn print_status() {
    println!("Status: {}", "Connected".green());
    println!("Height: {}", "800000".bright_blue());
    println!("Error:  {}", "Connection failed".red());
}
```

## Architecture

### Logging Pipeline

1. **Initialization**: `init()` configures `env_logger` with custom formatter
2. **Environment**: Reads `RUST_LOG` or uses default filter configuration
3. **Formatting**: Applies timestamp, level formatting, and color coding
4. **Dual Output**: Writes colored logs to console, plain logs to file (if configured)

### Color Scheme

- **ERROR**: Red text for critical failures
- **WARN**: Yellow text for warnings and issues
- **INFO**: Green text for normal operations
- **DEBUG**: Blue text for debugging information
- **TRACE**: Cyan text for detailed tracing
- **Timestamps**: Bright black (gray) for visual hierarchy

### File Handling

- **File Creation**: Creates log file if it doesn't exist
- **Append Mode**: Appends to existing files without truncation
- **Error Resilience**: Console logging continues even if file write fails
- **File Rotation**: Manual - application responsible for log rotation

### Filtering Strategy

Default filter prioritizes BRK application logs while suppressing:

- Verbose dependency logging that clutters output
- Internal library debug information not relevant to users
- Framework-level tracing that doesn't aid in debugging BRK logic

## Code Analysis Summary

**Main Function**: `init()` function that configures `env_logger` with custom formatting and dual output \
**Dependencies**: Built on `env_logger` for filtering, `jiff` for timestamps, `owo-colors` for terminal colors \
**Output Streams**: Simultaneous console (colored) and file (plain text) logging with different formatting \
**Timestamp Format**: System timezone-aware formatting using `%Y-%m-%d %H:%M:%S` pattern \
**Color Implementation**: Level-based color mapping with `owo-colors` for terminal output \
**Filter Configuration**: Predefined filter string that suppresses verbose dependencies while enabling BRK logs \
**Architecture**: Thin wrapper pattern that enhances `env_logger` with BRK-specific formatting and behavior

---

_This README was generated by Claude Code_
