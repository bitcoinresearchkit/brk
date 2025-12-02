use brk_traversable::Traversable;

use super::{Filter, Term};

#[derive(Default, Clone, Traversable)]
pub struct ByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> ByTerm<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            short: create(Filter::Term(Term::Sth)),
            long: create(Filter::Term(Term::Lth)),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.short, &self.long].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.short, &mut self.long].into_iter()
    }
}
