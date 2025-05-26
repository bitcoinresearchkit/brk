use brk_core::Dollars;

use super::{RealizedState, SupplyState};

// Vecs ? probably
#[derive(Debug, Default, Clone)]
pub struct CohortState {
    pub supply: SupplyState,
    pub realized: Option<RealizedState>,
    // pub price_to_amount: PriceToValue<Amount>, save it not rounded in fjall
}

impl CohortState {
    pub fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply += supply_state;
        if let Some(realized) = self.realized.as_mut() {
            realized.increment(supply_state, price.unwrap());
        }
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply -= supply_state;
        if let Some(realized) = self.realized.as_mut() {
            realized.decrement(supply_state, price.unwrap());
        }
    }

    pub fn send(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {}

    pub fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {}
}
