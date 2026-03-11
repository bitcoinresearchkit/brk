use brk_traversable::Traversable;
use rayon::prelude::*;

use crate::{
    AgeRange, AmountRange, ByEpoch, OverAmount, UnderAmount, UnderAge, OverAge,
    Class, SpendableType, ByTerm, Filter,
};

#[derive(Default, Clone, Traversable)]
pub struct UTXOGroups<T> {
    pub all: T,
    pub age_range: AgeRange<T>,
    pub epoch: ByEpoch<T>,
    pub class: Class<T>,
    pub over_age: OverAge<T>,
    pub over_amount: OverAmount<T>,
    pub amount_range: AmountRange<T>,
    pub term: ByTerm<T>,
    pub type_: SpendableType<T>,
    pub under_age: UnderAge<T>,
    pub under_amount: UnderAmount<T>,
}

impl<T> UTXOGroups<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            all: create(Filter::All, ""),
            age_range: AgeRange::new(&mut create),
            epoch: ByEpoch::new(&mut create),
            class: Class::new(&mut create),
            over_age: OverAge::new(&mut create),
            over_amount: OverAmount::new(&mut create),
            amount_range: AmountRange::new(&mut create),
            term: ByTerm::new(&mut create),
            type_: SpendableType::new(&mut create),
            under_age: UnderAge::new(&mut create),
            under_amount: UnderAmount::new(&mut create),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.all]
            .into_iter()
            .chain(self.term.iter())
            .chain(self.under_age.iter())
            .chain(self.over_age.iter())
            .chain(self.over_amount.iter())
            .chain(self.age_range.iter())
            .chain(self.epoch.iter())
            .chain(self.class.iter())
            .chain(self.amount_range.iter())
            .chain(self.under_amount.iter())
            .chain(self.type_.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.under_age.iter_mut())
            .chain(self.over_age.iter_mut())
            .chain(self.over_amount.iter_mut())
            .chain(self.age_range.iter_mut())
            .chain(self.epoch.iter_mut())
            .chain(self.class.iter_mut())
            .chain(self.amount_range.iter_mut())
            .chain(self.under_amount.iter_mut())
            .chain(self.type_.iter_mut())
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [&mut self.all]
            .into_par_iter()
            .chain(self.term.par_iter_mut())
            .chain(self.under_age.par_iter_mut())
            .chain(self.over_age.par_iter_mut())
            .chain(self.over_amount.par_iter_mut())
            .chain(self.age_range.par_iter_mut())
            .chain(self.epoch.par_iter_mut())
            .chain(self.class.par_iter_mut())
            .chain(self.amount_range.par_iter_mut())
            .chain(self.under_amount.par_iter_mut())
            .chain(self.type_.par_iter_mut())
    }

    pub fn iter_separate(&self) -> impl Iterator<Item = &T> {
        self.age_range
            .iter()
            .chain(self.epoch.iter())
            .chain(self.class.iter())
            .chain(self.amount_range.iter())
            .chain(self.type_.iter())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.age_range
            .iter_mut()
            .chain(self.epoch.iter_mut())
            .chain(self.class.iter_mut())
            .chain(self.amount_range.iter_mut())
            .chain(self.type_.iter_mut())
    }

    pub fn par_iter_separate_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.age_range
            .par_iter_mut()
            .chain(self.epoch.par_iter_mut())
            .chain(self.class.par_iter_mut())
            .chain(self.amount_range.par_iter_mut())
            .chain(self.type_.par_iter_mut())
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.under_age.iter_mut())
            .chain(self.over_age.iter_mut())
            .chain(self.under_amount.iter_mut())
            .chain(self.over_amount.iter_mut())
    }

    /// Iterator over aggregate cohorts (all, sth, lth) that compute values from sub-cohorts.
    /// These are cohorts with StateLevel::PriceOnly that derive values from stateful sub-cohorts.
    pub fn iter_aggregate(&self) -> impl Iterator<Item = &T> {
        [&self.all].into_iter().chain(self.term.iter())
    }

    pub fn par_iter_aggregate(&self) -> impl ParallelIterator<Item = &T>
    where
        T: Send + Sync,
    {
        [&self.all].into_par_iter().chain(self.term.par_iter())
    }

    /// Iterator over aggregate cohorts (all, sth, lth) that compute values from sub-cohorts.
    /// These are cohorts with StateLevel::PriceOnly that derive values from stateful sub-cohorts.
    pub fn iter_aggregate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all].into_iter().chain(self.term.iter_mut())
    }

    pub fn par_iter_aggregate_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [&mut self.all]
            .into_par_iter()
            .chain(self.term.par_iter_mut())
    }
}
