use std::ops::{Add, AddAssign, SubAssign};

use brk_types::{Dollars, SupplyState, Timestamp};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BlockState {
    #[serde(flatten)]
    pub supply: SupplyState,
    #[serde(skip)]
    pub price: Option<Dollars>,
    #[serde(skip)]
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
