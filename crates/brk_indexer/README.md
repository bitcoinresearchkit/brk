# brk_indexer

Full Bitcoin blockchain indexer for fast analytics queries.

## What It Enables

Transform raw Bitcoin blockchain data into indexed vectors and key-value stores optimized for analytics. Query any block, transaction, address, or UTXO without scanning the chain.

## Key Features

- **Multi-phase block processing**: Parallel TXID computation, input/output processing, sequential finalization
- **Address indexing**: Maps addresses to their transaction history and UTXOs per address type
- **UTXO tracking**: Live outpoint→value lookups, address→unspent outputs
- **Reorg handling**: Automatic rollback to valid chain state on reorganization
- **Collision detection**: Validates rapidhash-based prefix lookups against known duplicate TXIDs
- **Incremental snapshots**: Periodic checkpoints for crash recovery

## Core API

```rust,ignore
let mut indexer = Indexer::forced_import(&outputs_dir)?;

// Index new blocks
let starting_indexes = indexer.index(&blocks, &client, &exit)?;

// Access indexed data
let height = indexer.stores.txidprefix_to_txindex.get(&txid_prefix)?;
let blockhash = indexer.vecs.block.height_to_blockhash.get(height)?;
```

## Data Structures

**Vecs** (append-only vectors):
- Block: `height_to_blockhash`, `height_to_timestamp`, `height_to_difficulty`
- Transaction: `txindex_to_txid`, `txindex_to_height`, `txindex_to_base_size`
- Input/Output: `txinindex_to_outpoint`, `txoutindex_to_value`, `txoutindex_to_outputtype`
- Address: Per-type `typeindex_to_addressbytes`

**Stores** (key-value lookups):
- `txidprefix_to_txindex` - TXID lookup via 10-byte prefix
- `blockhashprefix_to_height` - Block lookup via 4-byte prefix
- `addresshash_to_addressindex` - Address lookup per type
- `addressindex_to_unspent_outpoints` - Live UTXO set per address

## Processing Pipeline

1. **Block metadata**: Store blockhash, difficulty, timestamp
2. **Compute TXIDs**: Parallel SHA256d across transactions
3. **Process inputs**: Lookup spent outpoints, resolve address info
4. **Process outputs**: Extract addresses, assign type indexes
5. **Finalize**: Sequential store updates, UTXO set mutations
6. **Commit**: Periodic flush to disk

## Performance

| Machine | Time | Disk | Peak Disk | Memory | Peak Memory |
|---------|------|------|-----------|--------|-------------|
| MBP M3 Pro (36GB, internal SSD) | 3.1h | 233 GB | 307 GB | 5.5 GB | 11 GB |
| Mac Mini M4 (16GB, external SSD) | 4.9h | 233 GB | 303 GB | 5.4 GB | 11 GB |

Full benchmark data: [`https://github.com/bitcoinresearchkit/benches/tree/main/brk_indexer`](/benches/brk_indexer)

## Recommended: mimalloc v3

Use [mimalloc v3](https://crates.io/crates/mimalloc) as the global allocator to reduce memory usage.

## Built On

- `brk_cohort` for address type handling
- `brk_iterator` for block iteration
- `brk_store` for key-value storage
- `brk_types` for domain types
