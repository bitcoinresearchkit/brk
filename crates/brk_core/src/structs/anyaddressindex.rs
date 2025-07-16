use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{
    TypeIndex,
    structs::{EmptyAddressIndex, LoadedAddressIndex},
};

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

impl Serialize for AnyAddressIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_enum().serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
pub enum AnyAddressDataIndexEnum {
    Loaded(LoadedAddressIndex),
    Empty(EmptyAddressIndex),
}

impl From<AnyAddressIndex> for AnyAddressDataIndexEnum {
    fn from(value: AnyAddressIndex) -> Self {
        let uvalue = u32::from(value.0);
        if uvalue >= MIN_EMPTY_INDEX {
            Self::Empty(EmptyAddressIndex::from(TypeIndex::from(
                uvalue - MIN_EMPTY_INDEX,
            )))
        } else {
            Self::Loaded(LoadedAddressIndex::from(value.0))
        }
    }
}
