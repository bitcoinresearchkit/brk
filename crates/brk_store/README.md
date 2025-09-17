# brk_store

High-performance transactional key-value store wrapper around Fjall with blockchain-aware versioning.

[![Crates.io](https://img.shields.io/crates/v/brk_store.svg)](https://crates.io/crates/brk_store)
[![Documentation](https://docs.rs/brk_store/badge.svg)](https://docs.rs/brk_store)

## Overview

This crate provides a type-safe wrapper around the Fjall LSM-tree database engine, specifically designed for Bitcoin blockchain data storage. It offers transactional operations, automatic version management, height-based synchronization, and optimized configuration for blockchain workloads with support for batch operations and efficient range queries.

**Key Features:**

- Transactional key-value storage with ACID guarantees
- Blockchain height-based synchronization and versioning
- Automatic metadata management with version compatibility checking
- Optimized configuration for Bitcoin data patterns (32MB write buffers, 8MB memtables)
- Type-safe generic interface with zero-copy ByteView integration
- Batch operations with deferred commits for performance
- Optional bloom filter configuration for space/speed tradeoffs

**Target Use Cases:**

- Bitcoin blockchain indexing with transactional consistency
- UTXO set management requiring atomic updates
- Address-to-transaction mapping with range queries
- Any Bitcoin data storage requiring versioned, transactional access

## Installation

```bash
cargo add brk_store
```

## Quick Start

```rust
use brk_store::{Store, open_keyspace};
use brk_structs::{Height, Version};
use std::path::Path;

// Open keyspace (database instance)
let keyspace = open_keyspace(Path::new("./data"))?;

// Create typed store for height-to-blockhash mapping
let mut store: Store<Height, BlockHash> = Store::import(
    &keyspace,
    Path::new("./data/height_to_hash"),
    "height-to-blockhash",
    Version::ONE,
    Some(true), // Enable bloom filters
)?;

// Insert data (batched in memory)
store.insert_if_needed(
    Height::new(750000),
    block_hash,
    Height::new(750000)
);

// Commit transaction to disk
store.commit(Height::new(750000))?;

// Query data
if let Some(hash) = store.get(&Height::new(750000))? {
    println!("Block hash: {}", hash);
}
```

## API Overview

### Core Types

- **`Store<Key, Value>`**: Generic transactional store with type-safe operations
- **`AnyStore`**: Trait for height-based synchronization and metadata operations
- **`StoreMeta`**: Version and height metadata management
- **`TransactionalKeyspace`**: Fjall keyspace wrapper for database management

### Key Methods

**`Store::import(keyspace, path, name, version, bloom_filters) -> Result<Self>`**
Creates or opens a store with automatic version checking and migration.

**`get(&self, key: &Key) -> Result<Option<Cow<Value>>>`**
Retrieves value by key, checking both pending writes and committed data.

**`insert_if_needed(&mut self, key: Key, value: Value, height: Height)`**
Conditionally inserts data based on blockchain height requirements.

**`commit(&mut self, height: Height) -> Result<()>`**
Atomically commits all pending operations and updates metadata.

### Height-Based Synchronization

The store implements blockchain-aware synchronization:

- **`has(height)`**: Checks if store contains data up to specified height
- **`needs(height)`**: Determines if store requires data for specified height
- **`height()`**: Returns current synchronized height

## Examples

### Basic Key-Value Operations

```rust
use brk_store::{Store, open_keyspace};
use brk_structs::{Height, TxId, Version};

let keyspace = open_keyspace(Path::new("./blockchain_data"))?;

// Create store for transaction index
let mut tx_store: Store<TxId, Height> = Store::import(
    &keyspace,
    Path::new("./blockchain_data/txid_to_height"),
    "txid-to-height",
    Version::ONE,
    Some(true),
)?;

// Insert transaction mapping
let txid = TxId::from_str("abcdef...")?;
let height = Height::new(800000);

tx_store.insert_if_needed(txid, height, height);
tx_store.commit(height)?;

// Query transaction height
if let Some(tx_height) = tx_store.get(&txid)? {
    println!("Transaction {} found at height {}", txid, tx_height);
}
```

### Batch Processing with Height Synchronization

```rust
use brk_store::{Store, AnyStore};

let mut store: Store<Address, AddressData> = Store::import(/* ... */)?;

// Process blocks sequentially
for block_height in 750000..750100 {
    let height = Height::new(block_height);

    // Skip if already processed
    if store.has(height) {
        continue;
    }

    // Process block transactions
    for (address, data) in process_block(block_height)? {
        store.insert_if_needed(address, data, height);
    }

    // Commit entire block atomically
    store.commit(height)?;

    println!("Processed block {}", block_height);
}

// Ensure data is persisted to disk
store.persist()?;
```

### Version Migration and Reset

```rust
use brk_store::{Store, AnyStore};
use brk_structs::Version;

// Open store with new version
let mut store: Store<Height, Data> = Store::import(
    &keyspace,
    path,
    "my-store",
    Version::TWO, // Upgraded from Version::ONE
    Some(false),  // Disable bloom filters for space
)?;

// Check if reset is needed for data consistency
if store.version() != Version::TWO {
    println!("Resetting store for version compatibility");
    store.reset()?;
}

// Verify store is empty after reset
assert!(store.is_empty()?);
```

### Iterator-Based Data Access

```rust
use brk_store::Store;

let store: Store<Height, BlockHash> = Store::import(/* ... */)?;

// Iterate over all key-value pairs
for (height, block_hash) in store.iter() {
    println!("Height {}: {}", height, block_hash);

    // Process in chunks for memory efficiency
    if height.as_u32() % 10000 == 0 {
        println!("Processed up to height {}", height);
    }
}
```

## Architecture

### Storage Engine

Built on Fjall LSM-tree engine with optimizations:

- **Write Buffers**: 32MB for high-throughput blockchain ingestion
- **Memtables**: 8MB for balanced memory usage
- **Manual Journal Persist**: Explicit control over durability guarantees
- **Bloom Filters**: Configurable for read-heavy vs. space-constrained workloads

### Transaction Model

- **Read Transactions**: Consistent point-in-time snapshots
- **Write Transactions**: ACID-compliant with rollback support
- **Batch Operations**: In-memory accumulation with atomic commits
- **Height Synchronization**: Blockchain-aware conflict resolution

### Version Management

Automatic handling of schema evolution:

1. **Version Detection**: Reads stored version from metadata
2. **Compatibility Check**: Compares with expected version
3. **Migration**: Automatic store reset for incompatible versions
4. **Metadata Update**: Persistent version tracking

### Memory Management

- **Zero-Copy**: ByteView integration for efficient serialization
- **Copy-on-Write**: Cow<Value> for memory-efficient reads
- **Parking Lot**: RwLock for concurrent partition access
- **Deferred Operations**: BTreeMap/BTreeSet for batched writes

## Configuration

### Keyspace Options

```rust
use fjall::Config;

let keyspace = Config::new(path)
    .max_write_buffer_size(32 * 1024 * 1024)  // 32MB write buffers
    .open_transactional()?;
```

### Partition Options

```rust
use fjall::PartitionCreateOptions;

let options = PartitionCreateOptions::default()
    .max_memtable_size(8 * 1024 * 1024)  // 8MB memtables
    .manual_journal_persist(true)        // Manual sync control
    .bloom_filter_bits(None);            // Disable bloom filters
```

## Code Analysis Summary

**Main Structure**: `Store<Key, Value>` generic wrapper around Fjall with typed operations and metadata management \
**Transaction Layer**: Read/write transaction abstraction with deferred batch operations via BTreeMap/BTreeSet \
**Metadata System**: `StoreMeta` for version compatibility and height tracking with automatic migration \
**Height Synchronization**: Blockchain-aware operations with `needs()`, `has()`, and conditional insertion logic \
**Memory Efficiency**: Zero-copy ByteView integration with parking_lot RwLock for concurrent access \
**Storage Engine**: Fjall LSM-tree with optimized configuration for blockchain workloads \
**Architecture**: Type-safe database abstraction with ACID guarantees and blockchain-specific synchronization patterns

---

_This README was generated by Claude Code_
