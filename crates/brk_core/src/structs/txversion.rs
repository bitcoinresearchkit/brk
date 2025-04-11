use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::StoredU8;

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
)]
pub struct TxVersion(i32);

impl TxVersion {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
}

impl From<bitcoin::transaction::Version> for TxVersion {
    fn from(value: bitcoin::transaction::Version) -> Self {
        Self(value.0)
    }
}

impl From<TxVersion> for bitcoin::transaction::Version {
    fn from(value: TxVersion) -> Self {
        Self(value.0)
    }
}

impl From<TxVersion> for StoredU8 {
    fn from(value: TxVersion) -> Self {
        Self::from(value.0 as u8)
    }
}
