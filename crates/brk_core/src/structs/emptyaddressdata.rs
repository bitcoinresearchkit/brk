use byteview::ByteView;
use serde::Serialize;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{LoadedAddressData, Sats};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
pub struct EmptyAddressData {
    pub transfered: Sats,
}

impl From<LoadedAddressData> for EmptyAddressData {
    fn from(value: LoadedAddressData) -> Self {
        Self::from(&value)
    }
}

impl From<&LoadedAddressData> for EmptyAddressData {
    fn from(value: &LoadedAddressData) -> Self {
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
