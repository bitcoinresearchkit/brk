# brk

Unified Bitcoin Research Kit crate providing optional feature-gated access to all BRK components.

[![Crates.io](https://img.shields.io/crates/v/brk.svg)](https://crates.io/crates/brk)
[![Documentation](https://docs.rs/brk/badge.svg)](https://docs.rs/brk)

## Overview

This crate serves as a unified entry point to the Bitcoin Research Kit ecosystem, providing feature-gated re-exports of all BRK components. It allows users to selectively include only the functionality they need while maintaining a single dependency declaration, with the `brk_cli` component always available for command-line interface access.

**Key Features:**

- Feature-gated modular access to 12 specialized BRK components
- Single dependency entry point with selective compilation
- Always-available CLI component for command-line operations
- Comprehensive documentation aggregation with inline re-exports
- `full` feature for complete BRK functionality inclusion
- Optimized build configuration with docs.rs integration

**Target Use Cases:**

- Applications requiring selective BRK functionality to minimize dependencies
- Library development where only specific Bitcoin analysis components are needed
- Prototyping and experimentation with different BRK component combinations
- Educational use cases demonstrating modular blockchain analytics architecture

## Installation

```toml
[dependencies]
# Minimal installation with CLI only
brk = "0.0.107"

# Full functionality
brk = { version = "0.0.107", features = ["full"] }

# Selective features
brk = { version = "0.0.107", features = ["indexer", "computer", "server"] }
```

## Quick Start

```rust
// CLI is always available
use brk::cli;

// Feature-gated components
#[cfg(feature = "indexer")]
use brk::indexer::Indexer;

#[cfg(feature = "computer")]
use brk::computer::Computer;

#[cfg(feature = "server")]
use brk::server::Server;

// Build complete pipeline with selected features
#[cfg(all(feature = "indexer", feature = "computer", feature = "server"))]
fn build_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    let indexer = Indexer::build("./data")?;
    let computer = Computer::build("./analytics", &indexer)?;
    let interface = brk::interface::Interface::build(&indexer, &computer);
    let server = Server::new(interface, None);
    Ok(())
}
```

## API Overview

### Feature Organization

The crate provides feature-gated access to BRK components organized by functionality:

**Core Data Processing:**
- `structs` - Bitcoin-aware data structures and type system
- `error` - Centralized error handling across components
- `store` - Transactional key-value storage wrapper

**Blockchain Processing:**
- `parser` - Multi-threaded Bitcoin block parsing
- `indexer` - Blockchain data indexing with columnar storage
- `computer` - Analytics computation engine

**Data Access:**
- `interface` - Unified query interface with fuzzy search
- `fetcher` - Multi-source price data aggregation

**Service Layer:**
- `server` - HTTP API server with caching and compression
- `mcp` - Model Context Protocol bridge for LLM integration
- `logger` - Enhanced logging with colored output

**Web Infrastructure:**
- `bundler` - Web asset bundling using Rolldown

### Always Available

**`cli`**: Command-line interface module (no feature gate required)
Provides access to the complete BRK command-line interface for running full instances.

## Examples

### Minimal Bitcoin Parser

```rust
use brk::parser::Parser;

fn parse_blocks() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "parser")]
    {
        let parser = Parser::new("/path/to/blocks", None, rpc_client);
        // Parse blockchain data
        Ok(())
    }
    #[cfg(not(feature = "parser"))]
    {
        Err("Parser feature not enabled".into())
    }
}
```

### Analytics Pipeline

```rust
use brk::{indexer, computer, interface};

#[cfg(all(feature = "indexer", feature = "computer", feature = "interface"))]
fn analytics_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize indexer
    let indexer = indexer::Indexer::build("./blockchain_data")?;

    // Compute analytics
    let computer = computer::Computer::build("./analytics", &indexer)?;

    // Create query interface
    let interface = interface::Interface::build(&indexer, &computer);

    // Query latest price
    let params = interface::Params {
        index: "date".to_string(),
        ids: vec!["price-close".to_string()].into(),
        from: Some(-1),
        ..Default::default()
    };

    let result = interface.search_and_format(params)?;
    println!("Latest Bitcoin price: {:?}", result);

    Ok(())
}
```

### Web Server Setup

```rust
use brk::{server, interface, indexer, computer};

#[cfg(all(feature = "server", feature = "interface", feature = "indexer", feature = "computer"))]
async fn web_server() -> Result<(), Box<dyn std::error::Error>> {
    let indexer = indexer::Indexer::build("./data")?;
    let computer = computer::Computer::build("./analytics", &indexer)?;
    let interface = interface::Interface::build(&indexer, &computer);

    let server = server::Server::new(interface, None);
    server.serve(true).await?;

    Ok(())
}
```

### MCP Integration

```rust
use brk::{mcp, interface, indexer, computer};

#[cfg(all(feature = "mcp", feature = "interface"))]
fn mcp_server(interface: &'static interface::Interface) -> mcp::MCP {
    mcp::MCP::new(interface)
}
```

## Feature Combinations

### Common Combinations

**Data Processing**: `["structs", "parser", "indexer"]`
Basic blockchain data processing and indexing.

**Analytics**: `["indexer", "computer", "fetcher", "interface"]`
Complete analytics pipeline with price data integration.

**API Server**: `["interface", "server", "logger"]`
HTTP API server with logging capabilities.

**Full Stack**: `["full"]`
All components for complete BRK functionality.

### Dependency Optimization

Feature selection allows for significant dependency reduction:

```toml
# Minimal parser-only dependency
brk = { version = "0.0.107", features = ["parser"] }

# Analytics without web server
brk = { version = "0.0.107", features = ["indexer", "computer", "interface"] }

# Web server without parsing
brk = { version = "0.0.107", features = ["interface", "server"] }
```

## Architecture

### Re-export Pattern

The crate uses `#[doc(inline)]` re-exports to provide seamless access to component APIs:

```rust
#[cfg(feature = "component")]
#[doc(inline)]
pub use brk_component as component;
```

This pattern ensures:
- Feature-gated compilation for dependency optimization
- Inline documentation for unified API reference
- Namespace preservation for component-specific functionality

### Build Configuration

- **Documentation**: `all-features = true` for complete docs.rs documentation
- **CLI Integration**: `brk_cli` always available without feature gates
- **Optional Dependencies**: All components except CLI are optional

## Configuration

### Feature Flags

| Feature | Component | Description |
|---------|-----------|-------------|
| `bundler` | `brk_bundler` | Web asset bundling |
| `computer` | `brk_computer` | Analytics computation |
| `error` | `brk_error` | Error handling |
| `fetcher` | `brk_fetcher` | Price data fetching |
| `indexer` | `brk_indexer` | Blockchain indexing |
| `interface` | `brk_interface` | Data query interface |
| `logger` | `brk_logger` | Enhanced logging |
| `mcp` | `brk_mcp` | Model Context Protocol |
| `parser` | `brk_reader` | Block parsing |
| `server` | `brk_server` | HTTP server |
| `store` | `brk_store` | Key-value storage |
| `structs` | `brk_structs` | Data structures |
| `full` | All components | Complete functionality |

### Documentation

Documentation is aggregated from all components with `#![doc = include_str!("../README.md")]` ensuring comprehensive API reference across all features.

## Code Analysis Summary

**Main Structure**: Feature-gated re-export crate providing unified access to 12 BRK components \
**Feature System**: Cargo features enabling selective compilation and dependency optimization \
**CLI Integration**: Always-available `brk_cli` access without feature requirements \
**Documentation**: Inline re-exports with comprehensive docs.rs integration \
**Dependency Management**: Optional dependencies for all components except CLI \
**Build Configuration**: Optimized compilation with all-features documentation \
**Architecture**: Modular aggregation crate enabling flexible BRK ecosystem usage

---

_This README was generated by Claude Code_
