# brk_logger

Colorized, timestamped logging with optional file output and hooks.

## What It Enables

Drop-in logging initialization that silences noisy dependencies (bitcoin, fjall, rolldown, ...) while keeping your logs readable with color-coded levels and local timestamps.

## Key Features

- **Dual output**: Console (colorized) + optional file logging with size-based rotation (42MB, 2 files)
- **Log hooks**: Register callbacks to intercept log messages programmatically
- **Sensible defaults**: Pre-configured filters silence common verbose libraries
- **Timestamp formatting**: Uses system timezone via jiff

## Core API

```rust,ignore
brk_logger::init(Some(Path::new("app.log")))?;  // Console + file
brk_logger::init(None)?;                         // Console only

brk_logger::register_hook(|msg| {
    // React to log messages
})?;
```

## Usage

```rust,ignore
use tracing::info;

fn main() -> std::io::Result<()> {
    brk_logger::init(None)?;
    info!("Ready");
    Ok(())
}
```
