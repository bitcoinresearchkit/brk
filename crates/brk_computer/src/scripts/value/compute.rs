use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, OutputType, Sats, TxOutIndex};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, WritableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, blocks, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        count_vecs: &blocks::CountVecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = count_vecs.window_starts();

        self.opreturn
            .compute(starting_indexes.height, &window_starts, prices, exit, |height_vec| {

                // Validate computed versions against dependencies
                let dep_version = indexer.vecs.outputs.first_txoutindex.version()
                    + indexer.vecs.outputs.outputtype.version()
                    + indexer.vecs.outputs.value.version();
                height_vec.validate_computed_version_or_reset(dep_version)?;

                // Get target height
                let target_len = indexer.vecs.outputs.first_txoutindex.len();
                if target_len == 0 {
                    return Ok(());
                }
                let target_height = Height::from(target_len - 1);

                // Find starting height for this vec
                let current_len = height_vec.len();
                let starting_height =
                    Height::from(current_len.min(starting_indexes.height.to_usize()));

                if starting_height > target_height {
                    return Ok(());
                }

                // Pre-collect height-indexed data
                let first_txoutindexes: Vec<TxOutIndex> = indexer.vecs.outputs.first_txoutindex
                    .collect_range_at(starting_height.to_usize(), target_height.to_usize() + 2.min(indexer.vecs.outputs.first_txoutindex.len()));

                // Iterate blocks
                for h in starting_height.to_usize()..=target_height.to_usize() {
                    let height = Height::from(h);
                    let local_idx = h - starting_height.to_usize();

                    // Get output range for this block
                    let first_txoutindex = first_txoutindexes[local_idx];
                    let next_first_txoutindex = if let Some(&next) = first_txoutindexes.get(local_idx + 1) {
                        next
                    } else {
                        TxOutIndex::from(indexer.vecs.outputs.value.len())
                    };

                    let out_start = first_txoutindex.to_usize();
                    let out_end = next_first_txoutindex.to_usize();

                    // Sum opreturn values — fold over both vecs without allocation
                    let opreturn_value = indexer.vecs.outputs.outputtype.fold_range_at(
                        out_start, out_end,
                        (Sats::ZERO, out_start),
                        |(sum, vi), ot| {
                            let new_sum = if ot == OutputType::OpReturn {
                                sum + indexer.vecs.outputs.value.collect_one_at(vi).unwrap()
                            } else {
                                sum
                            };
                            (new_sum, vi + 1)
                        },
                    ).0;

                    height_vec.truncate_push(height, opreturn_value)?;
                }

                height_vec.write()?;

                Ok(())
            })?;

        Ok(())
    }
}
