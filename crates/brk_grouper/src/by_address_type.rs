use std::ops::{Add, AddAssign};

use brk_error::Result;
use brk_traversable::{Traversable, TreeNode};
use brk_types::OutputType;
use rayon::prelude::*;
use vecdb::AnyCollectableVec;

use super::{Filter, Filtered};

pub const P2PK65: &str = "p2pk65";
pub const P2PK33: &str = "p2pk33";
pub const P2PKH: &str = "p2pkh";
pub const P2SH: &str = "p2sh";
pub const P2WPKH: &str = "p2wpkh";
pub const P2WSH: &str = "p2wsh";
pub const P2TR: &str = "p2tr";
pub const P2A: &str = "p2a";

#[derive(Default, Clone, Debug)]
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
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
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
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
    pub fn par_iter(&mut self) -> impl ParallelIterator<Item = &T>
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
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
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
    pub fn iter_typed(&self) -> impl Iterator<Item = (OutputType, &T)> {
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
    pub fn into_iter_typed(self) -> impl Iterator<Item = (OutputType, T)> {
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
    pub fn iter_typed_mut(&mut self) -> impl Iterator<Item = (OutputType, &mut T)> {
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

impl<T> ByAddressType<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self.p2pk65.1,
            &self.p2pk33.1,
            &self.p2pkh.1,
            &self.p2sh.1,
            &self.p2wpkh.1,
            &self.p2wsh.1,
            &self.p2tr.1,
            &self.p2a.1,
        ]
        .into_iter()
    }
}

impl<T> From<ByAddressType<T>> for ByAddressType<Filtered<T>> {
    #[inline]
    fn from(value: ByAddressType<T>) -> Self {
        Self {
            p2pk65: (Filter::Type(OutputType::P2PK65), value.p2pk65).into(),
            p2pk33: (Filter::Type(OutputType::P2PK33), value.p2pk33).into(),
            p2pkh: (Filter::Type(OutputType::P2PKH), value.p2pkh).into(),
            p2sh: (Filter::Type(OutputType::P2SH), value.p2sh).into(),
            p2wpkh: (Filter::Type(OutputType::P2WPKH), value.p2wpkh).into(),
            p2wsh: (Filter::Type(OutputType::P2WSH), value.p2wsh).into(),
            p2tr: (Filter::Type(OutputType::P2TR), value.p2tr).into(),
            p2a: (Filter::Type(OutputType::P2A), value.p2a).into(),
        }
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
        self.iter_mut().for_each(|opt| {
            opt.take();
        });
    }
}

impl<T: Traversable> Traversable for ByAddressType<T> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                (P2PK65, &self.p2pk65),
                (P2PK33, &self.p2pk33),
                (P2PKH, &self.p2pkh),
                (P2SH, &self.p2sh),
                (P2WPKH, &self.p2wpkh),
                (P2WSH, &self.p2wsh),
                (P2TR, &self.p2tr),
                (P2A, &self.p2a),
            ]
            .into_iter()
            .map(|(name, field)| (name.to_string(), field.to_tree_node()))
            .collect(),
        )
    }

    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self.p2pk65.iter_any_collectable());
        iter = Box::new(iter.chain(self.p2pk33.iter_any_collectable()));
        iter = Box::new(iter.chain(self.p2pkh.iter_any_collectable()));
        iter = Box::new(iter.chain(self.p2sh.iter_any_collectable()));
        iter = Box::new(iter.chain(self.p2wpkh.iter_any_collectable()));
        iter = Box::new(iter.chain(self.p2wsh.iter_any_collectable()));
        iter = Box::new(iter.chain(self.p2tr.iter_any_collectable()));
        iter = Box::new(iter.chain(self.p2a.iter_any_collectable()));
        iter
    }
}
