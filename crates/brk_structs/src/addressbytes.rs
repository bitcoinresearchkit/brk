use bitcoin::ScriptBuf;
use brk_error::Error;

use super::{
    OutputType, P2ABytes, P2PK33Bytes, P2PK65Bytes, P2PKHBytes, P2SHBytes, P2TRBytes, P2WPKHBytes,
    P2WSHBytes,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AddressBytes {
    P2PK65(P2PK65Bytes),
    P2PK33(P2PK33Bytes),
    P2PKH(P2PKHBytes),
    P2SH(P2SHBytes),
    P2WPKH(P2WPKHBytes),
    P2WSH(P2WSHBytes),
    P2TR(P2TRBytes),
    P2A(P2ABytes),
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
                    _ => {
                        dbg!(bytes);
                        return Err(Error::WrongLength);
                    }
                };
                Ok(Self::P2PK65(P2PK65Bytes::from(bytes)))
            }
            OutputType::P2PK33 => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    35 => &bytes[1..34],
                    _ => {
                        dbg!(bytes);
                        return Err(Error::WrongLength);
                    }
                };
                Ok(Self::P2PK33(P2PK33Bytes::from(bytes)))
            }
            OutputType::P2PKH => {
                let bytes = &script.as_bytes()[3..23];
                Ok(Self::P2PKH(P2PKHBytes::from(bytes)))
            }
            OutputType::P2SH => {
                let bytes = &script.as_bytes()[2..22];
                Ok(Self::P2SH(P2SHBytes::from(bytes)))
            }
            OutputType::P2WPKH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WPKH(P2WPKHBytes::from(bytes)))
            }
            OutputType::P2WSH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WSH(P2WSHBytes::from(bytes)))
            }
            OutputType::P2TR => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2TR(P2TRBytes::from(bytes)))
            }
            OutputType::P2A => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2A(P2ABytes::from(bytes)))
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
    fn from(value: P2PK65Bytes) -> Self {
        Self::P2PK65(value)
    }
}

impl From<P2PK33Bytes> for AddressBytes {
    fn from(value: P2PK33Bytes) -> Self {
        Self::P2PK33(value)
    }
}

impl From<P2PKHBytes> for AddressBytes {
    fn from(value: P2PKHBytes) -> Self {
        Self::P2PKH(value)
    }
}

impl From<P2SHBytes> for AddressBytes {
    fn from(value: P2SHBytes) -> Self {
        Self::P2SH(value)
    }
}

impl From<P2WPKHBytes> for AddressBytes {
    fn from(value: P2WPKHBytes) -> Self {
        Self::P2WPKH(value)
    }
}

impl From<P2WSHBytes> for AddressBytes {
    fn from(value: P2WSHBytes) -> Self {
        Self::P2WSH(value)
    }
}

impl From<P2TRBytes> for AddressBytes {
    fn from(value: P2TRBytes) -> Self {
        Self::P2TR(value)
    }
}

impl From<P2ABytes> for AddressBytes {
    fn from(value: P2ABytes) -> Self {
        Self::P2A(value)
    }
}
