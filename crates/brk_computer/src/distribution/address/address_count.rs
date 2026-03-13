use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode,
    WritableVec,
};

use crate::{indexes, internal::PerBlock};

#[derive(Deref, DerefMut, Traversable)]
pub struct AddressCountVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub PerBlock<StoredU64, M>,
);

impl AddressCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(PerBlock::forced_import(
            db, name, version, indexes,
        )?))
    }
}

/// Address count per address type (runtime state).
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToAddressCount(ByAddressType<u64>);

impl AddressTypeToAddressCount {
    #[inline]
    pub(crate) fn sum(&self) -> u64 {
        self.0.values().sum()
    }
}

impl From<(&AddressTypeToAddressCountVecs, Height)> for AddressTypeToAddressCount {
    #[inline]
    fn from((groups, starting_height): (&AddressTypeToAddressCountVecs, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            Self(ByAddressType {
                p2pk65: groups
                    .p2pk65
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2pk33: groups
                    .p2pk33
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2pkh: groups
                    .p2pkh
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2sh: groups
                    .p2sh
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2wpkh: groups
                    .p2wpkh
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2wsh: groups
                    .p2wsh
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2tr: groups
                    .p2tr
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
                p2a: groups
                    .p2a
                    .height
                    .collect_one(prev_height)
                    .unwrap()
                    .into(),
            })
        } else {
            Default::default()
        }
    }
}

/// Address count per address type, with height + derived indexes.
#[derive(Deref, DerefMut, Traversable)]
pub struct AddressTypeToAddressCountVecs<M: StorageMode = Rw>(ByAddressType<AddressCountVecs<M>>);

impl From<ByAddressType<AddressCountVecs>> for AddressTypeToAddressCountVecs {
    #[inline]
    fn from(value: ByAddressType<AddressCountVecs>) -> Self {
        Self(value)
    }
}

impl AddressTypeToAddressCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self::from(ByAddressType::<AddressCountVecs>::new_with_name(
            |type_name| {
                AddressCountVecs::forced_import(db, &format!("{type_name}_{name}"), version, indexes)
            },
        )?))
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.0.values().map(|v| v.height.len()).min().unwrap()
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.0
            .par_values_mut()
            .map(|v| &mut v.height as &mut dyn AnyStoredVec)
    }

    pub(crate) fn truncate_push_height(
        &mut self,
        height: Height,
        address_counts: &AddressTypeToAddressCount,
    ) -> Result<()> {
        for (vecs, &count) in self.0.values_mut().zip(address_counts.values()) {
            vecs.height.truncate_push(height, count.into())?;
        }
        Ok(())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        use vecdb::WritableVec;
        for v in self.0.values_mut() {
            v.height.reset()?;
        }
        Ok(())
    }

    pub(crate) fn by_height(&self) -> Vec<&EagerVec<PcoVec<Height, StoredU64>>> {
        self.0.values().map(|v| &v.height).collect()
    }
}

#[derive(Traversable)]
pub struct AddressCountsVecs<M: StorageMode = Rw> {
    pub all: AddressCountVecs<M>,
    #[traversable(flatten)]
    pub by_addresstype: AddressTypeToAddressCountVecs<M>,
}

impl AddressCountsVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            all: AddressCountVecs::forced_import(db, name, version, indexes)?,
            by_addresstype: AddressTypeToAddressCountVecs::forced_import(db, name, version, indexes)?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.all.height.len().min(self.by_addresstype.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.all.height as &mut dyn AnyStoredVec)
            .chain(self.by_addresstype.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.height.reset()?;
        self.by_addresstype.reset_height()?;
        Ok(())
    }

    pub(crate) fn truncate_push_height(
        &mut self,
        height: Height,
        total: u64,
        address_counts: &AddressTypeToAddressCount,
    ) -> Result<()> {
        self.all.height.truncate_push(height, total.into())?;
        self.by_addresstype
            .truncate_push_height(height, address_counts)?;
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let sources = self.by_addresstype.by_height();
        self.all
            .height
            .compute_sum_of_others(starting_indexes.height, &sources, exit)?;
        Ok(())
    }
}
