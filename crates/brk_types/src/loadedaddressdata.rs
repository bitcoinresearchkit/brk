use brk_error::{Error, Result};
use serde::Serialize;
use vecdb::CheckedSub;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Bitcoin, Dollars, EmptyAddressData, Sats};

/// Data for a loaded (non-empty) address with current balance
#[derive(Debug, Default, Clone, Serialize, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct LoadedAddressData {
    /// Total transaction count
    pub tx_count: u32,
    /// Number of transaction outputs funded to this address
    pub funded_txo_count: u32,
    /// Number of transaction outputs spent by this address
    pub spent_txo_count: u32,
    #[serde(skip)]
    padding: u32,
    /// Satoshis received by this address
    pub received: Sats,
    /// Satoshis sent by this address
    pub sent: Sats,
    /// The realized capitalization of this address
    pub realized_cap: Dollars,
}

impl LoadedAddressData {
    pub fn balance(&self) -> Sats {
        (u64::from(self.received) - u64::from(self.sent)).into()
    }

    pub fn realized_price(&self) -> Dollars {
        let p = (self.realized_cap / Bitcoin::from(self.balance())).round_to(4);
        if p.is_negative() {
            dbg!((
                self.realized_cap,
                self.balance(),
                Bitcoin::from(self.balance()),
                p
            ));
            panic!("");
        }
        p
    }

    #[inline]
    pub fn has_0_sats(&self) -> bool {
        self.balance() == Sats::ZERO
    }

    #[inline]
    pub fn has_0_utxos(&self) -> bool {
        self.funded_txo_count == self.spent_txo_count
    }

    pub fn receive(&mut self, amount: Sats, price: Option<Dollars>) {
        self.received += amount;
        self.funded_txo_count += 1;
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
        if self.balance() < amount {
            return Err(Error::Str("Previous_amount smaller than sent amount"));
        }
        self.sent += amount;
        self.spent_txo_count += 1;
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
            tx_count: value.tx_count,
            funded_txo_count: value.funded_txo_count,
            spent_txo_count: value.funded_txo_count,
            padding: 0,
            received: value.transfered,
            sent: value.transfered,
            realized_cap: Dollars::ZERO,
        }
    }
}

impl std::fmt::Display for LoadedAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx_count: {}, funded_txo_count: {}, spent_txo_count: {}, received: {}, sent: {}, realized_cap: {}",
            self.tx_count,
            self.funded_txo_count,
            self.spent_txo_count,
            self.received,
            self.sent,
            self.realized_cap,
        )
    }
}
