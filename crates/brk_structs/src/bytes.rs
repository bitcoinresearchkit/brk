use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x2([u8; 2]);
impl From<&[u8]> for U8x2 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 2];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x20([u8; 20]);
impl From<&[u8]> for U8x20 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 20];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x32([u8; 32]);
impl From<&[u8]> for U8x32 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 32];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x33(#[serde(with = "serde_bytes")] [u8; 33]);
impl From<&[u8]> for U8x33 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 33];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct U8x65(#[serde(with = "serde_bytes")] [u8; 65]);
impl From<&[u8]> for U8x65 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 65];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}
