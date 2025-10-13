use std::fmt;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::U8x20;

#[derive(
    Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize,
)]
pub struct P2PKHBytes(U8x20);

impl From<&[u8]> for P2PKHBytes {
    fn from(value: &[u8]) -> Self {
        Self(U8x20::from(value))
    }
}

impl From<U8x20> for P2PKHBytes {
    fn from(value: U8x20) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2PKHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
