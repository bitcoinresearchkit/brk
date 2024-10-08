use allocative::Allocative;
use bincode::{Decode, Encode};

// https://unchained.com/blog/bitcoin-address-types-compared/
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Encode, Decode, Allocative,
)]
pub enum AddressType {
    Empty,
    OpReturn,
    PushOnly,
    #[default]
    Unknown,
    MultiSig,
    P2PK,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}

impl TryFrom<u64> for AddressType {
    type Error = ();

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        match u {
            x if x == AddressType::Empty as u64 => Ok(AddressType::Empty),
            x if x == AddressType::OpReturn as u64 => Ok(AddressType::OpReturn),
            x if x == AddressType::PushOnly as u64 => Ok(AddressType::PushOnly),
            x if x == AddressType::Unknown as u64 => Ok(AddressType::Unknown),
            x if x == AddressType::MultiSig as u64 => Ok(AddressType::MultiSig),
            x if x == AddressType::P2PK as u64 => Ok(AddressType::P2PK),
            x if x == AddressType::P2PKH as u64 => Ok(AddressType::P2PKH),
            x if x == AddressType::P2SH as u64 => Ok(AddressType::P2SH),
            x if x == AddressType::P2WPKH as u64 => Ok(AddressType::P2WPKH),
            x if x == AddressType::P2WSH as u64 => Ok(AddressType::P2WSH),
            x if x == AddressType::P2TR as u64 => Ok(AddressType::P2TR),
            _ => Err(()),
        }
    }
}
