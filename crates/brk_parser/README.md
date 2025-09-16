# brk_parser

High-performance Bitcoin block parser for raw Bitcoin Core block files with XOR encryption support.

[![Crates.io](https://img.shields.io/crates/v/brk_parser.svg)](https://crates.io/crates/brk_parser)
[![Documentation](https://docs.rs/brk_parser/badge.svg)](https://docs.rs/brk_parser)

## Overview

This crate provides a multi-threaded Bitcoin block parser that processes raw Bitcoin Core `.dat` files from the blockchain directory. It supports XOR-encoded block data, parallel processing with `rayon`, and maintains chronological ordering through crossbeam channels. The parser integrates with Bitcoin Core RPC to validate block confirmations and handles file metadata tracking for incremental processing.

**Key Features:**

- Multi-threaded pipeline architecture with crossbeam channels
- XOR decryption support for encrypted block files
- Parallel block decoding with rayon thread pools
- Chronological block ordering with height-based validation
- Bitcoin Core RPC integration for confirmation checking
- File metadata tracking and incremental processing
- Magic byte detection for block boundary identification

**Target Use Cases:**

- Bitcoin blockchain analysis tools requiring raw block access
- Historical data processing applications
- Block explorers and analytics platforms
- Research tools needing ordered block iteration

## Installation

```toml
cargo add brk_parser
```

## Quick Start

```rust
use brk_parser::Parser;
use bitcoincore_rpc::{Client, Auth, RpcApi};
use brk_structs::Height;
use std::path::PathBuf;

// Initialize Bitcoin Core RPC client
let rpc = Box::leak(Box::new(Client::new(
    "http://localhost:8332",
    Auth::None
).unwrap()));

// Create parser with blocks directory
let blocks_dir = PathBuf::from("/path/to/bitcoin/blocks");
let outputs_dir = Some(PathBuf::from("./parser_output"));
let parser = Parser::new(blocks_dir, outputs_dir, rpc);

// Parse blocks in height range
let start_height = Some(Height::new(700000));
let end_height = Some(Height::new(700100));
let receiver = parser.parse(start_height, end_height);

// Process blocks as they arrive
for (height, block, block_hash) in receiver.iter() {
    println!("Block {}: {} transactions", height, block.txdata.len());
    println!("Block hash: {}", block_hash);
}
```

## API Overview

### Core Types

- **`Parser`**: Main parser coordinating multi-threaded block processing
- **`AnyBlock`**: Enum representing different block states (Raw, Decoded, Skipped)
- **`XORBytes`**: XOR key bytes for decrypting block data
- **`XORIndex`**: Circular index for XOR byte application
- **`BlkMetadata`**: Block file metadata including index and modification time

### Key Methods

**`Parser::new(blocks_dir: PathBuf, outputs_dir: Option<PathBuf>, rpc: &'static Client) -> Self`**
Creates a new parser instance with blockchain directory and RPC client.

**`parse(&self, start: Option<Height>, end: Option<Height>) -> Receiver<(Height, Block, BlockHash)>`**
Returns a channel receiver that yields blocks in chronological order for the specified height range.

### Processing Pipeline

The parser implements a three-stage pipeline:

1. **File Reading Stage**: Scans `.dat` files, identifies magic bytes, extracts raw block data
2. **Decoding Stage**: Parallel XOR decryption and Bitcoin block deserialization
3. **Ordering Stage**: RPC validation and chronological ordering by block height

## Examples

### Basic Block Iteration

```rust
use brk_parser::Parser;

let parser = Parser::new(blocks_dir, Some(output_dir), rpc);

// Parse all blocks from height 650000 onwards
let receiver = parser.parse(Some(Height::new(650000)), None);

for (height, block, hash) in receiver.iter() {
    println!("Processing block {} with {} transactions",
             height, block.txdata.len());

    // Process block transactions
    for (idx, tx) in block.txdata.iter().enumerate() {
        println!("  Tx {}: {}", idx, tx.txid());
    }
}
```

### Range-Based Processing

```rust
use brk_parser::Parser;

let parser = Parser::new(blocks_dir, Some(output_dir), rpc);

// Process specific block range
let start = Height::new(600000);
let end = Height::new(600999);
let receiver = parser.parse(Some(start), Some(end));

let mut total_tx_count = 0;
for (height, block, _hash) in receiver.iter() {
    total_tx_count += block.txdata.len();

    if height == end {
        break; // End of range reached
    }
}

println!("Processed 1000 blocks with {} total transactions", total_tx_count);
```

### Incremental Processing with Metadata

```rust
use brk_parser::Parser;

let parser = Parser::new(blocks_dir, Some(output_dir), rpc);

// Parser automatically handles file metadata tracking
// Only processes blocks that have been modified since last run
let receiver = parser.parse(None, None); // Process all available blocks

for (height, block, hash) in receiver.iter() {
    // Parser ensures blocks are delivered in chronological order
    // even when processing multiple .dat files in parallel

    if height.as_u32() % 10000 == 0 {
        println!("Reached block height {}", height);
    }
}
```

## Architecture

### Multi-Threading Design

The parser uses a sophisticated multi-threaded architecture:

- **File Scanner Thread**: Reads raw bytes from `.dat` files and identifies block boundaries
- **Decoder Thread Pool**: Parallel XOR decryption and block deserialization using rayon
- **Ordering Thread**: RPC validation and chronological ordering with future block buffering

### XOR Encryption Support

Bitcoin Core optionally XOR-encrypts block files using an 8-byte key stored in `xor.dat`. The parser:

- Automatically detects XOR encryption presence
- Implements circular XOR index for efficient decryption
- Supports both encrypted and unencrypted block files

### Block File Management

The parser handles Bitcoin Core's block file structure:

- Scans directory for `blk*.dat` files
- Tracks file modification times for incremental processing
- Maintains block height mappings with RPC validation
- Exports processing metadata for resumable operations

## Code Analysis Summary

**Main Type**: `Parser` struct coordinating multi-threaded block processing pipeline \
**Threading**: Three-stage pipeline using crossbeam channels with bounded capacity (50) \
**Parallelization**: rayon-based parallel block decoding with configurable batch sizes \
**XOR Handling**: Custom XORBytes and XORIndex types for efficient encryption/decryption \
**RPC Integration**: Bitcoin Core RPC validation for block confirmation and height mapping \
**File Processing**: Automatic `.dat` file discovery and magic byte boundary detection \
**Architecture**: Producer-consumer pattern with ordered delivery despite parallel processing

---

_This README was generated by Claude Code_
