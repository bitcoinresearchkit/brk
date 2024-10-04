use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::{Amount, Height, Price, Timestamp};

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Allocative)]
pub struct BlockData {
    pub height: Height,
    pub price: Price,
    pub timestamp: Timestamp,
    pub amount: Amount,
    pub utxos: u32,
}

impl BlockData {
    pub fn new(height: Height, price: Price, timestamp: Timestamp) -> Self {
        Self {
            height,
            price,
            timestamp,
            amount: Amount::ZERO,
            utxos: 0,
        }
    }

    pub fn send(&mut self, amount: Amount) {
        self.utxos -= 1;

        if self.amount < amount {
            unreachable!();
        }

        self.amount -= amount;
    }

    pub fn receive(&mut self, amount: Amount) {
        self.utxos += 1;

        self.amount += amount;
    }
}
