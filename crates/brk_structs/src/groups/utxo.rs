use crate::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByTerm, GroupFilter,
};

#[derive(Default, Clone)]
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
    pub fn as_boxed_mut_vecs(&mut self) -> Vec<Box<[&mut T]>> {
        vec![
            Box::new([&mut self.all]),
            Box::new(self.term.as_mut_vec()),
            Box::new(self.max_age.as_mut_vec()),
            Box::new(self.min_age.as_mut_vec()),
            Box::new(self.ge_amount.as_mut_vec()),
            Box::new(self.age_range.as_mut_vec()),
            Box::new(self.epoch.as_mut_vec()),
            Box::new(self.amount_range.as_mut_vec()),
            Box::new(self.lt_amount.as_mut_vec()),
            Box::new(self._type.as_mut_vec()),
        ]
    }

    pub fn as_mut_vecs(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.as_mut_vec())
            .chain(self.max_age.as_mut_vec())
            .chain(self.min_age.as_mut_vec())
            .chain(self.ge_amount.as_mut_vec())
            .chain(self.age_range.as_mut_vec())
            .chain(self.epoch.as_mut_vec())
            .chain(self.amount_range.as_mut_vec())
            .chain(self.lt_amount.as_mut_vec())
            .chain(self._type.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_separate_vecs(&mut self) -> Vec<&mut T> {
        self.age_range
            .as_mut_vec()
            .into_iter()
            .chain(self.epoch.as_mut_vec())
            .chain(self.amount_range.as_mut_vec())
            .chain(self._type.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_overlapping_vecs(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.as_mut_vec())
            .chain(self.max_age.as_mut_vec())
            .chain(self.min_age.as_mut_vec())
            .chain(self.lt_amount.as_mut_vec())
            .chain(self.ge_amount.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl<T> UTXOGroups<(GroupFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        [&self.all.1]
            .into_iter()
            .chain(self.term.vecs())
            .chain(self.max_age.vecs())
            .chain(self.min_age.vecs())
            .chain(self.age_range.vecs())
            .chain(self.epoch.vecs())
            .chain(self.amount_range.vecs())
            .chain(self._type.vecs())
            .chain(self.lt_amount.vecs())
            .chain(self.ge_amount.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<UTXOGroups<T>> for UTXOGroups<(GroupFilter, T)> {
    fn from(value: UTXOGroups<T>) -> Self {
        Self {
            all: (GroupFilter::All, value.all),
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
