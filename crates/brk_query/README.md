# brk_query

Query interface for Bitcoin indexed and computed data.

## What It Enables

Query blocks, transactions, addresses, and 1000+ on-chain metrics through a unified API. Supports pagination, range queries, and multiple output formats.

## Key Features

- **Unified access**: Single entry point to indexer, computer, and mempool data
- **Metric discovery**: List metrics, filter by index type, fuzzy search
- **Range queries**: By height, date, or relative offsets (`from=-100`)
- **Multi-metric bulk queries**: Fetch multiple metrics in one call
- **Async support**: Tokio-compatible with `AsyncQuery` wrapper
- **Format flexibility**: JSON, CSV, or raw values

## Core API

```rust,ignore
let query = Query::build(&reader, &indexer, &computer, Some(mempool));

// Current height
let height = query.height();

// Metric queries
let data = query.search_and_format(MetricSelection {
    vecids: vec!["supply".into()],
    index: Index::Height,
    range: DataRange::Last(100),
    ..Default::default()
})?;

// Block queries
let info = query.block_info(Height::new(840_000))?;

// Transaction queries
let tx = query.transaction(&txid)?;

// Address queries
let stats = query.address_stats(&address)?;
```

## Query Types

| Domain | Methods |
|--------|---------|
| Metrics | `metrics`, `search_and_format`, `metric_to_indexes` |
| Blocks | `block_info`, `block_txs`, `block_status`, `block_timestamp` |
| Transactions | `transaction`, `tx_status`, `tx_merkle_proof` |
| Addresses | `address_stats`, `address_txs`, `address_utxos` |
| Mining | `difficulty_adjustments`, `hashrate`, `pools`, `epochs` |
| Mempool | `mempool_info`, `mempool_fees`, `mempool_projected_blocks` |

## Async Usage

```rust,ignore
let async_query = AsyncQuery::build(&reader, &indexer, &computer, mempool);

// Run blocking queries in thread pool
let result = async_query.run(|q| q.block_info(height)).await;

// Access inner Query
let height = async_query.inner().height();
```

## Recommended: mimalloc v3

Use [mimalloc v3](https://crates.io/crates/mimalloc) as the global allocator to reduce memory usage.

## Built On

- `brk_indexer` for raw indexed data
- `brk_computer` for derived metrics
- `brk_mempool` for mempool queries
- `brk_reader` for raw block access
