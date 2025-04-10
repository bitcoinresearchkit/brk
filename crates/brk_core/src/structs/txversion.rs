use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::StoredU8;

#[derive(Debug, Deref, Clone, Copy, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct TxVersion(i32);

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
