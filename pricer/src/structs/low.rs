use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Deref, Serialize)]
#[repr(C)]
pub struct Low<T>(T);
impl<T> From<T> for Low<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
