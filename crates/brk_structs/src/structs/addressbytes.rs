use std::fmt;

use bitcoin::{
    Address, Network, ScriptBuf,
    hex::{Case, DisplayHex},
    opcodes,
    script::Builder,
};
use brk_error::Error;
use derive_deref::{Deref, DerefMut};
use serde::{Serialize, Serializer};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::OutputType;

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

impl fmt::Display for AddressBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AddressBytes::P2PK65(bytes) => bytes.to_string(),
                AddressBytes::P2PK33(bytes) => bytes.to_string(),
                AddressBytes::P2PKH(bytes) => bytes.to_string(),
                AddressBytes::P2SH(bytes) => bytes.to_string(),
                AddressBytes::P2WPKH(bytes) => bytes.to_string(),
                AddressBytes::P2WSH(bytes) => bytes.to_string(),
                AddressBytes::P2TR(bytes) => bytes.to_string(),
                AddressBytes::P2A(bytes) => bytes.to_string(),
            }
        )
    }
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

impl From<&Address> for AddressBytes {
    fn from(value: &Address) -> Self {
        Self::try_from((&value.script_pubkey(), OutputType::from(value))).unwrap()
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
                Ok(Self::P2PK65(P2PK65Bytes(U8x65::from(bytes))))
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
                Ok(Self::P2PK33(P2PK33Bytes(U8x33::from(bytes))))
            }
            OutputType::P2PKH => {
                let bytes = &script.as_bytes()[3..23];
                Ok(Self::P2PKH(P2PKHBytes(U8x20::from(bytes))))
            }
            OutputType::P2SH => {
                let bytes = &script.as_bytes()[2..22];
                Ok(Self::P2SH(P2SHBytes(U8x20::from(bytes))))
            }
            OutputType::P2WPKH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WPKH(P2WPKHBytes(U8x20::from(bytes))))
            }
            OutputType::P2WSH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2WSH(P2WSHBytes(U8x32::from(bytes))))
            }
            OutputType::P2TR => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2TR(P2TRBytes(U8x32::from(bytes))))
            }
            OutputType::P2A => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self::P2A(P2ABytes(U8x2::from(bytes))))
            }
            OutputType::P2MS => Err(Error::WrongAddressType),
            OutputType::Unknown => Err(Error::WrongAddressType),
            OutputType::Empty => Err(Error::WrongAddressType),
            OutputType::OpReturn => Err(Error::WrongAddressType),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2PK65Bytes(U8x65);

impl fmt::Display for P2PK65Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string(Case::Lower))
    }
}

impl Serialize for P2PK65Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2PK65Bytes> for AddressBytes {
    fn from(value: P2PK65Bytes) -> Self {
        Self::P2PK65(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2PK33Bytes(U8x33);

impl fmt::Display for P2PK33Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string(Case::Lower))
    }
}

impl Serialize for P2PK33Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2PK33Bytes> for AddressBytes {
    fn from(value: P2PK33Bytes) -> Self {
        Self::P2PK33(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2PKHBytes(U8x20);

impl fmt::Display for P2PKHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new()
            .push_opcode(opcodes::all::OP_DUP)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(*self.0)
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{address}")
    }
}

impl Serialize for P2PKHBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2PKHBytes> for AddressBytes {
    fn from(value: P2PKHBytes) -> Self {
        Self::P2PKH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2SHBytes(U8x20);

impl fmt::Display for P2SHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new()
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(*self.0)
            .push_opcode(opcodes::all::OP_EQUAL)
            .into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{address}")
    }
}

impl Serialize for P2SHBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2SHBytes> for AddressBytes {
    fn from(value: P2SHBytes) -> Self {
        Self::P2SH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2WPKHBytes(U8x20);

impl fmt::Display for P2WPKHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(0).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{address}")
    }
}

impl Serialize for P2WPKHBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2WPKHBytes> for AddressBytes {
    fn from(value: P2WPKHBytes) -> Self {
        Self::P2WPKH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2WSHBytes(U8x32);

impl fmt::Display for P2WSHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(0).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{address}")
    }
}

impl Serialize for P2WSHBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2WSHBytes> for AddressBytes {
    fn from(value: P2WSHBytes) -> Self {
        Self::P2WSH(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2TRBytes(U8x32);

impl fmt::Display for P2TRBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(1).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{address}")
    }
}

impl Serialize for P2TRBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2TRBytes> for AddressBytes {
    fn from(value: P2TRBytes) -> Self {
        Self::P2TR(value)
    }
}

#[derive(Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct P2ABytes(U8x2);

impl fmt::Display for P2ABytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script = Builder::new().push_int(1).push_slice(*self.0).into_script();
        let address = Address::from_script(&script, Network::Bitcoin).unwrap();
        write!(f, "{address}")
    }
}

impl Serialize for P2ABytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl From<P2ABytes> for AddressBytes {
    fn from(value: P2ABytes) -> Self {
        Self::P2A(value)
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
pub struct U8x2([u8; 2]);
impl From<&[u8]> for U8x2 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 2];
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
