use brk_traversable::Traversable;
use rayon::prelude::*;

use crate::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByTerm, Filter,
};

#[derive(Default, Clone, Traversable)]
pub struct UTXOGroups<T> {
    pub all: T,
    pub age_range: ByAgeRange<T>,
    pub epoch: ByEpoch<T>,
    pub min_age: ByMinAge<T>,
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub term: ByTerm<T>,
    pub type_: BySpendableType<T>,
    pub max_age: ByMaxAge<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> UTXOGroups<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            all: create(Filter::All),
            age_range: ByAgeRange::new(&mut create),
            epoch: ByEpoch::new(&mut create),
            min_age: ByMinAge::new(&mut create),
            ge_amount: ByGreatEqualAmount::new(&mut create),
            amount_range: ByAmountRange::new(&mut create),
            term: ByTerm::new(&mut create),
            type_: BySpendableType::new(&mut create),
            max_age: ByMaxAge::new(&mut create),
            lt_amount: ByLowerThanAmount::new(&mut create),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.all]
            .into_iter()
            .chain(self.term.iter())
            .chain(self.max_age.iter())
            .chain(self.min_age.iter())
            .chain(self.ge_amount.iter())
            .chain(self.age_range.iter())
            .chain(self.epoch.iter())
            .chain(self.amount_range.iter())
            .chain(self.lt_amount.iter())
            .chain(self.type_.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.max_age.iter_mut())
            .chain(self.min_age.iter_mut())
            .chain(self.ge_amount.iter_mut())
            .chain(self.age_range.iter_mut())
            .chain(self.epoch.iter_mut())
            .chain(self.amount_range.iter_mut())
            .chain(self.lt_amount.iter_mut())
            .chain(self.type_.iter_mut())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.age_range
            .iter_mut()
            .chain(self.epoch.iter_mut())
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
            .chain(self.amount_range.par_iter_mut())
            .chain(self.type_.par_iter_mut())
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.max_age.iter_mut())
            .chain(self.min_age.iter_mut())
            .chain(self.lt_amount.iter_mut())
            .chain(self.ge_amount.iter_mut())
    }

    /// Iterator over aggregate cohorts (all, sth, lth) that compute values from sub-cohorts.
    /// These are cohorts with StateLevel::PriceOnly that derive values from stateful sub-cohorts.
    pub fn iter_aggregate(&self) -> impl Iterator<Item = &T> {
        [&self.all].into_iter().chain(self.term.iter())
    }

    /// Iterator over aggregate cohorts (all, sth, lth) that compute values from sub-cohorts.
    /// These are cohorts with StateLevel::PriceOnly that derive values from stateful sub-cohorts.
    pub fn iter_aggregate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
    }
}
