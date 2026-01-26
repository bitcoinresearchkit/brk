//! Address activity tracking - per-block counts of address behaviors.
//!
//! Tracks global and per-address-type activity metrics:
//!
//! | Metric | Description |
//! |--------|-------------|
//! | `receiving` | Unique addresses that received this block |
//! | `sending` | Unique addresses that sent this block |
//! | `reactivated` | Addresses that were empty and now have funds |
//! | `both` | Addresses that both sent AND received same block |
//! | `balance_increased` | Receive-only addresses (balance definitely increased) |
//! | `balance_decreased` | Send-only addresses (balance definitely decreased) |
//!
//! Note: `balance_increased` and `balance_decreased` exclude "both" addresses
//! since their net balance change requires more complex tracking.

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU32, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec};

use crate::{ComputeIndexes, indexes, internal::ComputedFromHeightDistribution};

/// Per-block activity counts - reset each block.
///
/// Note: `balance_increased` and `balance_decreased` are derived:
/// - `balance_increased = receiving - both` (receive-only addresses)
/// - `balance_decreased = sending - both` (send-only addresses)
#[derive(Debug, Default, Clone)]
pub struct BlockActivityCounts {
    pub reactivated: u32,
    pub sending: u32,
    pub receiving: u32,
    pub both: u32,
}

impl BlockActivityCounts {
    /// Reset all counts to zero.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Per-address-type activity counts - aggregated during block processing.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToActivityCounts(pub ByAddressType<BlockActivityCounts>);

impl AddressTypeToActivityCounts {
    /// Reset all per-type counts.
    pub fn reset(&mut self) {
        self.0.values_mut().for_each(|v| v.reset());
    }

    /// Sum all types to get totals.
    pub fn totals(&self) -> BlockActivityCounts {
        let mut total = BlockActivityCounts::default();
        for counts in self.0.values() {
            total.reactivated += counts.reactivated;
            total.sending += counts.sending;
            total.receiving += counts.receiving;
            total.both += counts.both;
        }
        total
    }
}

/// Activity count vectors for a single category (e.g., one address type or "all").
#[derive(Clone, Traversable)]
pub struct ActivityCountVecs {
    pub reactivated: ComputedFromHeightDistribution<StoredU32>,
    pub sending: ComputedFromHeightDistribution<StoredU32>,
    pub receiving: ComputedFromHeightDistribution<StoredU32>,
    pub balance_increased: ComputedFromHeightDistribution<StoredU32>,
    pub balance_decreased: ComputedFromHeightDistribution<StoredU32>,
    pub both: ComputedFromHeightDistribution<StoredU32>,
}

impl ActivityCountVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            reactivated: ComputedFromHeightDistribution::forced_import(
                db,
                &format!("{name}_reactivated"),
                version,
                indexes,
            )?,
            sending: ComputedFromHeightDistribution::forced_import(
                db,
                &format!("{name}_sending"),
                version,
                indexes,
            )?,
            receiving: ComputedFromHeightDistribution::forced_import(
                db,
                &format!("{name}_receiving"),
                version,
                indexes,
            )?,
            balance_increased: ComputedFromHeightDistribution::forced_import(
                db,
                &format!("{name}_balance_increased"),
                version,
                indexes,
            )?,
            balance_decreased: ComputedFromHeightDistribution::forced_import(
                db,
                &format!("{name}_balance_decreased"),
                version,
                indexes,
            )?,
            both: ComputedFromHeightDistribution::forced_import(
                db,
                &format!("{name}_both"),
                version,
                indexes,
            )?,
        })
    }

    pub fn min_stateful_height(&self) -> usize {
        self.reactivated
            .height
            .len()
            .min(self.sending.height.len())
            .min(self.receiving.height.len())
            .min(self.balance_increased.height.len())
            .min(self.balance_decreased.height.len())
            .min(self.both.height.len())
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.reactivated.height as &mut dyn AnyStoredVec,
            &mut self.sending.height as &mut dyn AnyStoredVec,
            &mut self.receiving.height as &mut dyn AnyStoredVec,
            &mut self.balance_increased.height as &mut dyn AnyStoredVec,
            &mut self.balance_decreased.height as &mut dyn AnyStoredVec,
            &mut self.both.height as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    pub fn reset_height(&mut self) -> Result<()> {
        self.reactivated.height.reset()?;
        self.sending.height.reset()?;
        self.receiving.height.reset()?;
        self.balance_increased.height.reset()?;
        self.balance_decreased.height.reset()?;
        self.both.height.reset()?;
        Ok(())
    }

    pub fn truncate_push_height(
        &mut self,
        height: Height,
        counts: &BlockActivityCounts,
    ) -> Result<()> {
        self.reactivated
            .height
            .truncate_push(height, counts.reactivated.into())?;
        self.sending
            .height
            .truncate_push(height, counts.sending.into())?;
        self.receiving
            .height
            .truncate_push(height, counts.receiving.into())?;
        // Derived: balance_increased = receiving - both (receive-only addresses)
        self.balance_increased
            .height
            .truncate_push(height, (counts.receiving - counts.both).into())?;
        // Derived: balance_decreased = sending - both (send-only addresses)
        self.balance_decreased
            .height
            .truncate_push(height, (counts.sending - counts.both).into())?;
        self.both
            .height
            .truncate_push(height, counts.both.into())?;
        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.reactivated
            .compute_rest(indexes, starting_indexes, exit)?;
        self.sending
            .compute_rest(indexes, starting_indexes, exit)?;
        self.receiving
            .compute_rest(indexes, starting_indexes, exit)?;
        self.balance_increased
            .compute_rest(indexes, starting_indexes, exit)?;
        self.balance_decreased
            .compute_rest(indexes, starting_indexes, exit)?;
        self.both.compute_rest(indexes, starting_indexes, exit)?;
        Ok(())
    }
}

/// Per-address-type activity count vecs.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct AddressTypeToActivityCountVecs(ByAddressType<ActivityCountVecs>);

impl From<ByAddressType<ActivityCountVecs>> for AddressTypeToActivityCountVecs {
    #[inline]
    fn from(value: ByAddressType<ActivityCountVecs>) -> Self {
        Self(value)
    }
}

impl AddressTypeToActivityCountVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self::from(
            ByAddressType::<ActivityCountVecs>::new_with_name(|type_name| {
                ActivityCountVecs::forced_import(db, &format!("{type_name}_{name}"), version, indexes)
            })?,
        ))
    }

    pub fn min_stateful_height(&self) -> usize {
        self.0.values().map(|v| v.min_stateful_height()).min().unwrap_or(0)
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let inner = &mut self.0;
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        for type_vecs in [
            &mut inner.p2pk65,
            &mut inner.p2pk33,
            &mut inner.p2pkh,
            &mut inner.p2sh,
            &mut inner.p2wpkh,
            &mut inner.p2wsh,
            &mut inner.p2tr,
            &mut inner.p2a,
        ] {
            vecs.push(&mut type_vecs.reactivated.height);
            vecs.push(&mut type_vecs.sending.height);
            vecs.push(&mut type_vecs.receiving.height);
            vecs.push(&mut type_vecs.balance_increased.height);
            vecs.push(&mut type_vecs.balance_decreased.height);
            vecs.push(&mut type_vecs.both.height);
        }
        vecs.into_par_iter()
    }

    pub fn reset_height(&mut self) -> Result<()> {
        self.p2pk65.reset_height()?;
        self.p2pk33.reset_height()?;
        self.p2pkh.reset_height()?;
        self.p2sh.reset_height()?;
        self.p2wpkh.reset_height()?;
        self.p2wsh.reset_height()?;
        self.p2tr.reset_height()?;
        self.p2a.reset_height()?;
        Ok(())
    }

    pub fn truncate_push_height(
        &mut self,
        height: Height,
        counts: &AddressTypeToActivityCounts,
    ) -> Result<()> {
        self.p2pk65
            .truncate_push_height(height, &counts.p2pk65)?;
        self.p2pk33
            .truncate_push_height(height, &counts.p2pk33)?;
        self.p2pkh
            .truncate_push_height(height, &counts.p2pkh)?;
        self.p2sh.truncate_push_height(height, &counts.p2sh)?;
        self.p2wpkh
            .truncate_push_height(height, &counts.p2wpkh)?;
        self.p2wsh
            .truncate_push_height(height, &counts.p2wsh)?;
        self.p2tr.truncate_push_height(height, &counts.p2tr)?;
        self.p2a.truncate_push_height(height, &counts.p2a)?;
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
}

/// Storage for activity metrics (global + per type).
#[derive(Clone, Traversable)]
pub struct AddressActivityVecs {
    pub all: ActivityCountVecs,
    #[traversable(flatten)]
    pub by_addresstype: AddressTypeToActivityCountVecs,
}

impl AddressActivityVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            all: ActivityCountVecs::forced_import(db, name, version, indexes)?,
            by_addresstype: AddressTypeToActivityCountVecs::forced_import(
                db, name, version, indexes,
            )?,
        })
    }

    pub fn min_stateful_height(&self) -> usize {
        self.all.min_stateful_height().min(self.by_addresstype.min_stateful_height())
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.all
            .par_iter_height_mut()
            .chain(self.by_addresstype.par_iter_height_mut())
    }

    pub fn reset_height(&mut self) -> Result<()> {
        self.all.reset_height()?;
        self.by_addresstype.reset_height()?;
        Ok(())
    }

    pub fn truncate_push_height(
        &mut self,
        height: Height,
        counts: &AddressTypeToActivityCounts,
    ) -> Result<()> {
        let totals = counts.totals();
        self.all.truncate_push_height(height, &totals)?;
        self.by_addresstype.truncate_push_height(height, counts)?;
        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute_rest(indexes, starting_indexes, exit)?;
        self.by_addresstype
            .compute_rest(indexes, starting_indexes, exit)?;
        Ok(())
    }
}
