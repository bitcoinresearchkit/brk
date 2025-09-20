use bitcoin::{Address, AddressType, ScriptBuf, opcodes::all::OP_PUSHBYTES_2};
use brk_error::Error;
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
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
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
    Dummy10,
    Dummy11,
    Dummy12,
    Dummy13,
    Dummy14,
    Dummy15,
    Dummy16,
    Dummy17,
    Dummy18,
    Dummy19,
    Dummy20,
    Dummy21,
    Dummy22,
    Dummy23,
    Dummy24,
    Dummy25,
    Dummy26,
    Dummy27,
    Dummy28,
    Dummy29,
    Dummy30,
    Dummy31,
    Dummy32,
    Dummy33,
    Dummy34,
    Dummy35,
    Dummy36,
    Dummy37,
    Dummy38,
    Dummy39,
    Dummy40,
    Dummy41,
    Dummy42,
    Dummy43,
    Dummy44,
    Dummy45,
    Dummy46,
    Dummy47,
    Dummy48,
    Dummy49,
    Dummy50,
    Dummy51,
    Dummy52,
    Dummy53,
    Dummy54,
    Dummy55,
    Dummy56,
    Dummy57,
    Dummy58,
    Dummy59,
    Dummy60,
    Dummy61,
    Dummy62,
    Dummy63,
    Dummy64,
    Dummy65,
    Dummy66,
    Dummy67,
    Dummy68,
    Dummy69,
    Dummy70,
    Dummy71,
    Dummy72,
    Dummy73,
    Dummy74,
    Dummy75,
    Dummy76,
    Dummy77,
    Dummy78,
    Dummy79,
    Dummy80,
    Dummy81,
    Dummy82,
    Dummy83,
    Dummy84,
    Dummy85,
    Dummy86,
    Dummy87,
    Dummy88,
    Dummy89,
    Dummy90,
    Dummy91,
    Dummy92,
    Dummy93,
    Dummy94,
    Dummy95,
    Dummy96,
    Dummy97,
    Dummy98,
    Dummy99,
    Dummy100,
    Dummy101,
    Dummy102,
    Dummy103,
    Dummy104,
    Dummy105,
    Dummy106,
    Dummy107,
    Dummy108,
    Dummy109,
    Dummy110,
    Dummy111,
    Dummy112,
    Dummy113,
    Dummy114,
    Dummy115,
    Dummy116,
    Dummy117,
    Dummy118,
    Dummy119,
    Dummy120,
    Dummy121,
    Dummy122,
    Dummy123,
    Dummy124,
    Dummy125,
    Dummy126,
    Dummy127,
    Dummy128,
    Dummy129,
    Dummy130,
    Dummy131,
    Dummy132,
    Dummy133,
    Dummy134,
    Dummy135,
    Dummy136,
    Dummy137,
    Dummy138,
    Dummy139,
    Dummy140,
    Dummy141,
    Dummy142,
    Dummy143,
    Dummy144,
    Dummy145,
    Dummy146,
    Dummy147,
    Dummy148,
    Dummy149,
    Dummy150,
    Dummy151,
    Dummy152,
    Dummy153,
    Dummy154,
    Dummy155,
    Dummy156,
    Dummy157,
    Dummy158,
    Dummy159,
    Dummy160,
    Dummy161,
    Dummy162,
    Dummy163,
    Dummy164,
    Dummy165,
    Dummy166,
    Dummy167,
    Dummy168,
    Dummy169,
    Dummy170,
    Dummy171,
    Dummy172,
    Dummy173,
    Dummy174,
    Dummy175,
    Dummy176,
    Dummy177,
    Dummy178,
    Dummy179,
    Dummy180,
    Dummy181,
    Dummy182,
    Dummy183,
    Dummy184,
    Dummy185,
    Dummy186,
    Dummy187,
    Dummy188,
    Dummy189,
    Dummy190,
    Dummy191,
    Dummy192,
    Dummy193,
    Dummy194,
    Dummy195,
    Dummy196,
    Dummy197,
    Dummy198,
    Dummy199,
    Dummy200,
    Dummy201,
    Dummy202,
    Dummy203,
    Dummy204,
    Dummy205,
    Dummy206,
    Dummy207,
    Dummy208,
    Dummy209,
    Dummy210,
    Dummy211,
    Dummy212,
    Dummy213,
    Dummy214,
    Dummy215,
    Dummy216,
    Dummy217,
    Dummy218,
    Dummy219,
    Dummy220,
    Dummy221,
    Dummy222,
    Dummy223,
    Dummy224,
    Dummy225,
    Dummy226,
    Dummy227,
    Dummy228,
    Dummy229,
    Dummy230,
    Dummy231,
    Dummy232,
    Dummy233,
    Dummy234,
    Dummy235,
    Dummy236,
    Dummy237,
    Dummy238,
    Dummy239,
    Dummy240,
    Dummy241,
    Dummy242,
    Dummy243,
    Dummy244,
    Dummy245,
    Dummy246,
    Dummy247,
    Dummy248,
    Dummy249,
    Dummy250,
    Dummy251,
    Dummy252,
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

impl From<&Address> for OutputType {
    fn from(value: &Address) -> Self {
        Self::from(&value.script_pubkey())
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
