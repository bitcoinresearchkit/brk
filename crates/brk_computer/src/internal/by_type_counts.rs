//! Shared per-block per-address-type counters.
//!
//! Used by `outputs/by_type/` (counts outputs per type) and `inputs/by_type/`
//! (counts inputs per type). Walks each block's tx range, calls a scanner
//! callback that fills a `[u32; 12]` per-tx counter, and produces two
//! per-block aggregates in a single pass:
//!
//! - `entry_count` — total number of items (outputs / inputs) per type
//! - `tx_count`    — number of txs that contain at least one item of each type

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_types::{BasisPoints16, Height, Indexes, OutputType, StoredU64, TxIndex};
use vecdb::{AnyStoredVec, Exit, VecIndex, WritableVec};

use crate::internal::{
    PerBlockCumulativeRolling, PerBlockFull, PercentCumulativeRolling, RatioU64Bp16,
};

/// Per-block scan that simultaneously computes:
/// - `entry_count[type] += per_tx[type]` (sum of items)
/// - `tx_count[type] += 1 if per_tx[type] > 0` (presence flag)
///
/// `scan_tx` is called once per tx with a zeroed `[u32; 12]` buffer that
/// it must fill with the per-type item count for that tx.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_by_addr_type_block_counts(
    entry_count: &mut ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    tx_count: &mut ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    fi_batch: &[TxIndex],
    txid_len: usize,
    skip_first_tx: bool,
    starting_height: Height,
    exit: &Exit,
    mut scan_tx: impl FnMut(usize, &mut [u32; 12]) -> Result<()>,
) -> Result<()> {
    for (j, first_tx) in fi_batch.iter().enumerate() {
        let fi = first_tx.to_usize();
        let next_fi = fi_batch
            .get(j + 1)
            .map(|v| v.to_usize())
            .unwrap_or(txid_len);

        let start_tx = if skip_first_tx { fi + 1 } else { fi };

        let mut entries_per_block = [0u64; 12];
        let mut txs_per_block = [0u64; 12];

        for tx_pos in start_tx..next_fi {
            let mut per_tx = [0u32; 12];
            scan_tx(tx_pos, &mut per_tx)?;
            for (i, &n) in per_tx.iter().enumerate() {
                if n > 0 {
                    entries_per_block[i] += u64::from(n);
                    txs_per_block[i] += 1;
                }
            }
        }

        for otype in OutputType::ADDR_TYPES {
            let idx = otype as usize;
            entry_count
                .get_mut_unwrap(otype)
                .block
                .push(StoredU64::from(entries_per_block[idx]));
            tx_count
                .get_mut_unwrap(otype)
                .block
                .push(StoredU64::from(txs_per_block[idx]));
        }

        if entry_count.p2pkh.block.batch_limit_reached() {
            let _lock = exit.lock();
            for (_, v) in entry_count.iter_mut() {
                v.block.write()?;
            }
            for (_, v) in tx_count.iter_mut() {
                v.block.write()?;
            }
        }
    }

    {
        let _lock = exit.lock();
        for (_, v) in entry_count.iter_mut() {
            v.block.write()?;
        }
        for (_, v) in tx_count.iter_mut() {
            v.block.write()?;
        }
    }

    for (_, v) in entry_count.iter_mut() {
        v.compute_rest(starting_height, exit)?;
    }
    for (_, v) in tx_count.iter_mut() {
        v.compute_rest(starting_height, exit)?;
    }

    Ok(())
}

/// Compute per-type tx-count percent over total tx count, for all 8 address types.
pub(crate) fn compute_by_addr_type_tx_percents(
    tx_count: &ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    tx_percent: &mut ByAddrType<PercentCumulativeRolling<BasisPoints16>>,
    count_total: &PerBlockFull<StoredU64>,
    starting_indexes: &Indexes,
    exit: &Exit,
) -> Result<()> {
    for otype in OutputType::ADDR_TYPES {
        let source = tx_count.get_unwrap(otype);
        tx_percent
            .get_mut_unwrap(otype)
            .compute_binary::<StoredU64, StoredU64, RatioU64Bp16, _, _, _, _>(
                starting_indexes.height,
                &source.cumulative.height,
                &count_total.cumulative.height,
                source.sum.as_array().map(|w| &w.height),
                count_total.rolling.sum.as_array().map(|w| &w.height),
                exit,
            )?;
    }
    Ok(())
}
