# brk_indexer

**High-performance Bitcoin blockchain indexer with dual storage architecture**

`brk_indexer` processes raw Bitcoin Core block data and creates efficient storage structures using both vectors (time-series) and key-value stores (lookups). It serves as the foundation of BRK's data pipeline, organizing all blockchain data into optimized formats for fast retrieval and analysis.

## What it provides

- **Dual Storage Architecture**: Vectors for time-series data, key-value stores for lookups
- **Memory Efficiency**: ~5-6GB peak RAM usage during full blockchain indexing
- **Incremental Processing**: Resume from last indexed height with rollback protection
- **Data Integrity**: Collision detection and validation during indexing
- **All Bitcoin Data Types**: Complete support for blocks, transactions, inputs, outputs, and addresses

## Key Features

### Storage Strategy

**Vector Storage (time-series data):**
- Block metadata (height, timestamp, hash, difficulty, size)
- Transaction data (version, locktime, RBF flag, indices)
- Input/Output mappings and values
- Address bytes for all output types
- Efficient for range queries and analytics

**Key-Value Storage (lookups):**
- Block hash prefixes → heights
- Transaction ID prefixes → transaction indices  
- Address byte hashes → type indices
- Fast point queries by hash or address

### Performance Features
- **Parallel Processing**: Concurrent transaction and output processing using Rayon
- **Batch Operations**: Periodic commits every 1,000 blocks for optimal I/O
- **Memory Efficiency**: Optimized data structures minimize RAM usage
- **Incremental Updates**: Handles blockchain reorganizations automatically

### Address Type Support
Complete support for all Bitcoin address types:
- P2PK (65-byte and 33-byte), P2PKH, P2SH
- P2WPKH, P2WSH, P2TR, P2A
- P2MS (multisig), OpReturn, Empty, Unknown

## Usage

### Basic Indexing

```rust
use brk_indexer::Indexer;
use brk_parser::Parser;
use bitcoincore_rpc::{Auth, Client};
use vecdb::Exit;

// Setup Bitcoin Core RPC connection
let rpc = Box::leak(Box::new(Client::new(
    "http://localhost:8332",
    Auth::CookieFile(Path::new("~/.bitcoin/.cookie")),
)?));

// Create parser for Bitcoin Core block files
let parser = Parser::new(
    Path::new("~/.bitcoin/blocks").to_path_buf(),
    Path::new("./brk_data").to_path_buf(),
    rpc
);

// Create indexer with forced import (resets if needed)
let mut indexer = Indexer::forced_import(Path::new("./brk_data"))?;

// Setup graceful shutdown handler
let exit = Exit::new();
exit.set_ctrlc_handler();

// Index the blockchain
let indexes = indexer.index(&parser, rpc, &exit, true)?;
println!("Indexed up to height: {}", indexes.height);
```

### Continuous Indexing

```rust
use std::time::{Duration, Instant};
use std::thread::sleep;

// Continuous indexing loop for real-time updates
loop {
    let start_time = Instant::now();
    
    // Index new blocks
    let indexes = indexer.index(&parser, rpc, &exit, true)?;
    
    println!("Indexed to height {} in {:?}", 
             indexes.height, start_time.elapsed());
    
    // Check for exit signal
    if exit.is_signaled() {
        println!("Graceful shutdown requested");
        break;
    }
    
    // Wait before next update cycle
    sleep(Duration::from_secs(5 * 60));
}
```

### Accessing Indexed Data

```rust
// Access the underlying storage structures
let vecs = &indexer.vecs;
let stores = &indexer.stores;

// Get block hash at specific height
let block_hash = vecs.height_to_blockhash.get(Height::new(800_000))?;

// Look up transaction by prefix
let tx_prefix = TxidPrefix::from(&txid);
let tx_index = stores.txidprefix_to_txindex.get(&tx_prefix)?;

// Get address data
let address_hash = AddressBytesHash::from(&address_bytes);
let type_index = stores.addressbyteshash_to_anyaddressindex.get(&address_hash)?;
```

## Performance Characteristics

**Benchmarked on MacBook Pro M3 Pro (36GB RAM):**
- **Full blockchain sync** (to ~892k blocks): 7-8 hours
- **Peak memory usage**: 5-6GB
- **Storage overhead**: ~27% of Bitcoin Core block size
- **Incremental updates**: Very fast, efficient resume from last height

## Data Organization

The indexer creates this storage structure:
```
brk_data/
├── indexed/
│   ├── vecs/              # Vector storage
│   │   ├── height_to_*    # Height-indexed data
│   │   ├── txindex_to_*   # Transaction-indexed data
│   │   └── outputindex_to_* # Output-indexed data
│   └── stores/            # Key-value stores
│       ├── hash_lookups/  # Block/TX hash mappings
│       └── address_maps/  # Address type mappings
└── metadata/              # Versioning and state
```

## Indexes Tracking

The indexer maintains current indices during processing:

```rust
pub struct Indexes {
    pub height: Height,                      // Current block height
    pub txindex: TxIndex,                    // Current transaction index
    pub inputindex: InputIndex,              // Current input index
    pub outputindex: OutputIndex,            // Current output index
    pub p2pkhaddressindex: P2PKHAddressIndex, // P2PKH address index
    // ... indices for all address types
}
```

## Requirements

- **Bitcoin Core node** with RPC enabled
- **Block file access** to `~/.bitcoin/blocks/`
- **Storage space**: Minimum 500GB (scales with blockchain growth)
- **Memory**: 8GB+ RAM recommended
- **CPU**: Multi-core recommended for parallel processing

## Rollback and Recovery

- **Automatic rollback** on interruption or blockchain reorgs
- **State persistence** for efficient restart
- **Version management** for storage format compatibility
- **Graceful shutdown** with Ctrl+C handling

## Dependencies

- `brk_parser` - Bitcoin block parsing and sequential access
- `brk_store` - Key-value storage wrapper (fjall-based)
- `vecdb` - Vector database for time-series storage
- `bitcoin` - Bitcoin protocol types and parsing
- `rayon` - Parallel processing framework
- `bitcoincore_rpc` - Bitcoin Core RPC client

---

*This README was generated by Claude Code*