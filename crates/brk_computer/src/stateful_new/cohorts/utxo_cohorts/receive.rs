//! Processing received outputs (new UTXOs).

use brk_grouper::{Filter, Filtered};
use brk_types::{Dollars, Height};

use crate::states::Transacted;

use super::UTXOCohorts;

impl UTXOCohorts {
    /// Process received outputs for this block.
    ///
    /// New UTXOs are added to:
    /// - The "up_to_1d" age cohort (all new UTXOs start at 0 days old)
    /// - The appropriate epoch cohort based on block height
    /// - The appropriate output type cohort (P2PKH, P2SH, etc.)
    /// - The appropriate amount range cohort based on value
    pub fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>) {
        let supply_state = received.spendable_supply;

        // New UTXOs go into up_to_1d and current epoch
        [
            &mut self.0.age_range.up_to_1d,
            self.0.epoch.mut_vec_from_height(height),
        ]
        .into_iter()
        .for_each(|v| {
            v.state.as_mut().unwrap().receive(&supply_state, price);
        });

        // Update aggregate cohorts' price_to_amount
        // New UTXOs have days_old = 0, so check if filter includes day 0
        if let Some(price) = price
            && supply_state.value.is_not_zero()
        {
            self.0
                .iter_aggregate_mut()
                .filter(|v| v.filter().contains_time(0))
                .for_each(|v| {
                    v.price_to_amount
                        .as_mut()
                        .unwrap()
                        .increment(price, &supply_state);
                });
        }

        // Update output type cohorts
        self.type_.iter_mut().for_each(|vecs| {
            let output_type = match vecs.filter() {
                Filter::Type(output_type) => *output_type,
                _ => unreachable!(),
            };
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
