# brk_reader

High-performance Bitcoin block reader from raw blk files.

## What It Enables

Stream blocks directly from Bitcoin Core's `blk*.dat` files with parallel parsing, automatic XOR decoding, and chain-order delivery. Much faster than RPC for full-chain scans.

## Key Features

- **Direct blk file access**: Bypasses RPC overhead entirely
- **XOR decoding**: Handles Bitcoin Core's obfuscated block storage
- **Parallel parsing**: Multi-threaded block deserialization
- **Chain ordering**: Reorders out-of-sequence blocks before delivery
- **Smart start finding**: Binary search to locate starting height across blk files
- **Reorg detection**: Stops iteration on chain discontinuity

## Core API

```rust
let reader = Reader::new(blocks_dir, &rpc_client);

// Stream blocks from height 800,000 to 850,000
let receiver = reader.read(Some(Height::new(800_000)), Some(Height::new(850_000)));

for block in receiver {
    // Process block in chain order
}
```

## Architecture

1. **File scanner**: Maps `blk*.dat` files to indices
2. **Byte reader**: Streams raw bytes, finds magic bytes, segments blocks
3. **Parser pool**: Parallel deserialization with rayon
4. **Orderer**: Buffers and emits blocks in height order

## Performance

The parallel pipeline can saturate disk I/O while parsing on multiple cores. For recent blocks, falls back to RPC for lower latency.

## Built On

- `brk_error` for error handling
- `brk_rpc` for RPC client (height lookups, recent blocks)
- `brk_types` for `Height`, `BlockHash`, `BlkPosition`, `BlkMetadata`
