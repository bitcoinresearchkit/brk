use derive_more::{Deref, DerefMut};

use crate::{ByTerm, Filter, TERM_FILTERS, TERM_NAMES, Term, UTXOGroupCore};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

#[derive(Default, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOGroupsWithoutAmountOrType<T> {
    #[deref]
    #[deref_mut]
    #[cfg_attr(feature = "storage", traversable(flatten))]
    pub core: UTXOGroupCore<T>,
    pub term: ByTerm<T>,
}

impl<T> UTXOGroupsWithoutAmountOrType<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            core: UTXOGroupCore::new(&mut create),
            term: ByTerm::new(&mut create),
        }
    }

    pub fn get(&self, filter: &Filter) -> Option<&T> {
        match filter {
            Filter::Term(term) => match term {
                Term::Sth => Some(&self.term.short),
                Term::Lth => Some(&self.term.long),
            },
            _ => self.core.get(filter),
        }
    }

    pub fn map_named<U>(
        &self,
        mut map: impl FnMut(&Filter, &'static str, &T) -> U,
    ) -> UTXOGroupsWithoutAmountOrType<U> {
        UTXOGroupsWithoutAmountOrType {
            core: self.core.map_named(&mut map),
            term: ByTerm::from_fn(|id| {
                map(
                    id.select(&TERM_FILTERS),
                    id.select(&TERM_NAMES).id,
                    id.select(&self.term),
                )
            }),
        }
    }
}
