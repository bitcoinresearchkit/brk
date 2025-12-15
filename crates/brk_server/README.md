# brk_server

HTTP server for the Bitcoin Research Kit.

[![Crates.io](https://img.shields.io/crates/v/brk_server.svg)](https://crates.io/crates/brk_server)
[![Documentation](https://docs.rs/brk_server/badge.svg)](https://docs.rs/brk_server)

## Overview

This crate provides an HTTP server that exposes BRK's blockchain data through a REST API. It serves as the web interface layer for the Bitcoin Research Kit, making data accessible to applications, dashboards, and research tools.

Built on `axum` with automatic OpenAPI documentation via Scalar.

## Usage

```rust
use brk_server::Server;
use brk_query::AsyncQuery;

let query = AsyncQuery::build(&reader, &indexer, &computer, Some(mempool));
let server = Server::new(&query, None);

// Starts on port 3110 (or next available)
server.serve(true).await?;
```

Once running:
- **API Documentation**: `http://localhost:3110/api`
- **OpenAPI Spec**: `http://localhost:3110/api.json`

## Features

- **REST API** for addresses, blocks, transactions, mempool, mining stats, and metrics
- **OpenAPI documentation** with interactive Scalar UI
- **Multiple formats**: JSON and CSV output
- **HTTP caching**: ETag-based conditional requests
- **Compression**: Gzip, Brotli, Deflate, Zstd
- **MCP support**: Model Context Protocol for AI integrations
- **Static file serving**: Optional web interface hosting
