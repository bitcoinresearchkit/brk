# brk_indexer

High-performance Bitcoin blockchain indexer with parallel processing and dual storage architecture.

[![Crates.io](https://img.shields.io/crates/v/brk_indexer.svg)](https://crates.io/crates/brk_indexer)
[![Documentation](https://docs.rs/brk_indexer/badge.svg)](https://docs.rs/brk_indexer)

## Overview

This crate provides a comprehensive Bitcoin blockchain indexer built on top of `brk_reader`. It processes raw Bitcoin blocks in parallel, extracting and indexing transactions, addresses, inputs, outputs, and metadata into optimized storage structures. The indexer maintains two complementary storage systems: columnar vectors for analytics and key-value stores for fast lookups.

**Key Features:**

- Parallel block processing with multi-threaded transaction analysis
- Dual storage architecture: columnar vectors + key-value stores
- Address type classification and indexing for all Bitcoin script types
- Collision detection and validation for address hashes and transaction IDs
- Incremental processing with automatic rollback and recovery
- Height-based synchronization with Bitcoin Core RPC validation
- Optimized batch operations with configurable snapshot intervals

**Target Use Cases:**

- Bitcoin blockchain analysis requiring full transaction history
- Address clustering and UTXO set analysis
- Blockchain explorers needing fast address/transaction lookups
- Research applications requiring structured access to blockchain data

## Installation

```bash
cargo add brk_indexer
```

## Quick Start

```rust,ignore
use brk_indexer::Indexer;
use brk_reader::Parser;
use bitcoincore_rpc::{Client, Auth};
use vecdb::Exit;
use std::path::Path;

// Initialize Bitcoin Core RPC client
let rpc = Client::new("http://localhost:8332", Auth::None)?;
let rpc = Box::leak(Box::new(rpc));

// Create parser for raw block data
let blocks_dir = Path::new("/path/to/bitcoin/blocks");
let parser = Parser::new(blocks_dir, None, rpc);

// Initialize indexer with output directory
let outputs_dir = Path::new("./indexed_data");
let mut indexer = Indexer::forced_import(outputs_dir)?;

// Index blockchain data
let exit = Exit::default();
let starting_indexes = indexer.index(&parser, rpc, &exit, true)?;

println!("Indexed up to height: {}", starting_indexes.height);
```

## API Overview

### Core Types

- **`Indexer`**: Main coordinator managing vectors and stores
- **`Vecs`**: Columnar storage for blockchain data analytics
- **`Stores`**: Key-value storage for fast hash-based lookups
- **`Indexes`**: Current indexing state tracking progress across data types

### Key Methods

**`Indexer::forced_import(outputs_dir: &Path) -> Result<Self>`**
Creates or opens indexer instance with automatic version management.

**`index(&mut self, parser: &Parser, rpc: &'static Client, exit: &Exit, check_collisions: bool) -> Result<Indexes>`**
Main indexing function processing blocks from parser with collision detection.

### Storage Architecture

**Columnar Vectors (Vecs):**

- `height_to_*`: Block-level data (hash, timestamp, difficulty, size, weight)
- `txindex_to_*`: Transaction data (ID, version, locktime, size, RBF flag)
- `txoutindex_to_*`: Output data (value, type, address mapping)
- `txinindex_to_txoutindex`: Input-to-output relationship mapping

**Key-Value Stores:**

- `addresshash_to_typeindex`: Address hash to internal index mapping
- `blockhashprefix_to_height`: Block hash prefix to height lookup
- `txidprefix_to_txindex`: Transaction ID prefix to internal index
- `addresstype_to_typeindex_with_txoutindex`: Address type to output mappings

### Address Type Support

Complete coverage of Bitcoin script types:

- **P2PK**: Pay-to-Public-Key (33-byte and 65-byte variants)
- **P2PKH**: Pay-to-Public-Key-Hash
- **P2SH**: Pay-to-Script-Hash
- **P2WPKH**: Pay-to-Witness-Public-Key-Hash
- **P2WSH**: Pay-to-Witness-Script-Hash
- **P2TR**: Pay-to-Taproot
- **P2MS**: Pay-to-Multisig
- **P2A**: Pay-to-Address (custom type)
- **OpReturn**: OP_RETURN data outputs
- **Empty/Unknown**: Non-standard script types

## Examples

### Basic Indexing Operation

```rust,ignore
use brk_indexer::Indexer;
use brk_reader::Parser;
use std::path::Path;

// Initialize components
let outputs_dir = Path::new("./blockchain_index");
let mut indexer = Indexer::forced_import(outputs_dir)?;

let blocks_dir = Path::new("/Users/satoshi/.bitcoin/blocks");
let parser = Parser::new(blocks_dir, None, rpc);

// Index with collision checking enabled
let exit = vecdb::Exit::default();
let final_indexes = indexer.index(&parser, rpc, &exit, true)?;

println!("Final height: {}", final_indexes.height);
println!("Total transactions: {}", final_indexes.txindex);
println!("Total addresses: {}", final_indexes.total_address_count());
```

### Querying Indexed Data

```rust,ignore
use brk_indexer::Indexer;
use brk_types::{Height, TxidPrefix, AddressHash};

let indexer = Indexer::forced_import("./blockchain_index")?;

// Look up block hash by height
let height = Height::new(750000);
if let Some(block_hash) = indexer.vecs.block.height_to_blockhash.get(height)? {
    println!("Block 750000 hash: {}", block_hash);
}

// Look up transaction by ID prefix
let txid_prefix = TxidPrefix::from_str("abcdef123456")?;
if let Some(tx_index) = indexer.stores.txidprefix_to_txindex.get(&txid_prefix)? {
    println!("Transaction index: {}", tx_index);
}

// Query address information
let address_hash = AddressHash::from(/* address bytes */);
if let Some(type_index) = indexer.stores.addresshash_to_typeindex.get(&address_hash)? {
    println!("Address type index: {}", type_index);
}
```

### Incremental Processing

```rust,ignore
use brk_indexer::Indexer;

// Indexer automatically resumes from last processed height
let mut indexer = Indexer::forced_import("./blockchain_index")?;

let current_indexes = indexer.vecs.current_indexes(&indexer.stores, rpc)?;
println!("Resuming from height: {}", current_indexes.height);

// Process new blocks incrementally
let exit = vecdb::Exit::default();
let updated_indexes = indexer.index(&parser, rpc, &exit, true)?;

println!("Processed {} new blocks",
         updated_indexes.height.as_u32() - current_indexes.height.as_u32());
```

### Address Type Analysis

```rust,ignore
use brk_indexer::Indexer;
use brk_types::OutputType;

let indexer = Indexer::forced_import("./blockchain_index")?;

// Analyze address distribution by type
for output_type in OutputType::as_vec() {
    let count = indexer.vecs.txout.txoutindex_to_outputtype
        .iter()
        .filter(|&ot| ot == output_type)
        .count();

    println!("{:?}: {} outputs", output_type, count);
}

// Query specific address type data
let p2pkh_store = &indexer.stores.addresstype_to_typeindex_with_txoutindex
    .p2pkh;

println!("P2PKH addresses: {}", p2pkh_store.len());
```

## Architecture

### Parallel Processing

The indexer uses sophisticated parallel processing:

- **Block-Level Parallelism**: Concurrent processing of transactions within blocks
- **Transaction Analysis**: Parallel input/output processing with `rayon`
- **Address Resolution**: Multi-threaded address type classification and indexing
- **Collision Detection**: Parallel validation of hash collisions across address types

### Storage Optimization

**Columnar Storage (vecdb):**

- Compressed vectors for space-efficient analytics queries
- Raw vectors for frequently accessed data (heights, hashes)
- Page-aligned storage for memory mapping efficiency

**Key-Value Storage (Fjall):**

- LSM-tree architecture for write-heavy indexing workloads
- Bloom filters for fast negative lookups
- Transactional consistency with rollback support

### Memory Management

- **Batch Processing**: 1000-block snapshots to balance memory and I/O
- **Reader Management**: Static readers for consistent data access during processing
- **Collision Tracking**: BTreeMap-based collision detection with memory cleanup
- **Exit Handling**: Graceful shutdown with consistent state preservation

### Version Management

- **Schema Versioning**: Automatic migration on version changes (currently v21)
- **Rollback Support**: Automatic recovery from incomplete processing
- **State Tracking**: Height-based synchronization across all storage components

## Performance Characteristics

### Processing Speed

- **Parallel Transaction Processing**: Multi-core utilization for CPU-intensive operations
- **Optimized I/O**: Batch operations reduce disk overhead
- **Memory Efficiency**: Streaming processing without loading entire blockchain

### Storage Requirements

- **Columnar Compression**: Significant space savings for repetitive blockchain data
- **Index Optimization**: Bloom filters reduce lookup overhead
- **Incremental Growth**: Storage scales linearly with blockchain size

### Scalability

- **Height-Based Partitioning**: Enables distributed processing strategies
- **Modular Architecture**: Separate vector and store systems for flexible deployment
- **Resource Configuration**: Configurable batch sizes and memory limits

## Code Analysis Summary

**Main Structure**: `Indexer` coordinating `Vecs` (columnar analytics) and `Stores` (key-value lookups) \
**Processing Pipeline**: Multi-threaded block analysis with parallel transaction/address processing \
**Storage Architecture**: Dual system using vecdb for analytics and Fjall for lookups \
**Address Indexing**: Complete Bitcoin script type coverage with collision detection \
**Synchronization**: Height-based coordination with Bitcoin Core RPC validation \
**Parallel Processing**: rayon-based parallelism for transaction analysis and address resolution \
**Architecture**: High-performance blockchain indexer with ACID guarantees and incremental processing

---

_This README was generated by Claude Code_
