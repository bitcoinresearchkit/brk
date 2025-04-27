use std::ops::Add;

use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Error};

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
pub struct OutputTypeIndex(u32);

impl OutputTypeIndex {
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

impl From<u32> for OutputTypeIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for OutputTypeIndex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<OutputTypeIndex> for u64 {
    fn from(value: OutputTypeIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for OutputTypeIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<OutputTypeIndex> for usize {
    fn from(value: OutputTypeIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for OutputTypeIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl Add<OutputTypeIndex> for OutputTypeIndex {
    type Output = Self;
    fn add(self, rhs: OutputTypeIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl TryFrom<ByteView> for OutputTypeIndex {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<OutputTypeIndex> for ByteView {
    fn from(value: OutputTypeIndex) -> Self {
        Self::new(value.as_bytes())
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
pub struct EmptyOutputIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for EmptyOutputIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<EmptyOutputIndex> for usize {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for EmptyOutputIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for EmptyOutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<EmptyOutputIndex> for EmptyOutputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2MSIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2MSIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2MSIndex> for usize {
    fn from(value: P2MSIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2MSIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2MSIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2MSIndex> for P2MSIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2AIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2AIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2AIndex> for usize {
    fn from(value: P2AIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2AIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2AIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2AIndex> for P2AIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct OpReturnIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for OpReturnIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<OpReturnIndex> for usize {
    fn from(value: OpReturnIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for OpReturnIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for OpReturnIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<OpReturnIndex> for OpReturnIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct UnknownOutputIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for UnknownOutputIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<UnknownOutputIndex> for usize {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for UnknownOutputIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for UnknownOutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<UnknownOutputIndex> for UnknownOutputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2PK33Index(OutputTypeIndex);
impl From<OutputTypeIndex> for P2PK33Index {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2PK33Index> for usize {
    fn from(value: P2PK33Index) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PK33Index {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2PK33Index {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2PK33Index> for P2PK33Index {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2PK65Index(OutputTypeIndex);
impl From<OutputTypeIndex> for P2PK65Index {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2PK65Index> for usize {
    fn from(value: P2PK65Index) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PK65Index {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2PK65Index {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2PK65Index> for P2PK65Index {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2PKHIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2PKHIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2PKHIndex> for usize {
    fn from(value: P2PKHIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PKHIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2PKHIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2PKHIndex> for P2PKHIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2SHIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2SHIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2SHIndex> for usize {
    fn from(value: P2SHIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2SHIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2SHIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2SHIndex> for P2SHIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2TRIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2TRIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2TRIndex> for usize {
    fn from(value: P2TRIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2TRIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2TRIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2TRIndex> for P2TRIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2WPKHIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2WPKHIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2WPKHIndex> for usize {
    fn from(value: P2WPKHIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2WPKHIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2WPKHIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2WPKHIndex> for P2WPKHIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
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
pub struct P2WSHIndex(OutputTypeIndex);
impl From<OutputTypeIndex> for P2WSHIndex {
    fn from(value: OutputTypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2WSHIndex> for usize {
    fn from(value: P2WSHIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2WSHIndex {
    fn from(value: usize) -> Self {
        Self(OutputTypeIndex::from(value))
    }
}
impl Add<usize> for P2WSHIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2WSHIndex> for P2WSHIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.0.checked_sub(rhs.0.0).map(OutputTypeIndex).map(Self)
    }
}
