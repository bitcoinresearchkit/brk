use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Close;

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
