use byteview::ByteView;
use zerocopy::{FromBytes, IntoBytes};

use crate::{Bitcoin, CheckedSub, Dollars, EmptyAddressData, Error, Result, Sats};

#[derive(Debug, Default, Clone)]
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

    pub fn realized_price(&self) -> Dollars {
        (self.realized_cap / Bitcoin::from(self.amount())).round_nearest_cent()
    }

    #[inline(always)]
    pub fn has_0_sats(&self) -> bool {
        self.amount() == Sats::ZERO
    }

    #[inline(always)]
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
        Self {
            // MUST be same order as impl From<&AddressData> for ByteView
            sent: Sats::read_from_bytes(&value[..8]).unwrap(),
            received: Sats::read_from_bytes(&value[8..16]).unwrap(),
            realized_cap: Dollars::read_from_bytes(&value[16..24]).unwrap(),
            outputs_len: u32::read_from_bytes(&value[24..]).unwrap(),
        }
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
