use color_eyre::eyre::eyre;
use derive_deref::{Deref, DerefMut};
use iterator::bitcoin::ScriptBuf;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Addresstype;

#[derive(Debug, PartialEq, Eq)]
pub enum Addressbytes {
    P2PK65(P2PK65AddressBytes),
    P2PK33(P2PK33AddressBytes),
    P2PKH(P2PKHAddressBytes),
    P2SH(P2SHAddressBytes),
    P2WPKH(P2WPKHAddressBytes),
    P2WSH(P2WSHAddressBytes),
    P2TR(P2TRAddressBytes),
}

impl Addressbytes {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Addressbytes::P2PK65(bytes) => &bytes[..],
            Addressbytes::P2PK33(bytes) => &bytes[..],
            Addressbytes::P2PKH(bytes) => &bytes[..],
            Addressbytes::P2SH(bytes) => &bytes[..],
            Addressbytes::P2WPKH(bytes) => &bytes[..],
            Addressbytes::P2WSH(bytes) => &bytes[..],
            Addressbytes::P2TR(bytes) => &bytes[..],
        }
    }
}

impl TryFrom<(&ScriptBuf, Addresstype)> for Addressbytes {
    type Error = color_eyre::Report;
    fn try_from(tuple: (&ScriptBuf, Addresstype)) -> Result<Self, Self::Error> {
        let (script, addresstype) = tuple;

        match addresstype {
            Addresstype::P2PK65 => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    67 => &bytes[1..66],
                    _ => {
                        dbg!(bytes);
                        return Err(eyre!("Wrong len"));
                    }
                };
                Ok(Self::P2PK65(P2PK65AddressBytes(U8x65::from(bytes))))
            }
            Addresstype::P2PK33 => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    35 => &bytes[1..34],
                    _ => {
                        dbg!(bytes);
                        return Err(eyre!("Wrong len"));
                    }
                };
                Ok(Self::P2PK33(P2PK33AddressBytes(U8x33::from(bytes))))
            }
            Addresstype::P2PKH => {
                let bytes = &script.as_bytes()[3..23];
                Ok(Self::P2PKH(P2PKHAddressBytes(U8x20::from(bytes))))
            }
            Addresstype::P2SH => {
                let bytes = &script.as_bytes()[2..22];
                Ok(Self::P2SH(P2SHAddressBytes(U8x20::from(bytes))))
            }
            Addresstype::P2WPKH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WPKH(P2WPKHAddressBytes(U8x20::from(bytes))))
            }
            Addresstype::P2WSH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WSH(P2WSHAddressBytes(U8x32::from(bytes))))
            }
            Addresstype::P2TR => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2TR(P2TRAddressBytes(U8x32::from(bytes))))
            }
            Addresstype::Multisig => Err(eyre!("multisig address type")),
            Addresstype::PushOnly => Err(eyre!("push_only address type")),
            Addresstype::Unknown => Err(eyre!("unknown address type")),
            Addresstype::Empty => Err(eyre!("empty address type")),
            Addresstype::OpReturn => Err(eyre!("op_return address type")),
        }
    }
}

impl From<P2PK65AddressBytes> for Addressbytes {
    fn from(value: P2PK65AddressBytes) -> Self {
        Self::P2PK65(value)
    }
}
impl From<P2PK33AddressBytes> for Addressbytes {
    fn from(value: P2PK33AddressBytes) -> Self {
        Self::P2PK33(value)
    }
}
impl From<P2PKHAddressBytes> for Addressbytes {
    fn from(value: P2PKHAddressBytes) -> Self {
        Self::P2PKH(value)
    }
}
impl From<P2SHAddressBytes> for Addressbytes {
    fn from(value: P2SHAddressBytes) -> Self {
        Self::P2SH(value)
    }
}
impl From<P2WPKHAddressBytes> for Addressbytes {
    fn from(value: P2WPKHAddressBytes) -> Self {
        Self::P2WPKH(value)
    }
}
impl From<P2WSHAddressBytes> for Addressbytes {
    fn from(value: P2WSHAddressBytes) -> Self {
        Self::P2WSH(value)
    }
}
impl From<P2TRAddressBytes> for Addressbytes {
    fn from(value: P2TRAddressBytes) -> Self {
        Self::P2TR(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2PK65AddressBytes(U8x65);

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2PK33AddressBytes(U8x33);

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2PKHAddressBytes(U8x20);

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2SHAddressBytes(U8x20);

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2WPKHAddressBytes(U8x20);

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2WSHAddressBytes(U8x32);

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct P2TRAddressBytes(U8x32);

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct U8x20([u8; 20]);
impl From<&[u8]> for U8x20 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 20];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct U8x32([u8; 32]);
impl From<&[u8]> for U8x32 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 32];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct U8x33(#[serde(with = "serde_bytes")] [u8; 33]);
impl From<&[u8]> for U8x33 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 33];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct U8x64(#[serde(with = "serde_bytes")] [u8; 64]);
impl From<&[u8]> for U8x64 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 64];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct U8x65(#[serde(with = "serde_bytes")] [u8; 65]);
impl From<&[u8]> for U8x65 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 65];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}
