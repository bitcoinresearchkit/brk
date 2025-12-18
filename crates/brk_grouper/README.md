# brk_grouper

UTXO and address cohort filtering for on-chain analytics.

## What It Enables

Slice the UTXO set and address population by age, amount, output type, halving epoch, or holder classification (STH/LTH). Build complex cohorts by combining filters for metrics like "realized cap of 1+ BTC UTXOs older than 155 days."

## Key Features

- **Age-based**: `TimeFilter::GreaterOrEqual(155)`, `TimeFilter::Range(30..90)`, `TimeFilter::LowerThan(7)`
- **Amount-based**: `AmountFilter::GreaterOrEqual(Sats::_1BTC)`, `AmountFilter::Range(Sats::_100K..Sats::_1M)`
- **Term classification**: `Term::Sth` (short-term holders, <155 days), `Term::Lth` (long-term holders)
- **Epoch filters**: Group by halving epoch
- **Type filters**: Segment by output type (P2PKH, P2TR, etc.)
- **Context-aware naming**: Automatic prefix generation (`utxos_`, `addrs_`) based on cohort context
- **Inclusion logic**: Filter hierarchy for aggregation (`Filter::includes`)

## Filter Types

```rust
pub enum Filter {
    All,
    Term(Term),           // STH/LTH
    Time(TimeFilter),     // Age-based
    Amount(AmountFilter), // Value-based
    Epoch(HalvingEpoch),  // Halving epoch
    Type(OutputType),     // P2PKH, P2TR, etc.
}
```

## Core API

```rust
let filter = Filter::Time(TimeFilter::GreaterOrEqual(155));

// Check membership
filter.contains_time(200);  // true
filter.contains_amount(sats);

// Generate metric names
filter.to_full_name(CohortContext::Utxo);  // "utxos_min_age_155d"
```

## Built On

- `brk_error` for error handling
- `brk_types` for `Sats`, `HalvingEpoch`, `OutputType`
- `brk_traversable` for data structure traversal
