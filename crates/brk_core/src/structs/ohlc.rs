use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Cents, Dollars};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
#[repr(C)]
pub struct OHLCCents {
    pub open: Open<Cents>,
    pub high: High<Cents>,
    pub low: Low<Cents>,
    pub close: Close<Cents>,
}

impl From<(Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>)> for OHLCCents {
    fn from(value: (Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
        }
    }
}

impl From<Close<Cents>> for OHLCCents {
    fn from(value: Close<Cents>) -> Self {
        Self {
            open: Open::from(value),
            high: High::from(value),
            low: Low::from(value),
            close: value,
        }
    }
}

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
#[repr(C)]
pub struct OHLCDollars {
    pub open: Open<Dollars>,
    pub high: High<Dollars>,
    pub low: Low<Dollars>,
    pub close: Close<Dollars>,
}

impl From<(Open<Dollars>, High<Dollars>, Low<Dollars>, Close<Dollars>)> for OHLCDollars {
    fn from(value: (Open<Dollars>, High<Dollars>, Low<Dollars>, Close<Dollars>)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
        }
    }
}

impl From<Close<Dollars>> for OHLCDollars {
    fn from(value: Close<Dollars>) -> Self {
        Self {
            open: Open::from(value),
            high: High::from(value),
            low: Low::from(value),
            close: value,
        }
    }
}

impl From<OHLCCents> for OHLCDollars {
    fn from(value: OHLCCents) -> Self {
        Self::from(&value)
    }
}

impl From<&OHLCCents> for OHLCDollars {
    fn from(value: &OHLCCents) -> Self {
        Self {
            open: value.open.into(),
            high: value.high.into(),
            low: value.low.into(),
            close: value.close.into(),
        }
    }
}

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
    Deref,
    Serialize,
)]
#[repr(C)]
pub struct Open<T>(T);
impl<T> From<T> for Open<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<Close<T>> for Open<T>
where
    T: Copy,
{
    fn from(value: Close<T>) -> Self {
        Self(*value)
    }
}

impl From<Open<Cents>> for Open<Dollars> {
    fn from(value: Open<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Deref,
    Serialize,
)]
#[repr(C)]
pub struct High<T>(T);
impl<T> From<T> for High<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<Close<T>> for High<T>
where
    T: Copy,
{
    fn from(value: Close<T>) -> Self {
        Self(*value)
    }
}

impl From<High<Cents>> for High<Dollars> {
    fn from(value: High<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

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
    Deref,
    Serialize,
)]
#[repr(C)]
pub struct Low<T>(T);
impl<T> From<T> for Low<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<Close<T>> for Low<T>
where
    T: Copy,
{
    fn from(value: Close<T>) -> Self {
        Self(*value)
    }
}

impl From<Low<Cents>> for Low<Dollars> {
    fn from(value: Low<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

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
    Deref,
    Serialize,
)]
#[repr(C)]
pub struct Close<T>(T);
impl<T> From<T> for Close<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl From<Close<Cents>> for Close<Dollars> {
    fn from(value: Close<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}
