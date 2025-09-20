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
    pub utxos: u32,
    #[serde(skip)]
    padding: u32,
}

impl LoadedAddressData {
    pub fn amount(&self) -> Sats {
        (u64::from(self.received) - u64::from(self.sent)).into()
    }

    pub fn realized_price(&self) -> Dollars {
        let p = (self.realized_cap / Bitcoin::from(self.amount())).round_to(4);
        if p.is_negative() {
            dbg!((
                self.realized_cap,
                self.amount(),
                Bitcoin::from(self.amount()),
                p
            ));
            panic!("");
        }
        p
    }

    #[inline]
    pub fn has_0_sats(&self) -> bool {
        self.amount() == Sats::ZERO
    }

    #[inline]
    pub fn has_0_utxos(&self) -> bool {
        self.utxos == 0
    }

    pub fn receive(&mut self, amount: Sats, price: Option<Dollars>) {
        self.received += amount;
        self.utxos += 1;
        if let Some(price) = price {
            let added = price * amount;
            self.realized_cap += added;
            if added.is_negative() || self.realized_cap.is_negative() {
                dbg!((self.realized_cap, price, amount, added));
                panic!();
            }
        }
    }

    pub fn send(&mut self, amount: Sats, previous_price: Option<Dollars>) -> Result<()> {
        if self.amount() < amount {
            return Err(Error::Str("Previous_amount smaller than sent amount"));
        }
        self.sent += amount;
        self.utxos -= 1;
        if let Some(previous_price) = previous_price {
            let subtracted = previous_price * amount;
            let realized_cap = self.realized_cap.checked_sub(subtracted).unwrap();
            if self.realized_cap.is_negative() || realized_cap.is_negative() {
                dbg!((
                    self,
                    realized_cap,
                    previous_price,
                    amount,
                    previous_price * amount,
                    subtracted
                ));
                panic!();
            }
            self.realized_cap = realized_cap;
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
            utxos: 0,
            padding: 0,
        }
    }
}

impl std::fmt::Display for LoadedAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sent: {}, received: {}, realized_cap: {}, utxos: {}",
            self.sent, self.received, self.realized_cap, self.utxos
        )
    }
}
