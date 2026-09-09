use std::ops::AddAssign;

use derive_more::{Deref, DerefMut};

use crate::{
    AmountId, AmountRange, OverAge, OverAmount, SpendableType, UTXOAggregate, UTXOCoreValues,
    UTXOOverlappingValues, UnderAge, UnderAmount,
};

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

    pub fn aggregate(&self) -> UTXOOverlappingValues<T>
    where
        T: AddAssign + Copy,
    {
        let age_value = |id| self.core.value(id).expect("age cohort");
        UTXOOverlappingValues {
            aggregate: UTXOAggregate::from_fn(|id| age_value(id.cohort())),
            under_age: UnderAge::new(age_value),
            over_age: OverAge::new(age_value),
            under_amount: UnderAmount::from_fn(|id| {
                self.amount_range.aggregate(AmountId::Under(id))
            }),
            over_amount: OverAmount::from_fn(|id| self.amount_range.aggregate(AmountId::Over(id))),
        }
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
        assert_eq!(aggregates.aggregate.all, 3 * AGE_RANGE_COUNT as u64);
        assert_eq!(
            aggregates.aggregate.all,
            aggregates.aggregate.sth + aggregates.aggregate.lth
        );
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
        let expected_amount =
            |id: AmountId| id.ranges().map(|range| 1u64 << range.index()).sum::<u64>();

        UTXOAggregate::from_fn(|id| {
            assert_eq!(
                id.select(&aggregates.aggregate).0,
                expected_age(id.cohort())
            );
        });
        UnderAge::from_fn(|id| {
            assert_eq!(
                id.select(&aggregates.under_age).0,
                expected_age(id.cohort())
            );
        });
        OverAge::from_fn(|id| {
            assert_eq!(id.select(&aggregates.over_age).0, expected_age(id.cohort()));
        });
        UnderAmount::from_fn(|id| {
            assert_eq!(
                id.select(&aggregates.under_amount).0,
                expected_amount(AmountId::Under(id))
            );
        });
        OverAmount::from_fn(|id| {
            assert_eq!(
                id.select(&aggregates.over_amount).0,
                expected_amount(AmountId::Over(id))
            );
        });
    }
}
