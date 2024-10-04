use std::{cmp::Ordering, ops::Add};

use crate::structs::{Amount, Price};

#[derive(Debug, Default)]
pub struct UnrealizedState {
    supply_in_profit: Amount,
    unrealized_profit: Price,
    unrealized_loss: Price,
}

impl UnrealizedState {
    pub fn supply_in_profit(&self) -> Amount {
        self.supply_in_profit
    }

    pub fn unrealized_profit(&self) -> Price {
        self.unrealized_profit
    }

    pub fn unrealized_loss(&self) -> Price {
        self.unrealized_loss
    }

    #[inline]
    pub fn iterate(&mut self, price_then: Price, price_now: Price, amount: Amount) {
        match price_then.cmp(&price_now) {
            Ordering::Less => {
                self.unrealized_profit += (price_now - price_then) * amount;
                self.supply_in_profit += amount;
            }
            Ordering::Greater => {
                self.unrealized_loss += (price_then - price_now) * amount;
            }
            Ordering::Equal => {}
        }
    }
}

impl Add<UnrealizedState> for UnrealizedState {
    type Output = UnrealizedState;

    fn add(self, other: UnrealizedState) -> UnrealizedState {
        UnrealizedState {
            supply_in_profit: self.supply_in_profit + other.supply_in_profit,
            unrealized_profit: self.unrealized_profit + other.unrealized_profit,
            unrealized_loss: self.unrealized_loss + other.unrealized_loss,
        }
    }
}
