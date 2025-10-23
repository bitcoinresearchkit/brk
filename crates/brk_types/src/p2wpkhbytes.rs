use std::fmt;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::U8x20;

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
pub struct P2WPKHBytes(U8x20);

impl From<&[u8]> for P2WPKHBytes {
    fn from(value: &[u8]) -> Self {
        Self(U8x20::from(value))
    }
}

impl From<U8x20> for P2WPKHBytes {
    fn from(value: U8x20) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2WPKHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
