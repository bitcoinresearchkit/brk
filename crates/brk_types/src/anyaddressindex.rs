use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{EmptyAddressIndex, LoadedAddressIndex, TypeIndex};

const MIN_EMPTY_INDEX: u32 = u32::MAX - 4_000_000_000;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout,
)]
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

impl std::fmt::Display for AnyAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Serialize)]
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

// impl std::fmt::Display for AnyAddressDataIndexEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {

//         }
//         self.0.fmt(f)
//     }
// }
