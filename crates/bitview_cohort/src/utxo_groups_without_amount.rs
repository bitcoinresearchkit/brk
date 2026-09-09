use derive_more::{Deref, DerefMut};

use crate::{
    ByTerm, Filter, SPENDABLE_TYPE_FILTERS, SPENDABLE_TYPE_NAMES, SpendableType, TERM_FILTERS,
    TERM_NAMES, Term, UTXOGroupCore,
};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

#[derive(Default, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOGroupsWithoutAmount<T> {
    #[deref]
    #[deref_mut]
    #[cfg_attr(feature = "storage", traversable(flatten))]
    pub core: UTXOGroupCore<T>,
    pub term: ByTerm<T>,
    #[cfg_attr(feature = "storage", traversable(rename = "type"))]
    pub type_: SpendableType<T>,
}

impl<T> UTXOGroupsWithoutAmount<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            core: UTXOGroupCore::new(&mut create),
            term: ByTerm::new(&mut create),
            type_: SpendableType::new(&mut create),
        }
    }

    pub fn get(&self, filter: &Filter) -> Option<&T> {
        match filter {
            Filter::Term(term) => match term {
                Term::Sth => Some(&self.term.short),
                Term::Lth => Some(&self.term.long),
            },
            Filter::Type(output_type) => Some(self.type_.get(*output_type)),
            _ => self.core.get(filter),
        }
    }

    pub fn map_named<U>(
        &self,
        mut map: impl FnMut(&Filter, &'static str, &T) -> U,
    ) -> UTXOGroupsWithoutAmount<U> {
        UTXOGroupsWithoutAmount {
            core: self.core.map_named(&mut map),
            term: ByTerm::from_fn(|id| {
                map(
                    id.select(&TERM_FILTERS),
                    id.select(&TERM_NAMES).id,
                    id.select(&self.term),
                )
            }),
            type_: SpendableType::from_fn(|id| {
                map(
                    id.select(&SPENDABLE_TYPE_FILTERS),
                    id.select(&SPENDABLE_TYPE_NAMES).id,
                    id.select(&self.type_),
                )
            }),
        }
    }
}
