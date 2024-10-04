use std::ops::AddAssign;

use allocative::Allocative;

use crate::structs::Price;

#[derive(Debug, Default, Allocative)]
pub struct CapitalizationState {
    realized_cap: Price,
}

impl CapitalizationState {
    pub fn realized_cap(&self) -> Price {
        self.realized_cap
    }

    pub fn increment(&mut self, realized_cap: Price) {
        self.realized_cap += realized_cap;
    }

    pub fn decrement(&mut self, realized_cap: Price) {
        self.realized_cap -= realized_cap;
    }
}

impl AddAssign for CapitalizationState {
    fn add_assign(&mut self, rhs: Self) {
        self.realized_cap += rhs.realized_cap;
    }
}
