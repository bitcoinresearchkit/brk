use derive_deref::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::Bytes;

#[derive(
    Debug,
    Clone,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Bytes,
    Hash,
    JsonSchema,
)]
pub struct U8x2([u8; 2]);
impl From<&[u8]> for U8x2 {
    #[inline]
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
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Bytes,
    Hash,
    JsonSchema,
)]
pub struct U8x20([u8; 20]);
impl From<&[u8]> for U8x20 {
    #[inline]
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
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Bytes,
    Hash,
    JsonSchema,
)]
pub struct U8x32([u8; 32]);
impl From<&[u8]> for U8x32 {
    #[inline]
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
    PartialOrd,
    Ord,
    Bytes,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct U8x33(#[serde(with = "serde_bytes")] [u8; 33]);

impl JsonSchema for U8x33 {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "U8x33".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Represent as a byte string
        String::json_schema(_gen)
    }
}

impl From<&[u8]> for U8x33 {
    #[inline]
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
    PartialOrd,
    Ord,
    Bytes,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct U8x65(#[serde(with = "serde_bytes")] [u8; 65]);

impl JsonSchema for U8x65 {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "U8x65".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Represent as a byte string
        String::json_schema(_gen)
    }
}

impl From<&[u8]> for U8x65 {
    #[inline]
    fn from(slice: &[u8]) -> Self {
        let mut arr = [0; 65];
        arr.copy_from_slice(slice);
        Self(arr)
    }
}
