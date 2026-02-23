# brk_oracle

Pure on-chain BTC/USD price oracle. No exchange feeds, no external APIs. Derives the bitcoin price from transaction data alone. Tracks block by block from height 550,000 (November 2018) onward.

Inspired by [UTXOracle](https://utxo.live/oracle/) by [@SteveSimple](https://x.com/SteveSimple), which proved the concept. brk_oracle takes the same core insight and redesigns the algorithm for per-block resolution and rolling operation. See [comparison](#comparison-with-utxoracle) below.

## The signal

People buy bitcoin in round dollar amounts. Each purchase creates a transaction output whose satoshi value depends on the current price:

```
  $100 at  $50,000/BTC  →  200,000 sats
  $100 at $100,000/BTC  →  100,000 sats
```

Thousands of these round-dollar purchases happen every day: $10, $20, $50, $100, $200, $500. Plot every transaction output in a block on a log-scale histogram and clear spikes emerge at each round-dollar amount:

```
       $5  $10 $20 $50 $100 $200 $500  $1k       $5k $10k
        ↓   ↓   ↓   ↓    ↓    ↓    ↓    ↓         ↓   ↓
   │            ▌        ▌         ▌                  ▌
   │    ▌   █   █   ▌    █    ▌    █    █         ▌   █
   │   ▐█▌ ▐█▌  █▌  █   ▐█▌  ▐█   ▐█▌  ▐█▌  █    ▐█  ▐█▌
   │▄▄████▄███████▄▐█▌▄█████▄██▌▄█████▄███▄▐█▌▄▄▄███▄████▄▄
   └─────────────────────────────────────────────────────────→
                          log₁₀(satoshis)
```

On a log scale, when the price changes **all spikes shift together** by the same number of bins. A 2x price move always shifts the pattern by ~60 bins, whether bitcoin moves from $1k to $2k or from $50k to $100k:

```
  price × 2  →  sats ÷ 2  →  shift left by log₁₀(2) × 200 ≈ 60 bins

  $50k:  ···· █ ···· █ ···· █ ···· █ ····
  $100k: ·· █ ···· █ ···· █ ···· █ ······
              ◄── 60 bins ──►
```

The spacing between spikes is constant (set by the ratios between dollar amounts). Only the position changes. The oracle detects this pattern and reads the price from where it lands.

## How it works

The oracle tracks the price incrementally, block by block, starting from a known seed price. Each new block nudges the estimate. The search window is narrow (about ±10 bins, or ±12%), so the oracle can only follow gradual movement — it cannot jump to an arbitrary price from scratch. This is by design: it makes the algorithm resistant to noise.

For each new block:

### 1. Filter outputs

Skip the coinbase transaction, then exclude noisy outputs: script types dominated by protocol activity (P2TR, P2WSH by default), dust below 1,000 sats, and round BTC amounts (0.01, 0.1, 1.0 BTC, etc.) that create false spikes unrelated to dollar purchases.

### 2. Build a log-scale histogram

Each remaining output becomes a bin index in a 2,400-bin histogram spanning 12 decades (1 sat to 10¹² sats):

```
  bin = round(log₁₀(sats) × 200)       200 bins per decade
```

### 3. Smooth over recent blocks

A single block has too few outputs for a clean signal. The oracle keeps a ring buffer of the last 12 block histograms and combines them into an exponential moving average (EMA) that weights recent blocks more heavily:

```
  EMA[bin] = Σ  weight(age) × histogram[age][bin]
             age=0..11

  weight(age) = α × (1 − α)^age         default α = 2/7 (~6-block span)
```

The EMA is recomputed from the ring buffer each block. This makes the oracle deterministic: since only the last 12 histograms matter, any oracle started from a known price converges to the exact same state after 12 blocks, regardless of prior history. This is what makes checkpointing and restoring possible.

### 4. Score with a 19-point stencil

The fixed ratios between round-dollar amounts ($1, $2, $3, $5, ... $10,000) create a fingerprint: a pattern of 19 spikes with known spacing on the log scale. A stencil encodes this spacing as bin offsets from a $100 reference point:

```
   $1       $5     $10          $50  $100  $200        $1k          $10k
    ↓        ↓      ↓            ↓     ↓     ↓          ↓             ↓
    ·────────·──────·────────────·─────·─────·──────────·─────────────·
  -400     -260   -200          -60    0    +60       +200          +400
                      bin offsets from the $100 reference point
                                 (19 offsets total)
```

The oracle slides this stencil across the EMA histogram within the search window. At each candidate position:

1. **Read** the EMA value at all 19 expected spike locations
2. **Normalize** each value by dividing by that offset's peak within the search window — this gives rare amounts like $3 equal voting weight to common amounts like $100
3. **Sum** the 19 normalized values into a single score

The position with the highest score is where the fingerprint best matches the histogram.

### 5. Convert bin to price

A $100 purchase at price P produces `$100 / P × 10⁸` sats, which lands in bin:

```
  bin = log₁₀($100 / P × 10⁸) × 200
      = (2 + 8 − log₁₀(P)) × 200
      = (10 − log₁₀(P)) × 200
```

So the stencil's winning position — the bin where $100 purchases land — directly encodes the price:

```
  price = 10^(10 − bin / 200)  dollars
```

Parabolic interpolation between the best bin and its two neighbors refines the estimate to sub-bin precision.

## Pipeline

```
  block ──→ filter ──→ histogram ──→ ring buffer ──→ EMA ──→ stencil ──→ best bin ──→ $
             outputs     2,400 bins       ×12                  19-point    parabolic
                          log-scale                             scoring   interpolation
```

## Input formats

The oracle accepts three input formats:

- **Raw block**: `process_block(&block)` — filters and bins internally
- **Output pairs**: `process_outputs(iter)` — `(sats, output_type)` pairs, still applies configured filters
- **Histogram**: `process_histogram(&hist)` — pre-built `[u32; 2400]` array

The initial seed must be close to the real price at the starting height. The crate includes a `PRICES` constant with exchange prices for every height up to 630,000 to derive a seed from.

## Configuration

All parameters via `Config` with sensible defaults:

| Parameter | Default | Purpose |
|-----------|---------|---------|
| `alpha` | 2/7 | EMA decay rate (~6-block span) |
| `window_size` | 12 | Ring buffer depth in blocks |
| `search_below` / `search_above` | 9 / 11 | Search window around previous estimate (bins) |
| `min_sats` | 1,000 | Dust threshold |
| `exclude_common_round_values` | true | Filter d × 10ⁿ (d ∈ {1,2,3,5,6}) to prevent false stencil matches |
| `excluded_output_types` | P2TR, P2WSH | Script types dominated by protocol activity |

## Comparison with UTXOracle

[UTXOracle](https://utxo.live/oracle/) by [@SteveSimple](https://x.com/SteveSimple) proved that BTC/USD can be derived purely from on-chain data. Both projects share the same core insight (round-dollar detection via log-scale histogram) but make different engineering choices:

| | brk_oracle | UTXOracle |
|---|---|---|
| Resolution | Per-block (~10 min) + daily candles | Per-run consensus price + per-output intraday scatter |
| Operation | Rolling: EMA over ring buffer, updates each block | Batch: processes a full day from scratch, stateless |
| Algorithm | Single-pass stencil scoring with per-offset normalization | Multi-step: dual stencil → rough estimate → output-to-USD mapping → iterative convergence |
| Stencil | 19 round-USD offsets ($1 to $10k), each normalized to its own peak | 803-point Gaussian + weighted spike template targeting 17 round-USD amounts |
| Round BTC handling | Excluded from histogram entirely | Histogram bins smoothed by averaging neighbors |
| Output filtering | Per-output: script type, dust threshold, round BTC | Per-tx: exactly 2 outputs, ≤5 inputs, no same-day inputs, ≤500-byte witness |
| Validated from | Height 550,000 (November 2018) | December 2023 |
| Language | Rust | Python |
| Dependencies | None (pure computation, caller provides block data) | Bitcoin Core RPC |
| Bins per decade | 200 | 200 |

## Accuracy

Tested over 386,251 blocks (heights 550,000 to 937,447, as of February 2026) against exchange OHLC data. Error is measured per block as distance from the oracle estimate to the exchange high/low range at that height. If the oracle falls within the range, the error is zero.

### Per-block

| Metric | Value |
|--------|-------|
| Median error | 0.11% |
| 95th percentile | 0.66% |
| 99th percentile | 1.6% |
| 99.9th percentile | 6.2% |
| RMSE | 0.52% |
| Max error | 33.4% |
| Bias | +0.01 bins (essentially zero) |
| Blocks > 5% error | 519 (0.13%) |
| Blocks > 10% error | 203 |
| Blocks > 20% error | 5 |

### Daily candles

Oracle daily OHLC built from per-block prices vs exchange daily OHLC:

| | Median | RMSE | Max |
|-------|--------|------|-----|
| Open | 0.21% | 0.59% | 15.4% |
| High | 0.53% | 1.18% | 28.0% |
| Low | 0.50% | 1.52% | 19.6% |
| Close | 0.24% | 0.74% | 15.5% |

### By year

| Year | Blocks | Median | RMSE | Max | >5% | >10% | >20% | Price range |
|------|--------|--------|------|-----|-----|------|------|-------------|
| 2018 | 6,492 | 0.69% | 2.34% | 33.4% | 183 | 122 | 5 | $3,129–$6,293 |
| 2019 | 54,272 | 0.16% | 0.74% | 17.4% | 195 | 69 | 0 | $3,338–$13,868 |
| 2020 | 53,102 | 0.10% | 0.43% | 18.1% | 68 | 3 | 0 | $3,858–$29,322 |
| 2021 | 52,733 | 0.07% | 0.47% | 14.4% | 38 | 9 | 0 | $27,678–$69,000 |
| 2022 | 53,230 | 0.07% | 0.32% | 6.8% | 10 | 0 | 0 | $15,460–$48,240 |
| 2023 | 54,032 | 0.10% | 0.25% | 6.7% | 5 | 0 | 0 | $16,490–$44,700 |
| 2024 | 53,367 | 0.11% | 0.31% | 9.7% | 16 | 0 | 0 | $38,555–$108,298 |
| 2025 | 53,113 | 0.11% | 0.25% | 5.8% | 4 | 0 | 0 | $74,409–$126,198 |
| 2026 | 5,910 | 0.10% | 0.27% | 3.3% | 0 | 0 | 0 | $60,000–$97,900 |

The oracle is only as good as the signal it reads. In late 2018 on-chain transaction volume was low and the round-dollar pattern was weak, so the first few thousand blocks are noisy (33% max error, 2.3% RMSE). By 2020 the signal is strong enough for 0.1% median accuracy. Since 2022, zero blocks exceed 10% error.

### Why no outlier smoothing?

Post-hoc smoothing — for example, correcting any block whose price deviates more than 5% from both its neighbors — would improve the aggregate numbers. This is deliberately not done, for two reasons:

1. **Simplicity**: The oracle is a single forward pass with no lookback corrections. Adding smoothing means defining thresholds, neighbor windows, and replacement strategies, all of which add complexity for marginal gain.
2. **Finality**: Each block's price is produced once and never revised (unless the block itself is reorged). Downstream consumers can treat the oracle output as append-only. Smoothing would require retroactively changing already-published prices, breaking that property.
