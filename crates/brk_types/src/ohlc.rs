use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div},
};

use allocative::Allocative;
use derive_deref::{Deref, DerefMut};
use serde::{Serialize, Serializer, ser::SerializeTuple};
use vecdb::StoredCompressed;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::StoredF64;

use super::{Cents, Dollars, Sats};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct OHLCCents {
    pub open: Open<Cents>,
    pub high: High<Cents>,
    pub low: Low<Cents>,
    pub close: Close<Cents>,
}

impl From<(Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>)> for OHLCCents {
    #[inline]
    fn from(value: (Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
        }
    }
}

impl From<Close<Cents>> for OHLCCents {
    #[inline]
    fn from(value: Close<Cents>) -> Self {
        Self {
            open: Open::from(value),
            high: High::from(value),
            low: Low::from(value),
            close: value,
        }
    }
}

impl Serialize for OHLCCents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(4)?;
        tup.serialize_element(&self.open)?;
        tup.serialize_element(&self.high)?;
        tup.serialize_element(&self.low)?;
        tup.serialize_element(&self.close)?;
        tup.end()
    }
}

impl std::fmt::Display for OHLCCents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.open, self.high, self.low, self.close
        )
    }
}

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct OHLCDollars {
    pub open: Open<Dollars>,
    pub high: High<Dollars>,
    pub low: Low<Dollars>,
    pub close: Close<Dollars>,
}

impl Serialize for OHLCDollars {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(4)?;
        tup.serialize_element(&self.open)?;
        tup.serialize_element(&self.high)?;
        tup.serialize_element(&self.low)?;
        tup.serialize_element(&self.close)?;
        tup.end()
    }
}

impl From<(Open<Dollars>, High<Dollars>, Low<Dollars>, Close<Dollars>)> for OHLCDollars {
    #[inline]
    fn from(value: (Open<Dollars>, High<Dollars>, Low<Dollars>, Close<Dollars>)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
        }
    }
}

impl From<Close<Dollars>> for OHLCDollars {
    #[inline]
    fn from(value: Close<Dollars>) -> Self {
        Self {
            open: Open::from(value),
            high: High::from(value),
            low: Low::from(value),
            close: value,
        }
    }
}

impl From<OHLCCents> for OHLCDollars {
    #[inline]
    fn from(value: OHLCCents) -> Self {
        Self::from(&value)
    }
}

impl From<&OHLCCents> for OHLCDollars {
    #[inline]
    fn from(value: &OHLCCents) -> Self {
        Self {
            open: value.open.into(),
            high: value.high.into(),
            low: value.low.into(),
            close: value.close.into(),
        }
    }
}

impl std::fmt::Display for OHLCDollars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.open, self.high, self.low, self.close
        )
    }
}

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct OHLCSats {
    pub open: Open<Sats>,
    pub high: High<Sats>,
    pub low: Low<Sats>,
    pub close: Close<Sats>,
}

impl Serialize for OHLCSats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(4)?;
        tup.serialize_element(&self.open)?;
        tup.serialize_element(&self.high)?;
        tup.serialize_element(&self.low)?;
        tup.serialize_element(&self.close)?;
        tup.end()
    }
}

impl From<(Open<Sats>, High<Sats>, Low<Sats>, Close<Sats>)> for OHLCSats {
    #[inline]
    fn from(value: (Open<Sats>, High<Sats>, Low<Sats>, Close<Sats>)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
        }
    }
}

impl From<Close<Sats>> for OHLCSats {
    #[inline]
    fn from(value: Close<Sats>) -> Self {
        Self {
            open: Open::from(value),
            high: High::from(value),
            low: Low::from(value),
            close: value,
        }
    }
}

impl std::fmt::Display for OHLCSats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.open, self.high, self.low, self.close
        )
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Deref,
    DerefMut,
    Serialize,
    StoredCompressed,
)]
#[repr(transparent)]
pub struct Open<T>(T);

impl<T> Open<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<usize> for Open<T>
where
    T: From<usize>,
{
    #[inline]
    fn from(value: usize) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<f64> for Open<T>
where
    T: From<f64>,
{
    #[inline]
    fn from(value: f64) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<Open<T>> for f64
where
    f64: From<T>,
{
    #[inline]
    fn from(value: Open<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Open<T>> for StoredF64
where
    StoredF64: From<T>,
{
    #[inline]
    fn from(value: Open<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Close<T>> for Open<T>
where
    T: Copy,
{
    #[inline]
    fn from(value: Close<T>) -> Self {
        Self(*value)
    }
}

impl From<Open<Cents>> for Open<Dollars> {
    #[inline]
    fn from(value: Open<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

impl<T> Add for Open<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> AddAssign for Open<T>
where
    T: Add<Output = T> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        **self = self.0.clone() + rhs.0
    }
}

impl<T> Div<usize> for Open<T>
where
    T: Div<usize, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<T> std::fmt::Display for Open<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Deref,
    DerefMut,
    Serialize,
    StoredCompressed,
)]
#[repr(transparent)]
pub struct High<T>(T);

impl<T> High<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<usize> for High<T>
where
    T: From<usize>,
{
    #[inline]
    fn from(value: usize) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<f64> for High<T>
where
    T: From<f64>,
{
    #[inline]
    fn from(value: f64) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<High<T>> for f64
where
    f64: From<T>,
{
    #[inline]
    fn from(value: High<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<High<T>> for StoredF64
where
    StoredF64: From<T>,
{
    #[inline]
    fn from(value: High<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Close<T>> for High<T>
where
    T: Copy,
{
    #[inline]
    fn from(value: Close<T>) -> Self {
        Self(*value)
    }
}

impl From<High<Cents>> for High<Dollars> {
    #[inline]
    fn from(value: High<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

impl<T> Add for High<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> AddAssign for High<T>
where
    T: Add<Output = T> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        **self = self.0.clone() + rhs.0
    }
}

impl<T> Div<usize> for High<T>
where
    T: Div<usize, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<T> std::fmt::Display for High<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Deref,
    DerefMut,
    Serialize,
    StoredCompressed,
)]
#[repr(transparent)]
pub struct Low<T>(T);

impl<T> Low<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<usize> for Low<T>
where
    T: From<usize>,
{
    #[inline]
    fn from(value: usize) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<f64> for Low<T>
where
    T: From<f64>,
{
    #[inline]
    fn from(value: f64) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<Low<T>> for f64
where
    f64: From<T>,
{
    #[inline]
    fn from(value: Low<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Low<T>> for StoredF64
where
    StoredF64: From<T>,
{
    #[inline]
    fn from(value: Low<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Close<T>> for Low<T>
where
    T: Copy,
{
    #[inline]
    fn from(value: Close<T>) -> Self {
        Self(*value)
    }
}

impl From<Low<Cents>> for Low<Dollars> {
    #[inline]
    fn from(value: Low<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

impl<T> Add for Low<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> AddAssign for Low<T>
where
    T: Add<Output = T> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        **self = self.0.clone() + rhs.0
    }
}

impl<T> Div<usize> for Low<T>
where
    T: Div<usize, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<T> std::fmt::Display for Low<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Deref,
    DerefMut,
    Serialize,
    StoredCompressed,
    Allocative,
)]
#[repr(transparent)]
pub struct Close<T>(T);

impl<T> Close<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<usize> for Close<T>
where
    T: From<usize>,
{
    #[inline]
    fn from(value: usize) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<f32> for Close<T>
where
    T: From<f32>,
{
    #[inline]
    fn from(value: f32) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<f64> for Close<T>
where
    T: From<f64>,
{
    #[inline]
    fn from(value: f64) -> Self {
        Self(T::from(value))
    }
}

impl<T> From<Close<T>> for f32
where
    f32: From<T>,
{
    #[inline]
    fn from(value: Close<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Close<T>> for f64
where
    f64: From<T>,
{
    #[inline]
    fn from(value: Close<T>) -> Self {
        Self::from(value.0)
    }
}

impl<T> From<Close<T>> for StoredF64
where
    StoredF64: From<T>,
{
    #[inline]
    fn from(value: Close<T>) -> Self {
        Self::from(value.0)
    }
}

// impl<A, B> From<Close<A>> for Close<B>
// where
//     B: From<A>,
// {
// #[inline]
// fn from(value: Close<A>) -> Self {
//         Self(B::from(*value))
impl From<Close<Cents>> for Close<Dollars> {
    #[inline]
    fn from(value: Close<Cents>) -> Self {
        Self(Dollars::from(*value))
    }
}

impl<T> Add for Close<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> AddAssign for Close<T>
where
    T: Add<Output = T> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        **self = self.0.clone() + rhs.0
    }
}

impl<T> Div<usize> for Close<T>
where
    T: Div<usize, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Sum for Close<Dollars> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(Dollars::from(iter.map(|v| f64::from(v.0)).sum::<f64>()))
    }
}

impl<T> std::fmt::Display for Close<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
