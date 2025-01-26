use std::hash::Hasher;

use biter::bitcoin::{BlockHash, Txid};
use derive_deref::Deref;
use snkrj::{direct_repr, Storable, UnsizedStorable};

use super::{Addressbytes, Addresstype, SliceExtended};

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressbytesPrefix([u8; 8]);
direct_repr!(AddressbytesPrefix);
impl From<(&Addressbytes, Addresstype)> for AddressbytesPrefix {
    fn from((addressbytes, addresstype): (&Addressbytes, Addresstype)) -> Self {
        let mut hasher = rapidhash::RapidHasher::default();
        hasher.write(addressbytes.as_slice());
        let mut slice = hasher.finish().to_le_bytes();
        slice[0] = slice[0].wrapping_add(addresstype as u8);
        Self(slice)
    }
}
impl From<[u8; 8]> for AddressbytesPrefix {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHashPrefix([u8; 8]);
direct_repr!(BlockHashPrefix);
impl TryFrom<&BlockHash> for BlockHashPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &BlockHash) -> Result<Self, Self::Error> {
        Ok(Self((&value[..]).read_8x_u8()?))
    }
}

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxidPrefix([u8; 8]);
direct_repr!(TxidPrefix);
impl TryFrom<&Txid> for TxidPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &Txid) -> Result<Self, Self::Error> {
        Ok(Self((&value[..]).read_8x_u8()?))
    }
}
