use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF64, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, PcoVec, TypedVecIterator,
};

use crate::{ComputeIndexes, indexes, internal::{ComputedFromDateLast, ComputedFromHeightLast}};

/// Address count with 30d change metric for a single type.
#[derive(Clone, Traversable)]
pub struct AddrCountVecs {
    #[traversable(flatten)]
    pub count: ComputedFromHeightLast<StoredU64>,
    pub _30d_change: ComputedFromDateLast<StoredF64>,
}

impl AddrCountVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            count: ComputedFromHeightLast::forced_import(db, name, version, indexes)?,
            _30d_change: ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_30d_change"),
                version,
                indexes,
            )?,
        })
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.count.compute_rest(indexes, starting_indexes, exit)?;

        self._30d_change
            .compute_all(starting_indexes, exit, |v| {
                v.compute_change(
                    starting_indexes.dateindex,
                    &*self.count.dateindex,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}

/// Address count per address type (runtime state).
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToAddressCount(ByAddressType<u64>);

impl AddressTypeToAddressCount {
    #[inline]
    pub fn sum(&self) -> u64 {
        self.0.values().sum()
    }
}

impl From<(&AddressTypeToAddrCountVecs, Height)> for AddressTypeToAddressCount {
    #[inline]
    fn from((groups, starting_height): (&AddressTypeToAddrCountVecs, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            Self(ByAddressType {
                p2pk65: groups
                    .p2pk65
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2pk33: groups
                    .p2pk33
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2pkh: groups
                    .p2pkh
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2sh: groups
                    .p2sh
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2wpkh: groups
                    .p2wpkh
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2wsh: groups
                    .p2wsh
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2tr: groups
                    .p2tr
                    .count
                    .height
                    .into_iter()
                    .get_unwrap(prev_height)
                    .into(),
                p2a: groups.p2a.count.height.into_iter().get_unwrap(prev_height).into(),
            })
        } else {
            Default::default()
        }
    }
}

/// Address count per address type, with height + derived indexes + 30d change.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct AddressTypeToAddrCountVecs(ByAddressType<AddrCountVecs>);

impl From<ByAddressType<AddrCountVecs>> for AddressTypeToAddrCountVecs {
    #[inline]
    fn from(value: ByAddressType<AddrCountVecs>) -> Self {
        Self(value)
    }
}

impl AddressTypeToAddrCountVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self::from(
            ByAddressType::<AddrCountVecs>::new_with_name(|type_name| {
                AddrCountVecs::forced_import(
                    db,
                    &format!("{type_name}_{name}"),
                    version,
                    indexes,
                )
            })?,
        ))
    }

    pub fn min_stateful_height(&self) -> usize {
        self.p2pk65
            .count
            .height
            .len()
            .min(self.p2pk33.count.height.len())
            .min(self.p2pkh.count.height.len())
            .min(self.p2sh.count.height.len())
            .min(self.p2wpkh.count.height.len())
            .min(self.p2wsh.count.height.len())
            .min(self.p2tr.count.height.len())
            .min(self.p2a.count.height.len())
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let inner = &mut self.0;
        [
            &mut inner.p2pk65.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2pk33.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2pkh.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2sh.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2wpkh.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2wsh.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2tr.count.height as &mut dyn AnyStoredVec,
            &mut inner.p2a.count.height as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    pub fn truncate_push_height(
        &mut self,
        height: Height,
        addr_counts: &AddressTypeToAddressCount,
    ) -> Result<()> {
        self.p2pk65
            .count
            .height
            .truncate_push(height, addr_counts.p2pk65.into())?;
        self.p2pk33
            .count
            .height
            .truncate_push(height, addr_counts.p2pk33.into())?;
        self.p2pkh
            .count
            .height
            .truncate_push(height, addr_counts.p2pkh.into())?;
        self.p2sh
            .count
            .height
            .truncate_push(height, addr_counts.p2sh.into())?;
        self.p2wpkh
            .count
            .height
            .truncate_push(height, addr_counts.p2wpkh.into())?;
        self.p2wsh
            .count
            .height
            .truncate_push(height, addr_counts.p2wsh.into())?;
        self.p2tr
            .count
            .height
            .truncate_push(height, addr_counts.p2tr.into())?;
        self.p2a
            .count
            .height
            .truncate_push(height, addr_counts.p2a.into())?;
        Ok(())
    }

    pub fn reset_height(&mut self) -> Result<()> {
        use vecdb::GenericStoredVec;
        self.p2pk65.count.height.reset()?;
        self.p2pk33.count.height.reset()?;
        self.p2pkh.count.height.reset()?;
        self.p2sh.count.height.reset()?;
        self.p2wpkh.count.height.reset()?;
        self.p2wsh.count.height.reset()?;
        self.p2tr.count.height.reset()?;
        self.p2a.count.height.reset()?;
        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.p2pk65.compute_rest(indexes, starting_indexes, exit)?;
        self.p2pk33.compute_rest(indexes, starting_indexes, exit)?;
        self.p2pkh.compute_rest(indexes, starting_indexes, exit)?;
        self.p2sh.compute_rest(indexes, starting_indexes, exit)?;
        self.p2wpkh.compute_rest(indexes, starting_indexes, exit)?;
        self.p2wsh.compute_rest(indexes, starting_indexes, exit)?;
        self.p2tr.compute_rest(indexes, starting_indexes, exit)?;
        self.p2a.compute_rest(indexes, starting_indexes, exit)?;
        Ok(())
    }

    pub fn by_height(&self) -> Vec<&EagerVec<PcoVec<Height, StoredU64>>> {
        vec![
            &self.p2pk65.count.height,
            &self.p2pk33.count.height,
            &self.p2pkh.count.height,
            &self.p2sh.count.height,
            &self.p2wpkh.count.height,
            &self.p2wsh.count.height,
            &self.p2tr.count.height,
            &self.p2a.count.height,
        ]
    }
}

#[derive(Clone, Traversable)]
pub struct AddrCountsVecs {
    pub all: AddrCountVecs,
    #[traversable(flatten)]
    pub by_addresstype: AddressTypeToAddrCountVecs,
}

impl AddrCountsVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            all: AddrCountVecs::forced_import(db, name, version, indexes)?,
            by_addresstype: AddressTypeToAddrCountVecs::forced_import(db, name, version, indexes)?,
        })
    }

    pub fn min_stateful_height(&self) -> usize {
        self.all.count.height.len().min(self.by_addresstype.min_stateful_height())
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.all.count.height as &mut dyn AnyStoredVec)
            .chain(self.by_addresstype.par_iter_height_mut())
    }

    pub fn reset_height(&mut self) -> Result<()> {
        self.all.count.height.reset()?;
        self.by_addresstype.reset_height()?;
        Ok(())
    }

    pub fn truncate_push_height(
        &mut self,
        height: Height,
        total: u64,
        addr_counts: &AddressTypeToAddressCount,
    ) -> Result<()> {
        self.all.count.height.truncate_push(height, total.into())?;
        self.by_addresstype
            .truncate_push_height(height, addr_counts)?;
        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.by_addresstype
            .compute_rest(indexes, starting_indexes, exit)?;

        let sources = self.by_addresstype.by_height();
        self.all
            .count
            .compute_all(indexes, starting_indexes, exit, |height_vec| {
                Ok(height_vec.compute_sum_of_others(starting_indexes.height, &sources, exit)?)
            })?;

        self.all._30d_change
            .compute_all(starting_indexes, exit, |v| {
                v.compute_change(
                    starting_indexes.dateindex,
                    &*self.all.count.dateindex,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
