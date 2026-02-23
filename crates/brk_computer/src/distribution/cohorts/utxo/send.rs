use brk_types::{Age, Height};
use rustc_hash::FxHashMap;
use vecdb::VecIndex;

use crate::distribution::{
    compute::PriceRangeMax,
    state::{BlockState, Transacted},
};

use super::groups::UTXOCohorts;

impl UTXOCohorts {
    /// Process spent inputs for this block.
    ///
    /// Each input references a UTXO created at some previous height.
    /// We need to update the cohort states based on when that UTXO was created.
    ///
    /// `price_range_max` is used to compute the peak price during each UTXO's holding period
    /// for accurate peak regret calculation.
    pub(crate) fn send(
        &mut self,
        height_to_sent: FxHashMap<Height, Transacted>,
        chain_state: &mut [BlockState],
        price_range_max: &PriceRangeMax,
    ) {
        if chain_state.is_empty() {
            return;
        }

        let last_block = chain_state.last().unwrap();
        let last_timestamp = last_block.timestamp;
        let current_price = last_block.price;
        let chain_len = chain_state.len();
        let send_height = Height::from(chain_len - 1);

        for (receive_height, sent) in height_to_sent {
            // Update chain_state to reflect spent supply
            chain_state[receive_height.to_usize()].supply -= &sent.spendable_supply;

            let block_state = &chain_state[receive_height.to_usize()];
            let prev_price = block_state.price;
            let blocks_old = chain_len - 1 - receive_height.to_usize();
            let age = Age::new(last_timestamp, block_state.timestamp, blocks_old);

            // Compute peak price during holding period for peak regret
            // This is the max price between receive and send heights
            let peak_price = price_range_max.max_between(receive_height, send_height);

            // Update age range cohort (direct index lookup)
            self.0.age_range.get_mut(age).state.as_mut().unwrap().send_utxo(
                &sent.spendable_supply,
                current_price,
                prev_price,
                peak_price,
                age,
            );

            // Update epoch cohort (direct lookup by height)
            self.0
                .epoch
                .mut_vec_from_height(receive_height)
                .state
                .as_mut().unwrap()
                .send_utxo(
                    &sent.spendable_supply,
                    current_price,
                    prev_price,
                    peak_price,
                    age,
                );

            // Update year cohort (direct lookup by timestamp)
            self.0
                .year
                .mut_vec_from_timestamp(block_state.timestamp)
                .state
                .as_mut().unwrap()
                .send_utxo(
                    &sent.spendable_supply,
                    current_price,
                    prev_price,
                    peak_price,
                    age,
                );

            // Update output type cohorts
            sent.by_type
                .spendable
                .iter_typed()
                .for_each(|(output_type, supply_state)| {
                    self.0.type_.get_mut(output_type).state.as_mut().unwrap().send_utxo(
                        supply_state,
                        current_price,
                        prev_price,
                        peak_price,
                        age,
                    )
                });

            // Update amount range cohorts
            sent.by_size_group
                .iter_typed()
                .for_each(|(group, supply_state)| {
                    self.0.amount_range.get_mut(group).state.as_mut().unwrap().send_utxo(
                        supply_state,
                        current_price,
                        prev_price,
                        peak_price,
                        age,
                    );
                });
        }
    }
}
