use std::ops::AddAssign;

use derive_more::{Deref, DerefMut};

use crate::{
    AgeRangeId, AmountRange, AmountRangeId, Filter, OVER_AGE_FILTERS, OVER_AMOUNT_FILTERS, OverAge,
    OverAmount, SpendableType, TERM_FILTERS, UNDER_AGE_FILTERS, UNDER_AMOUNT_FILTERS,
    UTXOAggregate, UTXOCoreValues, UTXOOverlappingValues, UnderAge, UnderAmount,
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
        T: AddAssign + Clone + Default,
    {
        UTXOOverlappingValues {
            aggregate: UTXOAggregate {
                all: Self::sum(self.age_range.iter()),
                sth: self.sum_age(&TERM_FILTERS.short),
                lth: self.sum_age(&TERM_FILTERS.long),
            },
            under_age: UnderAge::from_fn(|id| self.sum_age(id.select(&UNDER_AGE_FILTERS))),
            over_age: OverAge::from_fn(|id| self.sum_age(id.select(&OVER_AGE_FILTERS))),
            under_amount: UnderAmount::from_fn(|id| {
                self.sum_amount(id.select(&UNDER_AMOUNT_FILTERS))
            }),
            over_amount: OverAmount::from_fn(|id| self.sum_amount(id.select(&OVER_AMOUNT_FILTERS))),
        }
    }

    fn sum_age(&self, filter: &Filter) -> T
    where
        T: AddAssign + Clone + Default,
    {
        Self::sum(AgeRangeId::included_by(filter).map(|id| id.select(&self.age_range)))
    }

    fn sum_amount(&self, filter: &Filter) -> T
    where
        T: AddAssign + Clone + Default,
    {
        Self::sum(AmountRangeId::included_by(filter).map(|id| id.select(&self.amount_range)))
    }

    fn sum<'a>(values: impl Iterator<Item = &'a T>) -> T
    where
        T: AddAssign + Clone + Default + 'a,
    {
        values.fold(T::default(), |mut total, value| {
            total += value.clone();
            total
        })
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
    use crate::AGE_RANGE_COUNT;

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
}
