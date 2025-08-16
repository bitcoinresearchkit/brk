# brk_logger

**Logging utilities with colored console output and file logging**

`brk_logger` provides a thin wrapper around `env_logger` with BRK-specific defaults, colored console output, and optional file logging. It's designed to provide clear, readable logs for Bitcoin data processing operations.

## What it provides

- **Colored Console Output**: Level-based color coding for easy visual parsing
- **File Logging**: Optional log file output with automatic rotation
- **Sensible Defaults**: Pre-configured log levels for Bitcoin and dependency crates
- **Timestamp Formatting**: Human-readable timestamps with system timezone

## Key Features

### Console Logging
- **Color-coded levels**: Error (red), Warn (yellow), Info (green), Debug (blue), Trace (cyan)
- **Formatted timestamps**: `YYYY-MM-DD HH:MM:SS` format with dimmed styling
- **Clean output**: Minimal formatting focused on readability

### File Logging
- **Optional file output**: Writes to specified file path
- **Automatic cleanup**: Removes existing log file on initialization
- **Append mode**: New log entries appended to file
- **Plain text format**: No colors in file output for better compatibility

### Dependency Filtering
Pre-configured to suppress noisy logs from common dependencies:
- `bitcoin=off` - Bitcoin protocol library
- `bitcoincore-rpc=off` - RPC client
- `fjall=off` - Key-value store
- `lsm_tree=off` - LSM tree implementation
- `rolldown=off` - Bundler
- `tracing=off` - Tracing framework

## Usage

### Basic Setup (Console Only)

```rust
use brk_logger;

// Initialize with console output only
brk_logger::init(None)?;

// Now use standard logging macros
log::info!("BRK starting up");
log::warn!("Bitcoin Core not fully synced");
log::error!("Failed to connect to RPC");
```

### With File Logging

```rust
use std::path::Path;

// Initialize with both console and file output
let log_path = Path::new("~/.brk/brk.log");
brk_logger::init(Some(log_path))?;

log::info!("Logs will appear in console and file");
```

### Environment Variable Control

```bash
# Set log level via environment variable
export RUST_LOG=debug
export RUST_LOG=info,brk_parser=debug  # Override for specific crates

# Run with custom log level
RUST_LOG=trace brk
```

### Using Color Utilities

The crate re-exports `OwoColorize` for consistent coloring:

```rust
use brk_logger::OwoColorize;

println!("Success: {}", "Operation completed".green());
println!("Warning: {}", "Low disk space".yellow());
println!("Error: {}", "Connection failed".red());
println!("Info: {}", "Processing block 800000".bright_black());
```

## Log Format

### Console Output
```
2024-12-25 10:30:15 - info  Starting BRK indexer
2024-12-25 10:30:16 - warn  Bitcoin Core still syncing (99.8% complete)
2024-12-25 10:30:45 - info  Indexed block 900000 (1.2M transactions)
2024-12-25 10:30:46 - error Connection to RPC failed, retrying...
```

### File Output
```
2024-12-25 10:30:15 - info  Starting BRK indexer
2024-12-25 10:30:16 - warn  Bitcoin Core still syncing (99.8% complete)
2024-12-25 10:30:45 - info  Indexed block 900000 (1.2M transactions)
2024-12-25 10:30:46 - error Connection to RPC failed, retrying...
```

## Default Log Levels

The logger uses these default settings:

- **Default level**: `info` - Shows important operational information
- **Suppressed crates**: Dependencies that produce excessive output are set to `off`
- **Override capability**: Can be overridden via `RUST_LOG` environment variable

### Common Log Level Settings

```bash
# Minimal output (errors and warnings only)
RUST_LOG=warn

# Standard output (recommended)
RUST_LOG=info

# Verbose output (for debugging)
RUST_LOG=debug

# Maximum output (for development)
RUST_LOG=trace

# Mixed levels (info by default, debug for specific crates)
RUST_LOG=info,brk_indexer=debug,brk_computer=trace
```

## Integration Examples

### In BRK CLI

```rust
use brk_logger;
use log::info;

fn main() -> Result<()> {
    // Initialize logging early in main
    brk_logger::init(Some(Path::new("~/.brk/brk.log")))?;
    
    info!("BRK CLI starting");
    
    // ... rest of application
    Ok(())
}
```

### In Custom Applications

```rust
use brk_logger::{self, OwoColorize};
use log::{info, warn, error};

fn setup_logging() -> std::io::Result<()> {
    // Console only for development
    brk_logger::init(None)?;
    
    info!("Application initialized");
    Ok(())
}

fn process_data() {
    info!("Processing Bitcoin data...");
    
    // Use color utilities for progress
    println!("Progress: {}", "50%".green());
    
    warn!("Large memory usage detected");
    error!("Critical error: {}", "Database connection lost".red());
}
```

## Performance Considerations

- **Minimal overhead**: Lightweight wrapper around `env_logger`
- **Lazy evaluation**: Log messages only formatted when level is enabled
- **File I/O**: Asynchronous file writing doesn't block main thread
- **Memory usage**: No buffering, logs written immediately

## Dependencies

- `env_logger` - Core logging implementation
- `owo_colors` - Terminal color support  
- `jiff` - Modern date/time handling for timestamps

---

*This README was generated by Claude Code*