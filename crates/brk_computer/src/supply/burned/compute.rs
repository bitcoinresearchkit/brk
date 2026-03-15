use brk_error::Result;
use brk_types::{Height, Indexes, Sats};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::Vecs;
use crate::{mining, prices, scripts};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        scripts: &scripts::Vecs,
        mining: &mining::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let op_return_height = &scripts.value.op_return.base.sats.height;
        let unclaimed_height = &mining.rewards.unclaimed.base.sats.height;

        self.unspendable.compute(
            starting_indexes.height,
            prices,
            exit,
            |height_vec| {
                let unspendable_dep_version =
                    op_return_height.version() + unclaimed_height.version();
                height_vec.validate_computed_version_or_reset(unspendable_dep_version)?;

                let op_return_target = op_return_height.len();
                if op_return_target > 0 {
                    let target_height = Height::from(op_return_target - 1);
                    let current_len = height_vec.len();
                    let starting_height =
                        Height::from(current_len.min(starting_indexes.height.to_usize()));

                    if starting_height <= target_height {
                        let start = starting_height.to_usize();
                        let end = target_height.to_usize() + 1;
                        let unclaimed_data = unclaimed_height.collect_range_at(start, end);
                        height_vec.truncate_if_needed(starting_height)?;
                        op_return_height.fold_range_at(start, end, start, |idx, op_return| {
                            let unclaimed = unclaimed_data[idx - start];
                            let genesis = if idx == 0 {
                                Sats::FIFTY_BTC
                            } else {
                                Sats::ZERO
                            };
                            let unspendable = genesis + op_return + unclaimed;
                            height_vec.push(unspendable);
                            idx + 1
                        });
                    }
                }

                height_vec.write()?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
