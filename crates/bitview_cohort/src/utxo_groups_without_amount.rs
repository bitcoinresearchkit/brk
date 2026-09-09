use derive_more::{Deref, DerefMut};

use crate::{ByTerm, CohortId, SpendableType, SpendableTypeId, UTXOGroupCore};

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
        F: FnMut(CohortId) -> T,
    {
        Self {
            core: UTXOGroupCore::new(&mut create),
            term: ByTerm::new(&mut create),
            type_: SpendableType::new(&mut create),
        }
    }

    pub fn get(&self, id: CohortId) -> Option<&T> {
        match id {
            CohortId::Term(term) => Some(self.term.get(term)),
            CohortId::Type(kind) => {
                SpendableTypeId::from_output_type(kind).map(|kind| kind.select(&self.type_))
            }
            _ => self.core.get(id),
        }
    }

    pub fn map_with_id<U>(
        &self,
        mut map: impl FnMut(CohortId, &T) -> U,
    ) -> UTXOGroupsWithoutAmount<U> {
        UTXOGroupsWithoutAmount {
            core: self.core.map_with_id(&mut map),
            term: self.term.map_with_id(&mut map),
            type_: self.type_.map_with_id(map),
        }
    }
}
