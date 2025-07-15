use byteview::ByteView;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{AddressData, Sats};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct EmptyAddressData {
    pub transfered: Sats,
}

impl From<AddressData> for EmptyAddressData {
    fn from(value: AddressData) -> Self {
        Self::from(&value)
    }
}

impl From<&AddressData> for EmptyAddressData {
    fn from(value: &AddressData) -> Self {
        if value.sent != value.received {
            dbg!(&value);
            panic!("Trying to convert not empty wallet to empty !");
        }
        Self {
            transfered: value.sent,
        }
    }
}

impl From<ByteView> for EmptyAddressData {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}
impl From<EmptyAddressData> for ByteView {
    fn from(value: EmptyAddressData) -> Self {
        Self::from(&value)
    }
}
impl From<&EmptyAddressData> for ByteView {
    fn from(value: &EmptyAddressData) -> Self {
        Self::new(value.as_bytes())
    }
}
