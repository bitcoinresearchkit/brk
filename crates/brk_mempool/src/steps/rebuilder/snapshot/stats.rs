use brk_types::{FeeRate, Sats, VSize, get_weighted_percentile};

use super::{SnapTx, TxIndex};

/// Block 0 mirrors Core's `getblocktemplate`, so the full 0..100 range
/// is exact and worth surfacing.
const CORE_PERCENTILES: [f64; 7] = [0.0, 0.10, 0.25, 0.50, 0.75, 0.90, 1.00];

/// Blocks 1..N are a coarse projection. Tighten to 5..95 so a single
/// stale-GBT leftover or CPFP orphan doesn't blow out the min/max
/// columns of an otherwise tightly clustered fee tier.
const PROJECTED_PERCENTILES: [f64; 7] = [0.05, 0.10, 0.25, 0.50, 0.75, 0.90, 0.95];

/// Per-block aggregate stats for a projected block.
///
/// `block_stats[0]` mirrors Bitcoin Core's `getblocktemplate` - the
/// node's actual next-block selection. `fee_range` spans the full
/// 0..100 percentiles.
///
/// `block_stats[1..]` are a coarse greedy-packed projection by
/// descending chunk rate, useful as a client-facing fee-tier gradient
/// but not a prediction of what miners will include. Their `fee_range`
/// is clipped to 5..95 percentiles so a single stale-GBT leftover or
/// CPFP orphan doesn't dominate the min/max columns.
#[derive(Debug, Clone, Default)]
pub struct BlockStats {
    pub tx_count: u32,
    pub total_size: u64,
    pub total_vsize: VSize,
    pub total_fee: Sats,
    pub fee_range: [FeeRate; 7],
}

impl BlockStats {
    /// Block 0 (Core's actual selection): exact 0/10/25/50/75/90/100.
    pub fn compute_core(block: &[TxIndex], txs: &[SnapTx]) -> Self {
        Self::compute(block, txs, CORE_PERCENTILES)
    }

    /// Blocks 1..N (projected): clipped 5/95 bounds to hide outliers.
    pub fn compute_projected(block: &[TxIndex], txs: &[SnapTx]) -> Self {
        Self::compute(block, txs, PROJECTED_PERCENTILES)
    }

    /// Vsize-weighted percentile distribution over `chunk_rate` -
    /// matches mempool.space's `feeRange` semantics where each tx's
    /// contribution scales with its vsize, so a tiny outlier rate
    /// only counts for its own vsize fraction.
    fn compute(block: &[TxIndex], txs: &[SnapTx], percentiles: [f64; 7]) -> Self {
        let mut total_fee = Sats::default();
        let mut total_vsize = VSize::default();
        let mut total_size: u64 = 0;
        let mut rates: Vec<(FeeRate, VSize)> = Vec::with_capacity(block.len());

        for &tx_index in block {
            let Some(t) = txs.get(tx_index.as_usize()) else {
                continue;
            };
            total_fee += t.fee;
            total_vsize += t.vsize;
            total_size += t.size;
            rates.push((t.chunk_rate, t.vsize));
        }

        rates.sort_unstable_by_key(|(r, _)| *r);

        let fee_range: [FeeRate; 7] = if rates.is_empty() {
            [FeeRate::default(); 7]
        } else {
            percentiles.map(|p| get_weighted_percentile(&rates, p))
        };

        Self {
            tx_count: rates.len() as u32,
            total_size,
            total_vsize,
            total_fee,
            fee_range,
        }
    }

    pub fn median_fee_rate(&self) -> FeeRate {
        self.fee_range[3]
    }
}
