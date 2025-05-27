use brk_core::Dollars;

use super::{RealizedState, SupplyState};

#[derive(Debug, Default, Clone)]
pub struct CohortState {
    pub supply: SupplyState,
    pub realized: Option<RealizedState>,
    // pub price_to_amount: PriceToValue<Amount>, save it not rounded in fjall
}

impl CohortState {
    pub fn reset_single_iteration_values(&mut self) {
        if let Some(realized) = self.realized.as_mut() {
            realized.reset_single_iteration_values();
        }
    }

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

    pub fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply += supply_state;
        if let Some(realized) = self.realized.as_mut() {
            realized.receive(supply_state, price.unwrap());
        }
    }

    pub fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        older_than_hour: bool,
    ) {
        self.supply -= supply_state;
        if let Some(realized) = self.realized.as_mut() {
            realized.send(
                supply_state,
                current_price.unwrap(),
                prev_price.unwrap(),
                older_than_hour,
            );
        }
    }
}
