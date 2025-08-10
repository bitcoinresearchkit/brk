use brk_error::{Error, Result};
use serde::Serialize;
use vecdb::CheckedSub;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Bitcoin, Dollars, EmptyAddressData, Sats};

#[derive(Debug, Default, Clone, Serialize, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct LoadedAddressData {
    pub sent: Sats,
    pub received: Sats,
    pub realized_cap: Dollars,
    pub outputs_len: u32,
    padding: u32,
}

impl LoadedAddressData {
    pub fn amount(&self) -> Sats {
        (u64::from(self.received) - u64::from(self.sent)).into()
    }

    pub fn realized_price(&self) -> Dollars {
        (self.realized_cap / Bitcoin::from(self.amount())).round_to_4_digits()
    }

    #[inline]
    pub fn has_0_sats(&self) -> bool {
        self.amount() == Sats::ZERO
    }

    #[inline]
    pub fn has_0_utxos(&self) -> bool {
        self.outputs_len == 0
    }

    pub fn receive(&mut self, amount: Sats, price: Option<Dollars>) {
        self.received += amount;
        self.outputs_len += 1;
        if let Some(price) = price {
            self.realized_cap += price * amount;
        }
    }

    pub fn send(&mut self, amount: Sats, previous_price: Option<Dollars>) -> Result<()> {
        if self.amount() < amount {
            return Err(Error::Str("Previous_amount smaller than sent amount"));
        }
        self.sent += amount;
        self.outputs_len -= 1;
        if let Some(previous_price) = previous_price {
            self.realized_cap = self
                .realized_cap
                .checked_sub(previous_price * amount)
                .unwrap();
        }
        Ok(())
    }
}

impl From<EmptyAddressData> for LoadedAddressData {
    fn from(value: EmptyAddressData) -> Self {
        Self::from(&value)
    }
}
impl From<&EmptyAddressData> for LoadedAddressData {
    fn from(value: &EmptyAddressData) -> Self {
        Self {
            sent: value.transfered,
            received: value.transfered,
            realized_cap: Dollars::ZERO,
            outputs_len: 0,
            padding: 0,
        }
    }
}
