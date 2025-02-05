use iterator::bitcoin::ScriptBuf;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(u8)]
pub enum Addresstype {
    P2PK65,
    P2PK33,
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
            let bytes = script.as_bytes();

            match bytes.len() {
                67 => Self::P2PK65,
                35 => Self::P2PK33,
                _ => {
                    dbg!(bytes);
                    unreachable!()
                }
            }
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
