use std::fmt;

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::U8x32;

#[derive(
    Debug,
    Clone,
    Deref,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Bytes,
    Hash,
    JsonSchema,
)]
pub struct P2TRBytes(U8x32);

impl From<&[u8]> for P2TRBytes {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(U8x32::from(value))
    }
}

impl From<U8x32> for P2TRBytes {
    #[inline]
    fn from(value: U8x32) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2TRBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Formattable for P2TRBytes {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}
