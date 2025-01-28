use std::hash::Hasher;

use biter::bitcoin::{BlockHash, Txid};
use derive_deref::Deref;
// use snkrj::{direct_repr, Storable, UnsizedStorable};
use unsafe_slice_serde::UnsafeSliceSerde;

use super::{Addressbytes, Addresstype, SliceExtended};

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressHash([u8; 8]);
// direct_repr!(AddressHash);
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
impl TryFrom<fjall::Slice> for AddressHash {
    type Error = color_eyre::Report;
    fn try_from(value: fjall::Slice) -> Result<Self, Self::Error> {
        Ok(*Self::unsafe_try_from_slice(&value)?)
    }
}
impl From<&AddressHash> for fjall::Slice {
    fn from(value: &AddressHash) -> Self {
        Self::new(value.unsafe_as_slice())
    }
}
impl From<AddressHash> for fjall::Slice {
    fn from(value: AddressHash) -> Self {
        Self::from(&value)
    }
}

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHashPrefix([u8; 8]);
// direct_repr!(BlockHashPrefix);
impl TryFrom<&BlockHash> for BlockHashPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &BlockHash) -> Result<Self, Self::Error> {
        Ok(Self((&value[..]).read_8x_u8()?))
    }
}
impl TryFrom<fjall::Slice> for BlockHashPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: fjall::Slice) -> Result<Self, Self::Error> {
        Ok(*Self::unsafe_try_from_slice(&value)?)
    }
}
impl From<&BlockHashPrefix> for fjall::Slice {
    fn from(value: &BlockHashPrefix) -> Self {
        Self::new(value.unsafe_as_slice())
    }
}
impl From<BlockHashPrefix> for fjall::Slice {
    fn from(value: BlockHashPrefix) -> Self {
        Self::from(&value)
    }
}

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxidPrefix([u8; 8]);
// direct_repr!(TxidPrefix);
impl TryFrom<&Txid> for TxidPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &Txid) -> Result<Self, Self::Error> {
        Ok(Self((&value[..]).read_8x_u8()?))
    }
}
impl TryFrom<fjall::Slice> for TxidPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: fjall::Slice) -> Result<Self, Self::Error> {
        Ok(*Self::unsafe_try_from_slice(&value)?)
    }
}
impl From<&TxidPrefix> for fjall::Slice {
    fn from(value: &TxidPrefix) -> Self {
        Self::new(value.unsafe_as_slice())
    }
}
impl From<TxidPrefix> for fjall::Slice {
    fn from(value: TxidPrefix) -> Self {
        Self::from(&value)
    }
}
