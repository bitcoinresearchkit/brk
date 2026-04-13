use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_types::{BasisPoints16, Height, OutputType, StoredU64, TxIndex};
use vecdb::{AnyStoredVec, Exit, VecIndex, WritableVec};

use crate::internal::{
    PerBlockCumulativeRolling, PerBlockFull, PercentCumulativeRolling, RatioU64Bp16,
};

pub(super) fn compute_type_counts(
    by_type: &mut ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    fi_batch: &[TxIndex],
    txid_len: usize,
    skip_first_tx: bool,
    starting_height: Height,
    exit: &Exit,
    mut scan_tx: impl FnMut(usize) -> Result<u16>,
) -> Result<()> {
    for (j, first_tx) in fi_batch.iter().enumerate() {
        let fi = first_tx.to_usize();
        let next_fi = fi_batch
            .get(j + 1)
            .map(|v| v.to_usize())
            .unwrap_or(txid_len);

        let start_tx = if skip_first_tx { fi + 1 } else { fi };

        let mut counts = [0u64; 12];

        for tx_pos in start_tx..next_fi {
            let seen = scan_tx(tx_pos)?;

            let mut bits = seen;
            while bits != 0 {
                let idx = bits.trailing_zeros() as usize;
                counts[idx] += 1;
                bits &= bits - 1;
            }
        }

        for otype in OutputType::ADDR_TYPES {
            by_type
                .get_mut_unwrap(otype)
                .block
                .push(StoredU64::from(counts[otype as usize]));
        }

        if by_type.p2pkh.block.batch_limit_reached() {
            let _lock = exit.lock();
            for (_, v) in by_type.iter_mut() {
                v.block.write()?;
            }
        }
    }

    {
        let _lock = exit.lock();
        for (_, v) in by_type.iter_mut() {
            v.block.write()?;
        }
    }

    for (_, v) in by_type.iter_mut() {
        v.compute_rest(starting_height, exit)?;
    }

    Ok(())
}

pub(super) fn compute_type_percents(
    by_type: &ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    percent: &mut ByAddrType<PercentCumulativeRolling<BasisPoints16>>,
    count_total: &PerBlockFull<StoredU64>,
    starting_height: Height,
    exit: &Exit,
) -> Result<()> {
    for otype in OutputType::ADDR_TYPES {
        let source = by_type.get_unwrap(otype);
        percent
            .get_mut_unwrap(otype)
            .compute_binary::<StoredU64, StoredU64, RatioU64Bp16, _, _, _, _>(
                starting_height,
                &source.cumulative.height,
                &count_total.cumulative.height,
                source.sum.as_array().map(|w| &w.height),
                count_total.rolling.sum.as_array().map(|w| &w.height),
                exit,
            )?;
    }
    Ok(())
}
