use super::GroupFilter;

#[derive(Default, Clone)]
pub struct GroupedByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> GroupedByTerm<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 2] {
        [&mut self.short, &mut self.long]
    }
}

impl<T> GroupedByTerm<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 2] {
        [&self.short.1, &self.long.1]
    }
}

impl<T> From<GroupedByTerm<T>> for GroupedByTerm<(GroupFilter, T)> {
    fn from(value: GroupedByTerm<T>) -> Self {
        Self {
            short: (GroupFilter::To(5 * 30), value.short),
            long: (GroupFilter::From(5 * 30), value.long),
        }
    }
}
