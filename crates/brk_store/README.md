# brk_store

Key-value storage layer built on fjall for Bitcoin indexing.

## What It Enables

Persist and query Bitcoin index data (address→outputs, txid→height, etc.) with access patterns optimized for different workloads: random lookups, sequential scans, and recent-data queries.

## Key Features

- **Workload-optimized configs**: `Kind::Random` (bloom filters, pinned blocks), `Kind::Recent` (point-read optimized), `Kind::Sequential` (scan-friendly), `Kind::Vec` (append-heavy)
- **Write batching**: Accumulate puts/deletes in memory, commit atomically
- **Tiered caching**: In-memory LRU cache layers before hitting disk
- **Version management**: Automatic schema versioning with `StoreMeta`
- **Height-aware operations**: `insert_if_needed` / `remove_if_needed` skip work at heights already processed

## Core API

```rust,ignore
let store: Store<Txid, Height> = Store::import(
    &db, &path, "txid_to_height",
    Version::new(1), Mode::default(), Kind::Random
)?;

store.insert(txid, height);
store.commit(height)?;

let height = store.get(&txid)?;
```

## Access Patterns

| Kind | Use Case | Optimization |
|------|----------|--------------|
| `Random` | UTXO lookups, txid queries | Aggressive bloom filters |
| `Recent` | Mempool, recent blocks | Point-read hints |
| `Sequential` | Full chain scans | Minimal indexing |
| `Vec` | Append-only series | Large memtables, no filters |

## Built On

- `brk_error` for error handling
- `brk_types` for `Height`, `Version`
