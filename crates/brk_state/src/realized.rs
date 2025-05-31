use std::cmp::Ordering;

use brk_core::{CheckedSub, Dollars};

use super::SupplyState;

#[derive(Debug, Default, Clone)]
pub struct RealizedState {
    pub cap: Dollars,
    pub profit: Dollars,
    pub loss: Dollars,
    pub value_created: Dollars,
    pub adj_value_created: Dollars,
    pub value_destroyed: Dollars,
    pub adj_value_destroyed: Dollars,
}

impl RealizedState {
    pub const NAN: Self = Self {
        cap: Dollars::NAN,
        profit: Dollars::NAN,
        loss: Dollars::NAN,
        value_created: Dollars::NAN,
        adj_value_created: Dollars::NAN,
        value_destroyed: Dollars::NAN,
        adj_value_destroyed: Dollars::NAN,
    };

    pub fn reset_single_iteration_values(&mut self) {
        if self.cap != Dollars::NAN {
            self.profit = Dollars::ZERO;
            self.loss = Dollars::ZERO;
            self.value_created = Dollars::ZERO;
            self.adj_value_created = Dollars::ZERO;
            self.value_destroyed = Dollars::ZERO;
            self.adj_value_destroyed = Dollars::ZERO;
        }
    }

    pub fn increment(&mut self, supply_state: &SupplyState, price: Dollars) {
        if supply_state.value.is_zero() {
            return;
        }

        if self.cap == Dollars::NAN {
            self.cap = Dollars::ZERO;
            self.profit = Dollars::ZERO;
            self.loss = Dollars::ZERO;
            self.value_created = Dollars::ZERO;
            self.adj_value_created = Dollars::ZERO;
            self.value_destroyed = Dollars::ZERO;
            self.adj_value_destroyed = Dollars::ZERO;
        }

        let value = price * supply_state.value;
        self.cap += value;
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Dollars) {
        let value = price * supply_state.value;
        self.cap = self.cap.checked_sub(value).unwrap();
    }

    pub fn receive(&mut self, supply_state: &SupplyState, current_price: Dollars) {
        self.increment(supply_state, current_price);
    }

    pub fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Dollars,
        prev_price: Dollars,
        older_than_hour: bool,
    ) {
        let current_value = current_price * supply_state.value;
        let prev_value = prev_price * supply_state.value;

        self.value_created += current_value;
        self.value_destroyed += prev_value;

        if older_than_hour {
            self.adj_value_created += current_value;
            self.adj_value_destroyed += prev_value;
        }

        match current_price.cmp(&prev_price) {
            Ordering::Greater => {
                self.profit += current_value.checked_sub(prev_value).unwrap();
            }
            Ordering::Less => {
                self.loss += prev_value.checked_sub(current_value).unwrap();
            }
            Ordering::Equal => {}
        }

        self.decrement(supply_state, prev_price);
    }
}
