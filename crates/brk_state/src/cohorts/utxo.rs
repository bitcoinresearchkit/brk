use std::path::Path;

use brk_core::{Dollars, Height, Result};
use derive_deref::{Deref, DerefMut};

use crate::{SupplyState, UnrealizedState};

use super::CohortState;

#[derive(Clone, Deref, DerefMut)]
pub struct UTXOCohortState(CohortState);

impl UTXOCohortState {
    pub fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self> {
        Ok(Self(CohortState::default_and_import(
            path,
            name,
            compute_dollars,
        )?))
    }
}

//     fn reset_single_iteration_values(&mut self) {
//         self.0.reset_single_iteration_values();
//     }

//     fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
//         self.0.increment(supply_state, price);
//     }

//     fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
//         self.0.decrement(supply_state, price);
//     }

//     fn decrement_price_to_amount(&mut self, supply_state: &SupplyState, price: Dollars) {
//         self.0.decrement_price_to_amount(supply_state, price);
//     }

//     fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
//         self.0.receive(supply_state, price);
//     }

//     fn send(
//         &mut self,
//         supply_state: &SupplyState,
//         current_price: Option<Dollars>,
//         prev_price: Option<Dollars>,
//         blocks_old: usize,
//         days_old: f64,
//         older_than_hour: bool,
//     ) {
//         self.0.send(
//             supply_state,
//             current_price,
//             prev_price,
//             blocks_old,
//             days_old,
//             older_than_hour,
//         );
//     }

//     fn compute_unrealized_states(
//         &self,
//         height_price: Dollars,
//         date_price: Option<Dollars>,
//     ) -> (UnrealizedState, Option<UnrealizedState>) {
//         self.0.compute_unrealized_states(height_price, date_price)
//     }

//     fn commit(&mut self, height: Height) -> Result<()> {
//         self.0.commit(height)
//     }
// }
