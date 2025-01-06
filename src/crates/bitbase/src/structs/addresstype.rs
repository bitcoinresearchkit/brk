use biter::bitcoin::ScriptBuf;
use color_eyre::eyre::eyre;
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Addresstype {
    P2PK,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
    Multisig = 251,
    PushOnly = 252,
    OpReturn = 253,
    Empty = 254,
    Unknown = 255,
}

impl From<&ScriptBuf> for Addresstype {
    fn from(script: &ScriptBuf) -> Self {
        if script.is_p2pk() {
            Self::P2PK
        } else if script.is_p2pkh() {
            Self::P2PKH
        } else if script.is_p2sh() {
            Self::P2SH
        } else if script.is_p2wpkh() {
            Self::P2WPKH
        } else if script.is_p2wsh() {
            Self::P2WSH
        } else if script.is_p2tr() {
            Self::P2TR
        } else if script.is_empty() {
            Self::Empty
        } else if script.is_op_return() {
            Self::OpReturn
        } else if script.is_push_only() {
            Self::PushOnly
        } else if script.is_multisig() {
            Self::Multisig
        } else {
            Self::Unknown
        }
    }
}

impl TryFrom<Slice> for Addresstype {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        match value.read_u8() {
            x if x == Addresstype::P2PK as u8 => Ok(Addresstype::P2PK),
            x if x == Addresstype::P2PKH as u8 => Ok(Addresstype::P2PKH),
            _ => Err(eyre!("Unknown type")),
        }
    }
}
impl From<Addresstype> for Slice {
    fn from(addresstype: Addresstype) -> Self {
        (addresstype as u8).to_be_bytes().into()
    }
}
