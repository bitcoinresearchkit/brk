use brk_traversable::Traversable;

use crate::Filtered;

use super::{ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount};

#[derive(Default, Clone, Traversable)]
pub struct AddressGroups<T> {
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> AddressGroups<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.ge_amount
            .iter_mut()
            .chain(self.amount_range.iter_mut())
            .chain(self.lt_amount.iter_mut())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.amount_range.iter_mut()
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.lt_amount.iter_mut().chain(self.ge_amount.iter_mut())
    }
}

impl<T> AddressGroups<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        self.amount_range
            .iter_right()
            .chain(self.lt_amount.iter_right())
            .chain(self.ge_amount.iter_right())
    }
}

impl<T> From<AddressGroups<T>> for AddressGroups<Filtered<T>> {
    fn from(value: AddressGroups<T>) -> Self {
        Self {
            amount_range: ByAmountRange::from(value.amount_range),
            lt_amount: ByLowerThanAmount::from(value.lt_amount),
            ge_amount: ByGreatEqualAmount::from(value.ge_amount),
        }
    }
}
