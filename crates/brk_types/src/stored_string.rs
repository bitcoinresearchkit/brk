use std::borrow::Cow;

use byteview::ByteView;
use derive_deref::Deref;
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
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Cow<'_, str>> for StoredString {
    fn from(value: Cow<'_, str>) -> Self {
        Self(value.to_string())
    }
}

impl From<ByteView> for StoredString {
    fn from(value: ByteView) -> Self {
        let bytes = &*value;
        Self(String::from_utf8_lossy(bytes).into_owned())
    }
}

impl From<StoredString> for ByteView {
    fn from(value: StoredString) -> Self {
        Self::from(&value)
    }
}

impl From<&StoredString> for ByteView {
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
