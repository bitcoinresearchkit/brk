use brk_traversable::Traversable;

use crate::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByTerm, Filter, Filtered,
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
    pub _type: BySpendableType<T>,
    pub max_age: ByMaxAge<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> UTXOGroups<T> {
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
            .chain(self._type.iter_mut())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.age_range
            .iter_mut()
            .chain(self.epoch.iter_mut())
            .chain(self.amount_range.iter_mut())
            .chain(self._type.iter_mut())
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
}

impl<T> UTXOGroups<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [&self.all.1]
            .into_iter()
            .chain(self.term.iter_right())
            .chain(self.max_age.iter_right())
            .chain(self.min_age.iter_right())
            .chain(self.age_range.iter_right())
            .chain(self.epoch.iter_right())
            .chain(self.amount_range.iter_right())
            .chain(self._type.iter_right())
            .chain(self.lt_amount.iter_right())
            .chain(self.ge_amount.iter_right())
    }
}

impl<T> From<UTXOGroups<T>> for UTXOGroups<Filtered<T>> {
    fn from(value: UTXOGroups<T>) -> Self {
        Self {
            all: (Filter::All, value.all).into(),
            term: ByTerm::from(value.term),
            max_age: ByMaxAge::from(value.max_age),
            min_age: ByMinAge::from(value.min_age),
            age_range: ByAgeRange::from(value.age_range),
            epoch: ByEpoch::from(value.epoch),
            amount_range: ByAmountRange::from(value.amount_range),
            lt_amount: ByLowerThanAmount::from(value.lt_amount),
            ge_amount: ByGreatEqualAmount::from(value.ge_amount),
            _type: BySpendableType::from(value._type),
        }
    }
}
