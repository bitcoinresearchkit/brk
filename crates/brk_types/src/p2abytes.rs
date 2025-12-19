use std::fmt;

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Bytes, Formattable};

use crate::U8x2;

#[derive(Debug, Clone, Deref, PartialEq, Eq, PartialOrd, Ord, Serialize, Bytes, Hash, JsonSchema)]
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

impl Formattable for P2ABytes {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}
