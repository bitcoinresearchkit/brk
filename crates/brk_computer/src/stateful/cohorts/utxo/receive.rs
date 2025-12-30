use brk_types::{Dollars, Height, Timestamp};

use crate::stateful::state::Transacted;

use super::groups::UTXOCohorts;

impl UTXOCohorts {
    /// Process received outputs for this block.
    ///
    /// New UTXOs are added to:
    /// - The "up_to_1d" age cohort (all new UTXOs start at 0 days old)
    /// - The appropriate epoch cohort based on block height
    /// - The appropriate year cohort based on block timestamp
    /// - The appropriate output type cohort (P2PKH, P2SH, etc.)
    /// - The appropriate amount range cohort based on value
    pub fn receive(
        &mut self,
        received: Transacted,
        height: Height,
        timestamp: Timestamp,
        price: Option<Dollars>,
    ) {
        let supply_state = received.spendable_supply;

        // New UTXOs go into up_to_1d, current epoch, and current year
        [
            &mut self.0.age_range.up_to_1d,
            self.0.epoch.mut_vec_from_height(height),
            self.0.year.mut_vec_from_timestamp(timestamp),
        ]
        .into_iter()
        .for_each(|v| {
            v.state.as_mut().unwrap().receive(&supply_state, price);
        });

        // Update output type cohorts
        self.type_
            .iter_typed_mut()
            .for_each(|(output_type, vecs)| {
                vecs.state
                    .as_mut()
                    .unwrap()
                    .receive(received.by_type.get(output_type), price)
            });

        // Update amount range cohorts
        received
            .by_size_group
            .iter_typed()
            .for_each(|(group, supply_state)| {
                self.amount_range
                    .get_mut(group)
                    .state
                    .as_mut()
                    .unwrap()
                    .receive(supply_state, price);
            });
    }
}
