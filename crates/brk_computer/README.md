# brk_computer

Advanced Bitcoin analytics engine that transforms indexed blockchain data into comprehensive metrics and financial analytics.

[![Crates.io](https://img.shields.io/crates/v/brk_computer.svg)](https://crates.io/crates/brk_computer)
[![Documentation](https://docs.rs/brk_computer/badge.svg)](https://docs.rs/brk_computer)

## Overview

This crate provides a sophisticated analytics engine that processes indexed Bitcoin blockchain data to compute comprehensive metrics, financial analytics, and statistical aggregations. Built on top of `brk_indexer`, it transforms raw blockchain data into actionable insights through state tracking, cohort analysis, market metrics, and advanced Bitcoin-specific calculations.

**Key Features:**

- Comprehensive Bitcoin analytics pipeline with 6 major computation modules
- UTXO and address cohort analysis with lifecycle tracking
- Market metrics integration with price data and financial calculations
- Cointime economics and realized/unrealized profit/loss analysis
- Supply dynamics and monetary policy metrics
- Pool analysis for centralization and mining statistics
- Memory allocation tracking and performance optimization
- Parallel computation with multi-threaded processing

**Target Use Cases:**

- Bitcoin market analysis and research platforms
- On-chain analytics for investment and trading decisions
- Academic research requiring comprehensive blockchain metrics
- Financial applications needing Bitcoin exposure and risk metrics

## Installation

```bash
cargo add brk_computer
```

## Quick Start

```rust
use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_fetcher::Fetcher;
use vecdb::Exit;
use std::path::Path;

// Initialize dependencies
let outputs_path = Path::new("./analytics_data");
let indexer = Indexer::forced_import(outputs_path)?;
let fetcher = Some(Fetcher::import(true, None)?);

// Create computer with price data support
let mut computer = Computer::forced_import(outputs_path, &indexer, fetcher)?;

// Compute analytics from indexer state
let exit = Exit::default();
let starting_indexes = brk_indexer::Indexes::default();
computer.compute(&indexer, starting_indexes, &exit)?;

println!("Analytics computation completed!");
```

## API Overview

### Core Structure

The Computer is organized into 7 specialized computation modules:

- **`indexes`**: Fundamental blockchain index computations
- **`constants`**: Network constants and protocol parameters
- **`market`**: Price-based financial metrics and market analysis
- **`pools`**: Mining pool analysis and centralization metrics
- **`chain`**: Core blockchain metrics (difficulty, hashrate, fees)
- **`stateful`**: Advanced state tracking (UTXO lifecycles, address behaviors)
- **`cointime`**: Cointime economics and value-time calculations

### Key Methods

**`Computer::forced_import(outputs_path, indexer, fetcher) -> Result<Self>`**
Creates computer instance with optional price data integration.

**`compute(&mut self, indexer: &Indexer, starting_indexes: Indexes, exit: &Exit) -> Result<()>`**
Main computation pipeline processing all analytics modules.

### Analytics Categories

**Market Analytics:**

- Price-based metrics (market cap, realized cap, MVRV)
- Trading volume analysis and liquidity metrics
- Return calculations and volatility measurements
- Dollar-cost averaging and investment strategy metrics

**On-Chain Analytics:**

- Transaction count and size statistics
- Fee analysis and block space utilization
- Address activity and entity clustering
- UTXO age distributions and spending patterns

**Monetary Analytics:**

- Circulating supply and issuance tracking
- Realized vs. unrealized gains/losses
- Cointime destruction and accumulation
- Velocity and economic activity indicators

## Examples

### Basic Analytics Computation

```rust
use brk_computer::Computer;

// Initialize with indexer and optional price data
let computer = Computer::forced_import(
    "./analytics_output",
    &indexer,
    Some(price_fetcher)
)?;

// Compute all analytics modules
let exit = vecdb::Exit::default();
computer.compute(&indexer, starting_indexes, &exit)?;

// Access computed metrics
println!("Market cap vectors computed: {}", computer.market.len());
println!("Chain metrics computed: {}", computer.chain.len());
println!("Stateful analysis completed: {}", computer.stateful.len());
```

### Market Analysis

```rust
use brk_computer::Computer;
use brk_types::{DateIndex, Height};

let computer = Computer::forced_import(/* ... */)?;

// Access market metrics after computation
if let Some(market) = &computer.market {
    // Daily market cap analysis
    let date_index = DateIndex::from_days_since_genesis(5000);
    if let Some(market_cap) = market.dateindex_to_market_cap.get(date_index)? {
        println!("Market cap on day {}: ${}", date_index, market_cap.to_dollars());
    }

    // MVRV (Market Value to Realized Value) ratio
    if let Some(mvrv) = market.dateindex_to_mvrv.get(date_index)? {
        println!("MVRV ratio: {:.2}", mvrv);
    }
}

// Chain-level metrics
let height = Height::new(800000);
if let Some(difficulty) = computer.chain.height_to_difficulty.get(height)? {
    println!("Network difficulty at height {}: {}", height, difficulty);
}
```

### Cohort Analysis

```rust
use brk_computer::Computer;
use brk_types::{DateIndex, CohortId};

let computer = Computer::forced_import(/* ... */)?;

// Address cohort analysis
let cohort_date = DateIndex::from_days_since_genesis(4000);

// Analyze address behavior patterns
if let Some(address_cohorts) = &computer.stateful.address_cohorts {
    for cohort_id in address_cohorts.get_cohort_ids_for_date(cohort_date)? {
        let cohort_data = address_cohorts.get_cohort(cohort_id)?;

        println!("Cohort {}: {} addresses created",
                 cohort_id, cohort_data.addresses.len());
        println!("Average holding period: {} days",
                 cohort_data.avg_holding_period.as_days());
    }
}

// UTXO cohort lifecycle analysis
if let Some(utxo_cohorts) = &computer.stateful.utxo_cohorts {
    let active_utxos = utxo_cohorts.get_active_utxos_for_date(cohort_date)?;
    println!("Active UTXOs from cohort: {}", active_utxos.len());
}
```

### Supply and Monetary Analysis

```rust
use brk_computer::Computer;
use brk_types::{Height, DateIndex};

let computer = Computer::forced_import(/* ... */)?;

// Supply dynamics
let height = Height::new(750000);
if let Some(supply) = computer.chain.height_to_circulating_supply.get(height)? {
    println!("Circulating supply: {} BTC", supply.to_btc());
}

// Realized vs unrealized analysis
let date = DateIndex::from_days_since_genesis(5000);
if let Some(realized_cap) = computer.market.dateindex_to_realized_cap.get(date)? {
    if let Some(market_cap) = computer.market.dateindex_to_market_cap.get(date)? {
        let unrealized_pnl = market_cap - realized_cap;
        println!("Unrealized P&L: ${:.2}B", unrealized_pnl.to_dollars() / 1e9);
    }
}
```

## Architecture

### Computation Pipeline

The computer implements a sophisticated multi-stage pipeline:

1. **Index Computation**: Fundamental blockchain metrics and time-based indexes
2. **Constants Computation**: Network parameters and protocol constants
3. **Price Integration**: Optional price data fetching and processing
4. **Parallel Computation**: Chain, market, pools, stateful, and cointime analytics
5. **Cross-Dependencies**: Advanced metrics requiring multiple data sources

### Memory Management

**Allocation Tracking:**

- `allocative` integration for memory usage analysis
- Efficient vector storage with compression options
- Strategic lazy vs. eager evaluation for memory optimization

**Performance Optimization:**

- `rayon` parallel processing for CPU-intensive calculations
- Vectorized operations for time-series computations
- Memory-mapped storage for large datasets

### State Management

**Stateful Analytics:**

- UTXO lifecycle tracking with creation/destruction events
- Address cohort analysis with behavioral clustering
- Transaction pattern recognition and anomaly detection
- Economic cycle analysis with market phase detection

**Cointime Economics:**

- Bitcoin days destroyed and accumulated calculations
- Velocity measurements and economic activity indicators
- Age-weighted value transfer analysis
- Long-term holder vs. active trader segmentation

### Modular Design

Each computation module operates independently:

- **Chain Module**: Basic blockchain metrics (fees, difficulty, hashrate)
- **Market Module**: Price-dependent financial calculations
- **Pools Module**: Mining centralization and pool analysis
- **Stateful Module**: Advanced lifecycle and behavior tracking
- **Cointime Module**: Economic time-value calculations

### Data Dependencies

**Required Dependencies:**

- `brk_indexer`: Raw blockchain data access
- `brk_types`: Type definitions and conversions

**Optional Dependencies:**

- `brk_fetcher`: Price data for financial metrics
- Market analysis requires price integration

### Computation Orchestration

**Sequential Stages:**

1. Indexes → Constants (foundational metrics)
2. Fetched → Price (price data processing)
3. Parallel: Chain, Market, Pools, Stateful, Cointime

**Exit Handling:**

- Graceful shutdown with consistent state preservation
- Checkpoint-based recovery for long-running computations
- Multi-threaded coordination with exit signaling

## Code Analysis Summary

**Main Structure**: `Computer` struct coordinating 7 specialized analytics modules (indexes, constants, market, pools, chain, stateful, cointime) \
**Computation Pipeline**: Multi-stage analytics processing with parallel execution and dependency management \
**State Tracking**: Advanced UTXO and address lifecycle analysis with cohort-based behavioral clustering \
**Financial Analytics**: Comprehensive market metrics including realized/unrealized analysis and cointime economics \
**Memory Optimization**: `allocative` tracking with lazy/eager evaluation strategies and compressed vector storage \
**Parallel Processing**: `rayon` integration for CPU-intensive calculations with coordinated exit handling \
**Architecture**: Modular analytics engine transforming indexed blockchain data into actionable financial and economic insights

---

_This README was generated by Claude Code_
