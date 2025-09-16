# brk_fetcher

Multi-source Bitcoin price data aggregator with automatic fallback between exchanges.

[![Crates.io](https://img.shields.io/crates/v/brk_fetcher.svg)](https://crates.io/crates/brk_fetcher)
[![Documentation](https://docs.rs/brk_fetcher/badge.svg)](https://docs.rs/brk_fetcher)

## Overview

This crate provides a unified interface for fetching Bitcoin price data from multiple sources including Binance, Kraken, and a custom BRK API. It implements automatic failover between data sources, retry mechanisms, and supports both real-time and historical price queries using blockchain height or date-based lookups.

**Key Features:**

- Multi-source price aggregation (Binance, Kraken, BRK API)
- Automatic fallback hierarchy with intelligent retry logic
- Historical price queries by blockchain height or date
- Support for both 1-minute and daily OHLC data
- HAR file import for extended historical data coverage
- Built-in caching with BTreeMap storage for performance

**Target Use Cases:**

- Bitcoin blockchain analyzers requiring accurate historical pricing
- Applications needing resilient price data with multiple fallbacks
- Tools processing large datasets requiring efficient price lookups

## Installation

```toml
cargo add brk_fetcher
```

## Quick Start

```rust
use brk_fetcher::Fetcher;
use brk_structs::{Date, Height, Timestamp};

// Initialize fetcher with exchange APIs enabled
let mut fetcher = Fetcher::import(true, None)?;

// Fetch price by date
let date = Date::from_ymd(2023, 6, 15)?;
let daily_price = fetcher.get_date(date)?;

// Fetch price by blockchain height
let height = Height::new(800000);
let timestamp = Timestamp::from(1684771200u32);
let block_price = fetcher.get_height(height, timestamp, None)?;

println!("Daily OHLC: {:?}", daily_price);
println!("Block OHLC: {:?}", block_price);
```

## API Overview

### Core Types

- **`Fetcher`**: Main aggregator managing multiple price data sources
- **`Binance`**: Binance exchange API client with HAR file support
- **`Kraken`**: Kraken exchange API client for OHLC data
- **`BRK`**: Custom API client for blockchain-indexed price data

### Key Methods

**`Fetcher::import(exchanges: bool, hars_path: Option<&Path>) -> Result<Self>`**
Creates a new fetcher instance with configurable data sources.

**`get_date(&mut self, date: Date) -> Result<OHLCCents>`**
Retrieves daily OHLC data for the specified date with automatic source fallback.

**`get_height(&mut self, height: Height, timestamp: Timestamp, previous_timestamp: Option<Timestamp>) -> Result<OHLCCents>`**
Fetches price data for a specific blockchain height using minute-level precision.

### Data Source Hierarchy

1. **Kraken API** - Primary source for both 1-minute and daily data
2. **Binance API** - Secondary source with extended HAR file support
3. **BRK API** - Fallback source using blockchain-indexed pricing data

### Error Handling

The fetcher implements aggressive retry logic with exponential backoff, attempting each source up to 12 hours (720 retries) before failing. Failed requests trigger cache clearing and source rotation.

## Examples

### Basic Price Fetching

```rust
use brk_fetcher::Fetcher;
use brk_structs::Date;

let mut fetcher = Fetcher::import(true, None)?;

// Fetch Bitcoin price for a specific date
let date = Date::from_ymd(2021, 1, 1)?;
match fetcher.get_date(date) {
    Ok(ohlc) => println!("BTC price on {}: ${}", date, ohlc.close.to_dollars()),
    Err(e) => eprintln!("Failed to fetch price: {}", e),
}
```

### Historical Data with HAR Files

```rust
use brk_fetcher::Fetcher;
use std::path::Path;

// Initialize with HAR file path for extended historical coverage
let har_path = Path::new("./import_data");
let mut fetcher = Fetcher::import(true, Some(har_path))?;

// Fetch minute-level data using HAR file fallback
let height = Height::new(650000);
let timestamp = Timestamp::from(1598918400u32); // August 2020
let price_data = fetcher.get_height(height, timestamp, None)?;
```

### Batch Processing with Caching

```rust
use brk_fetcher::Fetcher;

let mut fetcher = Fetcher::import(true, None)?;

// Process multiple heights - caching improves performance
for height in 800000..800100 {
    let timestamp = Timestamp::from(1684771200u32 + (height - 800000) * 600);

    match fetcher.get_height(Height::new(height), timestamp, None) {
        Ok(ohlc) => println!("Height {}: ${:.2}", height, ohlc.close.to_dollars()),
        Err(e) => eprintln!("Error at height {}: {}", height, e),
    }
}

// Clear caches when done
fetcher.clear();
```

## Architecture

### Retry Mechanism

The crate implements a sophisticated retry system with:

- **Default retry count**: 6 attempts with 5-second delays
- **Extended retry**: Up to 720 attempts (12 hours) for critical operations
- **Cache invalidation**: Automatic cache clearing between retry attempts
- **Exponential backoff**: 60-second delays for extended retries

### Data Aggregation

Price data is aggregated using OHLC (Open, High, Low, Close) calculations spanning timestamp ranges. The `find_height_ohlc` function computes accurate OHLC values by scanning time series data between block timestamps.

### HAR File Processing

Binance integration supports HTTP Archive (HAR) files for extended historical data coverage, parsing browser network captures to extract additional pricing data beyond API limitations.

## Code Analysis Summary

**Main Types**: `Fetcher` aggregator with `Binance`, `Kraken`, and `BRK` source implementations \
**Caching**: BTreeMap-based caching for both timestamp and date-indexed price data \
**Network Layer**: Built on `minreq` HTTP client with automatic JSON parsing \
**Error Handling**: Comprehensive retry logic with source rotation and cache management \
**Dependencies**: Integrates `brk_structs` for type definitions and `brk_error` for unified error handling \
**Architecture**: Multi-source aggregation pattern with hierarchical fallback and intelligent caching

---

_This README was generated by Claude Code_
