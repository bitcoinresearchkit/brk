use std::fmt;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::U8x2;

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
pub struct P2ABytes(U8x2);

impl From<&[u8]> for P2ABytes {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(U8x2::from(value))
    }
}

impl From<U8x2> for P2ABytes {
    #[inline]
    fn from(value: U8x2) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2ABytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
