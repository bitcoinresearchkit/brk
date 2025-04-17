use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Dollars;

#[derive(
    Debug,
    Default,
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
)]
pub struct Cents(u64);

impl From<Dollars> for Cents {
    fn from(value: Dollars) -> Self {
        Self((*value * 100.0).round() as u64)
    }
}

impl From<Cents> for f64 {
    fn from(value: Cents) -> Self {
        value.0 as f64
    }
}

impl From<u64> for Cents {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Cents> for u64 {
    fn from(value: Cents) -> Self {
        value.0
    }
}
