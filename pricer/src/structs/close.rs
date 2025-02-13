use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Deref, Serialize)]
#[repr(C)]
pub struct Close<T>(T);
impl<T> From<T> for Close<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
