use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
