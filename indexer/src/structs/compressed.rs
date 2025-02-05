use std::hash::Hasher;

use derive_deref::Deref;
use fjall::Slice;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Addressbytes, Addresstype, BlockHash, Txid};

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct AddressHash([u8; 8]);
impl From<(&Addressbytes, Addresstype)> for AddressHash {
    fn from((addressbytes, addresstype): (&Addressbytes, Addresstype)) -> Self {
        let mut hasher = rapidhash::RapidHasher::default();
        hasher.write(addressbytes.as_slice());
        let mut slice = hasher.finish().to_le_bytes();
        slice[0] = slice[0].wrapping_add(addresstype as u8);
        Self(slice)
    }
}
impl From<[u8; 8]> for AddressHash {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}
impl TryFrom<Slice> for AddressHash {
    type Error = storable_vec::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<&AddressHash> for Slice {
    fn from(value: &AddressHash) -> Self {
        Self::new(value.as_bytes())
    }
}
impl From<AddressHash> for Slice {
    fn from(value: AddressHash) -> Self {
        Self::from(&value)
    }
}

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct BlockHashPrefix([u8; 8]);
impl TryFrom<&BlockHash> for BlockHashPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &BlockHash) -> Result<Self, Self::Error> {
        Ok(Self(copy_first_8bytes(&value[..])))
    }
}
impl TryFrom<Slice> for BlockHashPrefix {
    type Error = storable_vec::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<&BlockHashPrefix> for Slice {
    fn from(value: &BlockHashPrefix) -> Self {
        Self::new(value.as_bytes())
    }
}
impl From<BlockHashPrefix> for Slice {
    fn from(value: BlockHashPrefix) -> Self {
        Self::from(&value)
    }
}

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct TxidPrefix([u8; 8]);
impl TryFrom<&Txid> for TxidPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &Txid) -> Result<Self, Self::Error> {
        Ok(Self(copy_first_8bytes(&value[..])))
    }
}
impl TryFrom<Slice> for TxidPrefix {
    type Error = storable_vec::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<&TxidPrefix> for Slice {
    fn from(value: &TxidPrefix) -> Self {
        Self::new(value.as_bytes())
    }
}
impl From<TxidPrefix> for Slice {
    fn from(value: TxidPrefix) -> Self {
        Self::from(&value)
    }
}

fn copy_first_8bytes(slice: &[u8]) -> [u8; 8] {
    let mut buf: [u8; 8] = [0; 8];
    let buf_len = buf.len();
    if slice.len() < buf_len {
        panic!("bad len");
    }
    slice.iter().take(buf_len).enumerate().for_each(|(i, r)| {
        buf[i] = *r;
    });
    buf
}
