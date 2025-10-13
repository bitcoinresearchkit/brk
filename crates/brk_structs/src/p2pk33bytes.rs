use std::fmt;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::U8x33;

#[derive(
    Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize,
)]
pub struct P2PK33Bytes(U8x33);

impl From<&[u8]> for P2PK33Bytes {
    fn from(value: &[u8]) -> Self {
        Self(U8x33::from(value))
    }
}

impl From<U8x33> for P2PK33Bytes {
    fn from(value: U8x33) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2PK33Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
