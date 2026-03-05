use brk_types::{Age, Height};
use rustc_hash::FxHashMap;
use vecdb::{Rw, VecIndex};

use crate::distribution::{
    compute::PriceRangeMax,
    state::{BlockState, CohortState, Transacted},
};

use super::groups::UTXOCohorts;

impl UTXOCohorts<Rw> {
    /// Process spent inputs for this block.
    ///
    /// Each input references a UTXO created at some previous height.
    /// We need to update the cohort states based on when that UTXO was created.
    ///
    /// `price_range_max` is used to compute the peak price during each UTXO's holding period
    /// for accurate peak regret calculation.
    /// Returns the minimum receive_height that was modified, if any.
    pub(crate) fn send(
        &mut self,
        height_to_sent: FxHashMap<Height, Transacted>,
        chain_state: &mut [BlockState],
        price_range_max: &PriceRangeMax,
    ) -> Option<Height> {
        if chain_state.is_empty() {
            return None;
        }

        let last_block = chain_state.last().unwrap();
        let last_timestamp = last_block.timestamp;
        let current_price = last_block.price;
        let chain_len = chain_state.len();
        let send_height = Height::from(chain_len - 1);
        let mut min_receive_height: Option<Height> = None;

        for (receive_height, sent) in height_to_sent {
            min_receive_height =
                Some(min_receive_height.map_or(receive_height, |cur| cur.min(receive_height)));
            // Update chain_state to reflect spent supply
            chain_state[receive_height.to_usize()].supply -= &sent.spendable_supply;

            let block_state = &chain_state[receive_height.to_usize()];
            let prev_price = block_state.price;
            let blocks_old = chain_len - 1 - receive_height.to_usize();
            let age = Age::new(last_timestamp, block_state.timestamp, blocks_old);

            // Compute peak price during holding period for peak regret
            // This is the max price between receive and send heights
            let peak_price = price_range_max.max_between(receive_height, send_height);

            // Pre-compute once for age_range, epoch, year (all share sent.spendable_supply)
            if let Some(pre) = CohortState::precompute_send(
                &sent.spendable_supply,
                current_price,
                prev_price,
                peak_price,
                age,
            ) {
                self.age_range
                    .get_mut(age)
                    .state
                    .as_mut()
                    .unwrap()
                    .send_utxo_precomputed(&sent.spendable_supply, &pre);
                self.epoch
                    .mut_vec_from_height(receive_height)
                    .state
                    .as_mut()
                    .unwrap()
                    .send_utxo_precomputed(&sent.spendable_supply, &pre);
                self.class
                    .mut_vec_from_timestamp(block_state.timestamp)
                    .state
                    .as_mut()
                    .unwrap()
                    .send_utxo_precomputed(&sent.spendable_supply, &pre);
            } else if sent.spendable_supply.utxo_count > 0 {
                // Zero-value UTXOs: just subtract supply
                self.age_range.get_mut(age).state.as_mut().unwrap().supply -=
                    &sent.spendable_supply;
                self.epoch
                    .mut_vec_from_height(receive_height)
                    .state
                    .as_mut()
                    .unwrap()
                    .supply -= &sent.spendable_supply;
                self.class
                    .mut_vec_from_timestamp(block_state.timestamp)
                    .state
                    .as_mut()
                    .unwrap()
                    .supply -= &sent.spendable_supply;
            }

            // Update output type cohorts
            sent.by_type
                .spendable
                .iter_typed()
                .for_each(|(output_type, supply_state)| {
                    self.type_
                        .get_mut(output_type)
                        .state
                        .as_mut()
                        .unwrap()
                        .send_utxo(supply_state, current_price, prev_price, peak_price, age)
                });

            // Update amount range cohorts
            sent.by_size_group
                .iter_typed()
                .for_each(|(group, supply_state)| {
                    self.amount_range
                        .get_mut(group)
                        .state
                        .as_mut()
                        .unwrap()
                        .send_utxo(supply_state, current_price, prev_price, peak_price, age);
                });
        }

        min_receive_height
    }
}
