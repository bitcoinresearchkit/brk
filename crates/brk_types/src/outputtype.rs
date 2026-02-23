use bitcoin::{AddressType, ScriptBuf, opcodes::all::OP_PUSHBYTES_2};
use brk_error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::Display;
use vecdb::{Bytes, Formattable, Pco, TransparentPco};

use crate::AddressBytes;

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Hash,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(u8)]
/// Type (P2PKH, P2WPKH, P2SH, P2TR, etc.)
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
    Empty,
    Unknown,
}

impl OutputType {
    pub const ADDRESS_TYPES: [Self; 8] = [
        Self::P2PK65,
        Self::P2PK33,
        Self::P2PKH,
        Self::P2SH,
        Self::P2WPKH,
        Self::P2WSH,
        Self::P2TR,
        Self::P2A,
    ];

    fn is_valid(value: u8) -> bool {
        value <= Self::Unknown as u8
    }

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

    pub fn is_not_address(&self) -> bool {
        !self.is_address()
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
    #[inline]
    fn from(script: &ScriptBuf) -> Self {
        if script.is_p2pkh() {
            Self::P2PKH
        } else if script.is_p2wpkh() {
            Self::P2WPKH
        } else if script.is_p2wsh() {
            Self::P2WSH
        } else if script.is_p2tr() {
            Self::P2TR
        } else if script.is_p2sh() {
            Self::P2SH
        } else if script.is_op_return() {
            Self::OpReturn
        } else if script.is_p2pk() {
            let bytes = script.as_bytes();

            match bytes.len() {
                67 => Self::P2PK65,
                35 => Self::P2PK33,
                _ => {
                    dbg!(bytes);
                    unreachable!()
                }
            }
        } else if script.is_multisig() {
            Self::P2MS
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

impl From<AddressType> for OutputType {
    #[inline]
    fn from(value: AddressType) -> Self {
        match value {
            AddressType::P2a => Self::P2A,
            AddressType::P2pkh => Self::P2PKH,
            AddressType::P2sh => Self::P2SH,
            AddressType::P2tr => Self::P2TR,
            AddressType::P2wpkh => Self::P2WPKH,
            AddressType::P2wsh => Self::P2WSH,
            _ => unreachable!(),
        }
    }
}

impl From<&AddressBytes> for OutputType {
    #[inline]
    fn from(bytes: &AddressBytes) -> Self {
        match bytes {
            AddressBytes::P2PK65(_) => Self::P2PK65,
            AddressBytes::P2PK33(_) => Self::P2PK33,
            AddressBytes::P2PKH(_) => Self::P2PKH,
            AddressBytes::P2SH(_) => Self::P2SH,
            AddressBytes::P2WPKH(_) => Self::P2WPKH,
            AddressBytes::P2WSH(_) => Self::P2WSH,
            AddressBytes::P2TR(_) => Self::P2TR,
            AddressBytes::P2A(_) => Self::P2A,
        }
    }
}

impl TryFrom<OutputType> for AddressType {
    type Error = Error;
    fn try_from(value: OutputType) -> Result<Self, Self::Error> {
        Ok(match value {
            OutputType::P2A => Self::P2a,
            OutputType::P2PKH => Self::P2pkh,
            OutputType::P2SH => Self::P2sh,
            OutputType::P2TR => Self::P2tr,
            OutputType::P2WPKH => Self::P2wpkh,
            OutputType::P2WSH => Self::P2wsh,
            _ => return Err(Error::UnsupportedType(format!("{:?}", value))),
        })
    }
}

impl Formattable for OutputType {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}

impl Bytes for OutputType {
    type Array = [u8; size_of::<Self>()];

    #[inline]
    fn to_bytes(&self) -> Self::Array {
        [*self as u8]
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        if bytes.len() != size_of::<Self>() {
            return Err(vecdb::Error::WrongLength {
                expected: size_of::<Self>(),
                received: bytes.len(),
            });
        };
        let value = bytes[0];
        if !Self::is_valid(value) {
            return Err(vecdb::Error::InvalidArgument("invalid OutputType"));
        }
        // SAFETY: We validated that value is a valid variant
        let s: Self = unsafe { std::mem::transmute(value) };
        Ok(s)
    }
}

impl Pco for OutputType {
    type NumberType = u8;
}

impl TransparentPco<u8> for OutputType {}
