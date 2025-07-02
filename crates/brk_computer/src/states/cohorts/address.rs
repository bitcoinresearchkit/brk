use std::path::Path;

use brk_core::{AddressData, Dollars, Height, Result, Sats};

use crate::SupplyState;

use super::CohortState;

#[derive(Clone)]
pub struct AddressCohortState {
    pub address_count: usize,
    pub inner: CohortState,
}

impl AddressCohortState {
    pub fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self> {
        Ok(Self {
            address_count: 0,
            inner: CohortState::default_and_import(path, name, compute_dollars)?,
        })
    }

    pub fn reset_single_iteration_values(&mut self) {
        self.inner.reset_single_iteration_values();
    }

    pub fn send(
        &mut self,
        value: Sats,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
    ) {
        self.inner.send(
            &SupplyState { utxos: 1, value },
            current_price,
            prev_price,
            blocks_old,
            days_old,
            older_than_hour,
        );
    }

    pub fn receive(&mut self, value: Sats, price: Option<Dollars>) {
        self.inner.receive(&SupplyState { utxos: 1, value }, price);
    }

    pub fn add(&mut self, addressdata: &AddressData) {
        self.address_count += 1;
        self.inner
            .increment_(&addressdata.into(), addressdata.realized_cap);
    }

    pub fn subtract(&mut self, addressdata: &AddressData) {
        self.address_count.checked_sub(1).unwrap();
        self.inner
            .decrement_(&addressdata.into(), addressdata.realized_cap);
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        self.inner.commit(height)
    }
}

//     fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
//         self.inner.decrement(supply_state, price);
//     }

//     fn decrement_price_to_amount(&mut self, supply_state: &SupplyState, price: Dollars) {
//         self.inner.decrement_price_to_amount(supply_state, price);
//     }

//     fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
//         self.inner.receive(supply_state, price);
//     }

//     fn compute_unrealized_states(
//         &self,
//         height_price: Dollars,
//         date_price: Option<Dollars>,
//     ) -> (UnrealizedState, Option<UnrealizedState>) {
//         self.inner
//             .compute_unrealized_states(height_price, date_price)
//     }

// }

// impl Deref for AddressCohortState {
//     type Target = CohortState;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl DerefMut for AddressCohortState {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }
