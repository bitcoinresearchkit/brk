use derive_deref::Deref;
use serde::Serialize;
use vecdb::{Printable, StoredCompressed};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    Deref,
    Clone,
    Default,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    StoredCompressed,
)]
pub struct StoredBool(u16);

impl StoredBool {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);

    pub fn is_true(&self) -> bool {
        *self == Self::TRUE
    }

    pub fn is_false(&self) -> bool {
        *self == Self::FALSE
    }
}

impl From<bool> for StoredBool {
    fn from(value: bool) -> Self {
        if value { Self(1) } else { Self(0) }
    }
}

impl From<StoredBool> for usize {
    fn from(value: StoredBool) -> Self {
        value.0 as usize
    }
}

impl Printable for StoredBool {
    fn to_string() -> &'static str {
        "bool"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["bool"]
    }
}
