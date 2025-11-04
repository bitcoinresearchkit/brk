use std::{borrow::Cow, str};

use byteview::ByteView;
use derive_deref::Deref;
use redb::{TypeName, Value};
use serde::Serialize;
use vecdb::PrintableIndex;

#[derive(Default, Debug, Deref, Clone, Serialize)]
pub struct StoredString(String);

impl StoredString {
    pub fn new(string: String) -> Self {
        Self(string)
    }

    pub fn as_str(&self) -> &str {
        self
    }

    pub fn as_string(&self) -> &String {
        self
    }
}

impl From<String> for StoredString {
    #[inline]
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Cow<'_, str>> for StoredString {
    #[inline]
    fn from(value: Cow<'_, str>) -> Self {
        Self(value.to_string())
    }
}

impl From<ByteView> for StoredString {
    #[inline]
    fn from(value: ByteView) -> Self {
        let bytes = &*value;
        Self(String::from_utf8_lossy(bytes).into_owned())
    }
}

impl From<StoredString> for ByteView {
    #[inline]
    fn from(value: StoredString) -> Self {
        Self::from(&value)
    }
}

impl From<&StoredString> for ByteView {
    #[inline]
    fn from(value: &StoredString) -> Self {
        Self::new(value.as_bytes())
    }
}

impl PrintableIndex for StoredString {
    fn to_string() -> &'static str {
        "string"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["string"]
    }
}

impl Value for StoredString {
    type SelfType<'a>
        = StoredString
    where
        Self: 'a;
    type AsBytes<'a>
        = &'a str
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> StoredString
    where
        Self: 'a,
    {
        StoredString(str::from_utf8(data).unwrap().to_string())
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> &'a str
    where
        Self: 'b,
    {
        value.as_str()
    }

    fn type_name() -> TypeName {
        TypeName::new("StoredString")
    }
}
