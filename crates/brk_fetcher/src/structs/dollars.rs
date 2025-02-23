use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Cents;

#[derive(Debug, Default, Clone, Copy, Deref, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
pub struct Dollars(f64);

impl From<f64> for Dollars {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Cents> for Dollars {
    fn from(value: Cents) -> Self {
        Self((*value as f64) / 100.0)
    }
}
