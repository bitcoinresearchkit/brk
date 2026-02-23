use brk_error::Result;
use brk_types::{Height, Sats};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, WritableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, mining, scripts};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        scripts: &scripts::Vecs,
        mining: &mining::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute opreturn supply - copy per-block opreturn values from scripts
        self.opreturn
            .compute(starting_indexes, exit, |height_vec| {
                // Validate computed versions against dependencies

                let opreturn_dep_version = scripts.value.opreturn.sats.height.version();
                height_vec.validate_computed_version_or_reset(opreturn_dep_version)?;

                // Copy per-block opreturn values from scripts
                let scripts_target = scripts.value.opreturn.sats.height.len();
                if scripts_target > 0 {
                    let target_height = Height::from(scripts_target - 1);
                    let current_len = height_vec.len();
                    let starting_height =
                        Height::from(current_len.min(starting_indexes.height.to_usize()));

                    if starting_height <= target_height {
                        let start = starting_height.to_usize();
                        let end = target_height.to_usize() + 1;
                        scripts.value.opreturn.sats.height.fold_range_at(
                            start, end, start,
                            |idx, value| {
                                height_vec.truncate_push(Height::from(idx), value).unwrap();
                                idx + 1
                            },
                        );
                    }
                }

                height_vec.write()?;
                Ok(())
            })?;

        // 2. Compute unspendable supply = opreturn + unclaimed_rewards + genesis (at height 0)
        // Get reference to opreturn height vec for computing unspendable
        let opreturn_height = &self.opreturn.sats.height;
        let unclaimed_height = &mining.rewards.unclaimed_rewards.sats.height;

        self.unspendable
            .compute(starting_indexes, exit, |height_vec| {
                let unspendable_dep_version =
                    opreturn_height.version() + unclaimed_height.version();
                height_vec.validate_computed_version_or_reset(unspendable_dep_version)?;

                let opreturn_target = opreturn_height.len();
                if opreturn_target > 0 {
                    let target_height = Height::from(opreturn_target - 1);
                    let current_len = height_vec.len();
                    let starting_height =
                        Height::from(current_len.min(starting_indexes.height.to_usize()));

                    if starting_height <= target_height {
                        let start = starting_height.to_usize();
                        let end = target_height.to_usize() + 1;
                        let unclaimed_data = unclaimed_height.collect_range_at(start, end);
                        opreturn_height.fold_range_at(
                            start, end, start,
                            |idx, opreturn| {
                                let unclaimed = unclaimed_data[idx - start];
                                let genesis = if idx == 0 { Sats::FIFTY_BTC } else { Sats::ZERO };
                                let unspendable = genesis + opreturn + unclaimed;
                                height_vec.truncate_push(Height::from(idx), unspendable).unwrap();
                                idx + 1
                            },
                        );
                    }
                }

                height_vec.write()?;
                Ok(())
            })?;

        Ok(())
    }
}
