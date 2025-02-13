use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Dollars;

#[derive(Debug, Default, Clone, Copy, Deref, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
pub struct Cents(u64);

impl From<Dollars> for Cents {
    fn from(value: Dollars) -> Self {
        Self((*value * 100.0).floor() as u64)
    }
}
