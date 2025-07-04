use super::{GroupFilter, GroupedByFromSize, GroupedBySizeRange, GroupedByUpToSize};

#[derive(Default, Clone)]
pub struct AddressGroups<T> {
    pub by_from_size: GroupedByFromSize<T>,
    pub by_size_range: GroupedBySizeRange<T>,
    pub by_up_to_size: GroupedByUpToSize<T>,
}

impl<T> AddressGroups<T> {
    pub fn as_mut_vecs(&mut self) -> Vec<&mut T> {
        self.by_from_size
            .as_mut_vec()
            .into_iter()
            .chain(self.by_size_range.as_mut_vec())
            .chain(self.by_up_to_size.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_separate_vecs(&mut self) -> Vec<&mut T> {
        self.by_size_range
            .as_mut_vec()
            .into_iter()
            .collect::<Vec<_>>()
    }

    pub fn as_mut_overlapping_vecs(&mut self) -> Vec<&mut T> {
        self.by_up_to_size
            .as_mut_vec()
            .into_iter()
            .chain(self.by_from_size.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl<T> AddressGroups<(GroupFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        self.by_size_range
            .vecs()
            .into_iter()
            .chain(self.by_up_to_size.vecs())
            .chain(self.by_from_size.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<AddressGroups<T>> for AddressGroups<(GroupFilter, T)> {
    fn from(value: AddressGroups<T>) -> Self {
        Self {
            by_size_range: GroupedBySizeRange::from(value.by_size_range),
            by_up_to_size: GroupedByUpToSize::from(value.by_up_to_size),
            by_from_size: GroupedByFromSize::from(value.by_from_size),
        }
    }
}
