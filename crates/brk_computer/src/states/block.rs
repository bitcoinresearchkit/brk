use std::ops::{Add, AddAssign, SubAssign};

use brk_core::{Dollars, Timestamp};

use super::SupplyState;

#[derive(Debug, Clone)]
pub struct BlockState {
    pub supply: SupplyState,
    pub price: Option<Dollars>,
    pub timestamp: Timestamp,
}

impl Add<BlockState> for BlockState {
    type Output = Self;
    fn add(mut self, rhs: BlockState) -> Self::Output {
        self.supply += &rhs.supply;
        self
    }
}

impl AddAssign<&BlockState> for BlockState {
    fn add_assign(&mut self, rhs: &Self) {
        self.supply += &rhs.supply;
    }
}

impl SubAssign<&BlockState> for BlockState {
    fn sub_assign(&mut self, rhs: &Self) {
        self.supply -= &rhs.supply;
    }
}
