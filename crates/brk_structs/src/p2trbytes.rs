use std::fmt;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::U8x32;

#[derive(
    Debug, Clone, Deref, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize,
)]
pub struct P2TRBytes(U8x32);

impl From<&[u8]> for P2TRBytes {
    fn from(value: &[u8]) -> Self {
        Self(U8x32::from(value))
    }
}

impl From<U8x32> for P2TRBytes {
    fn from(value: U8x32) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2TRBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
