use serde::Serialize;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::OutputType;

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
pub enum AddressType {
    P2PK65,
    P2PK33,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
    P2A,
}

impl From<OutputType> for AddressType {
    fn from(value: OutputType) -> Self {
        match value {
            OutputType::P2A => Self::P2A,
            OutputType::P2PK33 => Self::P2PK33,
            OutputType::P2PK65 => Self::P2PK65,
            OutputType::P2PKH => Self::P2PKH,
            OutputType::P2SH => Self::P2SH,
            OutputType::P2TR => Self::P2TR,
            OutputType::P2WPKH => Self::P2WPKH,
            OutputType::P2WSH => Self::P2WSH,
            OutputType::Empty | OutputType::OpReturn | OutputType::P2MS | OutputType::Unknown => {
                unreachable!()
            }
        }
    }
}
