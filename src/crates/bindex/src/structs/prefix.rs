use biter::bitcoin::{BlockHash, Txid};
use derive_deref::Deref;
use snkrj::{direct_repr, Storable, UnsizedStorable};

use super::{Addressbytes, Addresstype, SliceExtended};

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressbytesPrefix([u8; 8]);
direct_repr!(AddressbytesPrefix);
impl From<(&Addressbytes, Addresstype)> for AddressbytesPrefix {
    fn from((addressbytes, addresstype): (&Addressbytes, Addresstype)) -> Self {
        let shorten = |slice: &[u8]| {
            let len = slice.len();
            let mut buf: [u8; 8] = [0; 8];
            // Using both ends for collision reasons despite rehashing the addresses
            (0..4_usize).for_each(|i| {
                buf[i] = slice[i];
                buf[4 + i] = slice[len - 4 + i];
            });
            buf[4] = addresstype as u8;
            // Put in the middle and not at the start because either the first or the last byte can be used to split and if the type is used it wouldn't have the 0..256 range
            // End result:
            // [ i=0, i=1, i=2, i=3, type as u8, i=len-3, i=len-2, i=len-1 ]
            buf
        };
        Self(shorten(
            bitcoin_hashes::hash160::Hash::hash(addressbytes.as_slice()).as_byte_array(),
        ))
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
