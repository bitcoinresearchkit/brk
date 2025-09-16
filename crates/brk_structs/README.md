# brk_structs

Bitcoin-aware type system and zero-copy data structures for blockchain analysis.

[![Crates.io](https://img.shields.io/crates/v/brk_structs.svg)](https://crates.io/crates/brk_structs)
[![Documentation](https://docs.rs/brk_structs/badge.svg)](https://docs.rs/brk_structs)

## Overview

This crate provides a comprehensive type system for Bitcoin blockchain analysis, featuring zero-copy data structures, memory-efficient storage types, and Bitcoin-specific primitives. Built on `zerocopy` and `vecdb`, it offers type-safe representations for blockchain data with optimized serialization and database integration.

**Key Features:**

- Zero-copy data structures with `zerocopy` derives for high-performance serialization
- Bitcoin-specific types for heights, timestamps, addresses, and transaction data
- OHLC (Open, High, Low, Close) price data structures for financial analysis
- Comprehensive address type classification (P2PK, P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.)
- Time-based indexing types (Date, DateIndex, WeekIndex, MonthIndex, etc.)
- Memory allocation tracking with `allocative` integration
- Stored primitive wrappers for space-efficient database storage

**Target Use Cases:**

- Bitcoin blockchain analysis and research tools
- Financial data processing and OHLC calculations
- Database-backed blockchain indexing systems
- Memory-efficient UTXO set management

## Installation

```toml
cargo add brk_structs
```

## Quick Start

```rust
use brk_structs::*;

// Bitcoin-specific types
let height = Height::new(800000);
let timestamp = Timestamp::from(1684771200u32);
let date = Date::from(timestamp);
let sats = Sats::new(100_000_000); // 1 BTC in satoshis

// Price data structures
let price_cents = Cents::from(Dollars::from(50000.0));
let ohlc = OHLCCents::from((
    Open::new(price_cents),
    High::new(price_cents * 1.02),
    Low::new(price_cents * 0.98),
    Close::new(price_cents * 1.01),
));

// Address classification
let output_type = OutputType::P2PKH;
println!("Is spendable: {}", output_type.is_spendable());
println!("Has address: {}", output_type.is_address());

// Time indexing
let date_index = DateIndex::try_from(date)?;
let week_index = WeekIndex::from(date);
```

## API Overview

### Core Bitcoin Types

- **`Height`**: Block height with overflow-safe arithmetic operations
- **`Timestamp`**: Unix timestamp with Bitcoin genesis epoch support
- **`Date`**: Calendar date with blockchain-specific formatting (YYYYMMDD)
- **`Sats`**: Satoshi amounts with comprehensive arithmetic operations
- **`Bitcoin`**: Floating-point BTC amounts with satoshi conversion
- **`BlockHash`** / **`TxId`**: Cryptographic hash identifiers

### Address and Output Types

- **`OutputType`**: Comprehensive Bitcoin script type classification

  - Standard types: P2PK (33/65-byte), P2PKH, P2SH, P2WPKH, P2WSH, P2TR
  - Special types: P2MS (multisig), OpReturn, P2A (address), Empty, Unknown
  - Type-checking methods: `is_spendable()`, `is_address()`, `is_unspendable()`

- **Address Index Types**: Specialized indexes for each address type
  - `P2PKHAddressIndex`, `P2SHAddressIndex`, `P2WPKHAddressIndex`
  - `P2WSHAddressIndex`, `P2TRAddressIndex`, etc.

### Financial Data Types

**Price Representations:**

- **`Cents`**: Integer cent representation for precise financial calculations
- **`Dollars`**: Floating-point dollar amounts with cent conversion
- **`OHLCCents`** / **`OHLCDollars`** / **`OHLCSats`**: OHLC data in different denominations

**OHLC Components:**

- **`Open<T>`**, **`High<T>`**, **`Low<T>`**, **`Close<T>`**: Generic price point wrappers
- Support for arithmetic operations and automatic conversions

### Time Indexing System

**Calendar Types:**

- **`DateIndex`**: Days since Bitcoin genesis (2009-01-03)
- **`WeekIndex`**: Week-based indexing for aggregation
- **`MonthIndex`**, **`QuarterIndex`**, **`SemesterIndex`**: Hierarchical time periods
- **`YearIndex`**, **`DecadeIndex`**: Long-term time categorization

**Epoch Types:**

- **`HalvingEpoch`**: Bitcoin halving period classification
- **`DifficultyEpoch`**: Difficulty adjustment epoch tracking

### Storage Types

**Stored Primitives:** Memory-efficient wrappers for database storage

- **`StoredU8`**, **`StoredU16`**, **`StoredU32`**, **`StoredU64`**: Unsigned integers
- **`StoredI16`**: Signed integers
- **`StoredF32`**, **`StoredF64`**: Floating-point numbers
- **`StoredBool`**: Boolean values
- **`StoredString`**: String storage optimization

## Examples

### Block Height Operations

```rust
use brk_structs::Height;

let current_height = Height::new(800000);
let next_height = current_height.incremented();

// Check halving schedule
let blocks_until_halving = current_height.left_before_next_halving();
println!("Blocks until next halving: {}", blocks_until_halving);

// Difficulty adjustment tracking
let blocks_until_adjustment = current_height.left_before_next_diff_adj();
```

### Price Data Processing

```rust
use brk_structs::*;

// Create OHLC data from individual price points
let daily_ohlc = OHLCDollars::from((
    Open::from(45000.0),
    High::from(47500.0),
    Low::from(44000.0),
    Close::from(46800.0),
));

// Convert between price denominations
let ohlc_cents: OHLCCents = daily_ohlc.into();
let sats_per_dollar = Sats::_1BTC / daily_ohlc.close;

// Aggregate multiple OHLC periods
let weekly_close = vec![
    Close::from(46800.0),
    Close::from(48200.0),
    Close::from(47100.0),
].iter().sum::<Close<Dollars>>();
```

### Address Type Classification

```rust
use brk_structs::OutputType;
use bitcoin::{Address, Network};

let address_str = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
let address = Address::from_str(address_str)?.assume_checked();
let output_type = OutputType::from(&address);

match output_type {
    OutputType::P2PKH => println!("Legacy address"),
    OutputType::P2WPKH => println!("Native SegWit address"),
    OutputType::P2SH => println!("Script hash address"),
    OutputType::P2TR => println!("Taproot address"),
    _ => println!("Other address type: {:?}", output_type),
}

// Check spend conditions
if output_type.is_spendable() && output_type.is_address() {
    println!("Spendable address-based output");
}
```

### Time-Based Indexing

```rust
use brk_structs::*;

let date = Date::new(2023, 6, 15);
let date_index = DateIndex::try_from(date)?;

// Convert to different time granularities
let week_index = WeekIndex::from(date);
let month_index = MonthIndex::from(date);
let quarter_index = QuarterIndex::from(date);

// Calculate completion percentage for current day
let completion_pct = date.completion();
println!("Day {}% complete", completion_pct * 100.0);
```

## Architecture

### Zero-Copy Design

All major types implement `zerocopy` traits (`FromBytes`, `IntoBytes`, `KnownLayout`) enabling:

- Direct memory mapping from serialized data
- Efficient network protocol handling
- High-performance database operations
- Memory layout guarantees for cross-platform compatibility

### Type Safety

The type system enforces Bitcoin domain constraints:

- `Height` prevents integer overflow in block calculations
- `Timestamp` handles Unix epoch edge cases
- `OutputType` enum covers all Bitcoin script patterns
- Address types ensure correct hash length validation

### Memory Efficiency

Storage types provide space optimization:

- Stored primitives reduce allocation overhead
- OHLC structures support both heap and stack allocation
- Index types enable efficient range queries
- Hash types use fixed-size arrays for predictable memory usage

## Code Analysis Summary

**Main Categories**: 70+ struct types across Bitcoin primitives, financial data, time indexing, and storage optimization \
**Zero-Copy Support**: Comprehensive `zerocopy` implementation for all major types \
**Type Safety**: Bitcoin domain-specific constraints with overflow protection and validation \
**Financial Types**: Multi-denomination OHLC support with automatic conversions \
**Address System**: Complete Bitcoin script type classification with 280 enum variants \
**Time Indexing**: Hierarchical calendar system from daily to decade-level granularity \
**Storage Integration**: `vecdb::StoredCompressed` traits for efficient database operations \
**Architecture**: Type-driven design prioritizing memory efficiency and domain correctness

---

_This README was generated by Claude Code_
