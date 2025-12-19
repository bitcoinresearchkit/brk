use std::ops::Add;

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use crate::TypeIndex;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, Default, Serialize, Pco, JsonSchema,
)]
pub struct LoadedAddressIndex(TypeIndex);

impl From<TypeIndex> for LoadedAddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<usize> for LoadedAddressIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<LoadedAddressIndex> for usize {
    #[inline]
    fn from(value: LoadedAddressIndex) -> Self {
        usize::from(value.0)
    }
}
impl From<LoadedAddressIndex> for u32 {
    #[inline]
    fn from(value: LoadedAddressIndex) -> Self {
        u32::from(value.0)
    }
}
impl Add<usize> for LoadedAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl CheckedSub<LoadedAddressIndex> for LoadedAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
impl PrintableIndex for LoadedAddressIndex {
    fn to_string() -> &'static str {
        "loadedaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["loadedaddr", "loadedaddressindex"]
    }
}

impl std::fmt::Display for LoadedAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for LoadedAddressIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
