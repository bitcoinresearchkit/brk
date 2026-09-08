#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

use crate::{
    ByAge, ByEntry, ByEpoch, CLASS_FILTERS, CLASS_NAMES, Class, ClassId, ENTRY_FILTERS,
    ENTRY_NAMES, EPOCH_FILTERS, EPOCH_NAMES, EntryId, EpochId, Filter,
};

#[derive(Default, Clone)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOGroupCore<T> {
    /// Uses all UTXOs.
    pub all: T,
    pub age: ByAge<T>,
    pub epoch: ByEpoch<T>,
    pub class: Class<T>,
    pub entry: ByEntry<T>,
}

impl<T> UTXOGroupCore<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            all: create(Filter::All, ""),
            age: ByAge::new(&mut create),
            epoch: ByEpoch::new(&mut create),
            class: Class::new(&mut create),
            entry: ByEntry::new(&mut create),
        }
    }

    pub fn get(&self, filter: &Filter) -> Option<&T> {
        match filter {
            Filter::All => Some(&self.all),
            Filter::Time(_) => self.age.get(filter),
            Filter::Epoch(_) => EpochId::matching(filter).map(|id| id.select(&self.epoch)),
            Filter::Class(_) => ClassId::matching(filter).map(|id| id.select(&self.class)),
            Filter::Entry(_) => EntryId::matching(filter).map(|id| id.select(&self.entry)),
            Filter::Term(_) | Filter::Amount(_) | Filter::Type(_) => None,
        }
    }

    pub fn map_named<U>(
        &self,
        mut map: impl FnMut(&Filter, &'static str, &T) -> U,
    ) -> UTXOGroupCore<U> {
        UTXOGroupCore {
            all: map(&Filter::All, "", &self.all),
            age: self.age.map_named(&mut map),
            epoch: ByEpoch::from_fn(|id| {
                map(
                    id.select(&EPOCH_FILTERS),
                    id.select(&EPOCH_NAMES).id,
                    id.select(&self.epoch),
                )
            }),
            class: Class::from_fn(|id| {
                map(
                    id.select(&CLASS_FILTERS),
                    id.select(&CLASS_NAMES).id,
                    id.select(&self.class),
                )
            }),
            entry: ByEntry::from_fn(|id| {
                map(
                    id.select(&ENTRY_FILTERS),
                    id.select(&ENTRY_NAMES).id,
                    id.select(&self.entry),
                )
            }),
        }
    }
}
