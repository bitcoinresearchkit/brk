use byteview::ByteView;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Dollars, Sats};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C, packed)]
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
        Self::new(value.as_bytes())
    }
}
