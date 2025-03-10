use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Addresstypeindex(u32);

impl Addresstypeindex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn copy_then_increment(&mut self) -> Self {
        let i = *self;
        self.increment();
        i
    }
}

impl From<u32> for Addresstypeindex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Addresstypeindex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<Addresstypeindex> for u64 {
    fn from(value: Addresstypeindex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for Addresstypeindex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Addresstypeindex> for usize {
    fn from(value: Addresstypeindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Addresstypeindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl Add<Addresstypeindex> for Addresstypeindex {
    type Output = Self;
    fn add(self, rhs: Addresstypeindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

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
pub struct Emptyindex(Addresstypeindex);
impl From<Addresstypeindex> for Emptyindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<Emptyindex> for usize {
    fn from(value: Emptyindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for Emptyindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for Emptyindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct Multisigindex(Addresstypeindex);
impl From<Addresstypeindex> for Multisigindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<Multisigindex> for usize {
    fn from(value: Multisigindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for Multisigindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for Multisigindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct Opreturnindex(Addresstypeindex);
impl From<Addresstypeindex> for Opreturnindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<Opreturnindex> for usize {
    fn from(value: Opreturnindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for Opreturnindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for Opreturnindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct Pushonlyindex(Addresstypeindex);
impl From<Addresstypeindex> for Pushonlyindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<Pushonlyindex> for usize {
    fn from(value: Pushonlyindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for Pushonlyindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for Pushonlyindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct Unknownindex(Addresstypeindex);
impl From<Addresstypeindex> for Unknownindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<Unknownindex> for usize {
    fn from(value: Unknownindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for Unknownindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for Unknownindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2PK33index(Addresstypeindex);
impl From<Addresstypeindex> for P2PK33index {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2PK33index> for usize {
    fn from(value: P2PK33index) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PK33index {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2PK33index {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2PK65index(Addresstypeindex);
impl From<Addresstypeindex> for P2PK65index {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2PK65index> for usize {
    fn from(value: P2PK65index) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PK65index {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2PK65index {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2PKHindex(Addresstypeindex);
impl From<Addresstypeindex> for P2PKHindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2PKHindex> for usize {
    fn from(value: P2PKHindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PKHindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2PKHindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2SHindex(Addresstypeindex);
impl From<Addresstypeindex> for P2SHindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2SHindex> for usize {
    fn from(value: P2SHindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2SHindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2SHindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2TRindex(Addresstypeindex);
impl From<Addresstypeindex> for P2TRindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2TRindex> for usize {
    fn from(value: P2TRindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2TRindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2TRindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2WPKHindex(Addresstypeindex);
impl From<Addresstypeindex> for P2WPKHindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2WPKHindex> for usize {
    fn from(value: P2WPKHindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2WPKHindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2WPKHindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

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
pub struct P2WSHindex(Addresstypeindex);
impl From<Addresstypeindex> for P2WSHindex {
    fn from(value: Addresstypeindex) -> Self {
        Self(value)
    }
}
impl From<P2WSHindex> for usize {
    fn from(value: P2WSHindex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2WSHindex {
    fn from(value: usize) -> Self {
        Self(Addresstypeindex::from(value))
    }
}
impl Add<usize> for P2WSHindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
