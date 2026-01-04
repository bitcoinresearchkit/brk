use std::cmp::Ordering;

use brk_types::{CheckedSub, Dollars, SupplyState};

#[derive(Debug, Default, Clone)]
pub struct RealizedState {
    pub cap: Dollars,
    pub profit: Dollars,
    pub loss: Dollars,
    pub value_created: Dollars,
    pub value_destroyed: Dollars,
}

impl RealizedState {
    pub const NAN: Self = Self {
        cap: Dollars::NAN,
        profit: Dollars::NAN,
        loss: Dollars::NAN,
        value_created: Dollars::NAN,
        value_destroyed: Dollars::NAN,
    };

    pub fn reset_single_iteration_values(&mut self) {
        if self.cap != Dollars::NAN {
            self.profit = Dollars::ZERO;
            self.loss = Dollars::ZERO;
            self.value_created = Dollars::ZERO;
            self.value_destroyed = Dollars::ZERO;
        }
    }

    pub fn increment(&mut self, supply_state: &SupplyState, price: Dollars) {
        if supply_state.value.is_zero() {
            return;
        }

        self.increment_(price * supply_state.value)
    }

    pub fn increment_(&mut self, realized_cap: Dollars) {
        if self.cap == Dollars::NAN {
            self.cap = Dollars::ZERO;
            self.profit = Dollars::ZERO;
            self.loss = Dollars::ZERO;
            self.value_created = Dollars::ZERO;
            self.value_destroyed = Dollars::ZERO;
        }

        self.cap += realized_cap;
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Dollars) {
        self.decrement_(price * supply_state.value);
    }

    pub fn decrement_(&mut self, realized_cap: Dollars) {
        self.cap = self.cap.checked_sub(realized_cap).unwrap();
    }

    pub fn receive(&mut self, supply_state: &SupplyState, current_price: Dollars) {
        self.increment(supply_state, current_price);
    }

    pub fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Dollars,
        prev_price: Dollars,
    ) {
        let current_value = current_price * supply_state.value;
        let prev_value = prev_price * supply_state.value;

        self.value_created += current_value;
        self.value_destroyed += prev_value;

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
