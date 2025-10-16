use allocative::Allocative;
use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_2bytes;

/// Index of the output being spent in the previous transaction
#[derive(
    Debug,
    Default,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    Allocative,
    JsonSchema,
)]
pub struct Vout(u16);

impl Vout {
    pub const ZERO: Self = Vout(0);
    pub const MAX: Self = Vout(u16::MAX);

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

const U16_MAX_AS_U32: u32 = u16::MAX as u32;
impl From<u32> for Vout {
    fn from(value: u32) -> Self {
        if value > U16_MAX_AS_U32 {
            panic!()
        }
        Self(value as u16)
    }
}

const U16_MAX_AS_USIZE: usize = u16::MAX as usize;
impl From<usize> for Vout {
    fn from(value: usize) -> Self {
        if value > U16_MAX_AS_USIZE {
            panic!()
        }
        Self(value as u16)
    }
}

impl From<Vout> for u64 {
    fn from(value: Vout) -> Self {
        value.0 as u64
    }
}

impl From<&[u8]> for Vout {
    fn from(value: &[u8]) -> Self {
        Self(u16::from_be_bytes(copy_first_2bytes(value).unwrap()))
    }
}

impl std::fmt::Display for Vout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
