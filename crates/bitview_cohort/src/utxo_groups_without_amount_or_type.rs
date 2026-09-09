use derive_more::{Deref, DerefMut};

use crate::{ByTerm, CohortId, UTXOGroupCore};

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
    pub fn try_new<E>(mut create: impl FnMut(CohortId) -> Result<T, E>) -> Result<Self, E> {
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
        F: FnMut(CohortId) -> T,
    {
        Self {
            core: UTXOGroupCore::new(&mut create),
            term: ByTerm::new(&mut create),
        }
    }

    pub fn get(&self, id: CohortId) -> Option<&T> {
        match id {
            CohortId::Term(term) => Some(self.term.get(term)),
            _ => self.core.get(id),
        }
    }

    pub fn map_with_id<U>(
        &self,
        mut map: impl FnMut(CohortId, &T) -> U,
    ) -> UTXOGroupsWithoutAmountOrType<U> {
        UTXOGroupsWithoutAmountOrType {
            core: self.core.map_with_id(&mut map),
            term: self.term.map_with_id(map),
        }
    }
}
