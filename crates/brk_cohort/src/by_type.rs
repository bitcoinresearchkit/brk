use std::ops::{Add, AddAssign};

use brk_traversable::Traversable;
use brk_types::OutputType;
use rayon::prelude::*;

use super::{Filter, SpendableType, UnspendableType};

pub const OP_RETURN: &str = "op_return";

#[derive(Default, Clone, Debug, Traversable)]
pub struct ByType<T> {
    #[traversable(flatten)]
    pub spendable: SpendableType<T>,
    #[traversable(flatten)]
    pub unspendable: UnspendableType<T>,
}

impl<T> ByType<T> {
    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(Filter, &'static str) -> Result<T, E>,
    {
        Ok(Self {
            spendable: SpendableType::try_new(&mut create)?,
            unspendable: UnspendableType {
                op_return: create(Filter::Type(OutputType::OpReturn), OP_RETURN)?,
            },
        })
    }

    pub fn get(&self, output_type: OutputType) -> &T {
        match output_type {
            OutputType::P2PK65 => &self.spendable.p2pk65,
            OutputType::P2PK33 => &self.spendable.p2pk33,
            OutputType::P2PKH => &self.spendable.p2pkh,
            OutputType::P2MS => &self.spendable.p2ms,
            OutputType::P2SH => &self.spendable.p2sh,
            OutputType::P2WPKH => &self.spendable.p2wpkh,
            OutputType::P2WSH => &self.spendable.p2wsh,
            OutputType::P2TR => &self.spendable.p2tr,
            OutputType::P2A => &self.spendable.p2a,
            OutputType::Empty => &self.spendable.empty,
            OutputType::Unknown => &self.spendable.unknown,
            OutputType::OpReturn => &self.unspendable.op_return,
        }
    }

    pub fn get_mut(&mut self, output_type: OutputType) -> &mut T {
        match output_type {
            OutputType::P2PK65 => &mut self.spendable.p2pk65,
            OutputType::P2PK33 => &mut self.spendable.p2pk33,
            OutputType::P2PKH => &mut self.spendable.p2pkh,
            OutputType::P2MS => &mut self.spendable.p2ms,
            OutputType::P2SH => &mut self.spendable.p2sh,
            OutputType::P2WPKH => &mut self.spendable.p2wpkh,
            OutputType::P2WSH => &mut self.spendable.p2wsh,
            OutputType::P2TR => &mut self.spendable.p2tr,
            OutputType::P2A => &mut self.spendable.p2a,
            OutputType::Unknown => &mut self.spendable.unknown,
            OutputType::Empty => &mut self.spendable.empty,
            OutputType::OpReturn => &mut self.unspendable.op_return,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.spendable
            .iter()
            .chain(std::iter::once(&self.unspendable.op_return))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.spendable
            .iter_mut()
            .chain(std::iter::once(&mut self.unspendable.op_return))
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        let Self {
            spendable,
            unspendable,
        } = self;
        spendable
            .par_iter_mut()
            .chain([&mut unspendable.op_return].into_par_iter())
    }

    pub fn iter_typed(&self) -> impl Iterator<Item = (OutputType, &T)> {
        self.spendable.iter_typed().chain(std::iter::once((
            OutputType::OpReturn,
            &self.unspendable.op_return,
        )))
    }

    pub fn iter_typed_mut(&mut self) -> impl Iterator<Item = (OutputType, &mut T)> {
        self.spendable.iter_typed_mut().chain(std::iter::once((
            OutputType::OpReturn,
            &mut self.unspendable.op_return,
        )))
    }
}

impl<T> Add for ByType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            spendable: self.spendable + rhs.spendable,
            unspendable: self.unspendable + rhs.unspendable,
        }
    }
}

impl<T> AddAssign for ByType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.spendable += rhs.spendable;
        self.unspendable += rhs.unspendable;
    }
}
