use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Deref, Clone, Copy, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct Weight(u64);

impl From<bitcoin::Weight> for Weight {
    fn from(value: bitcoin::Weight) -> Self {
        Self(value.to_wu())
    }
}

impl From<Weight> for bitcoin::Weight {
    fn from(value: Weight) -> Self {
        Self::from_wu(*value)
    }
}
