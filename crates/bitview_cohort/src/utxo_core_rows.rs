use std::ops::AddAssign;

use crate::{AgeRange, ByEntry, ByEpoch, Class, UTXORows};

/// Disjoint rows shared by UTXO column families, without unused amount/type axes.
#[derive(Clone, Default)]
pub struct UTXOCoreRows<T> {
    pub age_range: AgeRange<T>,
    pub epoch: ByEpoch<T>,
    pub class: Class<T>,
    pub entry: ByEntry<T>,
}

impl<T> UTXOCoreRows<T> {
    pub fn map<U>(&self, mut map: impl FnMut(&T) -> U) -> UTXOCoreRows<U> {
        UTXOCoreRows {
            age_range: AgeRange::from_fn(|id| map(id.select(&self.age_range))),
            epoch: ByEpoch::from_fn(|id| map(id.select(&self.epoch))),
            class: Class::from_fn(|id| map(id.select(&self.class))),
            entry: ByEntry::from_fn(|id| map(id.select(&self.entry))),
        }
    }
}

impl<T> From<UTXORows<T>> for UTXOCoreRows<T> {
    fn from(rows: UTXORows<T>) -> Self {
        rows.core
    }
}

impl<T: AddAssign + Copy> AddAssign for UTXOCoreRows<T> {
    fn add_assign(&mut self, rhs: Self) {
        for (left, right) in self.age_range.iter_mut().zip(rhs.age_range.iter()) {
            *left += *right;
        }
        for (left, right) in self.epoch.iter_mut().zip(rhs.epoch.iter()) {
            *left += *right;
        }
        for (left, right) in self.class.iter_mut().zip(rhs.class.iter()) {
            *left += *right;
        }
        for (left, right) in self.entry.iter_mut().zip(rhs.entry.iter()) {
            *left += *right;
        }
    }
}
