use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Close;

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
