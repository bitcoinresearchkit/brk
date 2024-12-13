use allocative::Allocative;
use color_eyre::eyre::eyre;
use sanakirja::{direct_repr, Storable, UnsizedStorable};

use super::{AddressType, Amount, EmptyAddressData, LiquidityClassification, Price};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Allocative)]
pub struct AddressData {
    pub address_type: AddressType,
    pub amount: Amount,
    pub sent: Amount,
    pub received: Amount,
    pub realized_cap: Price,
    pub outputs_len: u32,
}
direct_repr!(AddressData);

impl AddressData {
    pub fn new(address_type: AddressType) -> Self {
        Self {
            address_type,
            amount: Amount::ZERO,
            sent: Amount::ZERO,
            received: Amount::ZERO,
            realized_cap: Price::ZERO,
            outputs_len: 0,
        }
    }

    pub fn receive(&mut self, amount: Amount, price: Price) {
        let previous_amount = self.amount;

        let new_amount = previous_amount + amount;

        self.amount = new_amount;

        self.received += amount;

        self.outputs_len += 1;

        let received_value = price * amount;

        self.realized_cap += received_value;
    }

    pub fn send(&mut self, amount: Amount, previous_price: Price) -> color_eyre::Result<()> {
        let previous_amount = self.amount;

        if previous_amount < amount {
            return Err(eyre!("previous_amount smaller than sent amount"));
        }

        let new_amount = previous_amount - amount;

        self.amount = new_amount;

        self.sent += amount;

        self.outputs_len -= 1;

        let previous_sent_dollar_value = previous_price * amount;

        self.realized_cap -= previous_sent_dollar_value;

        Ok(())
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        if self.amount == Amount::ZERO {
            if self.outputs_len != 0 {
                unreachable!();
            }

            true
        } else {
            false
        }
    }

    pub fn from_empty(empty: &EmptyAddressData) -> Self {
        let address_type = empty.address_type();
        let transfered = empty.transfered();

        Self {
            address_type,
            amount: Amount::ZERO,
            sent: transfered,
            received: transfered,
            realized_cap: Price::ZERO,
            outputs_len: 0,
        }
    }

    pub fn compute_liquidity_classification(&self) -> LiquidityClassification {
        LiquidityClassification::new(self.sent, self.received)
    }
}
