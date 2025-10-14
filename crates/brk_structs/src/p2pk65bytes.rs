use std::fmt;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::U8x65;

#[derive(
    Debug,
    Clone,
    Deref,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
    Hash,
)]
pub struct P2PK65Bytes(U8x65);

impl From<&[u8]> for P2PK65Bytes {
    fn from(value: &[u8]) -> Self {
        Self(U8x65::from(value))
    }
}

impl From<U8x65> for P2PK65Bytes {
    fn from(value: U8x65) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2PK65Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
