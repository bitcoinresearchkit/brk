use brk_traversable::Traversable;

use crate::Filtered;

use super::Filter;

#[derive(Default, Clone, Traversable)]
pub struct ByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> ByTerm<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.short, &mut self.long].into_iter()
    }
}

impl<T> ByTerm<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [&self.short.1, &self.long.1].into_iter()
    }
}

impl<T> From<ByTerm<T>> for ByTerm<Filtered<T>> {
    fn from(value: ByTerm<T>) -> Self {
        Self {
            short: (Filter::LowerThan(5 * 30), value.short).into(),
            long: (Filter::GreaterOrEqual(5 * 30), value.long).into(),
        }
    }
}
