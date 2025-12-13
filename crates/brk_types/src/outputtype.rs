use bitcoin::{AddressType, ScriptBuf, opcodes::all::OP_PUSHBYTES_2};
use brk_error::Error;
use schemars::JsonSchema;
use serde::Serialize;
use strum::Display;
use vecdb::{Bytes, Formattable};

use crate::AddressBytes;

#[derive(
    Debug, Clone, Copy, Display, PartialEq, Eq, PartialOrd, Ord, Serialize, JsonSchema, Hash,
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
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy10,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy11,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy12,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy13,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy14,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy15,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy16,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy17,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy18,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy19,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy20,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy21,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy22,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy23,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy24,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy25,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy26,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy27,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy28,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy29,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy30,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy31,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy32,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy33,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy34,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy35,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy36,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy37,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy38,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy39,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy40,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy41,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy42,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy43,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy44,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy45,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy46,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy47,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy48,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy49,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy50,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy51,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy52,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy53,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy54,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy55,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy56,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy57,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy58,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy59,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy60,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy61,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy62,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy63,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy64,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy65,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy66,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy67,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy68,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy69,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy70,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy71,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy72,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy73,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy74,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy75,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy76,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy77,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy78,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy79,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy80,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy81,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy82,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy83,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy84,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy85,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy86,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy87,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy88,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy89,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy90,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy91,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy92,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy93,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy94,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy95,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy96,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy97,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy98,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy99,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy100,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy101,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy102,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy103,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy104,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy105,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy106,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy107,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy108,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy109,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy110,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy111,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy112,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy113,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy114,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy115,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy116,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy117,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy118,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy119,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy120,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy121,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy122,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy123,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy124,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy125,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy126,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy127,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy128,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy129,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy130,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy131,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy132,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy133,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy134,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy135,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy136,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy137,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy138,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy139,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy140,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy141,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy142,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy143,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy144,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy145,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy146,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy147,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy148,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy149,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy150,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy151,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy152,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy153,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy154,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy155,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy156,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy157,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy158,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy159,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy160,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy161,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy162,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy163,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy164,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy165,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy166,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy167,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy168,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy169,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy170,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy171,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy172,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy173,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy174,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy175,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy176,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy177,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy178,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy179,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy180,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy181,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy182,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy183,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy184,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy185,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy186,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy187,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy188,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy189,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy190,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy191,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy192,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy193,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy194,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy195,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy196,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy197,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy198,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy199,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy200,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy201,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy202,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy203,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy204,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy205,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy206,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy207,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy208,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy209,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy210,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy211,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy212,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy213,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy214,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy215,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy216,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy217,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy218,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy219,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy220,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy221,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy222,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy223,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy224,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy225,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy226,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy227,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy228,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy229,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy230,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy231,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy232,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy233,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy234,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy235,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy236,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy237,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy238,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy239,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy240,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy241,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy242,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy243,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy244,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy245,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy246,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy247,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy248,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy249,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy250,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy251,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy252,
    #[doc(hidden)]
    #[schemars(skip)]
    Dummy253,
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
            _ => unreachable!(),
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
            _ => unreachable!(),
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
            _ => return Err(Error::Str("Bad output format")),
        })
    }
}

impl Formattable for OutputType {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
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
        // SAFETY: OutputType is repr(u8) and we're transmuting from u8
        // All values 0-255 are valid (includes dummy variants)
        let s: Self = unsafe { std::mem::transmute(bytes[0]) };
        Ok(s)
    }
}
