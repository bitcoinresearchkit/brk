use brk_traversable::Traversable;
use rayon::prelude::*;

use crate::Filter;

use super::{AmountRange, OverAmount, UnderAmount};

#[derive(Default, Clone, Traversable)]
pub struct AddressGroups<T> {
    pub over_amount: OverAmount<T>,
    pub amount_range: AmountRange<T>,
    pub under_amount: UnderAmount<T>,
}

impl<T> AddressGroups<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            over_amount: OverAmount::new(&mut create),
            amount_range: AmountRange::new(&mut create),
            under_amount: UnderAmount::new(&mut create),
        }
    }

    pub fn try_new<F, E>(create: &F) -> Result<Self, E>
    where
        F: Fn(Filter, &'static str) -> Result<T, E>,
    {
        Ok(Self {
            over_amount: OverAmount::try_new(create)?,
            amount_range: AmountRange::try_new(create)?,
            under_amount: UnderAmount::try_new(create)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.over_amount
            .iter()
            .chain(self.amount_range.iter())
            .chain(self.under_amount.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.over_amount
            .iter_mut()
            .chain(self.amount_range.iter_mut())
            .chain(self.under_amount.iter_mut())
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.over_amount
            .par_iter_mut()
            .chain(self.amount_range.par_iter_mut())
            .chain(self.under_amount.par_iter_mut())
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
        self.under_amount.iter_mut().chain(self.over_amount.iter_mut())
    }
}

