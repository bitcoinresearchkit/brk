use byteview::ByteView;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, KnownLayout};

use crate::{CheckedSub, Dollars, EmptyAddressData, Error, Result, Sats};

#[derive(Debug, Default, Clone, FromBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct AddressData {
    pub sent: Sats,
    pub received: Sats,
    pub realized_cap: Dollars,
    pub outputs_len: u32,
}

impl AddressData {
    pub fn amount(&self) -> Sats {
        (u64::from(self.received) - u64::from(self.sent)).into()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        if self.amount() == Sats::ZERO {
            if self.outputs_len != 0 {
                unreachable!();
            }

            true
        } else {
            false
        }
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
            return Err(Error::String("Previous_amount smaller than sent amount"));
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

impl From<EmptyAddressData> for AddressData {
    fn from(value: EmptyAddressData) -> Self {
        Self::from(&value)
    }
}
impl From<&EmptyAddressData> for AddressData {
    fn from(value: &EmptyAddressData) -> Self {
        Self {
            sent: value.transfered,
            received: value.transfered,
            realized_cap: Dollars::ZERO,
            outputs_len: 0,
        }
    }
}

impl From<ByteView> for AddressData {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}
impl From<AddressData> for ByteView {
    fn from(value: AddressData) -> Self {
        Self::from(&value)
    }
}
impl From<&AddressData> for ByteView {
    fn from(value: &AddressData) -> Self {
        Self::new(
            &[
                value.sent.as_bytes(),
                value.received.as_bytes(),
                value.realized_cap.as_bytes(),
                value.outputs_len.as_bytes(),
            ]
            .concat(),
        )
    }
}
