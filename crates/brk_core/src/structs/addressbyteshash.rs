use std::hash::Hasher;

use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Error;

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
        let mut hasher = rapidhash::RapidHasher::default();
        hasher.write(address_bytes.as_slice());
        let mut slice = hasher.finish().to_le_bytes();
        slice[0] = slice[0].wrapping_add(outputtype as u8);
        Self(slice)
    }
}

impl From<[u8; 8]> for AddressBytesHash {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}

impl TryFrom<ByteView> for AddressBytesHash {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
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
