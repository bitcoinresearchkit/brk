use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::AddressBytes;

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
    Hash,
)]
pub struct AddressBytesHash(u64);

impl From<&AddressBytes> for AddressBytesHash {
    #[inline]
    fn from(address_bytes: &AddressBytes) -> Self {
        Self(address_bytes.hash())
    }
}

impl From<ByteView> for AddressBytesHash {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}

impl From<AddressBytesHash> for ByteView {
    #[inline]
    fn from(value: AddressBytesHash) -> Self {
        Self::from(&value)
    }
}

impl From<&AddressBytesHash> for ByteView {
    #[inline]
    fn from(value: &AddressBytesHash) -> Self {
        Self::new(value.as_bytes())
    }
}
