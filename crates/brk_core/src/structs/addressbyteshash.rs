use std::hash::Hasher;

use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{AddressBytes, OutputType};

#[derive(
    Debug,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
)]
pub struct AddressBytesHash([u8; 8]);

impl From<(&AddressBytes, OutputType)> for AddressBytesHash {
    fn from((address_bytes, outputtype): (&AddressBytes, OutputType)) -> Self {
        let mut slice = rapidhash::v3::rapidhash_v3(address_bytes.as_slice()).to_le_bytes();
        slice[0] = slice[0].wrapping_add(outputtype as u8);
        Self(slice)
    }
}

impl From<[u8; 8]> for AddressBytesHash {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}

impl From<ByteView> for AddressBytesHash {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}

impl From<&AddressBytesHash> for ByteView {
    fn from(value: &AddressBytesHash) -> Self {
        Self::new(value.as_bytes())
    }
}

impl From<AddressBytesHash> for ByteView {
    fn from(value: AddressBytesHash) -> Self {
        Self::from(&value)
    }
}
