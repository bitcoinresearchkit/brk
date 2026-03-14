use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, Indexes, OutputType, Sats, TxOutIndex};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::Vecs;
use crate::prices;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.op_return.compute(
            starting_indexes.height,
            prices,
            exit,
            |height_vec| {
                // Validate computed versions against dependencies
                let dep_version = indexer.vecs.outputs.first_txout_index.version()
                    + indexer.vecs.outputs.output_type.version()
                    + indexer.vecs.outputs.value.version();
                height_vec.validate_computed_version_or_reset(dep_version)?;

                // Get target height
                let target_len = indexer.vecs.outputs.first_txout_index.len();
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
                let first_txout_indexes: Vec<TxOutIndex> =
                    indexer.vecs.outputs.first_txout_index.collect_range_at(
                        starting_height.to_usize(),
                        target_height.to_usize()
                            + 2.min(indexer.vecs.outputs.first_txout_index.len()),
                    );

                let mut output_types_buf: Vec<OutputType> = Vec::new();
                let mut values_buf: Vec<Sats> = Vec::new();

                // Iterate blocks
                for h in starting_height.to_usize()..=target_height.to_usize() {
                    let height = Height::from(h);
                    let local_idx = h - starting_height.to_usize();

                    // Get output range for this block
                    let first_txout_index = first_txout_indexes[local_idx];
                    let next_first_txout_index =
                        if let Some(&next) = first_txout_indexes.get(local_idx + 1) {
                            next
                        } else {
                            TxOutIndex::from(indexer.vecs.outputs.value.len())
                        };

                    let out_start = first_txout_index.to_usize();
                    let out_end = next_first_txout_index.to_usize();

                    // Pre-collect both vecs into reusable buffers
                    indexer
                        .vecs
                        .outputs
                        .output_type
                        .collect_range_into_at(out_start, out_end, &mut output_types_buf);
                    indexer
                        .vecs
                        .outputs
                        .value
                        .collect_range_into_at(out_start, out_end, &mut values_buf);

                    let mut op_return_value = Sats::ZERO;
                    for (ot, val) in output_types_buf.iter().zip(values_buf.iter()) {
                        if *ot == OutputType::OpReturn {
                            op_return_value += *val;
                        }
                    }

                    height_vec.truncate_push(height, op_return_value)?;
                }

                height_vec.write()?;

                Ok(())
            },
        )?;

        Ok(())
    }
}
