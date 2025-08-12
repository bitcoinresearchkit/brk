# brk_structs

Core data structures and types used throughout the Bitcoin Research Kit that provide efficient, zero-copy serializable representations of Bitcoin blockchain data. This crate defines strongly-typed wrappers around primitive types with specialized functionality for Bitcoin analysis, storage optimization, and data grouping operations.

## Core Types

### Blockchain Data
- **Height**: Block height with arithmetic operations
- **Date**, **Timestamp**: Time representations with Bitcoin epoch awareness
- **Txid**, **BlockHash**: Transaction and block identifiers with prefix variants
- **TxIndex**, **InputIndex**, **OutputIndex**: Transaction component indices

### Value Types
- **Sats**: Satoshi amounts with conversion utilities
- **Bitcoin**: BTC amounts with precision handling
- **Dollars**, **Cents**: Fiat currency representations
- **OHLC**: Open/High/Low/Close price data structures

### Address Types
- **AddressBytes**: Raw address data with type information
- **P2PKH**, **P2SH**, **P2WPKH**, **P2WSH**, **P2TR**: Address type indices
- **AnyAddressIndex**: Unified address index type

### Storage Types
- **StoredU8/U16/U32/U64**: Optimized integer storage
- **StoredF32/F64**: Floating-point storage with compression
- **StoredBool**: Compact boolean storage

### Time Indices
- **DateIndex**, **WeekIndex**, **MonthIndex**, **QuarterIndex**
- **SemesterIndex**, **YearIndex**, **DecadeIndex**: Time-based grouping
- **HalvingEpoch**, **DifficultyEpoch**: Bitcoin-specific time periods

## Grouping Operations

The crate provides powerful grouping and filtering capabilities:

```rust
use brk_structs::*;

// Group by address type
let p2pkh_addresses = ByAddressType::P2PKH;

// Group by value ranges
let small_utxos = ByLtAmount::new(Sats::_1BTC);
let large_utxos = ByGeAmount::new(Sats::_10BTC);

// Group by age ranges
let recent_utxos = ByMaxAge::new(Height::new(144)); // Last day
let old_utxos = ByMinAge::new(Height::new(52560));  // Last year

// Combine filters
let filter = Filter::new()
    .by_spendable_type()
    .by_amount_range(Sats::_1K, Sats::_1M)
    .by_address_type(ByAddressType::P2WPKH);
```

## Features

- **Zero-copy serialization**: All types implement `FromBytes`/`IntoBytes` for efficient storage
- **Storage compression**: Built-in compression support via `StoredCompressed` trait
- **Type safety**: Strongly-typed wrappers prevent value confusion
- **Bitcoin-aware**: Constants and operations specific to Bitcoin protocol
- **Efficient grouping**: Flexible data filtering and categorization system
- **Time handling**: Comprehensive time representation with epoch support

## Usage

```rust
use brk_structs::*;

// Create blockchain data types
let height = Height::new(800_000);
let amount = Sats::_1BTC;
let date = Date::new(2024, 1, 15);

// Work with addresses
let address_data = AddressBytes::from_script(&script);
let address_index = P2WPKHAddressIndex::from_address_bytes(&address_data);

// Price data
let ohlc = OHLCCents::new(
    Open::new(45000_00),
    High::new(46000_00),
    Low::new(44000_00),
    Close::new(45500_00)
);

// Time-based analysis
let month_index = MonthIndex::from(date);
let halving_epoch = HalvingEpoch::from(height);
```
