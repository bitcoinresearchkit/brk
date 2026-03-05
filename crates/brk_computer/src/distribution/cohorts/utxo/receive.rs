use brk_types::{CostBasisSnapshot, Cents, Height, Timestamp};
use vecdb::Rw;

use crate::distribution::state::Transacted;

use super::groups::UTXOCohorts;

impl UTXOCohorts<Rw> {
    /// Process received outputs for this block.
    ///
    /// New UTXOs are added to:
    /// - The "up_to_1h" age cohort (all new UTXOs start at 0 hours old)
    /// - The appropriate epoch cohort based on block height
    /// - The appropriate class cohort based on block timestamp
    /// - The appropriate output type cohort (P2PKH, P2SH, etc.)
    /// - The appropriate amount range cohort based on value
    pub(crate) fn receive(
        &mut self,
        received: Transacted,
        height: Height,
        timestamp: Timestamp,
        price: Cents,
    ) {
        let supply_state = received.spendable_supply;

        // Pre-compute snapshot once for the 3 cohorts sharing the same supply_state
        let snapshot = CostBasisSnapshot::from_utxo(price, &supply_state);

        // New UTXOs go into up_to_1h, current epoch, and current class
        self.age_range
            .up_to_1h
            .state
            .as_mut()
            .unwrap()
            .receive_utxo_snapshot(&supply_state, &snapshot);
        self.epoch
            .mut_vec_from_height(height)
            .state
            .as_mut()
            .unwrap()
            .receive_utxo_snapshot(&supply_state, &snapshot);
        self.class
            .mut_vec_from_timestamp(timestamp)
            .state
            .as_mut()
            .unwrap()
            .receive_utxo_snapshot(&supply_state, &snapshot);

        // Update output type cohorts
        self.type_.iter_typed_mut().for_each(|(output_type, vecs)| {
            vecs.state
                .as_mut()
                .unwrap()
                .receive_utxo(received.by_type.get(output_type), price)
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
                    .receive_utxo(supply_state, price);
            });
    }
}
