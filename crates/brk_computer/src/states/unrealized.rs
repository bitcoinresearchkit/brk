use brk_types::{Dollars, Sats};

#[derive(Debug, Default, Clone)]
pub struct UnrealizedState {
    pub supply_in_profit: Sats,
    pub supply_in_loss: Sats,
    pub unrealized_profit: Dollars,
    pub unrealized_loss: Dollars,
}

impl UnrealizedState {
    pub const NAN: Self = Self {
        supply_in_profit: Sats::ZERO,
        supply_in_loss: Sats::ZERO,
        unrealized_profit: Dollars::NAN,
        unrealized_loss: Dollars::NAN,
    };

    pub const ZERO: Self = Self {
        supply_in_profit: Sats::ZERO,
        supply_in_loss: Sats::ZERO,
        unrealized_profit: Dollars::ZERO,
        unrealized_loss: Dollars::ZERO,
    };
}
