use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, TxInIndex, TxOutIndex};
use tracing::info;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, WritableVec, ReadableVec, Stamp, VecIndex,
};

use super::Vecs;
use crate::{inputs, ComputeIndexes};

const HEIGHT_BATCH: u32 = 10_000;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        db: &Database,
        indexer: &Indexer,
        inputs: &inputs::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let target_height = indexer.vecs.blocks.blockhash.len();
        if target_height == 0 {
            return Ok(());
        }
        let target_height = Height::from(target_height - 1);

        // Find min_height from current vec length
        let current_txoutindex = self.txinindex.len();
        let min_txoutindex = current_txoutindex.min(starting_indexes.txoutindex.to_usize());

        let starting_stamp = Stamp::from(starting_indexes.height);
        let _ = self.txinindex.rollback_before(starting_stamp);

        self.txinindex
            .truncate_if_needed(TxOutIndex::from(min_txoutindex))?;

        let txinindex_to_txoutindex = &inputs.spent.txoutindex;

        // Find min_height via binary search (first_txoutindex is monotonically non-decreasing)
        let first_txoutindex_vec = &indexer.vecs.outputs.first_txoutindex;
        let total_heights = target_height.to_usize() + 1;
        let min_height = if min_txoutindex == 0 {
            Height::ZERO
        } else {
            let mut lo = 0usize;
            let mut hi = total_heights;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if first_txoutindex_vec.collect_one_at(mid).unwrap().to_usize() <= min_txoutindex {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            Height::from(lo.saturating_sub(1))
        };

        // Only collect from min_height onward (not from 0)
        let offset = min_height.to_usize();
        let first_txoutindex_data = first_txoutindex_vec
            .collect_range_at(offset, target_height.to_usize() + 1);
        let first_txinindex_data = indexer
            .vecs
            .inputs
            .first_txinindex
            .collect_range_at(offset, target_height.to_usize() + 2);

        // Validate: computed height must not exceed starting height
        assert!(
            min_height <= starting_indexes.height,
            "txouts min_height ({}) exceeds starting_indexes.height ({})",
            min_height,
            starting_indexes.height
        );

        let mut pairs: Vec<(TxOutIndex, TxInIndex)> = Vec::new();

        let mut batch_start_height = min_height;
        while batch_start_height <= target_height {
            let batch_end_height = (batch_start_height + HEIGHT_BATCH).min(target_height);

            // Fill txoutindex up to batch_end_height + 1
            let batch_txoutindex = if batch_end_height >= target_height {
                indexer.vecs.outputs.value.len()
            } else {
                first_txoutindex_data[batch_end_height.to_usize() + 1 - offset].to_usize()
            };
            self.txinindex
                .fill_to(batch_txoutindex, TxInIndex::UNSPENT)?;

            // Get txin range for this height batch
            let txin_start = first_txinindex_data[batch_start_height.to_usize() - offset].to_usize();
            let txin_end = if batch_end_height >= target_height {
                inputs.spent.txoutindex.len()
            } else {
                first_txinindex_data[batch_end_height.to_usize() + 1 - offset].to_usize()
            };

            // Collect and process txins
            pairs.clear();
            let txoutindexes: Vec<TxOutIndex> = txinindex_to_txoutindex.collect_range_at(txin_start, txin_end);
            for (j, txoutindex) in txoutindexes.into_iter().enumerate() {
                let txinindex = TxInIndex::from(txin_start + j);

                if txoutindex.is_coinbase() {
                    continue;
                }

                pairs.push((txoutindex, txinindex));
            }

            pairs.sort_unstable_by_key(|(txoutindex, _)| *txoutindex);

            for &(txoutindex, txinindex) in &pairs {
                self.txinindex.update(txoutindex, txinindex)?;
            }

            if batch_end_height < target_height {
                let _lock = exit.lock();
                self.txinindex.write()?;
                info!(
                    "TxOuts: {:.2}%",
                    batch_end_height.to_usize() as f64 / target_height.to_usize() as f64 * 100.0
                );
                db.flush()?;
            }

            batch_start_height = batch_end_height + 1_u32;
        }

        let _lock = exit.lock();
        self.txinindex
            .stamped_write_with_changes(Stamp::from(target_height))?;
        db.flush()?;

        Ok(())
    }
}
