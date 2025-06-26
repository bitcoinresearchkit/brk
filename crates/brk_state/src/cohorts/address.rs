use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use brk_core::{Dollars, Height, Result};

use crate::{CohortStateTrait, SupplyState, UnrealizedState};

use super::CohortState;

#[derive(Clone)]
pub struct AddressCohortState {
    pub address_count: usize,
    pub inner: CohortState,
}

impl CohortStateTrait for AddressCohortState {
    fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self> {
        Ok(Self {
            address_count: 0,
            inner: CohortState::default_and_import(path, name, compute_dollars)?,
        })
    }

    fn reset_single_iteration_values(&mut self) {
        self.inner.reset_single_iteration_values();
    }

    fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.inner.increment(supply_state, price);
    }

    fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.inner.decrement(supply_state, price);
    }

    fn decrement_price_to_amount(&mut self, supply_state: &SupplyState, price: Dollars) {
        self.inner.decrement_price_to_amount(supply_state, price);
    }

    fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.inner.receive(supply_state, price);
    }

    fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
    ) {
        self.inner.send(
            supply_state,
            current_price,
            prev_price,
            blocks_old,
            days_old,
            older_than_hour,
        );
    }

    fn compute_unrealized_states(
        &self,
        height_price: Dollars,
        date_price: Option<Dollars>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        self.inner
            .compute_unrealized_states(height_price, date_price)
    }

    fn commit(&mut self, height: Height) -> Result<()> {
        self.inner.commit(height)
    }
}

impl Deref for AddressCohortState {
    type Target = CohortState;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AddressCohortState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
