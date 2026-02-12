# brk_oracle

BTC/USD price oracle from on-chain Bitcoin data alone. No exchange feeds, no external APIs. Given an initial price estimate, tracks block by block from height 575,000 (May 2019) onward.

## The insight

When someone buys $100 of bitcoin at $50,000/BTC, the output is 200,000 sats. At $60,000 it would be 166,667 sats. Millions of round-dollar purchases happen every day at common amounts like $1, $5, $10, $20, $50, $100, $200, $500. Each amount creates its own spike in the histogram of transaction outputs, at a position that depends on the current price. As the price moves, all spikes shift together. The oracle finds those spikes and reads the price from their position.

## How it works

For each new block:

1. **Filter outputs.** Skip the coinbase transaction, then apply the configured filters: excluded script types, dust threshold, and round BTC exclusion.

2. **Map to bins.** Each output's satoshi value is placed into a log-scale histogram with 2,400 bins (200 per 10x): bin = round(log10(sats) * 200). Log-scale is key: if the price doubles, all spikes shift by 60 bins whether bitcoin goes from $1k to $2k or from $50k to $100k.

3. **Store in ring buffer.** The block histogram goes into a ring buffer of configurable depth. A single block is too sparse to get a clean signal, so the oracle accumulates several.

4. **Compute EMA.** The stored histograms are combined into a weighted average where recent blocks count more than older ones: weight = alpha * (1 - alpha)^age. Fully recomputed from the ring buffer each block.

5. **Score with stencil.** A 19-point stencil encodes where the spikes from round-dollar amounts ($1 through $10,000) should appear relative to each other. The oracle slides this stencil across the EMA histogram within a search window around the previous estimate. At each position, it reads the EMA value at each of the 19 expected spike locations, divides each by that offset's peak in the window, and sums them into a score. This gives every dollar amount, common or rare, an equal vote.

6. **Pick the best.** The position with the highest score is the new price estimate. Parabolic interpolation between neighbors refines it to fractional-bin precision.

The resulting bin converts to a dollar price: 10^(10 - bin/200). The search is bounded to prevent the stencil from matching at wrong price levels, so the oracle tracks incrementally block by block.

The oracle accepts three input formats: raw block data, an iterator of (sats, output type) pairs, or a pre-built histogram. Each call returns the current estimate as a fractional bin, convertible to cents or dollars. Daily candles can be built from the per-block prices.

The initial seed must be close to the real price at the starting height. The crate includes a PRICES constant with exchange prices for every height before 630,000 to derive a seed from.

## Config

All parameters are exposed via Config with sensible defaults:

- **alpha** (2/7): EMA decay rate, ~6-block span
- **window_size** (12): number of block histograms in the ring buffer
- **search_below / search_above** (9 / 11): how far to search around the previous estimate, in bins
- **min_sats** (1,000): minimum output value, filters dust
- **exclude_round_btc** (true): exclude round BTC amounts that create false stencil matches
- **excluded_output_types** (P2TR, P2WSH): script types dominated by protocol activity, not round-dollar purchases

## Inspiration

Inspired by [UTXOracle](https://utxo.live/oracle/) by [@SteveSimple](https://x.com/SteveSimple), which showed that the BTC/USD price can be derived from on-chain data alone. brk_oracle takes the same core insight (round-dollar detection via log-scale histogram) and redesigns the algorithm for per-block resolution and rolling operation.

### Differences from UTXOracle

| | brk_oracle | UTXOracle |
|---|---|---|
| Resolution | Per-block (~10 min) and daily candles | Per-day |
| Algorithm | Single-pass stencil scoring | Multi-step: rough stencil match, output-to-USD mapping, iterative median convergence |
| Operation | Rolling EMA over configurable window | Stateless, processes a full day from scratch |
| Stencil | 19 offsets with per-offset peak normalization | Gaussian smooth + empirically weighted spikes |
| Round BTC handling | Excludes outputs entirely | Smooths histogram bins by averaging neighbors |
| Output filtering | Script type, dust threshold, round BTC | 2-output txs only, input count limits, same-day exclusion, witness size limits |
| Validated from | Height 575,000 (May 2019) | December 2023 |

Both use 200 bins per 10x on a log scale.

## Accuracy

Tested over 361,245 blocks (heights 575,000 to 936,244) against exchange OHLC data. Error is measured per block as the distance from the oracle's estimate to the exchange high-low range at that height. If the oracle falls within the range, the error is zero.

### Per-block

| Metric | Value |
|--------|-------|
| Median error | 0.10% |
| 95th percentile | 0.55% |
| 99th percentile | 1.4% |
| 99.9th percentile | 4.4% |
| RMSE | 0.39% |
| Max error | 18.2% |
| Bias | +0.04 bins (essentially zero) |
| Blocks > 5% error | 261 (0.07%) |
| Blocks > 10% error | 40 (0.01%) |
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

Accuracy improves over time as on-chain transaction volume grows. Since 2022, zero blocks exceed 10% error. All worst-case errors occur during the fastest intraday price moves in 2019–2021.
