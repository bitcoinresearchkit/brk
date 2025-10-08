use bitcoin::{AddressType, ScriptBuf, opcodes::all::OP_PUSHBYTES_2};
use brk_error::Error;
use schemars::JsonSchema;
use serde::Serialize;
use strum::Display;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    JsonSchema,
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
    #[schemars(skip)]
    Dummy10,
    #[schemars(skip)]
    Dummy11,
    #[schemars(skip)]
    Dummy12,
    #[schemars(skip)]
    Dummy13,
    #[schemars(skip)]
    Dummy14,
    #[schemars(skip)]
    Dummy15,
    #[schemars(skip)]
    Dummy16,
    #[schemars(skip)]
    Dummy17,
    #[schemars(skip)]
    Dummy18,
    #[schemars(skip)]
    Dummy19,
    #[schemars(skip)]
    Dummy20,
    #[schemars(skip)]
    Dummy21,
    #[schemars(skip)]
    Dummy22,
    #[schemars(skip)]
    Dummy23,
    #[schemars(skip)]
    Dummy24,
    #[schemars(skip)]
    Dummy25,
    #[schemars(skip)]
    Dummy26,
    #[schemars(skip)]
    Dummy27,
    #[schemars(skip)]
    Dummy28,
    #[schemars(skip)]
    Dummy29,
    #[schemars(skip)]
    Dummy30,
    #[schemars(skip)]
    Dummy31,
    #[schemars(skip)]
    Dummy32,
    #[schemars(skip)]
    Dummy33,
    #[schemars(skip)]
    Dummy34,
    #[schemars(skip)]
    Dummy35,
    #[schemars(skip)]
    Dummy36,
    #[schemars(skip)]
    Dummy37,
    #[schemars(skip)]
    Dummy38,
    #[schemars(skip)]
    Dummy39,
    #[schemars(skip)]
    Dummy40,
    #[schemars(skip)]
    Dummy41,
    #[schemars(skip)]
    Dummy42,
    #[schemars(skip)]
    Dummy43,
    #[schemars(skip)]
    Dummy44,
    #[schemars(skip)]
    Dummy45,
    #[schemars(skip)]
    Dummy46,
    #[schemars(skip)]
    Dummy47,
    #[schemars(skip)]
    Dummy48,
    #[schemars(skip)]
    Dummy49,
    #[schemars(skip)]
    Dummy50,
    #[schemars(skip)]
    Dummy51,
    #[schemars(skip)]
    Dummy52,
    #[schemars(skip)]
    Dummy53,
    #[schemars(skip)]
    Dummy54,
    #[schemars(skip)]
    Dummy55,
    #[schemars(skip)]
    Dummy56,
    #[schemars(skip)]
    Dummy57,
    #[schemars(skip)]
    Dummy58,
    #[schemars(skip)]
    Dummy59,
    #[schemars(skip)]
    Dummy60,
    #[schemars(skip)]
    Dummy61,
    #[schemars(skip)]
    Dummy62,
    #[schemars(skip)]
    Dummy63,
    #[schemars(skip)]
    Dummy64,
    #[schemars(skip)]
    Dummy65,
    #[schemars(skip)]
    Dummy66,
    #[schemars(skip)]
    Dummy67,
    #[schemars(skip)]
    Dummy68,
    #[schemars(skip)]
    Dummy69,
    #[schemars(skip)]
    Dummy70,
    #[schemars(skip)]
    Dummy71,
    #[schemars(skip)]
    Dummy72,
    #[schemars(skip)]
    Dummy73,
    #[schemars(skip)]
    Dummy74,
    #[schemars(skip)]
    Dummy75,
    #[schemars(skip)]
    Dummy76,
    #[schemars(skip)]
    Dummy77,
    #[schemars(skip)]
    Dummy78,
    #[schemars(skip)]
    Dummy79,
    #[schemars(skip)]
    Dummy80,
    #[schemars(skip)]
    Dummy81,
    #[schemars(skip)]
    Dummy82,
    #[schemars(skip)]
    Dummy83,
    #[schemars(skip)]
    Dummy84,
    #[schemars(skip)]
    Dummy85,
    #[schemars(skip)]
    Dummy86,
    #[schemars(skip)]
    Dummy87,
    #[schemars(skip)]
    Dummy88,
    #[schemars(skip)]
    Dummy89,
    #[schemars(skip)]
    Dummy90,
    #[schemars(skip)]
    Dummy91,
    #[schemars(skip)]
    Dummy92,
    #[schemars(skip)]
    Dummy93,
    #[schemars(skip)]
    Dummy94,
    #[schemars(skip)]
    Dummy95,
    #[schemars(skip)]
    Dummy96,
    #[schemars(skip)]
    Dummy97,
    #[schemars(skip)]
    Dummy98,
    #[schemars(skip)]
    Dummy99,
    #[schemars(skip)]
    Dummy100,
    #[schemars(skip)]
    Dummy101,
    #[schemars(skip)]
    Dummy102,
    #[schemars(skip)]
    Dummy103,
    #[schemars(skip)]
    Dummy104,
    #[schemars(skip)]
    Dummy105,
    #[schemars(skip)]
    Dummy106,
    #[schemars(skip)]
    Dummy107,
    #[schemars(skip)]
    Dummy108,
    #[schemars(skip)]
    Dummy109,
    #[schemars(skip)]
    Dummy110,
    #[schemars(skip)]
    Dummy111,
    #[schemars(skip)]
    Dummy112,
    #[schemars(skip)]
    Dummy113,
    #[schemars(skip)]
    Dummy114,
    #[schemars(skip)]
    Dummy115,
    #[schemars(skip)]
    Dummy116,
    #[schemars(skip)]
    Dummy117,
    #[schemars(skip)]
    Dummy118,
    #[schemars(skip)]
    Dummy119,
    #[schemars(skip)]
    Dummy120,
    #[schemars(skip)]
    Dummy121,
    #[schemars(skip)]
    Dummy122,
    #[schemars(skip)]
    Dummy123,
    #[schemars(skip)]
    Dummy124,
    #[schemars(skip)]
    Dummy125,
    #[schemars(skip)]
    Dummy126,
    #[schemars(skip)]
    Dummy127,
    #[schemars(skip)]
    Dummy128,
    #[schemars(skip)]
    Dummy129,
    #[schemars(skip)]
    Dummy130,
    #[schemars(skip)]
    Dummy131,
    #[schemars(skip)]
    Dummy132,
    #[schemars(skip)]
    Dummy133,
    #[schemars(skip)]
    Dummy134,
    #[schemars(skip)]
    Dummy135,
    #[schemars(skip)]
    Dummy136,
    #[schemars(skip)]
    Dummy137,
    #[schemars(skip)]
    Dummy138,
    #[schemars(skip)]
    Dummy139,
    #[schemars(skip)]
    Dummy140,
    #[schemars(skip)]
    Dummy141,
    #[schemars(skip)]
    Dummy142,
    #[schemars(skip)]
    Dummy143,
    #[schemars(skip)]
    Dummy144,
    #[schemars(skip)]
    Dummy145,
    #[schemars(skip)]
    Dummy146,
    #[schemars(skip)]
    Dummy147,
    #[schemars(skip)]
    Dummy148,
    #[schemars(skip)]
    Dummy149,
    #[schemars(skip)]
    Dummy150,
    #[schemars(skip)]
    Dummy151,
    #[schemars(skip)]
    Dummy152,
    #[schemars(skip)]
    Dummy153,
    #[schemars(skip)]
    Dummy154,
    #[schemars(skip)]
    Dummy155,
    #[schemars(skip)]
    Dummy156,
    #[schemars(skip)]
    Dummy157,
    #[schemars(skip)]
    Dummy158,
    #[schemars(skip)]
    Dummy159,
    #[schemars(skip)]
    Dummy160,
    #[schemars(skip)]
    Dummy161,
    #[schemars(skip)]
    Dummy162,
    #[schemars(skip)]
    Dummy163,
    #[schemars(skip)]
    Dummy164,
    #[schemars(skip)]
    Dummy165,
    #[schemars(skip)]
    Dummy166,
    #[schemars(skip)]
    Dummy167,
    #[schemars(skip)]
    Dummy168,
    #[schemars(skip)]
    Dummy169,
    #[schemars(skip)]
    Dummy170,
    #[schemars(skip)]
    Dummy171,
    #[schemars(skip)]
    Dummy172,
    #[schemars(skip)]
    Dummy173,
    #[schemars(skip)]
    Dummy174,
    #[schemars(skip)]
    Dummy175,
    #[schemars(skip)]
    Dummy176,
    #[schemars(skip)]
    Dummy177,
    #[schemars(skip)]
    Dummy178,
    #[schemars(skip)]
    Dummy179,
    #[schemars(skip)]
    Dummy180,
    #[schemars(skip)]
    Dummy181,
    #[schemars(skip)]
    Dummy182,
    #[schemars(skip)]
    Dummy183,
    #[schemars(skip)]
    Dummy184,
    #[schemars(skip)]
    Dummy185,
    #[schemars(skip)]
    Dummy186,
    #[schemars(skip)]
    Dummy187,
    #[schemars(skip)]
    Dummy188,
    #[schemars(skip)]
    Dummy189,
    #[schemars(skip)]
    Dummy190,
    #[schemars(skip)]
    Dummy191,
    #[schemars(skip)]
    Dummy192,
    #[schemars(skip)]
    Dummy193,
    #[schemars(skip)]
    Dummy194,
    #[schemars(skip)]
    Dummy195,
    #[schemars(skip)]
    Dummy196,
    #[schemars(skip)]
    Dummy197,
    #[schemars(skip)]
    Dummy198,
    #[schemars(skip)]
    Dummy199,
    #[schemars(skip)]
    Dummy200,
    #[schemars(skip)]
    Dummy201,
    #[schemars(skip)]
    Dummy202,
    #[schemars(skip)]
    Dummy203,
    #[schemars(skip)]
    Dummy204,
    #[schemars(skip)]
    Dummy205,
    #[schemars(skip)]
    Dummy206,
    #[schemars(skip)]
    Dummy207,
    #[schemars(skip)]
    Dummy208,
    #[schemars(skip)]
    Dummy209,
    #[schemars(skip)]
    Dummy210,
    #[schemars(skip)]
    Dummy211,
    #[schemars(skip)]
    Dummy212,
    #[schemars(skip)]
    Dummy213,
    #[schemars(skip)]
    Dummy214,
    #[schemars(skip)]
    Dummy215,
    #[schemars(skip)]
    Dummy216,
    #[schemars(skip)]
    Dummy217,
    #[schemars(skip)]
    Dummy218,
    #[schemars(skip)]
    Dummy219,
    #[schemars(skip)]
    Dummy220,
    #[schemars(skip)]
    Dummy221,
    #[schemars(skip)]
    Dummy222,
    #[schemars(skip)]
    Dummy223,
    #[schemars(skip)]
    Dummy224,
    #[schemars(skip)]
    Dummy225,
    #[schemars(skip)]
    Dummy226,
    #[schemars(skip)]
    Dummy227,
    #[schemars(skip)]
    Dummy228,
    #[schemars(skip)]
    Dummy229,
    #[schemars(skip)]
    Dummy230,
    #[schemars(skip)]
    Dummy231,
    #[schemars(skip)]
    Dummy232,
    #[schemars(skip)]
    Dummy233,
    #[schemars(skip)]
    Dummy234,
    #[schemars(skip)]
    Dummy235,
    #[schemars(skip)]
    Dummy236,
    #[schemars(skip)]
    Dummy237,
    #[schemars(skip)]
    Dummy238,
    #[schemars(skip)]
    Dummy239,
    #[schemars(skip)]
    Dummy240,
    #[schemars(skip)]
    Dummy241,
    #[schemars(skip)]
    Dummy242,
    #[schemars(skip)]
    Dummy243,
    #[schemars(skip)]
    Dummy244,
    #[schemars(skip)]
    Dummy245,
    #[schemars(skip)]
    Dummy246,
    #[schemars(skip)]
    Dummy247,
    #[schemars(skip)]
    Dummy248,
    #[schemars(skip)]
    Dummy249,
    #[schemars(skip)]
    Dummy250,
    #[schemars(skip)]
    Dummy251,
    #[schemars(skip)]
    Dummy252,
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
