use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::{EmptyAddressIndex, FundedAddressIndex, TypeIndex};

const MIN_EMPTY_INDEX: u32 = u32::MAX - 4_000_000_000;

/// Unified index for any address type (funded or empty)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Bytes, JsonSchema)]
pub struct AnyAddressIndex(TypeIndex);

impl AnyAddressIndex {
    pub fn to_enum(&self) -> AnyAddressDataIndexEnum {
        AnyAddressDataIndexEnum::from(*self)
    }
}

impl From<FundedAddressIndex> for AnyAddressIndex {
    #[inline]
    fn from(value: FundedAddressIndex) -> Self {
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
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnyAddressDataIndexEnum {
    Funded(FundedAddressIndex),
    Empty(EmptyAddressIndex),
}

impl From<AnyAddressIndex> for AnyAddressDataIndexEnum {
    #[inline]
    fn from(value: AnyAddressIndex) -> Self {
        let uvalue = u32::from(value.0);
        if uvalue >= MIN_EMPTY_INDEX {
            Self::Empty(EmptyAddressIndex::from(uvalue - MIN_EMPTY_INDEX))
        } else {
            Self::Funded(FundedAddressIndex::from(value.0))
        }
    }
}

impl From<AnyAddressDataIndexEnum> for AnyAddressIndex {
    #[inline]
    fn from(value: AnyAddressDataIndexEnum) -> Self {
        match value {
            AnyAddressDataIndexEnum::Funded(idx) => Self::from(idx),
            AnyAddressDataIndexEnum::Empty(idx) => Self::from(idx),
        }
    }
}
