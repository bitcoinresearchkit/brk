# brk_oracle

Pure on-chain BTC/USD price oracle. No exchange feeds, no external APIs. Derives the bitcoin price from transaction data alone. Tracks block by block from height 575,000 (May 2019) onward with 0.1% median error.

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

For each new block:

### 1. Filter outputs

Skip the coinbase transaction, then exclude noisy outputs: script types dominated by protocol activity (P2TR, P2WSH by default), dust below 1,000 sats, and round BTC amounts (0.01, 0.1, 1.0 BTC, etc.) that create false spikes unrelated to dollar purchases.

### 2. Map to log-scale bins

Each remaining output becomes a bin index in a 2,400-bin histogram:

```
  bin = round(log₁₀(sats) × 200)       200 bins per decade
```

### 3. Accumulate in ring buffer

A single block is too sparse for a clean signal. The histogram goes into a ring buffer (default depth: 12 blocks) so the pattern accumulates over recent blocks.

### 4. Compute EMA

The buffered histograms combine into an exponential moving average, weighting recent blocks more heavily:

```
  weight = α × (1 − α)^age             default α = 2/7 (~6-block span)
```

Fully recomputed from the ring buffer each block.

### 5. Score with a 19-point stencil

The core detection step. A stencil encodes where spikes from 19 round-dollar amounts ($1 through $10,000) should appear relative to each other on the log scale:

```
   $1       $5     $10          $50  $100  $200        $1k          $10k
    ↓        ↓      ↓            ↓     ↓     ↓          ↓             ↓
    ·────────·──────·────────────·─────·─────·──────────·─────────────·
  -400     -260   -200          -60    0    +60       +200          +400
                      bin offsets from the $100 reference point
                                 (19 offsets total)
```

The oracle slides this stencil across the EMA histogram within a narrow search window around the previous estimate. At each candidate position it reads the EMA value at all 19 expected spike locations, divides each by that offset's peak in the window (so rare amounts like $3 get equal voting weight to common amounts like $100) and sums the normalized values into a score.

### 6. Pick the best position

The position with the highest score is the new price estimate. Parabolic interpolation between the best bin and its neighbors refines it to sub-bin precision:

```
  price = 10^(10 − bin / 200)  dollars
```

The search window is bounded, so the oracle must track incrementally block by block from a known seed price.

## Pipeline

```
  block ──→ filter ──→ histogram ──→ ring ──→ EMA ──→ stencil ──→ best bin ──→ $
             outputs     2,400 bins    buffer           19-point     parabolic
                          log-scale     ×12              scoring    interpolation
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
| Validated from | Height 575,000 (May 2019) | December 2023 |
| Language | Rust | Python |
| Dependencies | None (pure computation, caller provides block data) | Bitcoin Core RPC |
| Bins per decade | 200 | 200 |

## Accuracy

Tested over 361,245 blocks (heights 575,000 to 936,244) against exchange OHLC data. Error is measured per block as distance from the oracle estimate to the exchange high/low range at that height. If the oracle falls within the range, the error is zero.

### Per-block

| Metric | Value |
|--------|-------|
| Median error | 0.10% |
| 95th percentile | 0.55% |
| 99th percentile | 1.4% |
| 99.9th percentile | 4.4% |
| RMSE | 0.38% |
| Max error | 18.1% |
| Bias | +0.04 bins (essentially zero) |
| Blocks > 5% error | 237 (0.07%) |
| Blocks > 10% error | 22 (0.006%) |
| Blocks > 20% error | 0 |

### Daily candles

Oracle daily OHLC built from per-block prices vs exchange daily OHLC:

| | Median | RMSE | Max |
|-------|--------|------|-----|
| Open | 0.20% | 0.49% | 5.9% |
| High | 0.54% | 0.87% | 9.1% |
| Low | 0.48% | 1.31% | 19.7% |
| Close | 0.23% | 0.58% | 6.9% |

### By year

| Year | Blocks | Median | RMSE | Max | >5% | >10% | Price range |
|------|--------|--------|------|-----|-----|------|-------------|
| 2019 | 35,764 | 0.10% | 0.61% | 17.2% | 103 | 16 | $5,656–$13,868 |
| 2020 | 53,102 | 0.10% | 0.48% | 18.2% | 85 | 15 | $3,858–$29,322 |
| 2021 | 52,733 | 0.07% | 0.47% | 14.4% | 38 | 9 | $27,678–$69,000 |
| 2022 | 53,230 | 0.07% | 0.32% | 6.8% | 10 | 0 | $15,460–$48,240 |
| 2023 | 54,032 | 0.10% | 0.25% | 6.7% | 5 | 0 | $16,490–$44,700 |
| 2024 | 53,367 | 0.11% | 0.31% | 9.7% | 16 | 0 | $38,555–$108,298 |
| 2025 | 53,113 | 0.11% | 0.25% | 5.8% | 4 | 0 | $74,409–$126,198 |
| 2026 | 5,904 | 0.11% | 0.27% | 3.3% | 0 | 0 | $60,000–$97,900 |

Accuracy improves over time as on-chain transaction volume grows. Since 2022, zero blocks exceed 10% error. All worst-case errors occur during the fastest intraday price moves in 2019 to 2021.
