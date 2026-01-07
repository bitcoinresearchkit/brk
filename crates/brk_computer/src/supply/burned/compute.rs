use brk_error::Result;
use brk_types::{Height, Sats};
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::{blocks, indexes, price, scripts, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        scripts: &scripts::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute opreturn supply - copy per-block opreturn values from scripts
        self.indexes_to_opreturn.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |height_vec| {
                // Validate computed versions against dependencies
                // KISS: height is now inside indexes_to_opreturn_value.sats.height
                let opreturn_dep_version =
                    scripts.value.indexes_to_opreturn_value.sats.height.version();
                height_vec.validate_computed_version_or_reset(opreturn_dep_version)?;

                // Copy per-block opreturn values from scripts
                let scripts_target = scripts.value.indexes_to_opreturn_value.sats.height.len();
                if scripts_target > 0 {
                    let target_height = Height::from(scripts_target - 1);
                    let current_len = height_vec.len();
                    let starting_height =
                        Height::from(current_len.min(starting_indexes.height.to_usize()));

                    if starting_height <= target_height {
                        let mut opreturn_value_iter =
                            scripts.value.indexes_to_opreturn_value.sats.height.into_iter();

                        for h in starting_height.to_usize()..=target_height.to_usize() {
                            let height = Height::from(h);
                            let value = opreturn_value_iter.get_unwrap(height);
                            height_vec.truncate_push(height, value)?;
                        }
                    }
                }

                height_vec.write()?;
                Ok(())
            },
        )?;

        // 2. Compute unspendable supply = opreturn + unclaimed_rewards + genesis (at height 0)
        // Get reference to opreturn height vec for computing unspendable
        let opreturn_height = &self.indexes_to_opreturn.sats.height;
        let unclaimed_height = &blocks.rewards.indexes_to_unclaimed_rewards.sats.height;

        self.indexes_to_unspendable.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |height_vec| {
                // KISS: height is now a concrete field (no Option)
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
                        let mut opreturn_iter = opreturn_height.into_iter();
                        // KISS: height is now a concrete field (no Option)
                        let mut unclaimed_rewards_iter = unclaimed_height.into_iter();

                        for h in starting_height.to_usize()..=target_height.to_usize() {
                            let height = Height::from(h);

                            // Genesis block 50 BTC is unspendable (only at height 0)
                            let genesis = if height == Height::ZERO {
                                Sats::FIFTY_BTC
                            } else {
                                Sats::ZERO
                            };

                            // Per-block opreturn value
                            let opreturn = opreturn_iter.get_unwrap(height);

                            // Per-block unclaimed rewards
                            let unclaimed = unclaimed_rewards_iter.get_unwrap(height);

                            let unspendable = genesis + opreturn + unclaimed;
                            height_vec.truncate_push(height, unspendable)?;
                        }
                    }
                }

                height_vec.write()?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
