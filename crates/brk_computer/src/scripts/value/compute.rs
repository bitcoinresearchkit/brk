use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, OutputType, Sats, TxOutIndex};
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::{indexes, price, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Validate computed versions against dependencies
        let dep_version = indexer.vecs.txout.height_to_first_txoutindex.version()
            + indexer.vecs.txout.txoutindex_to_outputtype.version()
            + indexer.vecs.txout.txoutindex_to_value.version();
        self.height_to_opreturn_value
            .validate_computed_version_or_reset(dep_version)?;

        // Get target height
        let target_len = indexer.vecs.txout.height_to_first_txoutindex.len();
        if target_len == 0 {
            return Ok(());
        }
        let target_height = Height::from(target_len - 1);

        // Find starting height for this vec
        let current_len = self.height_to_opreturn_value.len();
        let starting_height = Height::from(current_len.min(starting_indexes.height.to_usize()));

        if starting_height > target_height {
            return Ok(());
        }

        // Prepare iterators
        let mut height_to_first_txoutindex =
            indexer.vecs.txout.height_to_first_txoutindex.iter()?;
        let mut txoutindex_to_outputtype = indexer.vecs.txout.txoutindex_to_outputtype.iter()?;
        let mut txoutindex_to_value = indexer.vecs.txout.txoutindex_to_value.iter()?;

        // Iterate blocks
        for h in starting_height.to_usize()..=target_height.to_usize() {
            let height = Height::from(h);

            // Get output range for this block
            let first_txoutindex = height_to_first_txoutindex.get_unwrap(height);
            let next_first_txoutindex = if height < target_height {
                height_to_first_txoutindex.get_unwrap(height.incremented())
            } else {
                TxOutIndex::from(indexer.vecs.txout.txoutindex_to_value.len())
            };

            // Sum opreturn values
            let mut opreturn_value = Sats::ZERO;
            for i in first_txoutindex.to_usize()..next_first_txoutindex.to_usize() {
                let txoutindex = TxOutIndex::from(i);
                let outputtype = txoutindex_to_outputtype.get_unwrap(txoutindex);

                if outputtype == OutputType::OpReturn {
                    let value = txoutindex_to_value.get_unwrap(txoutindex);
                    opreturn_value += value;
                }
            }

            self.height_to_opreturn_value
                .truncate_push(height, opreturn_value)?;
        }

        self.height_to_opreturn_value.write()?;

        // Compute derived vecs (dateindex aggregations, etc.)
        self.indexes_to_opreturn_value.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_opreturn_value),
        )?;

        Ok(())
    }
}
