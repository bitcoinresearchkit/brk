# brk_types

Domain types for Bitcoin data analysis with serialization and indexing support.

## What It Enables

Work with Bitcoin primitives (heights, satoshis, addresses, transactions) through purpose-built types that handle encoding, arithmetic, time conversions, and database storage automatically.

## Key Features

- **Bitcoin primitives**: `Height`, `Sats`, `Txid`, `BlockHash`, `Outpoint` with full arithmetic and conversion support
- **Address types**: All output types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, P2PK, P2A, OP_RETURN) with address index variants
- **Time indexes**: `DateIndex`, `WeekIndex`, `MonthIndex`, `QuarterIndex`, `YearIndex`, `DecadeIndex` with cross-index conversion
- **Protocol types**: `DifficultyEpoch`, `HalvingEpoch`, `TxVersion`, `RawLocktime`
- **Financial types**: `Dollars`, `Cents`, `OHLC` (Open/High/Low/Close)
- **Serialization**: Serde + JSON Schema generation via schemars
- **Compression**: PCO (Pco) derive for columnar compression in vecdb

## Type Categories

| Category | Examples |
|----------|----------|
| Block metadata | `Height`, `BlockHash`, `BlockTimestamp`, `BlkPosition` |
| Transaction | `Txid`, `TxIndex`, `TxIn`, `TxOut`, `Vsize`, `Weight` |
| Address | `P2PKHAddressIndex`, `P2TRBytes`, `AnyAddressIndex`, `AddressStats` |
| Value | `Sats`, `Dollars`, `Cents`, `Bitcoin` |
| Time | `Date`, `DateIndex`, `WeekIndex`, `MonthIndex`, ... |
| Metric | `Metric`, `MetricData`, `MetricSelection` |
| API | `Pagination`, `Health`, `RecommendedFees`, `MempoolInfo` |

## Core API

All types implement standard traits: `Debug`, `Clone`, `Serialize`, `Deserialize`, plus domain-specific operations like `CheckedSub`, `Formattable`, and `PrintableIndex`.

```rust,ignore
use brk_types::{Height, Sats, DateIndex, Date};

let height = Height::new(840_000);
let reward = Sats::FIFTY_BTC / 16;  // Post-4th-halving reward
let date_idx = DateIndex::try_from(Date::new(2024, 4, 20))?;
```

## Built On

- `brk_error` for error handling
