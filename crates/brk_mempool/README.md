# brk_mempool

Real-time Bitcoin mempool monitoring with fee estimation.

## What It Enables

Track mempool state, estimate transaction fees via projected block building, and query address mempool activity. Updates automatically with 1-second sync cycles.

## Key Features

- **Projected blocks**: Simulates Bitcoin Core's block template algorithm with CPFP awareness
- **Fee estimation**: Multi-tier fee recommendations (fastest, half-hour, hour, economy, minimum)
- **Address tracking**: Maps addresses to their pending transactions
- **Dependency handling**: Respects transaction ancestry for accurate fee calculations
- **Rate-limited rebuilds**: Throttles expensive projections to 1/second

## Core API

```rust,ignore
let mempool = Mempool::new(&rpc_client);

// Start background sync loop
std::thread::spawn(move || mempool.start());

// Query current state
let fees = mempool.get_fees();
let info = mempool.get_info();
let blocks = mempool.get_block_stats();
let snapshot = mempool.get_snapshot();

// Address lookups
let tracker = mempool.get_addresses();
```

## Fee Estimation

Returns `RecommendedFees` with sat/vB rates for different confirmation targets:

- `fastest_fee` - Next block
- `half_hour_fee` - ~3 blocks
- `hour_fee` - ~6 blocks
- `economy_fee` - ~144 blocks
- `minimum_fee` - Relay minimum

## Block Projection

Builds projected blocks by:
1. Constructing transaction dependency graph
2. Calculating effective fee rates (including ancestors)
3. Selecting transactions greedily by ancestor-aware fee rate
4. Partitioning into ~4MB virtual blocks

## Built On

- `brk_error` for error handling
- `brk_rpc` for mempool RPC calls
- `brk_types` for `MempoolInfo`, `MempoolEntryInfo`, `RecommendedFees`
