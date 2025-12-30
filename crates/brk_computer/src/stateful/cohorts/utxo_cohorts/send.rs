use brk_types::{CheckedSub, Height};
use rustc_hash::FxHashMap;
use vecdb::VecIndex;

use crate::{
    stateful::state::{BlockState, Transacted},
    utils::OptionExt,
};

use super::UTXOCohorts;

impl UTXOCohorts {
    /// Process spent inputs for this block.
    ///
    /// Each input references a UTXO created at some previous height.
    /// We need to update the cohort states based on when that UTXO was created.
    pub fn send(
        &mut self,
        height_to_sent: FxHashMap<Height, Transacted>,
        chain_state: &mut [BlockState],
    ) {
        if chain_state.is_empty() {
            return;
        }

        let last_block = chain_state.last().unwrap();
        let last_timestamp = last_block.timestamp;
        let current_price = last_block.price;
        let chain_len = chain_state.len();

        for (height, sent) in height_to_sent {
            // Update chain_state to reflect spent supply
            chain_state[height.to_usize()].supply -= &sent.spendable_supply;

            let block_state = &chain_state[height.to_usize()];
            let prev_price = block_state.price;
            let blocks_old = chain_len - 1 - height.to_usize();
            let days_old = last_timestamp.difference_in_days_between(block_state.timestamp);
            let days_old_float =
                last_timestamp.difference_in_days_between_float(block_state.timestamp);
            let older_than_hour = last_timestamp
                .checked_sub(block_state.timestamp)
                .unwrap()
                .is_more_than_hour();

            // Update age range cohort (direct index lookup)
            self.0
                .age_range
                .get_mut_by_days_old(days_old)
                .state
                .um()
                .send(
                    &sent.spendable_supply,
                    current_price,
                    prev_price,
                    blocks_old,
                    days_old_float,
                    older_than_hour,
                );

            // Update epoch cohort (direct lookup by height)
            self.0.epoch.mut_vec_from_height(height).state.um().send(
                &sent.spendable_supply,
                current_price,
                prev_price,
                blocks_old,
                days_old_float,
                older_than_hour,
            );

            // Update year cohort (direct lookup by timestamp)
            self.0
                .year
                .mut_vec_from_timestamp(block_state.timestamp)
                .state
                .um()
                .send(
                    &sent.spendable_supply,
                    current_price,
                    prev_price,
                    blocks_old,
                    days_old_float,
                    older_than_hour,
                );

            // Update output type cohorts
            sent.by_type
                .spendable
                .iter_typed()
                .for_each(|(output_type, supply_state)| {
                    self.0.type_.get_mut(output_type).state.um().send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    )
                });

            // Update amount range cohorts
            sent.by_size_group
                .iter_typed()
                .for_each(|(group, supply_state)| {
                    self.0.amount_range.get_mut(group).state.um().send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    );
                });
        }
    }
}
