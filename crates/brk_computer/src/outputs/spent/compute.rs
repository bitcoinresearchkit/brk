use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, TxInIndex, TxOutIndex};
use tracing::info;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec, Stamp, TypedVecIterator, VecIndex,
};

use super::Vecs;
use crate::{ComputeIndexes, inputs};

const HEIGHT_BATCH: u32 = 10_000;

impl Vecs {
    pub fn compute(
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

        let mut height_to_first_txoutindex = indexer.vecs.outputs.first_txoutindex.iter()?;
        let mut height_to_first_txinindex = indexer.vecs.inputs.first_txinindex.iter()?;
        let mut txinindex_to_txoutindex = inputs.spent.txoutindex.iter()?;

        // Find starting height from min_txoutindex
        let mut min_height = Height::ZERO;
        for h in 0..=target_height.to_usize() {
            let txoutindex = height_to_first_txoutindex.get_unwrap(Height::from(h));
            if txoutindex.to_usize() > min_txoutindex {
                break;
            }
            min_height = Height::from(h);
        }

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
                height_to_first_txoutindex
                    .get_unwrap(batch_end_height + 1_u32)
                    .to_usize()
            };
            self.txinindex
                .fill_to(batch_txoutindex, TxInIndex::UNSPENT)?;

            // Get txin range for this height batch
            let txin_start = height_to_first_txinindex
                .get_unwrap(batch_start_height)
                .to_usize();
            let txin_end = if batch_end_height >= target_height {
                inputs.spent.txoutindex.len()
            } else {
                height_to_first_txinindex
                    .get_unwrap(batch_end_height + 1_u32)
                    .to_usize()
            };

            // Collect and process txins
            pairs.clear();
            for i in txin_start..txin_end {
                let txinindex = TxInIndex::from(i);
                let txoutindex = txinindex_to_txoutindex.get_unwrap(txinindex);

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
