use crate::{
    GroupFilter, GroupedByDateRange, GroupedByEpoch, GroupedByFromDate, GroupedByFromSize,
    GroupedBySizeRange, GroupedBySpendableType, GroupedByTerm, GroupedByUpToDate,
    GroupedByUpToSize,
};

#[derive(Default, Clone)]
pub struct UTXOGroups<T> {
    pub all: T,
    pub by_date_range: GroupedByDateRange<T>,
    pub by_epoch: GroupedByEpoch<T>,
    pub by_from_date: GroupedByFromDate<T>,
    pub by_from_size: GroupedByFromSize<T>,
    pub by_size_range: GroupedBySizeRange<T>,
    pub by_term: GroupedByTerm<T>,
    pub by_type: GroupedBySpendableType<T>,
    pub by_up_to_date: GroupedByUpToDate<T>,
    pub by_up_to_size: GroupedByUpToSize<T>,
}

impl<T> UTXOGroups<T> {
    pub fn as_mut_vecs(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.by_term.as_mut_vec())
            .chain(self.by_up_to_date.as_mut_vec())
            .chain(self.by_from_date.as_mut_vec())
            .chain(self.by_from_size.as_mut_vec())
            .chain(self.by_date_range.as_mut_vec())
            .chain(self.by_epoch.as_mut_vec())
            .chain(self.by_size_range.as_mut_vec())
            .chain(self.by_up_to_size.as_mut_vec())
            .chain(self.by_type.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_separate_vecs(&mut self) -> Vec<&mut T> {
        self.by_date_range
            .as_mut_vec()
            .into_iter()
            .chain(self.by_epoch.as_mut_vec())
            .chain(self.by_size_range.as_mut_vec())
            .chain(self.by_type.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_overlapping_vecs(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.by_term.as_mut_vec())
            .chain(self.by_up_to_date.as_mut_vec())
            .chain(self.by_from_date.as_mut_vec())
            .chain(self.by_up_to_size.as_mut_vec())
            .chain(self.by_from_size.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl<T> UTXOGroups<(GroupFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        [&self.all.1]
            .into_iter()
            .chain(self.by_term.vecs())
            .chain(self.by_up_to_date.vecs())
            .chain(self.by_from_date.vecs())
            .chain(self.by_date_range.vecs())
            .chain(self.by_epoch.vecs())
            .chain(self.by_size_range.vecs())
            .chain(self.by_type.vecs())
            .chain(self.by_up_to_size.vecs())
            .chain(self.by_from_size.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<UTXOGroups<T>> for UTXOGroups<(GroupFilter, T)> {
    fn from(value: UTXOGroups<T>) -> Self {
        Self {
            all: (GroupFilter::All, value.all),
            by_term: GroupedByTerm::from(value.by_term),
            by_up_to_date: GroupedByUpToDate::from(value.by_up_to_date),
            by_from_date: GroupedByFromDate::from(value.by_from_date),
            by_date_range: GroupedByDateRange::from(value.by_date_range),
            by_epoch: GroupedByEpoch::from(value.by_epoch),
            by_size_range: GroupedBySizeRange::from(value.by_size_range),
            by_up_to_size: GroupedByUpToSize::from(value.by_up_to_size),
            by_from_size: GroupedByFromSize::from(value.by_from_size),
            by_type: GroupedBySpendableType::from(value.by_type),
        }
    }
}
