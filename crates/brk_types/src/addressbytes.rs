use std::str::FromStr;

use bitcoin::{Network, PublicKey, opcodes, script::Builder, ScriptBuf};
use brk_error::Error;

use super::{
    OutputType, P2ABytes, P2PK33Bytes, P2PK65Bytes, P2PKHBytes, P2SHBytes, P2TRBytes, P2WPKHBytes,
    P2WSHBytes,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddressBytes {
    P2PK65(Box<P2PK65Bytes>), // 65
    P2PK33(Box<P2PK33Bytes>), // 33
    P2PKH(Box<P2PKHBytes>),   // 20
    P2SH(Box<P2SHBytes>),     // 20
    P2WPKH(Box<P2WPKHBytes>), // 20
    P2WSH(Box<P2WSHBytes>),   // 32
    P2TR(Box<P2TRBytes>),     // 32
    P2A(Box<P2ABytes>),       // 2
}

impl AddressBytes {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            AddressBytes::P2PK65(bytes) => &bytes[..],
            AddressBytes::P2PK33(bytes) => &bytes[..],
            AddressBytes::P2PKH(bytes) => &bytes[..],
            AddressBytes::P2SH(bytes) => &bytes[..],
            AddressBytes::P2WPKH(bytes) => &bytes[..],
            AddressBytes::P2WSH(bytes) => &bytes[..],
            AddressBytes::P2TR(bytes) => &bytes[..],
            AddressBytes::P2A(bytes) => &bytes[..],
        }
    }

    pub fn hash(&self) -> u64 {
        rapidhash::v3::rapidhash_v3(self.as_slice()).to_le()
    }

    /// Reconstruct the script_pubkey from the address bytes
    pub fn to_script_pubkey(&self) -> ScriptBuf {
        match self {
            AddressBytes::P2PK65(b) => Builder::new()
                .push_slice(****b)
                .push_opcode(opcodes::all::OP_CHECKSIG)
                .into_script(),
            AddressBytes::P2PK33(b) => Builder::new()
                .push_slice(****b)
                .push_opcode(opcodes::all::OP_CHECKSIG)
                .into_script(),
            AddressBytes::P2PKH(b) => Builder::new()
                .push_opcode(opcodes::all::OP_DUP)
                .push_opcode(opcodes::all::OP_HASH160)
                .push_slice(****b)
                .push_opcode(opcodes::all::OP_EQUALVERIFY)
                .push_opcode(opcodes::all::OP_CHECKSIG)
                .into_script(),
            AddressBytes::P2SH(b) => Builder::new()
                .push_opcode(opcodes::all::OP_HASH160)
                .push_slice(****b)
                .push_opcode(opcodes::all::OP_EQUAL)
                .into_script(),
            AddressBytes::P2WPKH(b) => {
                Builder::new().push_int(0).push_slice(****b).into_script()
            }
            AddressBytes::P2WSH(b) => {
                Builder::new().push_int(0).push_slice(****b).into_script()
            }
            AddressBytes::P2TR(b) => {
                Builder::new().push_int(1).push_slice(****b).into_script()
            }
            AddressBytes::P2A(b) => {
                Builder::new().push_int(1).push_slice(****b).into_script()
            }
        }
    }
}

impl TryFrom<&ScriptBuf> for AddressBytes {
    type Error = Error;
    fn try_from(script: &ScriptBuf) -> Result<Self, Self::Error> {
        Self::try_from((script, OutputType::from(script)))
    }
}

impl TryFrom<(&ScriptBuf, OutputType)> for AddressBytes {
    type Error = Error;
    fn try_from(tuple: (&ScriptBuf, OutputType)) -> Result<Self, Self::Error> {
        let (script, outputtype) = tuple;

        match outputtype {
            OutputType::P2PK65 => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    67 => &bytes[1..66],
                    len => {
                        dbg!(bytes);
                        return Err(Error::WrongLength {
                            expected: 67,
                            received: len,
                        });
                    }
                };
                Ok(Self::P2PK65(Box::new(P2PK65Bytes::from(bytes))))
            }
            OutputType::P2PK33 => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    35 => &bytes[1..34],
                    len => {
                        dbg!(bytes);
                        return Err(Error::WrongLength {
                            expected: 35,
                            received: len,
                        });
                    }
                };
                Ok(Self::P2PK33(Box::new(P2PK33Bytes::from(bytes))))
            }
            OutputType::P2PKH => {
                let bytes = &script.as_bytes()[3..23];
                Ok(Self::P2PKH(Box::new(P2PKHBytes::from(bytes))))
            }
            OutputType::P2SH => {
                let bytes = &script.as_bytes()[2..22];
                Ok(Self::P2SH(Box::new(P2SHBytes::from(bytes))))
            }
            OutputType::P2WPKH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WPKH(Box::new(P2WPKHBytes::from(bytes))))
            }
            OutputType::P2WSH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WSH(Box::new(P2WSHBytes::from(bytes))))
            }
            OutputType::P2TR => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2TR(Box::new(P2TRBytes::from(bytes))))
            }
            OutputType::P2A => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2A(Box::new(P2ABytes::from(bytes))))
            }
            OutputType::P2MS => Err(Error::WrongAddressType),
            OutputType::Unknown => Err(Error::WrongAddressType),
            OutputType::Empty => Err(Error::WrongAddressType),
            OutputType::OpReturn => Err(Error::WrongAddressType),
            _ => unreachable!(),
        }
    }
}

impl From<P2PK65Bytes> for AddressBytes {
    #[inline]
    fn from(value: P2PK65Bytes) -> Self {
        Self::P2PK65(Box::new(value))
    }
}

impl From<P2PK33Bytes> for AddressBytes {
    #[inline]
    fn from(value: P2PK33Bytes) -> Self {
        Self::P2PK33(Box::new(value))
    }
}

impl From<P2PKHBytes> for AddressBytes {
    #[inline]
    fn from(value: P2PKHBytes) -> Self {
        Self::P2PKH(Box::new(value))
    }
}

impl From<P2SHBytes> for AddressBytes {
    #[inline]
    fn from(value: P2SHBytes) -> Self {
        Self::P2SH(Box::new(value))
    }
}

impl From<P2WPKHBytes> for AddressBytes {
    #[inline]
    fn from(value: P2WPKHBytes) -> Self {
        Self::P2WPKH(Box::new(value))
    }
}

impl From<P2WSHBytes> for AddressBytes {
    #[inline]
    fn from(value: P2WSHBytes) -> Self {
        Self::P2WSH(Box::new(value))
    }
}

impl From<P2TRBytes> for AddressBytes {
    #[inline]
    fn from(value: P2TRBytes) -> Self {
        Self::P2TR(Box::new(value))
    }
}

impl From<P2ABytes> for AddressBytes {
    #[inline]
    fn from(value: P2ABytes) -> Self {
        Self::P2A(Box::new(value))
    }
}

impl AddressBytes {
    /// Parse an address string to a ScriptBuf
    pub fn address_to_script(address: &str) -> Result<ScriptBuf, Error> {
        if let Ok(address) = bitcoin::Address::from_str(address) {
            if !address.is_valid_for_network(Network::Bitcoin) {
                return Err(Error::InvalidNetwork);
            }
            let address = address.assume_checked();
            Ok(address.script_pubkey())
        } else if let Ok(pubkey) = PublicKey::from_str(address) {
            Ok(ScriptBuf::new_p2pk(&pubkey))
        } else {
            Err(Error::InvalidAddress)
        }
    }
}

impl FromStr for AddressBytes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let script = Self::address_to_script(s)?;
        let outputtype = OutputType::from(&script);
        Self::try_from((&script, outputtype))
    }
}
