# brk_iterator

Unified block iteration with automatic source selection.

## What It Enables

Iterate over Bitcoin blocks with a simple API that automatically chooses between RPC (for small ranges) and direct blk file reading (for large scans). Handles reorgs gracefully.

## Key Features

- **Smart source selection**: RPC for ≤10 blocks, Reader for larger ranges
- **Flexible ranges**: By height span, from start, to end, last N blocks, or after hash
- **Reorg-safe**: Iteration may end early if chain reorganizes
- **Thread-safe**: Clone and share freely

## Core API

```rust,ignore
let blocks = Blocks::new(&rpc_client, &reader);

// Various range specifications
for block in blocks.range(Height::new(800_000), Height::new(800_100))? { ... }
for block in blocks.start(Height::new(840_000))? { ... }
for block in blocks.last(10)? { ... }
for block in blocks.after(Some(last_known_hash))? { ... }
```

## Source Modes

```rust,ignore
// Auto-select (default)
let blocks = Blocks::new(&client, &reader);

// Force RPC only
let blocks = Blocks::new_rpc(&client);

// Force Reader only
let blocks = Blocks::new_reader(&reader);
```

## Range Types

| Method | Description |
|--------|-------------|
| `range(start, end)` | Inclusive height range |
| `start(height)` | From height to chain tip |
| `end(height)` | From genesis to height |
| `last(n)` | Last n blocks from tip |
| `after(hash)` | All blocks after given hash |

## Built On

- `brk_error` for error handling
- `brk_reader` for direct blk file access
- `brk_rpc` for RPC queries
- `brk_types` for `Height`, `BlockHash`
