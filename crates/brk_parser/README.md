# brk_parser

High-performance Bitcoin block parser that reads raw Bitcoin Core block files (`blkXXXXX.dat`) and provides a sequential iterator over blocks with fork filtering and XOR support. This crate processes the entire Bitcoin blockchain efficiently using parallel processing and maintains state for fast restarts.

## Features

- **Fast sequential parsing**: Iterates blocks in height order (0, 1, 2, ...)
- **Fork filtering**: Uses Bitcoin Core RPC to exclude orphaned blocks
- **XOR support**: Handles XOR-encrypted block files automatically
- **Parallel processing**: Multi-threaded parsing and decoding for maximum speed
- **State caching**: Saves parsing state for faster subsequent runs
- **Memory efficient**: ~500MB peak memory usage
- **Range queries**: Parse specific height ranges or single blocks

## Requirements

- Running Bitcoin Core node with RPC enabled
- Access to Bitcoin Core's `blocks/` directory containing `blkXXXXX.dat` files
- Bitcoin Core versions v25.0 through v29.0 supported

## Usage

```rust
use brk_parser::Parser;
use brk_structs::Height;
use bitcoincore_rpc::{Auth, Client};
use std::path::Path;

fn main() -> bitcoincore_rpc::Result<()> {
    // Setup Bitcoin Core RPC client
    let rpc = Box::leak(Box::new(Client::new(
        "http://localhost:8332",
        Auth::CookieFile(Path::new("~/.bitcoin/.cookie")),
    )?));

    // Create parser
    let parser = Parser::new(
        Path::new("~/.bitcoin/blocks").to_path_buf(),
        Path::new("./brk_data").to_path_buf(),  // Output directory
        rpc,
    );

    // Parse all blocks
    parser.parse(None, None)
        .iter()
        .for_each(|(height, block, hash)| {
            println!("Block {}: {} ({} transactions)",
                height, hash, block.txdata.len());
        });

    // Parse specific range
    let start = Some(Height::new(800_000));
    let end = Some(Height::new(800_100));

    parser.parse(start, end)
        .iter()
        .for_each(|(height, block, hash)| {
            println!("Block {}: {}", height, hash);
        });

    // Get single block
    let block = parser.get(Height::new(0)); // Genesis block
    println!("Genesis block has {} transactions", block.txdata.len());

    Ok(())
}
```

## Output Format

The parser returns tuples containing:
- `Height`: Block height (0, 1, 2, ...)
- `Block`: Complete block data (from `bitcoin` crate)
- `BlockHash`: Block's cryptographic hash

## Performance

Benchmarked on MacBook Pro M3 Pro:
- Full blockchain (0 to 855,000): **4 minutes 10 seconds**
- Recent blocks (800,000 to 855,000): **52 seconds** (4m 10s on first run)
- Peak memory usage: ~500MB

## State Management

The parser saves state in `{output_dir}/blk_index_to_blk_recap.json` for faster restarts. This file tracks block file indices and heights to avoid re-scanning unchanged files.

**Note**: Only one parser instance should run at a time as the state file doesn't yet support concurrent access.
