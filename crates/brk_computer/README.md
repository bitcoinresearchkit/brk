# brk_computer

Derived metrics computation engine for Bitcoin on-chain analytics.

## What It Enables

Compute 1000+ on-chain metrics from indexed blockchain data: supply breakdowns, realized/unrealized P&L, SOPR, MVRV, cohort analysis (by age, amount, address type), cointime economics, mining pool attribution, and price-weighted valuations.

## Key Features

- **Cohort metrics**: Filter by UTXO age (STH/LTH, age bands), amount ranges, address types
- **Stateful computation**: Track per-UTXO cost basis, realized/unrealized states
- **Multi-index support**: Metrics available by height, date, week, month, year, decade
- **Price integration**: USD-denominated metrics when price data available
- **Mining pool attribution**: Tag blocks/rewards to known pools
- **Cointime economics**: Liveliness, vaultedness, activity-weighted metrics
- **Incremental updates**: Resume from checkpoints, compute only new blocks

## Core API

```rust,ignore
let mut computer = Computer::forced_import(&outputs_path, &indexer, fetcher)?;

// Compute all metrics for new blocks
computer.compute(&indexer, starting_indexes, &reader, &exit)?;

// Access computed data
let supply = computer.chain.height_to_supply.get(height)?;
let realized_cap = computer.stateful.utxo.all.height_to_realized_cap.get(height)?;
```

## Metric Categories

| Module | Examples |
|--------|----------|
| `chain` | Supply, subsidy, fees, transaction counts |
| `stateful` | Realized cap, MVRV, SOPR, unrealized P&L |
| `cointime` | Liveliness, vaultedness, true market mean |
| `pools` | Per-pool block counts, rewards, fees |
| `market` | Market cap, NVT, Puell multiple |
| `price` | Height-to-price mapping from fetched data |

## Cohort System

UTXO and address cohorts support filtering by:
- **Age**: STH (<150d), LTH (≥150d), age bands (1d, 1w, 1m, 3m, 6m, 1y, 2y, ...)
- **Amount**: 0-0.001 BTC, 0.001-0.01, ..., 10k+ BTC
- **Type**: P2PKH, P2SH, P2WPKH, P2WSH, P2TR
- **Epoch**: By halving epoch

## Recommended: mimalloc v3

Use [mimalloc v3](https://crates.io/crates/mimalloc) as the global allocator to reduce memory usage.

## Built On

- `brk_indexer` for indexed blockchain data
- `brk_fetcher` for price data
- `brk_reader` for raw block access
- `brk_grouper` for cohort filtering
- `brk_traversable` for data export
