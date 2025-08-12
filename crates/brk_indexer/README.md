# brk_indexer

Bitcoin blockchain indexer that processes raw block data from Bitcoin Core and creates efficient storage structures using vectors and key-value stores for fast data retrieval and analysis. This crate builds the foundation for BRK's data pipeline by extracting and organizing blockchain data into optimized storage formats.

## Features

- **Block-by-block processing**: Iterates through blockchain using brk_parser
- **Dual storage architecture**: Combines vectors (brk_vec) for time-series data and key-value stores (brk_store) for lookups
- **Memory efficient**: ~5GB peak RAM usage during indexing
- **Collision detection**: Validates data integrity with optional collision checking
- **Incremental updates**: Supports resuming from last indexed height
- **Rollback protection**: Automatic rollback on interruption or errors

## Storage Strategy

### Vectors (brk_vec)

Used for sequential, time-indexed data:
- Block metadata (height, timestamp, hash)
- Transaction counts and statistics
- Price data and market metrics
- Efficient for range queries and analytics

### Key-Value Stores (brk_store)

Used for lookup operations:
- Address mappings and balances
- Transaction and UTXO data
- Script and output type indices
- Fast point queries by hash/address

## Usage

```rust
use brk_indexer::Indexer;
use brk_parser::Parser;
use bitcoincore_rpc::{Auth, Client};
use vecdb::Exit;
use std::path::Path;

fn main() -> brk_error::Result<()> {
    // Setup paths and RPC
    let bitcoin_dir = Path::new("~/.bitcoin");
    let outputs_dir = Path::new("./brk_data");

    let rpc = Box::leak(Box::new(Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));

    // Create parser and indexer
    let parser = Parser::new(
        bitcoin_dir.join("blocks"),
        outputs_dir.to_path_buf(),
        rpc
    );

    let mut indexer = Indexer::forced_import(outputs_dir)?;

    // Setup exit handler
    let exit = Exit::new();
    exit.set_ctrlc_handler();

    // Index the blockchain
    let indexes = indexer.index(&parser, rpc, &exit, true)?;

    println!("Indexed up to height: {}", indexes.height);

    Ok(())
}
```

## Performance

Benchmarked on MacBook Pro M3 Pro (36GB RAM):
- **Full sync to ~892k blocks**: 7-8 hours
- **Peak memory usage**: 5-6GB
- **Storage overhead**: ~27% of Bitcoin Core `/blocks` size (193GB as of 2025/08)
- **Incremental updates**: Resumes from last height efficiently

## Data Organization

The indexer creates the following storage structure:
```
brk_data/
├── indexed/
│   ├── vecs/          # Vector storage for time-series data
│   └── stores/        # Key-value stores for lookups
└── ...
```

## Requirements

- Running Bitcoin Core node with RPC access
- Access to Bitcoin Core's block files
- Minimum 500GB free storage space
- 8GB+ RAM recommended for optimal performance

## Incremental Indexing

The indexer supports continuous operation:
- Automatically detects last indexed height
- Processes new blocks as they arrive
- Handles blockchain reorganizations
- Provides graceful shutdown with Ctrl+C
