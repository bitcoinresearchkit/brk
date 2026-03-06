use brk_traversable::Traversable;
use rayon::prelude::*;

use crate::Filter;

use super::{ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount};

#[derive(Default, Clone, Traversable)]
pub struct AddressGroups<T> {
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> AddressGroups<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            ge_amount: ByGreatEqualAmount::new(&mut create),
            amount_range: ByAmountRange::new(&mut create),
            lt_amount: ByLowerThanAmount::new(&mut create),
        }
    }

    pub fn try_new<F, E>(create: &F) -> Result<Self, E>
    where
        F: Fn(Filter, &'static str) -> Result<T, E>,
    {
        Ok(Self {
            ge_amount: ByGreatEqualAmount::try_new(create)?,
            amount_range: ByAmountRange::try_new(create)?,
            lt_amount: ByLowerThanAmount::try_new(create)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.ge_amount
            .iter()
            .chain(self.amount_range.iter())
            .chain(self.lt_amount.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.ge_amount
            .iter_mut()
            .chain(self.amount_range.iter_mut())
            .chain(self.lt_amount.iter_mut())
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.ge_amount
            .par_iter_mut()
            .chain(self.amount_range.par_iter_mut())
            .chain(self.lt_amount.par_iter_mut())
    }

    pub fn iter_separate(&self) -> impl Iterator<Item = &T> {
        self.amount_range.iter()
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.amount_range.iter_mut()
    }

    pub fn par_iter_separate_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.amount_range.par_iter_mut()
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.lt_amount.iter_mut().chain(self.ge_amount.iter_mut())
    }
}

