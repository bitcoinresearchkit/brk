use bitcoin::{ScriptBuf, opcodes::all::OP_PUSHBYTES_2};
use serde::Serialize;
use zerocopy_derive::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
#[repr(u8)]
pub enum OutputType {
    P2PK65,
    P2PK33,
    P2PKH,
    P2MS,
    P2SH,
    OpReturn,
    P2WPKH,
    P2WSH,
    P2TR,
    P2A,
    Empty = 254,
    Unknown = 255,
}

impl OutputType {
    pub fn is_spendable(&self) -> bool {
        match self {
            Self::P2PK65 => true,
            Self::P2PK33 => true,
            Self::P2PKH => true,
            Self::P2MS => true,
            Self::P2SH => true,
            Self::OpReturn => false,
            Self::P2WPKH => true,
            Self::P2WSH => true,
            Self::P2TR => true,
            Self::P2A => true,
            Self::Empty => true,
            Self::Unknown => true,
        }
    }

    pub fn is_address(&self) -> bool {
        match self {
            Self::P2PK65 => true,
            Self::P2PK33 => true,
            Self::P2PKH => true,
            Self::P2MS => false,
            Self::P2SH => true,
            Self::OpReturn => false,
            Self::P2WPKH => true,
            Self::P2WSH => true,
            Self::P2TR => true,
            Self::P2A => true,
            Self::Empty => false,
            Self::Unknown => false,
        }
    }

    pub fn is_unspendable(&self) -> bool {
        !self.is_spendable()
    }

    pub fn as_vec() -> Vec<Self> {
        vec![
            Self::P2PK65,
            Self::P2PK33,
            Self::P2PKH,
            Self::P2MS,
            Self::P2SH,
            Self::OpReturn,
            Self::P2WPKH,
            Self::P2WSH,
            Self::P2TR,
            Self::P2A,
            Self::Empty,
            Self::Unknown,
        ]
    }
}

impl From<&ScriptBuf> for OutputType {
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
        } else if script.is_multisig() {
            Self::P2MS
        } else if script.is_p2sh() {
            Self::P2SH
        } else if script.is_op_return() {
            Self::OpReturn
        } else if script.is_p2wpkh() {
            Self::P2WPKH
        } else if script.is_p2wsh() {
            Self::P2WSH
        } else if script.is_p2tr() {
            Self::P2TR
        } else if script.witness_version() == Some(bitcoin::WitnessVersion::V1)
            && script.len() == 4
            && script.as_bytes()[1] == OP_PUSHBYTES_2.to_u8()
            && script.as_bytes()[2..4] == [78, 115]
        {
            Self::P2A
        } else if script.is_empty() {
            Self::Empty
        } else {
            Self::Unknown
        }
    }
}
