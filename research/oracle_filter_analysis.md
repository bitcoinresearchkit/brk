# Oracle Filter Analysis

## Summary

Analysis of ~20M outputs across 2017-2018 to find filters that distinguish accurate price signals from noise.

## Key Finding: Round USD is the Only Reliable Filter

| Filter | Accuracy Advantage | Consistency |
|--------|-------------------|-------------|
| **Round USD = True** | **+20% to +29%** | **12/12 months** |
| Round BTC | +12% to -8% | Flips with price |
| Value range/Decade | varies | Shifts with price |
| Same-day spend | ~3% | Weak |
| Micro-round sats | 0-5% | Inconsistent |
| Tx pattern | <5% | Weak |
| Is smaller output | ~3-4% | Weak |

## Why Other Filters Fail

### Round BTC (Unreliable)
- Jan-Mar 2017 ($1k): Round BTC = True is GOOD (+10-12%)
- Jun-Jul 2017 ($2.5k): Round BTC = True is BAD (-7%)
- Reason: Round BTC only correlates with accuracy when it happens to align with round USD at current price

### Value Range / Decade (Price-Dependent)
- At $1,000/BTC: Decade 5 (100k-1M sats) is good
- At $10,000/BTC: Decade 6 (1M-10M sats) is good
- At $100,000/BTC: Decade 7 (10M-100M sats) would be good
- These shift with price, making them useless as static filters

## The Round USD Insight

Round USD amounts ($1, $5, $10, $20, $50, $100, etc.) always map to the **same phase bins** regardless of price level:

```
$100 at $10,000/BTC  = 1,000,000 sats  → log10 = 6.0 → phase = 0.0 → bin 0
$100 at $100,000/BTC = 100,000 sats    → log10 = 5.0 → phase = 0.0 → bin 0
$100 at $1,000/BTC   = 10,000,000 sats → log10 = 7.0 → phase = 0.0 → bin 0
```

The phase = `frac(log10(sats))` is **invariant** to price decade!

## Round USD Phase Bins

| USD Amount | log10(USD) | Phase = frac(log10) | Bin (×100) |
|------------|------------|---------------------|------------|
| $1, $10, $100, $1000 | 0, 1, 2, 3 | 0.00 | 0 |
| $1.50, $15, $150 | 0.18, 1.18, 2.18 | 0.18 | 18 |
| $2, $20, $200 | 0.30, 1.30, 2.30 | 0.30 | 30 |
| $2.50, $25, $250 | 0.40, 1.40, 2.40 | 0.40 | 40 |
| $3, $30, $300 | 0.48, 1.48, 2.48 | 0.48 | 48 |
| $4, $40, $400 | 0.60, 1.60, 2.60 | 0.60 | 60 |
| $5, $50, $500 | 0.70, 1.70, 2.70 | 0.70 | 70 |
| $6, $60, $600 | 0.78, 1.78, 2.78 | 0.78 | 78 |
| $7, $70, $700 | 0.85, 1.85, 2.85 | 0.85 | 85 |
| $8, $80, $800 | 0.90, 1.90, 2.90 | 0.90 | 90 |
| $9, $90, $900 | 0.95, 1.95, 2.95 | 0.95 | 95 |

## Implementation Plan

### Approach: Phase-Based Round USD Filtering

Filter outputs to only those whose phase bin corresponds to a round USD amount. No price knowledge needed.

```rust
/// Phase bins where round USD amounts cluster
/// Computed as: bin = round(frac(log10(usd_cents)) * 100)
const ROUND_USD_BINS: &[u8] = &[
    0,   // $1, $10, $100, $1000 (and $0.10, $0.01)
    18,  // $1.50, $15, $150
    30,  // $2, $20, $200
    40,  // $2.50, $25, $250
    48,  // $3, $30, $300
    60,  // $4, $40, $400
    70,  // $5, $50, $500
    78,  // $6, $60, $600
    85,  // $7, $70, $700
    90,  // $8, $80, $800
    95,  // $9, $90, $900
];

/// Check if a histogram bin corresponds to a round USD amount
fn is_round_usd_bin(bin: usize, tolerance: u8) -> bool {
    let phase_bin = (bin % 100) as u8;
    ROUND_USD_BINS.iter().any(|&round_bin| {
        let diff = if phase_bin >= round_bin {
            phase_bin - round_bin
        } else {
            round_bin - phase_bin
        };
        // Handle wraparound (bin 99 is close to bin 0)
        diff <= tolerance || (100 - diff) <= tolerance
    })
}
```

### Where to Apply Filter

In `compute.rs`, when adding outputs to histogram:

```rust
for sats in values {
    if let Some(bin) = Histogram::sats_to_bin(sats) {
        // Only include outputs in round-USD phase bins
        if is_round_usd_bin(bin, 2) {  // ±2 bin tolerance
            block_sparse.push((bin as u16, 1.0));
            // ... rest of processing
        }
    }
}
```

### Expected Impact

- Reduces histogram noise by ~60-70% (only ~35% of accurate outputs are round USD)
- Remaining outputs are 2-3x more likely to be accurate signals
- Stencil matching should be more reliable with cleaner signal
- Decade selection via anchors remains unchanged

### Alternative: Weighted Approach

Instead of hard filtering, weight round-USD bins higher:

```rust
let weight = if is_round_usd_bin(bin, 2) { 3.0 } else { 1.0 };
block_sparse.push((bin as u16, weight));
```

This preserves some signal from non-round outputs while emphasizing round USD.

## Bin Resolution: 100 vs 200

UTXOracle uses **200 bins per decade**. Current phase oracle uses 100.

| Resolution | Precision | Round USD cluster |
|------------|-----------|-------------------|
| 100 bins | 1% per bin | Wider, more overlap |
| 200 bins | 0.5% per bin | Tighter, cleaner separation |

**Round USD bins at 200 resolution:**
| USD Amount | Phase = frac(log10) | Bin (×200) |
|------------|---------------------|------------|
| $1, $10, $100 | 0.000 | 0 |
| $1.50, $15, $150 | 0.176 | 35 |
| $2, $20, $200 | 0.301 | 60 |
| $2.50, $25, $250 | 0.398 | 80 |
| $3, $30, $300 | 0.477 | 95 |
| $4, $40, $400 | 0.602 | 120 |
| $5, $50, $500 | 0.699 | 140 |
| $6, $60, $600 | 0.778 | 156 |
| $7, $70, $700 | 0.845 | 169 |
| $8, $80, $800 | 0.903 | 181 |
| $9, $90, $900 | 0.954 | 191 |

**Recommendation**: Use 200 bins for:
1. Compatibility with UTXOracle stencil
2. Tighter round-USD detection
3. Better separation of signal from noise

## Questions to Resolve

1. **Tolerance**: ±2 bins (at 200) = ±1% vs ±4 bins = ±2%
2. **Hard filter vs weight**: Filter completely or just weight higher?
3. **Minimum count threshold**: What if too few outputs pass filter?
4. **Interaction with existing smooth_round_btc()**: Still needed?
5. **Migration**: Update PHASE_BINS constant from 100 to 200

## Validation Plan

1. Implement phase-based filtering
2. Run on 2017-2018 data
3. Compare accuracy vs current approach
4. Tune tolerance parameter
