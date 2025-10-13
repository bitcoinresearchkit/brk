use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::StoredCompressed;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::StoredU16;

/// Transaction version number
#[derive(
    Debug,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
    StoredCompressed,
    JsonSchema,
)]
pub struct TxVersion(u16);

impl TxVersion {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const NON_STANDARD: Self = Self(u16::MAX);
}

impl From<bitcoin::transaction::Version> for TxVersion {
    fn from(value: bitcoin::transaction::Version) -> Self {
        match value.0 {
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            _ => Self::NON_STANDARD,
        }
    }
}

impl From<TxVersion> for bitcoin::transaction::Version {
    fn from(value: TxVersion) -> Self {
        Self(value.0 as i32)
    }
}

impl From<TxVersion> for StoredU16 {
    fn from(value: TxVersion) -> Self {
        Self::from(value.0)
    }
}

impl std::fmt::Display for TxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
