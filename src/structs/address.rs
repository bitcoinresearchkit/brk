use bitcoin_hashes::hash160;
use biter::bitcoin::TxOut;

use super::{AddressType, Counter, U8x19, U8x31};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Address {
    // https://mempool.space/tx/7bd54def72825008b4ca0f4aeff13e6be2c5fe0f23430629a9d484a1ac2a29b8
    Empty(u32),
    OpReturn(u32),
    PushOnly(u32),
    Unknown(u32),
    // https://mempool.space/tx/274f8be3b7b9b1a220285f5f71f61e2691dd04df9d69bb02a8b3b85f91fb1857
    MultiSig(u32),
    P2PK((u16, U8x19)),
    P2PKH((u16, U8x19)),
    P2SH((u16, U8x19)),
    P2WPKH((u16, U8x19)),
    P2WSH((u16, U8x31)),
    P2TR((u16, U8x31)),
}

impl Address {
    pub fn to_type(&self) -> AddressType {
        match self {
            Self::Empty(_) => AddressType::Empty,
            Self::OpReturn(_) => AddressType::OpReturn,
            Self::PushOnly(_) => AddressType::PushOnly,
            Self::Unknown(_) => AddressType::Unknown,
            Self::MultiSig(_) => AddressType::MultiSig,
            Self::P2PK(_) => AddressType::P2PK,
            Self::P2PKH(_) => AddressType::P2PKH,
            Self::P2SH(_) => AddressType::P2SH,
            Self::P2WPKH(_) => AddressType::P2WPKH,
            Self::P2WSH(_) => AddressType::P2WSH,
            Self::P2TR(_) => AddressType::P2TR,
        }
    }

    pub fn from(
        txout: &TxOut,
        multisig_addresses: &mut Counter,
        op_return_addresses: &mut Counter,
        push_only_addresses: &mut Counter,
        unknown_addresses: &mut Counter,
        empty_addresses: &mut Counter,
    ) -> Self {
        let script = &txout.script_pubkey;

        if script.is_p2pk() {
            let pk = match script.as_bytes().len() {
                67 => &script.as_bytes()[1..66],
                35 => &script.as_bytes()[1..34],
                _ => unreachable!(),
            };

            let hash = hash160::Hash::hash(pk);

            let (prefix, rest) = Self::split_slice(&hash.as_byte_array()[..]);

            Self::P2PK((prefix, rest.into()))
        } else if script.is_p2pkh() {
            let (prefix, rest) = Self::split_slice(&script.as_bytes()[3..23]);
            Self::P2PKH((prefix, rest.into()))
        } else if script.is_p2sh() {
            let (prefix, rest) = Self::split_slice(&script.as_bytes()[2..22]);
            Self::P2SH((prefix, rest.into()))
        } else if script.is_p2wpkh() {
            let (prefix, rest) = Self::split_slice(&script.as_bytes()[2..]);
            Self::P2WPKH((prefix, rest.into()))
        } else if script.is_p2wsh() {
            let (prefix, rest) = Self::split_slice(&script.as_bytes()[2..]);
            Self::P2WSH((prefix, rest.into()))
        } else if script.is_p2tr() {
            let (prefix, rest) = Self::split_slice(&script.as_bytes()[2..]);
            Self::P2TR((prefix, rest.into()))
        } else if script.is_empty() {
            let index = empty_addresses.inner();

            empty_addresses.increment();

            Self::Empty(index)
        } else if script.is_op_return() {
            let index = op_return_addresses.inner();

            op_return_addresses.increment();

            Self::OpReturn(index)
        } else if script.is_multisig() {
            let index = multisig_addresses.inner();

            multisig_addresses.increment();

            Self::MultiSig(index)
        } else if script.is_push_only() {
            let index = push_only_addresses.inner();

            push_only_addresses.increment();

            Self::PushOnly(index)
        } else {
            Self::new_unknown(unknown_addresses)
        }
    }

    fn new_unknown(unknown_addresses: &mut Counter) -> Address {
        let index = unknown_addresses.inner();
        unknown_addresses.increment();
        Self::Unknown(index)
    }

    fn split_slice(slice: &[u8]) -> (u16, &[u8]) {
        let prefix = ((slice[0] as u16) << 2) + ((slice[1] as u16) >> 6);
        let rest = &slice[1..];
        (prefix, rest)
    }
}
