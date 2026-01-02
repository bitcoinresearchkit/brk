use brk_error::Result;
use brk_types::{Height, Sats};
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::{blocks, indexes, price, scripts, utils::OptionExt, ComputeIndexes};

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
        // Validate computed versions against dependencies
        let opreturn_dep_version = scripts.value.height_to_opreturn_value.version();
        self.height_to_opreturn
            .validate_computed_version_or_reset(opreturn_dep_version)?;

        let unspendable_dep_version = self.height_to_opreturn.version()
            + blocks
                .rewards
                .indexes_to_unclaimed_rewards
                .sats
                .height
                .u()
                .version();
        self.height_to_unspendable
            .validate_computed_version_or_reset(unspendable_dep_version)?;

        // 1. Copy per-block opreturn values from scripts
        let scripts_target = scripts.value.height_to_opreturn_value.len();
        if scripts_target > 0 {
            let target_height = Height::from(scripts_target - 1);
            let current_len = self.height_to_opreturn.len();
            let starting_height = Height::from(current_len.min(starting_indexes.height.to_usize()));

            if starting_height <= target_height {
                let mut opreturn_value_iter = scripts.value.height_to_opreturn_value.into_iter();

                for h in starting_height.to_usize()..=target_height.to_usize() {
                    let height = Height::from(h);
                    let value = opreturn_value_iter.get_unwrap(height);
                    self.height_to_opreturn.truncate_push(height, value)?;
                }
            }
        }

        self.height_to_opreturn.write()?;

        // 2. Compute per-block unspendable = opreturn + unclaimed_rewards + genesis (at height 0)
        let opreturn_target = self.height_to_opreturn.len();
        if opreturn_target > 0 {
            let target_height = Height::from(opreturn_target - 1);
            let current_len = self.height_to_unspendable.len();
            let starting_height = Height::from(current_len.min(starting_indexes.height.to_usize()));

            if starting_height <= target_height {
                let mut opreturn_iter = self.height_to_opreturn.into_iter();
                let mut unclaimed_rewards_iter = blocks
                    .rewards
                    .indexes_to_unclaimed_rewards
                    .sats
                    .height
                    .u()
                    .into_iter();

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
                    self.height_to_unspendable
                        .truncate_push(height, unspendable)?;
                }
            }
        }

        self.height_to_unspendable.write()?;

        // Compute index aggregations
        self.indexes_to_opreturn.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_opreturn),
        )?;

        self.indexes_to_unspendable.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_unspendable),
        )?;

        Ok(())
    }
}
