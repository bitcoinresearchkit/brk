# brk_logger

Colorful logging utility built on `env_logger` that provides clean, timestamped console output with optional file logging. This crate wraps `env_logger` to display logs from the `log` crate in a readable format with color-coded log levels and configurable filtering to suppress noisy third-party library logs.

## Features

- **Colorized output**: Log levels are color-coded (error=red, warn=yellow, info=green, debug=blue, trace=cyan)
- **Timestamps**: Each log entry includes a formatted timestamp
- **File logging**: Optional file output alongside console logging
- **Noise filtering**: Pre-configured to suppress verbose logs from Bitcoin Core RPC and other dependencies
- **Environment control**: Respects `RUST_LOG` environment variable for custom filtering

## Usage

```rust
use log::info;

fn main() -> std::io::Result<()> {
    // Initialize with console output only
    brk_logger::init(None)?;

    // Or initialize with file logging
    brk_logger::init(Some(std::path::Path::new("app.log")))?;

    info!("Application started");
    Ok(())
}
```

## Default Log Filtering

By default, the following crates are filtered to `off` to reduce noise:
- `bitcoin`, `bitcoincore-rpc` - Bitcoin Core libraries
- `fjall`, `lsm_tree` - Storage engine logs
- `rolldown`, `brk_rolldown` - Bundler logs
- `rmcp`, `brk_rmcp` - MCP protocol logs
- `tracing` - Tracing framework logs
