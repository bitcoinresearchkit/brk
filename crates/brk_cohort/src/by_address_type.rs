use std::ops::{Add, AddAssign};

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::OutputType;
use rayon::prelude::*;

use super::Filter;

pub const P2PK65: &str = "p2pk65";
pub const P2PK33: &str = "p2pk33";
pub const P2PKH: &str = "p2pkh";
pub const P2SH: &str = "p2sh";
pub const P2WPKH: &str = "p2wpkh";
pub const P2WSH: &str = "p2wsh";
pub const P2TR: &str = "p2tr";
pub const P2A: &str = "p2a";

#[derive(Default, Clone, Debug, Traversable)]
pub struct ByAddressType<T> {
    pub p2pk65: T,
    pub p2pk33: T,
    pub p2pkh: T,
    pub p2sh: T,
    pub p2wpkh: T,
    pub p2wsh: T,
    pub p2tr: T,
    pub p2a: T,
}

impl<T> ByAddressType<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            p2pk65: create(Filter::Type(OutputType::P2PK65)),
            p2pk33: create(Filter::Type(OutputType::P2PK33)),
            p2pkh: create(Filter::Type(OutputType::P2PKH)),
            p2sh: create(Filter::Type(OutputType::P2SH)),
            p2wpkh: create(Filter::Type(OutputType::P2WPKH)),
            p2wsh: create(Filter::Type(OutputType::P2WSH)),
            p2tr: create(Filter::Type(OutputType::P2TR)),
            p2a: create(Filter::Type(OutputType::P2A)),
        }
    }

    pub fn new_with_name<F>(f: F) -> Result<Self>
    where
        F: Fn(&'static str) -> Result<T>,
    {
        Ok(Self {
            p2pk65: f(P2PK65)?,
            p2pk33: f(P2PK33)?,
            p2pkh: f(P2PKH)?,
            p2sh: f(P2SH)?,
            p2wpkh: f(P2WPKH)?,
            p2wsh: f(P2WSH)?,
            p2tr: f(P2TR)?,
            p2a: f(P2A)?,
        })
    }

    pub fn new_with_index<F>(f: F) -> Result<Self>
    where
        F: Fn(usize) -> Result<T>,
    {
        Ok(Self {
            p2pk65: f(0)?,
            p2pk33: f(1)?,
            p2pkh: f(2)?,
            p2sh: f(3)?,
            p2wpkh: f(4)?,
            p2wsh: f(5)?,
            p2tr: f(6)?,
            p2a: f(7)?,
        })
    }

    #[inline]
    pub fn get_unwrap(&self, addresstype: OutputType) -> &T {
        self.get(addresstype).unwrap()
    }

    #[inline]
    pub fn get(&self, address_type: OutputType) -> Option<&T> {
        match address_type {
            OutputType::P2PK65 => Some(&self.p2pk65),
            OutputType::P2PK33 => Some(&self.p2pk33),
            OutputType::P2PKH => Some(&self.p2pkh),
            OutputType::P2SH => Some(&self.p2sh),
            OutputType::P2WPKH => Some(&self.p2wpkh),
            OutputType::P2WSH => Some(&self.p2wsh),
            OutputType::P2TR => Some(&self.p2tr),
            OutputType::P2A => Some(&self.p2a),
            _ => None,
        }
    }

    #[inline]
    pub fn get_mut_unwrap(&mut self, addresstype: OutputType) -> &mut T {
        self.get_mut(addresstype).unwrap()
    }

    #[inline]
    pub fn get_mut(&mut self, address_type: OutputType) -> Option<&mut T> {
        match address_type {
            OutputType::P2PK65 => Some(&mut self.p2pk65),
            OutputType::P2PK33 => Some(&mut self.p2pk33),
            OutputType::P2PKH => Some(&mut self.p2pkh),
            OutputType::P2SH => Some(&mut self.p2sh),
            OutputType::P2WPKH => Some(&mut self.p2wpkh),
            OutputType::P2WSH => Some(&mut self.p2wsh),
            OutputType::P2TR => Some(&mut self.p2tr),
            OutputType::P2A => Some(&mut self.p2a),
            _ => None,
        }
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        [
            &self.p2pk65,
            &self.p2pk33,
            &self.p2pkh,
            &self.p2sh,
            &self.p2wpkh,
            &self.p2wsh,
            &self.p2tr,
            &self.p2a,
        ]
        .into_iter()
    }

    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self.p2pk65,
            &mut self.p2pk33,
            &mut self.p2pkh,
            &mut self.p2sh,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.p2a,
        ]
        .into_iter()
    }

    #[inline]
    pub fn par_values(&mut self) -> impl ParallelIterator<Item = &T>
    where
        T: Send + Sync,
    {
        [
            &self.p2pk65,
            &self.p2pk33,
            &self.p2pkh,
            &self.p2sh,
            &self.p2wpkh,
            &self.p2wsh,
            &self.p2tr,
            &self.p2a,
        ]
        .into_par_iter()
    }

    #[inline]
    pub fn par_values_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self.p2pk65,
            &mut self.p2pk33,
            &mut self.p2pkh,
            &mut self.p2sh,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.p2a,
        ]
        .into_par_iter()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (OutputType, &T)> {
        [
            (OutputType::P2PK65, &self.p2pk65),
            (OutputType::P2PK33, &self.p2pk33),
            (OutputType::P2PKH, &self.p2pkh),
            (OutputType::P2SH, &self.p2sh),
            (OutputType::P2WPKH, &self.p2wpkh),
            (OutputType::P2WSH, &self.p2wsh),
            (OutputType::P2TR, &self.p2tr),
            (OutputType::P2A, &self.p2a),
        ]
        .into_iter()
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = (OutputType, T)> {
        [
            (OutputType::P2PK65, self.p2pk65),
            (OutputType::P2PK33, self.p2pk33),
            (OutputType::P2PKH, self.p2pkh),
            (OutputType::P2SH, self.p2sh),
            (OutputType::P2WPKH, self.p2wpkh),
            (OutputType::P2WSH, self.p2wsh),
            (OutputType::P2TR, self.p2tr),
            (OutputType::P2A, self.p2a),
        ]
        .into_iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (OutputType, &mut T)> {
        [
            (OutputType::P2PK65, &mut self.p2pk65),
            (OutputType::P2PK33, &mut self.p2pk33),
            (OutputType::P2PKH, &mut self.p2pkh),
            (OutputType::P2SH, &mut self.p2sh),
            (OutputType::P2WPKH, &mut self.p2wpkh),
            (OutputType::P2WSH, &mut self.p2wsh),
            (OutputType::P2TR, &mut self.p2tr),
            (OutputType::P2A, &mut self.p2a),
        ]
        .into_iter()
    }
}

impl<T> Add for ByAddressType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            p2pk65: self.p2pk65 + rhs.p2pk65,
            p2pk33: self.p2pk33 + rhs.p2pk33,
            p2pkh: self.p2pkh + rhs.p2pkh,
            p2sh: self.p2sh + rhs.p2sh,
            p2wpkh: self.p2wpkh + rhs.p2wpkh,
            p2wsh: self.p2wsh + rhs.p2wsh,
            p2tr: self.p2tr + rhs.p2tr,
            p2a: self.p2a + rhs.p2a,
        }
    }
}

impl<T> AddAssign for ByAddressType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.p2pk65 += rhs.p2pk65;
        self.p2pk33 += rhs.p2pk33;
        self.p2pkh += rhs.p2pkh;
        self.p2sh += rhs.p2sh;
        self.p2wpkh += rhs.p2wpkh;
        self.p2wsh += rhs.p2wsh;
        self.p2tr += rhs.p2tr;
        self.p2a += rhs.p2a;
    }
}

impl<T> ByAddressType<Option<T>> {
    pub fn take(&mut self) {
        self.values_mut().for_each(|opt| {
            opt.take();
        });
    }
}

/// Zip one ByAddressType with a function, producing a new ByAddressType.
pub fn zip_by_addresstype<S, R, F>(source: &ByAddressType<S>, f: F) -> Result<ByAddressType<R>>
where
    F: Fn(&'static str, &S) -> Result<R>,
{
    Ok(ByAddressType {
        p2pk65: f(P2PK65, &source.p2pk65)?,
        p2pk33: f(P2PK33, &source.p2pk33)?,
        p2pkh: f(P2PKH, &source.p2pkh)?,
        p2sh: f(P2SH, &source.p2sh)?,
        p2wpkh: f(P2WPKH, &source.p2wpkh)?,
        p2wsh: f(P2WSH, &source.p2wsh)?,
        p2tr: f(P2TR, &source.p2tr)?,
        p2a: f(P2A, &source.p2a)?,
    })
}

/// Zip two ByAddressTypes with a function, producing a new ByAddressType.
pub fn zip2_by_addresstype<S1, S2, R, F>(
    a: &ByAddressType<S1>,
    b: &ByAddressType<S2>,
    f: F,
) -> Result<ByAddressType<R>>
where
    F: Fn(&'static str, &S1, &S2) -> Result<R>,
{
    Ok(ByAddressType {
        p2pk65: f(P2PK65, &a.p2pk65, &b.p2pk65)?,
        p2pk33: f(P2PK33, &a.p2pk33, &b.p2pk33)?,
        p2pkh: f(P2PKH, &a.p2pkh, &b.p2pkh)?,
        p2sh: f(P2SH, &a.p2sh, &b.p2sh)?,
        p2wpkh: f(P2WPKH, &a.p2wpkh, &b.p2wpkh)?,
        p2wsh: f(P2WSH, &a.p2wsh, &b.p2wsh)?,
        p2tr: f(P2TR, &a.p2tr, &b.p2tr)?,
        p2a: f(P2A, &a.p2a, &b.p2a)?,
    })
}
