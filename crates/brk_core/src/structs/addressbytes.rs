use std::fmt;

use bitcoin::{
    Address, Network, ScriptBuf,
    hex::{Case, DisplayHex},
    opcodes,
    script::Builder,
};
use derive_deref::{Deref, DerefMut};
use serde::{Serialize, Serializer};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Error;

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
    type Error = Error;
    fn try_from(tuple: (&ScriptBuf, Addresstype)) -> Result<Self, Self::Error> {
        let (script, addresstype) = tuple;

        match addresstype {
            Addresstype::P2PK65 => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    67 => &bytes[1..66],
                    _ => {
                        dbg!(bytes);
                        return Err(Error::WrongLength);
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
                        return Err(Error::WrongLength);
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
            Addresstype::Multisig => Err(Error::WrongAddressType),
            Addresstype::PushOnly => Err(Error::WrongAddressType),
            Addresstype::Unknown => Err(Error::WrongAddressType),
            Addresstype::Empty => Err(Error::WrongAddressType),
            Addresstype::OpReturn => Err(Error::WrongAddressType),
        }
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2PK65AddressBytes(U8x65);

impl fmt::Display for P2PK65AddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string(Case::Lower))
    }
}

impl Serialize for P2PK65AddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2PK65AddressBytes> for Addressbytes {
    fn from(value: P2PK65AddressBytes) -> Self {
        Self::P2PK65(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2PK33AddressBytes(U8x33);

impl fmt::Display for P2PK33AddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string(Case::Lower))
    }
}

impl Serialize for P2PK33AddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2PK33AddressBytes> for Addressbytes {
    fn from(value: P2PK33AddressBytes) -> Self {
        Self::P2PK33(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2PKHAddressBytes(U8x20);

impl fmt::Display for P2PKHAddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new()
            .push_opcode(opcodes::all::OP_DUP)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(*self.0)
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{}", address)
    }
}

impl Serialize for P2PKHAddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2PKHAddressBytes> for Addressbytes {
    fn from(value: P2PKHAddressBytes) -> Self {
        Self::P2PKH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2SHAddressBytes(U8x20);

impl fmt::Display for P2SHAddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new()
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(*self.0)
            .push_opcode(opcodes::all::OP_EQUAL)
            .into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{}", address)
    }
}

impl Serialize for P2SHAddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2SHAddressBytes> for Addressbytes {
    fn from(value: P2SHAddressBytes) -> Self {
        Self::P2SH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2WPKHAddressBytes(U8x20);

impl fmt::Display for P2WPKHAddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(0).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{}", address)
    }
}

impl Serialize for P2WPKHAddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2WPKHAddressBytes> for Addressbytes {
    fn from(value: P2WPKHAddressBytes) -> Self {
        Self::P2WPKH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2WSHAddressBytes(U8x32);

impl fmt::Display for P2WSHAddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(0).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{}", address)
    }
}

impl Serialize for P2WSHAddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2WSHAddressBytes> for Addressbytes {
    fn from(value: P2WSHAddressBytes) -> Self {
        Self::P2WSH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2TRAddressBytes(U8x32);

impl fmt::Display for P2TRAddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(1).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{}", address)
    }
}

impl Serialize for P2TRAddressBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2TRAddressBytes> for Addressbytes {
    fn from(value: P2TRAddressBytes) -> Self {
        Self::P2TR(value)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x20([u8; 20]);
impl From<&[u8]> for U8x20 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 20];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x32([u8; 32]);
impl From<&[u8]> for U8x32 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 32];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x33(#[serde(with = "serde_bytes")] [u8; 33]);
impl From<&[u8]> for U8x33 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 33];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x64(#[serde(with = "serde_bytes")] [u8; 64]);
impl From<&[u8]> for U8x64 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 64];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x65(#[serde(with = "serde_bytes")] [u8; 65]);
impl From<&[u8]> for U8x65 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 65];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}
