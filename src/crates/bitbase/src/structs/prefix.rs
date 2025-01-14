use biter::bitcoin::{BlockHash, Txid};
use derive_deref::Deref;
use fjall::Slice;

use super::Addressbytes;

#[derive(Debug, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prefix(Slice);
impl From<&[u8]> for Prefix {
    fn from(value: &[u8]) -> Self {
        Self(Slice::from(&value[..8]))
    }
}
// pub struct Prefix([u8; 8]);
// impl From<&[u8]> for Prefix {
//     fn from(value: &[u8]) -> Self {
//         let mut buf: [u8; 8] = [0; 8];
//         value.iter().take(8).enumerate().for_each(|(i, v)| {
//             buf[i] = *v;
//         });
//         Self(buf)
//     }
// }

#[derive(Debug, Deref)]
pub struct AddressbytesPrefix(Prefix);
impl From<&Addressbytes> for AddressbytesPrefix {
    fn from(value: &Addressbytes) -> Self {
        Self(Prefix::from(match value {
            Addressbytes::P2PK65(bytes) => &bytes[..],
            Addressbytes::P2PK33(bytes) => &bytes[..],
            Addressbytes::P2PKH(bytes) => &bytes[..],
            Addressbytes::P2SH(bytes) => &bytes[..],
            Addressbytes::P2WPKH(bytes) => &bytes[..],
            Addressbytes::P2WSH(bytes) => &bytes[..],
            Addressbytes::P2TR(bytes) => &bytes[..],
        }))
    }
}

#[derive(Debug, Deref)]
pub struct BlockHashPrefix(Prefix);
impl From<&BlockHash> for BlockHashPrefix {
    fn from(value: &BlockHash) -> Self {
        Self(Prefix::from(&value[..]))
    }
}

#[derive(Debug, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxidPrefix(Prefix);
impl From<&Txid> for TxidPrefix {
    fn from(value: &Txid) -> Self {
        Self(Prefix::from(&value[..]))
    }
}
