use std::{fmt, mem};

use bitcoin::hashes::Hash;
use derive_deref::Deref;
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug, Deref, Clone, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, JsonSchema,
)]
pub struct Txid([u8; 32]);

impl From<bitcoin::Txid> for Txid {
    fn from(value: bitcoin::Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<Txid> for bitcoin::Txid {
    fn from(value: Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&Txid> for bitcoin::Txid {
    fn from(value: &Txid) -> Self {
        bitcoin::Txid::from_slice(&value.0).unwrap()
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
