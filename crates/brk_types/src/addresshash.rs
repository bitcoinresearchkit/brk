use byteview::ByteView;
use derive_more::Deref;
use vecdb::Bytes;

use super::AddressBytes;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Bytes, Hash)]
pub struct AddressHash(u64);

impl From<&AddressBytes> for AddressHash {
    #[inline]
    fn from(address_bytes: &AddressBytes) -> Self {
        Self(address_bytes.hash())
    }
}

impl From<ByteView> for AddressHash {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self(u64::from_be_bytes((&*value).try_into().unwrap()))
    }
}
impl From<AddressHash> for ByteView {
    #[inline]
    fn from(value: AddressHash) -> Self {
        Self::from(&value)
    }
}
impl From<&AddressHash> for ByteView {
    #[inline]
    fn from(value: &AddressHash) -> Self {
        Self::new(&value.0.to_be_bytes())
    }
}
