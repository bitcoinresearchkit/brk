use brk_types::{FeeRate, Sats, VSize};

use crate::{block_builder::Package, entry::Entry};

/// Statistics for a single projected block.
#[derive(Debug, Clone, Default)]
pub struct BlockStats {
    pub tx_count: u32,
    /// Total serialized size of all txs in bytes (witness + non-witness).
    pub total_size: u64,
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

/// Compute statistics for a single block. Each tx contributes its
/// containing package's `fee_rate` to the percentile distribution,
/// since that's the rate the miner collects per vsize.
pub fn compute_block_stats(block: &[Package], entries: &[Option<Entry>]) -> BlockStats {
    if block.is_empty() {
        return BlockStats::default();
    }

    let mut total_fee = Sats::default();
    let mut total_vsize = VSize::default();
    let mut total_size: u64 = 0;
    let mut fee_rates: Vec<FeeRate> = Vec::new();

    for pkg in block {
        for &tx_index in &pkg.txs {
            if let Some(entry) = &entries[tx_index.as_usize()] {
                total_fee += entry.fee;
                total_vsize += entry.vsize;
                total_size += entry.size;
                fee_rates.push(pkg.fee_rate);
            }
        }
    }

    let tx_count = fee_rates.len() as u32;
    fee_rates.sort_unstable();

    BlockStats {
        tx_count,
        total_size,
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
