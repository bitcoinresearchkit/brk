use std::ops::Add;

use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable, TypeIndex};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct AnyAddressDataIndex(TypeIndex);
// impl From<TypeIndex> for AddressDataIndex {
//     fn from(value: TypeIndex) -> Self {
//         Self(value)
//     }
// }
// impl From<AddressDataIndex> for TypeIndex {
//     fn from(value: AddressDataIndex) -> Self {
//         value.0
//     }
// }
// impl From<AddressDataIndex> for u32 {
//     fn from(value: AddressDataIndex) -> Self {
//         Self::from(*value)
//     }
// }
// impl From<u32> for AddressDataIndex {
//     fn from(value: u32) -> Self {
//         Self(TypeIndex::from(value))
//     }
// }
// impl From<AddressDataIndex> for usize {
//     fn from(value: AddressDataIndex) -> Self {
//         Self::from(*value)
//     }
// }
// impl From<usize> for AddressDataIndex {
//     fn from(value: usize) -> Self {
//         Self(TypeIndex::from(value))
//     }
// }
// impl Add<usize> for AddressDataIndex {
//     type Output = Self;
//     fn add(self, rhs: usize) -> Self::Output {
//         Self(*self + rhs)
//     }
// }
// impl CheckedSub<AddressDataIndex> for AddressDataIndex {
//     fn checked_sub(self, rhs: Self) -> Option<Self> {
//         self.0.checked_sub(rhs.0).map(Self)
//     }
// }

// impl Printable for AddressDataIndex {
//     fn to_string() -> &'static str {
//         "p2pk33addressindex"
//     }

//     fn to_possible_strings() -> &'static [&'static str] {
//         &["addr", "p2pk33addr", "p2pk33addressindex"]
//     }
// }

// impl From<ByteView> for AddressDataIndex {
//     fn from(value: ByteView) -> Self {
//         Self::read_from_bytes(&value).unwrap()
//     }
// }
// impl From<AddressDataIndex> for ByteView {
//     fn from(value: AddressDataIndex) -> Self {
//         Self::from(&value)
//     }
// }
// impl From<&AddressDataIndex> for ByteView {
//     fn from(value: &AddressDataIndex) -> Self {
//         Self::new(value.as_bytes())
//     }
// }
