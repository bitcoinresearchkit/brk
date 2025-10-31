use std::{fmt, mem};

use bitcoin::hashes::Hash;
use derive_deref::Deref;
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Transaction ID (hash)
#[derive(
    Debug,
    Deref,
    Clone,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    JsonSchema,
    Hash,
)]
#[schemars(
    example = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
    example = "2bb85f4b004be6da54f766c17c1e855187327112c231ef2ff35ebad0ea67c69e",
    example = "9a0b3b8305bb30cacf9e8443a90d53a76379fb3305047fdeaa4e4b0934a2a1ba"
)]
#[repr(C)]
pub struct Txid([u8; 32]);

impl From<bitcoin::Txid> for Txid {
    #[inline]
    fn from(value: bitcoin::Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&bitcoin::Txid> for &Txid {
    #[inline]
    fn from(value: &bitcoin::Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<Txid> for bitcoin::Txid {
    #[inline]
    fn from(value: Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&Txid> for bitcoin::Txid {
    #[inline]
    fn from(value: &Txid) -> Self {
        bitcoin::Txid::from_slice(&value.0).unwrap()
    }
}

impl From<&Txid> for &bitcoin::Txid {
    #[inline]
    fn from(value: &Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl fmt::Display for Txid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bitcoin::Txid::from(self).to_string())
    }
}

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
