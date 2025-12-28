use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::{EmptyAddressIndex, LoadedAddressIndex, TypeIndex};

const MIN_EMPTY_INDEX: u32 = u32::MAX - 4_000_000_000;

/// Unified index for any address type (loaded or empty)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Bytes, JsonSchema)]
pub struct AnyAddressIndex(TypeIndex);

impl AnyAddressIndex {
    pub fn to_enum(&self) -> AnyAddressDataIndexEnum {
        AnyAddressDataIndexEnum::from(*self)
    }
}

impl From<LoadedAddressIndex> for AnyAddressIndex {
    #[inline]
    fn from(value: LoadedAddressIndex) -> Self {
        if u32::from(value) >= MIN_EMPTY_INDEX {
            panic!("{value} is higher than MIN_EMPTY_INDEX ({MIN_EMPTY_INDEX})")
        }
        Self(*value)
    }
}

impl From<EmptyAddressIndex> for AnyAddressIndex {
    #[inline]
    fn from(value: EmptyAddressIndex) -> Self {
        Self(*value + MIN_EMPTY_INDEX)
    }
}

impl Serialize for AnyAddressIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_enum().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnyAddressIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = AnyAddressDataIndexEnum::deserialize(deserializer)?;
        Ok(Self::from(variant))
    }
}

impl fmt::Display for AnyAddressIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for AnyAddressIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnyAddressDataIndexEnum {
    Loaded(LoadedAddressIndex),
    Empty(EmptyAddressIndex),
}

impl From<AnyAddressIndex> for AnyAddressDataIndexEnum {
    #[inline]
    fn from(value: AnyAddressIndex) -> Self {
        let uvalue = u32::from(value.0);
        if uvalue >= MIN_EMPTY_INDEX {
            Self::Empty(EmptyAddressIndex::from(uvalue - MIN_EMPTY_INDEX))
        } else {
            Self::Loaded(LoadedAddressIndex::from(value.0))
        }
    }
}

impl From<AnyAddressDataIndexEnum> for AnyAddressIndex {
    #[inline]
    fn from(value: AnyAddressDataIndexEnum) -> Self {
        match value {
            AnyAddressDataIndexEnum::Loaded(idx) => Self::from(idx),
            AnyAddressDataIndexEnum::Empty(idx) => Self::from(idx),
        }
    }
}
