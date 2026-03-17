use byteview::ByteView;
use derive_more::Deref;
use vecdb::Bytes;

use super::AddrBytes;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Bytes, Hash)]
pub struct AddrHash(u64);

impl From<&AddrBytes> for AddrHash {
    #[inline]
    fn from(addr_bytes: &AddrBytes) -> Self {
        Self(addr_bytes.hash())
    }
}

impl From<ByteView> for AddrHash {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self(u64::from_be_bytes((&*value).try_into().unwrap()))
    }
}
impl From<AddrHash> for ByteView {
    #[inline]
    fn from(value: AddrHash) -> Self {
        Self::from(&value)
    }
}
impl From<&AddrHash> for ByteView {
    #[inline]
    fn from(value: &AddrHash) -> Self {
        Self::new(&value.0.to_be_bytes())
    }
}
