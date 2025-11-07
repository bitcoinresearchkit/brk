use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_8bytes;

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
        Self(u64::from_be_bytes(copy_first_8bytes(&value).unwrap()))
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
        Self::new(&value.0.to_be_bytes())
    }
}
