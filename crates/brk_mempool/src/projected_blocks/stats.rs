use brk_types::{FeeRate, Sats, VSize};

use crate::entry::Entry;
use crate::types::SelectedTx;

/// Statistics for a single projected block.
#[derive(Debug, Clone, Default)]
pub struct BlockStats {
    pub tx_count: u32,
    pub total_vsize: VSize,
    pub total_fee: Sats,
    /// Fee rate percentiles: [0%, 10%, 25%, 50%, 75%, 90%, 100%]
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

/// Compute statistics for a single block using effective fee rates from selection time.
pub fn compute_block_stats(selected: &[SelectedTx], entries: &[Option<Entry>]) -> BlockStats {
    if selected.is_empty() {
        return BlockStats::default();
    }

    let mut total_fee = Sats::default();
    let mut total_vsize = VSize::default();
    let mut fee_rates: Vec<FeeRate> = Vec::with_capacity(selected.len());

    for sel in selected {
        if let Some(entry) = &entries[sel.tx_index.as_usize()] {
            total_fee += entry.fee;
            total_vsize += entry.vsize;
            fee_rates.push(sel.effective_fee_rate);
        }
    }

    fee_rates.sort_unstable();

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
