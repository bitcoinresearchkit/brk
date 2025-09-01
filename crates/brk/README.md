# brk

**Main wrapper crate for the Bitcoin Research Kit (BRK)**

The `brk` crate serves as the primary entry point for the Bitcoin Research Kit, providing a unified interface to all BRK components through feature flags. It enables developers to selectively include only the components they need while maintaining a consistent API.

## What it provides

- **Unified Access**: Single crate providing access to the entire BRK ecosystem
- **Feature-based Selection**: Choose only the components you need
- **Consistent Versioning**: All components versioned together for compatibility
- **Simplified Dependencies**: Single dependency instead of multiple individual crates

## Available Components

### Core Data Pipeline
- **`parser`** ([brk_parser](../brk_parser/)) - High-performance Bitcoin block parser
- **`indexer`** ([brk_indexer](../brk_indexer/)) - Blockchain data indexer with dual storage
- **`computer`** ([brk_computer](../brk_computer/)) - Analytics engine for computed datasets
- **`interface`** ([brk_interface](../brk_interface/)) - Unified data query and formatting API

### Infrastructure
- **`structs`** ([brk_structs](../brk_structs/)) - Bitcoin-aware type system and data structures
- **`error`** ([brk_error](../brk_error/)) - Centralized error handling
- **`store`** ([brk_store](../brk_store/)) - Blockchain-aware key-value storage

### External Integration
- **`fetcher`** ([brk_fetcher](../brk_fetcher/)) - Bitcoin price data fetcher with multi-source fallback
- **`server`** ([brk_server](../brk_server/)) - HTTP server with REST API
- **`mcp`** ([brk_mcp](../brk_mcp/)) - Model Context Protocol for LLM integration

### Utilities
- **`cli`** ([brk_cli](../brk_cli/)) - Command line interface (always enabled)
- **`logger`** ([brk_logger](../brk_logger/)) - Logging utilities
- **`bundler`** ([brk_bundler](../brk_bundler/)) - Asset bundling for web interfaces

## Usage

### Full Installation

```toml
[dependencies]
brk = { version = "0.0.88", features = ["full"] }
```

```rust
use brk::*;

// Access all components
let config = cli::Config::load()?;
let parser = parser::Parser::new(/* ... */);
let indexer = indexer::Indexer::forced_import("./data")?;
let computer = computer::Computer::forced_import("./data", &indexer, None)?;
```

### Selective Components

```toml
[dependencies]
brk = { version = "0.0.88", features = ["parser", "indexer", "computer"] }
```

```rust
use brk::{parser, indexer, computer};

// Core data pipeline only
let parser = parser::Parser::new(blocks_dir, Some(output_dir), rpc);
let mut indexer = indexer::Indexer::forced_import(output_dir)?;
let mut computer = computer::Computer::forced_import(output_dir, &indexer, None)?;
```

### Minimal Setup

```toml
[dependencies]
brk = { version = "0.0.88", features = ["structs", "parser"] }
```

```rust
use brk::{structs, parser};

// Just parsing and types
let height = structs::Height::new(800_000);
let parser = parser::Parser::new(blocks_dir, Some(output_dir), rpc);
```

## Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `full` | Enable all components | All crates |
| `cli` | Command line interface | Always enabled |
| `structs` | Core type system | Foundation for other crates |
| `error` | Error handling | Used by most crates |
| `parser` | Block parsing | `structs`, `error` |
| `store` | Key-value storage | `structs`, `error` |
| `indexer` | Blockchain indexing | `parser`, `store` |
| `computer` | Analytics computation | `indexer`, `fetcher` (optional) |
| `fetcher` | Price data fetching | `structs`, `error` |
| `interface` | Data query API | `indexer`, `computer` |
| `server` | HTTP server | `interface`, `mcp` |
| `mcp` | LLM integration | `interface` |
| `logger` | Logging utilities | Standalone |
| `bundler` | Asset bundling | Standalone |

## Common Usage Patterns

### Complete BRK Instance

```rust
use brk::*;

// Full data pipeline setup
let config = cli::Config::load()?;
let rpc = /* Bitcoin Core RPC client */;
let parser = parser::Parser::new(config.blocks_dir, Some(config.output_dir), rpc);
let mut indexer = indexer::Indexer::forced_import(&config.output_dir)?;
let mut computer = computer::Computer::forced_import(&config.output_dir, &indexer, None)?;
let interface = interface::Interface::build(&indexer, &computer);
let server = server::Server::new(interface, config.website_path);

// Start server
server.serve(true).await?;
```

### Data Analysis

```rust
use brk::{indexer, computer, interface};

// Analysis-focused setup
let indexer = indexer::Indexer::forced_import("./brk_data")?;
let computer = computer::Computer::forced_import("./brk_data", &indexer, None)?;
let interface = interface::Interface::build(&indexer, &computer);

// Query data
let params = interface::Params {
    index: interface::Index::Height,
    ids: vec!["price_usd", "difficulty"].into(),
    rest: interface::ParamsOpt::default()
        .set_from(-100)
        .set_format(interface::Format::CSV),
};

let csv_data = interface.search_and_format(params)?;
```

### Custom Integration

```rust
use brk::{structs, parser, error};

// Custom application with BRK components
fn analyze_blocks() -> error::Result<()> {
    let parser = parser::Parser::new(blocks_dir, Some(output_dir), rpc);

    parser.parse(None, None)
        .iter()
        .take(1000)  // First 1000 blocks
        .for_each(|(height, block, hash)| {
            println!("Block {}: {} transactions", height, block.txdata.len());
        });

    Ok(())
}
```

## Version Compatibility

All BRK crates are released together with synchronized versions. When using the `brk` wrapper crate, you're guaranteed compatibility between all components.

- **Current version**: 0.0.88
- **Rust MSRV**: 1.89+
- **Bitcoin Core**: v25.0 - v29.0

## Performance Characteristics

The `brk` crate itself adds no runtime overhead - it simply re-exports the underlying crates. Performance characteristics depend on which components you use:

- **Full pipeline**: ~13-15 hours initial sync, ~40GB storage overhead
- **Parser only**: ~4 minutes to parse entire blockchain
- **Indexer only**: ~7-8 hours to index blockchain, ~5-6GB RAM usage
- **Server**: Low latency API responses with caching and compression

## Dependencies

The `brk` crate's dependencies are determined by enabled features. Core dependencies include:

- `brk_cli` - Always included for configuration and CLI support
- Individual `brk_*` crates based on enabled features
- Transitive dependencies from enabled components

For specific dependency information, see individual crate READMEs.

---

*This README was generated by Claude Code*
