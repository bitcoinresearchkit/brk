# brk_computer

**Bitcoin analytics engine that transforms indexed blockchain data into comprehensive metrics**

`brk_computer` is the computational layer of BRK that processes indexed blockchain data to generate analytics across multiple specialized domains. It provides comprehensive Bitcoin metrics with efficient storage and lazy computation for optimal performance.

## What it provides

- **Comprehensive Analytics**: 9 specialized domains covering all aspects of Bitcoin analysis
- **Lazy Computation**: On-demand calculation with dependency tracking and caching
- **Incremental Updates**: Only processes new data since last computation
- **Memory Efficiency**: ~100MB operation footprint via compressed storage and memory mapping
- **Multi-timeframe Analysis**: Daily, weekly, monthly, quarterly, yearly perspectives

## Nine Analytics Domains

The computer processes data through a fixed dependency chain:

1. **indexes** - Time-based indexing (date/height mappings, epoch calculations)
2. **constants** - Baseline values and reference metrics
3. **blocks** - Block analytics (sizes, intervals, transaction counts, weight)
4. **mining** - Mining economics (hashrate, difficulty, rewards, epochs)
5. **fetched** - External price data integration (optional)
6. **price** - OHLC data across multiple timeframes (optional, requires fetched)
7. **transactions** - Transaction analysis (fees, sizes, patterns, RBF detection)
8. **market** - Price correlations and market metrics (optional, requires price)
9. **stateful** - UTXO tracking and accumulated state computations
10. **cointime** - Coin age and time-based value analysis

## Key Features

### Computation Strategy
- **Fixed dependency chain**: Ensures data consistency across all domains
- **Parallel processing**: Uses Rayon for performance optimization
- **State management**: Rollback capabilities for error recovery
- **Incremental updates**: Only computes new data since last run

### Analytics Capabilities
- **Multi-timeframe analysis**: Daily, weekly, monthly, quarterly, yearly aggregations
- **Chain-based metrics**: Height, difficulty epoch, halving epoch indexing
- **Price correlation**: Both dollar and satoshi denominated metrics
- **DCA analysis**: Dollar Cost Averaging with configurable periods
- **Supply analysis**: Circulating, realized, unrealized supply metrics
- **Address cohort tracking**: Analysis across different Bitcoin address types
- **UTXO cohort analysis**: Realized/unrealized gains tracking
- **Coin time analysis**: Understanding Bitcoin velocity and dormancy

### Storage Optimization
- **Compressed vectors**: Efficient disk storage with lazy computation
- **Memory mapping**: Minimal RAM usage during operation
- **Version management**: Automatic invalidation on schema changes
- **Dependency tracking**: Smart recomputation based on data changes

## Usage

### Basic Setup (No Price Data)

```rust
use brk_computer::Computer;
use brk_indexer::Indexer;
use vecdb::Exit;

// Setup without external price data
let indexer = Indexer::forced_import("./brk_data")?;
let mut computer = Computer::forced_import("./brk_data", &indexer, None)?;

// Setup exit handler
let exit = Exit::new();
exit.set_ctrlc_handler();

// Compute all analytics
let starting_indexes = indexer.get_starting_indexes();
computer.compute(&indexer, starting_indexes, &exit)?;
```

### Advanced Setup (With Price Data)

```rust
use brk_fetcher::Fetcher;

// Setup with external price data for market analytics
let fetcher = Some(Fetcher::import(true, None)?);
let mut computer = Computer::forced_import("./brk_data", &indexer, fetcher)?;

// Compute all analytics including price/market domains
computer.compute(&indexer, starting_indexes, &exit)?;
```

### Accessing Computed Data

```rust
// Access all computed vectors
let all_vecs = computer.vecs(); // Returns Vec<&dyn AnyCollectableVec>

// Access specific domain data
let block_metrics = &computer.blocks;
let mining_data = &computer.mining;
let transaction_stats = &computer.transactions;

// Access price data (if available)
if let Some(price_data) = &computer.price {
    // Use OHLC data
}
```

### Incremental Updates

```rust
// Continuous computation loop
loop {
    // Get latest indexes from indexer
    let current_indexes = indexer.get_current_indexes();
    
    // Compute only new data
    computer.compute(&indexer, current_indexes, &exit)?;
    
    // Check for exit signal
    if exit.is_signaled() {
        break;
    }
    
    // Wait before next update
    sleep(Duration::from_secs(60));
}
```

## Core Computer Structure

```rust
pub struct Computer {
    pub indexes: indexes::Vecs,           // Time indexing
    pub constants: constants::Vecs,       // Baseline values
    pub blocks: blocks::Vecs,            // Block analytics
    pub mining: mining::Vecs,            // Mining economics
    pub market: market::Vecs,            // Market metrics (optional)
    pub price: Option<price::Vecs>,      // OHLC price data (optional)
    pub transactions: transactions::Vecs, // Transaction analysis
    pub stateful: stateful::Vecs,        // UTXO tracking
    pub fetched: Option<fetched::Vecs>,  // External data (optional)
    pub cointime: cointime::Vecs,        // Coin age analysis
}
```

## Performance Characteristics

**Benchmarked on MacBook Pro M3 Pro:**
- **Initial computation**: ~6-7 hours for complete Bitcoin blockchain
- **Storage efficiency**: All computed datasets total ~40GB
- **Incremental updates**: 3-5 seconds per new block
- **Memory footprint**: Peak ~7-8GB during computation, ~100MB during operation
- **Dependencies**: Price data domains optional (fetched, price, market)

## Domain-Specific Analytics

### Block Analytics
- Block sizes, weights, transaction counts
- Block intervals and mining statistics
- Fee analysis per block

### Mining Economics
- Hashrate estimation and difficulty tracking
- Mining reward analysis
- Epoch-based calculations

### Transaction Analysis
- Fee rate distributions
- RBF (Replace-By-Fee) detection
- Output type analysis
- Transaction size patterns

### Market Metrics (Optional)
- Price correlations with on-chain metrics
- Market cap calculations
- DCA analysis across timeframes

### Stateful Analysis
- UTXO set tracking
- Address cohort analysis
- Realized/unrealized gains
- Supply distribution metrics

## Requirements

- **Indexed data**: Requires completed `brk_indexer` output
- **Storage space**: Additional ~40GB for computed datasets
- **Memory**: 8GB+ RAM recommended for initial computation
- **CPU**: Multi-core recommended for parallel processing
- **Price data**: Optional external price feeds for market analytics

## Dependencies

- `brk_indexer` - Source of indexed blockchain data
- `brk_fetcher` - External price data (optional)
- `vecdb` - Vector database with lazy computation
- `rayon` - Parallel processing framework
- `brk_structs` - Bitcoin-aware type system

---

*This README was generated by Claude Code*