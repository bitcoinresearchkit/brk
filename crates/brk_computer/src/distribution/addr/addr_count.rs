use brk_cohort::ByAddrType;
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
pub struct AddrCountVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub PerBlock<StoredU64, M>,
);

impl AddrCountVecs {
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
pub struct AddrTypeToAddrCount(ByAddrType<u64>);

impl AddrTypeToAddrCount {
    #[inline]
    pub(crate) fn sum(&self) -> u64 {
        self.0.values().sum()
    }
}

impl From<(&AddrTypeToAddrCountVecs, Height)> for AddrTypeToAddrCount {
    #[inline]
    fn from((groups, starting_height): (&AddrTypeToAddrCountVecs, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            Self(ByAddrType {
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
pub struct AddrTypeToAddrCountVecs<M: StorageMode = Rw>(ByAddrType<AddrCountVecs<M>>);

impl From<ByAddrType<AddrCountVecs>> for AddrTypeToAddrCountVecs {
    #[inline]
    fn from(value: ByAddrType<AddrCountVecs>) -> Self {
        Self(value)
    }
}

impl AddrTypeToAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self::from(ByAddrType::<AddrCountVecs>::new_with_name(
            |type_name| {
                AddrCountVecs::forced_import(db, &format!("{type_name}_{name}"), version, indexes)
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

    #[inline(always)]
    pub(crate) fn push_height(&mut self, addr_counts: &AddrTypeToAddrCount) {
        for (vecs, &count) in self.0.values_mut().zip(addr_counts.values()) {
            vecs.height.push(count.into());
        }
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
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
pub struct AddrCountsVecs<M: StorageMode = Rw> {
    pub all: AddrCountVecs<M>,
    #[traversable(flatten)]
    pub by_addr_type: AddrTypeToAddrCountVecs<M>,
}

impl AddrCountsVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            all: AddrCountVecs::forced_import(db, name, version, indexes)?,
            by_addr_type: AddrTypeToAddrCountVecs::forced_import(db, name, version, indexes)?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.all.height.len().min(self.by_addr_type.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.all.height as &mut dyn AnyStoredVec)
            .chain(self.by_addr_type.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.height.reset()?;
        self.by_addr_type.reset_height()?;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height(&mut self, total: u64, addr_counts: &AddrTypeToAddrCount) {
        self.all.height.push(total.into());
        self.by_addr_type.push_height(addr_counts);
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let sources = self.by_addr_type.by_height();
        self.all
            .height
            .compute_sum_of_others(starting_indexes.height, &sources, exit)?;
        Ok(())
    }
}
