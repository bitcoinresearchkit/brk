use std::fmt;

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::U8x20;

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
pub struct P2PKHBytes(U8x20);

impl From<&[u8]> for P2PKHBytes {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(U8x20::from(value))
    }
}

impl From<U8x20> for P2PKHBytes {
    #[inline]
    fn from(value: U8x20) -> Self {
        Self(value)
    }
}

impl fmt::Display for P2PKHBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Formattable for P2PKHBytes {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}
