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
    pub fn try_new<E>(
        mut create: impl FnMut(Filter, &'static str) -> Result<T, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            core: UTXOGroupCore::try_new(&mut create)?,
            term: ByTerm::try_new(create)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.core.iter().chain(self.term.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.core.iter_mut().chain(self.term.iter_mut())
    }

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
