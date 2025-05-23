use std::ops::{Add, AddAssign, SubAssign};

use brk_core::{Dollars, Sats, Timestamp};

use super::{OutputsByType, SupplyState};

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

pub struct ReceivedBlockStateData<'a> {
    pub received: &'a OutputsByType<(SupplyState, Vec<Sats>)>,
    pub timestamp: Timestamp,
    pub price: Option<Dollars>,
}
impl<'a> From<ReceivedBlockStateData<'a>> for BlockState {
    fn from(
        ReceivedBlockStateData {
            received,
            timestamp,
            price,
        }: ReceivedBlockStateData<'a>,
    ) -> Self {
        let mut block_state = BlockState {
            supply: SupplyState::default(),
            price,
            timestamp,
        };
        received
            .spendable
            .as_vec()
            .into_iter()
            .for_each(|spendable_block_state| {
                block_state.supply += &spendable_block_state.0;
            });
        block_state.supply.utxos += received.unspendable.empty.0.utxos;
        block_state
    }
}
