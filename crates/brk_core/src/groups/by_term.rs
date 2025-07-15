use super::GroupFilter;

#[derive(Default, Clone)]
pub struct ByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> ByTerm<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 2] {
        [&mut self.short, &mut self.long]
    }
}

impl<T> ByTerm<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 2] {
        [&self.short.1, &self.long.1]
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
