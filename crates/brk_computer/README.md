# brk_computer

Bitcoin analytics engine that transforms indexed blockchain data into computed datasets and metrics. Uses a modular vector architecture with lazy computation and compressed storage for optimal performance.

## Overview

Computes analytics across 9 specialized domains, each implementing the compute trait pattern:

- **indexes** - Time-based indexing (date/height mappings, epoch calculations)
- **constants** - Baseline values for calculations
- **blocks** - Block analytics (sizes, intervals, transaction counts)
- **mining** - Mining economics (hashrate, difficulty, rewards)
- **transactions** - Transaction analysis (fees, sizes, patterns, RBF)
- **stateful** - UTXO tracking and accumulated state computations
- **cointime** - Coin age and time-based value analysis
- **fetched** - External price data integration (optional)
- **price** - OHLC data across timeframes (optional, requires fetched)
- **market** - Price correlations and market metrics (optional, requires price)

**Computation order**: Fixed dependency chain ensures data consistency (indexes → constants → blocks → mining → fetched → price → transactions → market → stateful → cointime).

**Storage**: Uses vecdb with lazy computation and compressed format for efficient disk usage and memory management.

## Usage

```rust
use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_fetcher::Fetcher;

// Basic setup - computes all domains except price/market
let indexer = Indexer::forced_import("./brk_data")?;
let mut computer = Computer::forced_import("./brk_data", &indexer, None)?;

// With price data - enables market analytics
let fetcher = Some(Fetcher::import(true, None)?);
let mut computer = Computer::forced_import("./brk_data", &indexer, fetcher)?;

// Compute all analytics from starting point
let starting_indexes = indexer.get_starting_indexes();
computer.compute(&indexer, starting_indexes, &exit)?;

// Access computed vectors
let all_vecs = computer.vecs(); // Returns Vec<&dyn AnyCollectableVec>
```

## Key Implementation Details

- **Forced import pattern**: Single computer instance per output directory to prevent conflicts
- **Lazy computation**: Vectors computed on-demand, cached with dependency tracking
- **Incremental updates**: Only processes new data since last computation
- **Memory efficient**: ~100MB max via compressed storage and memory mapping
- **Exit handling**: Graceful shutdown support with computation state preservation

## Performance

Benchmarked on MacBook Pro M3 Pro:

- **Initial computation**: ~6-7 hours for complete Bitcoin blockchain analysis
- **Storage efficiency**: All computed datasets total only ~40GB
- **Incremental updates**: 3-5 seconds per new block
- **Memory footprint**: Peak ~7-8GB during computation, ~100MB during operation

The initial computation processes the entire blockchain history once to generate all analytical datasets. Subsequent updates are near-instant, making BRK suitable for real-time analysis and production deployments.
