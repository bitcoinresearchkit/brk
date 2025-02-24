use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Cents;

pub type OHLCCents = (Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>);

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
