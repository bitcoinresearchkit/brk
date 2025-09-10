use super::GroupFilter;

#[derive(Default, Clone)]
pub struct ByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> ByTerm<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.short, &mut self.long].into_iter()
    }
}

impl<T> ByTerm<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [&self.short.1, &self.long.1].into_iter()
    }
}

impl<T> From<ByTerm<T>> for ByTerm<(GroupFilter, T)> {
    fn from(value: ByTerm<T>) -> Self {
        Self {
            short: (GroupFilter::LowerThan(5 * 30), value.short),
            long: (GroupFilter::GreaterOrEqual(5 * 30), value.long),
        }
    }
}
