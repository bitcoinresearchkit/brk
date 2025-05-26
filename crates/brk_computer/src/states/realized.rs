use brk_core::{Bitcoin, CheckedSub, Dollars};

use super::SupplyState;

#[derive(Debug, Default, Clone)]
pub struct RealizedState {
    pub realized_cap: Dollars,
    // pub realized_profit: Dollars, // sent price vs now price
    // pub realized_loss: Dollars, // sent price vs now price
    // pub value_created: Dollars, // supply * price now
    // pub adjusted_value_created: Dollars, // supply - up to 1 hour supply * price now
    // pub value_destroyed: Dollars, // supply * price then
    // pub adjusted_value_destroyed: Dollars, // supply - up to 1 hour supply * price then
}

impl RealizedState {
    pub const NAN: Self = Self {
        realized_cap: Dollars::NAN,
        // realized_profit: Dollars::NAN,
        // realized_loss: Dollars::NAN,
        // value_created: Dollars::NAN,
        // adjusted_value_created: Dollars::NAN,
        // value_destroyed: Dollars::NAN,
        // adjusted_value_destroyed: Dollars::NAN,
    };

    pub fn increment(&mut self, supply_state: &SupplyState, price: Dollars) {
        if supply_state.value.is_not_zero() {
            if self.realized_cap == Dollars::NAN {
                self.realized_cap = Dollars::ZERO;
                // self.realized_profit = Dollars::ZERO;
                // self.realized_loss = Dollars::ZERO;
                // self.value_created = Dollars::ZERO;
                // self.adjusted_value_created = Dollars::ZERO;
                // self.value_destroyed = Dollars::ZERO;
                // self.adjusted_value_destroyed = Dollars::ZERO;
            }

            self.realized_cap += price * Bitcoin::from(supply_state.value);
        }
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Dollars) {
        self.realized_cap = self
            .realized_cap
            .checked_sub(price * Bitcoin::from(supply_state.value))
            .unwrap();
    }
}
