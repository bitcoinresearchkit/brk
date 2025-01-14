use biter::bitcoin::{BlockHash, Txid};
use derive_deref::Deref;
use snkrj::{direct_repr, Storable, UnsizedStorable};

use super::{Addressbytes, SliceExtended};

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prefix([u8; 8]);
direct_repr!(Prefix);
impl TryFrom<&[u8]> for Prefix {
    type Error = color_eyre::Report;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.read_8xU8()?))
    }
}

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressbytesPrefix(Prefix);
direct_repr!(AddressbytesPrefix);
impl TryFrom<&Addressbytes> for AddressbytesPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &Addressbytes) -> Result<Self, Self::Error> {
        Ok(Self(Prefix::try_from(match value {
            Addressbytes::P2PK65(bytes) => &bytes[..],
            Addressbytes::P2PK33(bytes) => &bytes[..],
            Addressbytes::P2PKH(bytes) => &bytes[..],
            Addressbytes::P2SH(bytes) => &bytes[..],
            Addressbytes::P2WPKH(bytes) => &bytes[..],
            Addressbytes::P2WSH(bytes) => &bytes[..],
            Addressbytes::P2TR(bytes) => &bytes[..],
        })?))
    }
}

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHashPrefix(Prefix);
direct_repr!(BlockHashPrefix);
impl TryFrom<&BlockHash> for BlockHashPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &BlockHash) -> Result<Self, Self::Error> {
        Ok(Self(Prefix::try_from(&value[..])?))
    }
}

#[derive(Debug, Deref, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxidPrefix(Prefix);
direct_repr!(TxidPrefix);
impl TryFrom<&Txid> for TxidPrefix {
    type Error = color_eyre::Report;
    fn try_from(value: &Txid) -> Result<Self, Self::Error> {
        Ok(Self(Prefix::try_from(&value[..])?))
    }
}
