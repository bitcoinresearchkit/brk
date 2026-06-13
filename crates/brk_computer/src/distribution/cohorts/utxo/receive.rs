use brk_cohort::EntryPrice;
use brk_types::{Cents, CostBasisSnapshot, Height, Timestamp};
use vecdb::Rw;

use crate::distribution::state::Transacted;

use super::groups::UTXOCohorts;

impl UTXOCohorts<Rw> {
    /// Process received outputs for this block.
    ///
    /// New UTXOs are added to:
    /// - The "under_1h" age cohort (all new UTXOs start at 0 hours old)
    /// - The appropriate epoch cohort based on block height
    /// - The appropriate class cohort based on block timestamp
    /// - The immutable entry valuation cohort based on creation price versus anchor
    /// - The appropriate output type cohort (P2PKH, P2SH, etc.)
    /// - The appropriate amount range cohort based on value
    pub(crate) fn receive(
        &mut self,
        received: Transacted,
        height: Height,
        timestamp: Timestamp,
        price: Cents,
        entry: EntryPrice,
    ) {
        let supply_state = received.spendable_supply;

        // Pre-compute snapshot once for cohorts sharing the block-level supply_state
        let snapshot = CostBasisSnapshot::from_utxo(price, &supply_state);

        // New UTXOs go into under_1h plus immutable creation cohorts
        self.age_range
            .under_1h
            .state
            .as_mut()
            .unwrap()
            .receive_utxo_snapshot(&supply_state, &snapshot);
        if let Some(v) = self.epoch.mut_vec_from_height(height) {
            v.state
                .as_mut()
                .unwrap()
                .receive_utxo_snapshot(&supply_state, &snapshot);
        }
        if let Some(v) = self.class.mut_vec_from_timestamp(timestamp) {
            v.state
                .as_mut()
                .unwrap()
                .receive_utxo_snapshot(&supply_state, &snapshot);
        }
        self.entry
            .get_mut(entry)
            .state
            .as_mut()
            .unwrap()
            .receive_utxo_snapshot(&supply_state, &snapshot);

        // Update output type cohorts (skip types with no outputs this block)
        self.type_.iter_typed_mut().for_each(|(output_type, vecs)| {
            let supply_state = received.by_type.get(output_type);
            if supply_state.utxo_count > 0 {
                vecs.state
                    .as_mut()
                    .unwrap()
                    .receive_utxo(supply_state, price)
            }
        });

        // Update amount range cohorts (skip empty ranges)
        received
            .by_size_group
            .iter_typed()
            .filter(|(_, supply_state)| supply_state.utxo_count > 0)
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
