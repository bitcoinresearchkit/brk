use brk_core::{Bitcoin, CheckedSub, Dollars};

use super::SupplyState;

// Vecs ? probably
#[derive(Debug, Default, Clone)]
pub struct CohortState {
    pub supply: SupplyState,
    pub realized_cap: Option<Dollars>,
    // pub price_to_amount: PriceToValue<Amount>, save it not rounded in fjall
}

impl CohortState {
    pub fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply += supply_state;
        if let Some(realized_cap) = self.realized_cap.as_mut() {
            *realized_cap += price.unwrap() * Bitcoin::from(supply_state.value);
        }
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        if let Some(realized_cap) = self.realized_cap.as_mut() {
            *realized_cap = realized_cap
                .checked_sub(price.unwrap() * Bitcoin::from(supply_state.value))
                .unwrap();
        }
        self.supply -= supply_state;
    }
}
