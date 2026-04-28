use brk_types::{FeeRate, Sats, VSize};

use crate::TxEntry;

use super::super::linearize::Package;

/// Percentile points reported in [`BlockStats::fee_range`], in the
/// same order: 0% (min), 10%, 25%, median, 75%, 90%, 100% (max).
const PERCENTILES: [usize; 7] = [0, 10, 25, 50, 75, 90, 100];

#[derive(Debug, Clone, Default)]
pub struct BlockStats {
    pub tx_count: u32,
    /// Total serialized size of all txs in bytes (witness + non-witness).
    pub total_size: u64,
    pub total_vsize: VSize,
    pub total_fee: Sats,
    /// Fee-rate samples at the points listed in `PERCENTILES`.
    pub fee_range: [FeeRate; PERCENTILES.len()],
}

impl BlockStats {
    /// Each tx contributes its containing package's `fee_rate` to the
    /// percentile distribution, since that's the rate the miner
    /// collects per vsize.
    pub fn compute(block: &[Package], entries: &[Option<TxEntry>]) -> Self {
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

        Self {
            tx_count,
            total_size,
            total_vsize,
            total_fee,
            fee_range: PERCENTILES.map(|p| percentile(&fee_rates, p)),
        }
    }

    pub fn min_fee_rate(&self) -> FeeRate {
        self.fee_range[0]
    }

    pub fn median_fee_rate(&self) -> FeeRate {
        self.fee_range[3]
    }

    pub fn max_fee_rate(&self) -> FeeRate {
        self.fee_range[PERCENTILES.len() - 1]
    }
}

fn percentile(sorted: &[FeeRate], p: usize) -> FeeRate {
    if sorted.is_empty() {
        return FeeRate::default();
    }
    let idx = (p * (sorted.len() - 1)) / 100;
    sorted[idx]
}
