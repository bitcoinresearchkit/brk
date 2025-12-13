use brk_types::{FeeRate, RecommendedFees, Sats, VSize};

use super::{MempoolEntry, MempoolTxIndex, SelectedTx};

/// Minimum fee rate for estimation (sat/vB)
const MIN_FEE_RATE: f64 = 1.0;

/// Immutable snapshot of projected blocks.
/// Stores indices into live entries + pre-computed stats.
#[derive(Debug, Clone, Default)]
pub struct ProjectedSnapshot {
    /// Block structure: indices into entries Vec
    pub blocks: Vec<Vec<MempoolTxIndex>>,
    /// Pre-computed stats per block
    pub block_stats: Vec<BlockStats>,
    /// Pre-computed fee recommendations
    pub fees: RecommendedFees,
}

/// Statistics for a single projected block.
#[derive(Debug, Clone, Default)]
pub struct BlockStats {
    pub tx_count: u32,
    pub total_vsize: VSize,
    pub total_fee: Sats,
    /// Fee rate percentiles: [0%, 10%, 25%, 50%, 75%, 90%, 100%]
    /// - fee_range[0] = min, fee_range[3] = median, fee_range[6] = max
    pub fee_range: [FeeRate; 7],
}

impl BlockStats {
    pub fn min_fee_rate(&self) -> FeeRate {
        self.fee_range[0]
    }

    pub fn median_fee_rate(&self) -> FeeRate {
        self.fee_range[3]
    }

    pub fn max_fee_rate(&self) -> FeeRate {
        self.fee_range[6]
    }
}

impl ProjectedSnapshot {
    /// Build snapshot from selected transactions (with effective fee rates) and entries.
    pub fn build(blocks: Vec<Vec<SelectedTx>>, entries: &[Option<MempoolEntry>]) -> Self {
        let block_stats: Vec<BlockStats> = blocks
            .iter()
            .map(|selected| compute_block_stats(selected, entries))
            .collect();

        let fees = compute_recommended_fees(&block_stats);

        // Convert to just indices for storage
        let blocks: Vec<Vec<MempoolTxIndex>> = blocks
            .into_iter()
            .map(|selected| selected.into_iter().map(|s| s.entries_idx).collect())
            .collect();

        Self {
            blocks,
            block_stats,
            fees,
        }
    }
}

/// Compute statistics for a single block using effective fee rates from selection time.
fn compute_block_stats(selected: &[SelectedTx], entries: &[Option<MempoolEntry>]) -> BlockStats {
    if selected.is_empty() {
        return BlockStats::default();
    }

    let mut total_fee = Sats::default();
    let mut total_vsize = VSize::default();
    let mut fee_rates: Vec<FeeRate> = Vec::with_capacity(selected.len());

    for sel in selected {
        if let Some(entry) = &entries[sel.entries_idx.as_usize()] {
            total_fee += entry.fee;
            total_vsize += entry.vsize;
            // Use the effective fee rate captured at selection time
            // This is the actual mining score that determined this tx's block placement
            fee_rates.push(sel.effective_fee_rate);
        }
    }

    fee_rates.sort();

    BlockStats {
        tx_count: selected.len() as u32,
        total_vsize,
        total_fee,
        fee_range: [
            percentile(&fee_rates, 0),
            percentile(&fee_rates, 10),
            percentile(&fee_rates, 25),
            percentile(&fee_rates, 50),
            percentile(&fee_rates, 75),
            percentile(&fee_rates, 90),
            percentile(&fee_rates, 100),
        ],
    }
}

/// Get percentile value from sorted array.
fn percentile(sorted: &[FeeRate], p: usize) -> FeeRate {
    if sorted.is_empty() {
        return FeeRate::default();
    }
    let idx = (p * (sorted.len() - 1)) / 100;
    sorted[idx]
}

/// Compute recommended fees from block stats (mempool.space style).
fn compute_recommended_fees(stats: &[BlockStats]) -> RecommendedFees {
    RecommendedFees {
        // High priority: median of block 1
        fastest_fee: median_fee_for_block(stats, 0),
        // Medium priority: median of blocks 2-3
        half_hour_fee: median_fee_for_block(stats, 2),
        // Low priority: median of blocks 4-6
        hour_fee: median_fee_for_block(stats, 5),
        // No priority: median of later blocks
        economy_fee: median_fee_for_block(stats, 7),
        minimum_fee: FeeRate::from(MIN_FEE_RATE),
    }
}

/// Get the median fee rate for block N.
fn median_fee_for_block(stats: &[BlockStats], block_index: usize) -> FeeRate {
    stats
        .get(block_index)
        .map(|s| s.median_fee_rate())
        .unwrap_or_else(|| FeeRate::from(MIN_FEE_RATE))
}
