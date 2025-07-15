use super::{ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount, GroupFilter};

#[derive(Default, Clone)]
pub struct AddressGroups<T> {
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> AddressGroups<T> {
    pub fn as_mut_vecs(&mut self) -> Vec<&mut T> {
        self.ge_amount
            .as_mut_vec()
            .into_iter()
            .chain(self.amount_range.as_mut_vec())
            .chain(self.lt_amount.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_separate_vecs(&mut self) -> Vec<&mut T> {
        self.amount_range
            .as_mut_vec()
            .into_iter()
            .collect::<Vec<_>>()
    }

    pub fn as_mut_overlapping_vecs(&mut self) -> Vec<&mut T> {
        self.lt_amount
            .as_mut_vec()
            .into_iter()
            .chain(self.ge_amount.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl<T> AddressGroups<(GroupFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        self.amount_range
            .vecs()
            .into_iter()
            .chain(self.lt_amount.vecs())
            .chain(self.ge_amount.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<AddressGroups<T>> for AddressGroups<(GroupFilter, T)> {
    fn from(value: AddressGroups<T>) -> Self {
        Self {
            amount_range: ByAmountRange::from(value.amount_range),
            lt_amount: ByLowerThanAmount::from(value.lt_amount),
            ge_amount: ByGreatEqualAmount::from(value.ge_amount),
        }
    }
}
