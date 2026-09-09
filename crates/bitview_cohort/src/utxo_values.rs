use std::ops::AddAssign;

use derive_more::{Deref, DerefMut};

use crate::{AmountRange, SpendableType, UTXOAggregate, UTXOCoreValues};

#[derive(Clone, Default, Deref, DerefMut)]
pub struct UTXOValues<T> {
    #[deref]
    #[deref_mut]
    pub core: UTXOCoreValues<T>,
    pub amount_range: AmountRange<T>,
    pub type_: SpendableType<T>,
}

impl<T> UTXOValues<T> {
    pub fn map<U>(&self, mut map: impl FnMut(&T) -> U) -> UTXOValues<U> {
        UTXOValues {
            core: self.core.map(&mut map),
            amount_range: AmountRange::from_fn(|id| map(id.select(&self.amount_range))),
            type_: SpendableType::from_fn(|id| map(id.select(&self.type_))),
        }
    }

    pub fn aggregate(&self) -> UTXOAggregate<T>
    where
        T: AddAssign + Copy,
    {
        UTXOAggregate::from_fn(|id| self.core.value(id.cohort()).expect("aggregate cohort"))
    }
}

impl<T> AddAssign for UTXOValues<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.core += rhs.core;
        for (left, right) in self.amount_range.iter_mut().zip(rhs.amount_range.iter()) {
            *left += *right;
        }
        for (left, right) in self.type_.iter_mut().zip(rhs.type_.iter()) {
            *left += *right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AGE_RANGE_COUNT, AgeRange, CohortId};

    #[test]
    fn addition_and_aggregation_cover_every_cohort_axis() {
        let mut cohort_values = UTXOValues::<u64>::default().map(|_| 1);
        cohort_values += cohort_values.map(|_| 2);
        cohort_values.map(|value| assert_eq!(*value, 3));
        let aggregates = cohort_values.aggregate();
        assert_eq!(aggregates.all, 3 * AGE_RANGE_COUNT as u64);
        assert_eq!(aggregates.all, aggregates.sth + aggregates.lth);
    }

    #[test]
    fn aggregation_uses_canonical_ranges_without_requiring_default() {
        #[derive(Clone, Copy)]
        struct Value(u64);

        impl AddAssign for Value {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        let mut values = UTXOValues::<u64>::default().map(|_| Value(0));
        values.age_range = AgeRange::from_fn(|id| Value(1 << id.index()));
        values.amount_range = AmountRange::from_fn(|id| Value(1 << id.index()));
        let aggregates = values.aggregate();
        let expected_age = |id: CohortId| {
            id.age_ranges()
                .unwrap()
                .map(|range| 1u64 << range.index())
                .sum::<u64>()
        };
        UTXOAggregate::from_fn(|id| {
            assert_eq!(id.select(&aggregates).0, expected_age(id.cohort()));
        });
    }
}
