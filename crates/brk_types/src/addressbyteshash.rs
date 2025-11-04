use std::{cmp::Ordering, mem};

use byteview::ByteView;
use derive_deref::Deref;
use redb::{Key, TypeName, Value};
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

impl Value for AddressBytesHash {
    type SelfType<'a> = AddressBytesHash;
    type AsBytes<'a>
        = [u8; mem::size_of::<u64>()]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(mem::size_of::<u64>())
    }

    fn from_bytes<'a>(data: &'a [u8]) -> AddressBytesHash
    where
        Self: 'a,
    {
        AddressBytesHash(u64::from_le_bytes(data.try_into().unwrap()))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> [u8; mem::size_of::<u64>()]
    where
        Self: 'a,
        Self: 'b,
    {
        value.0.to_le_bytes()
    }

    fn type_name() -> TypeName {
        TypeName::new("AddressBytesHash")
    }
}

impl Key for AddressBytesHash {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
    }
}
