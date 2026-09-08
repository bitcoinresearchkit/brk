use std::ops::AddAssign;

use bitview_cohort::{AgeRangeId, ClassId, EntryId, EpochId, Filter, UTXOCoreRows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{
    AnyStoredVec, CacheBudget, CachedBoxedVec, Database, PcoVecValue, ReadableVec, Rw, StorageMode,
};

use crate::ColumnarPerBlock;

/// Disjoint UTXO age, epoch, creation-year, and entry-price columns.
/// Aggregate cohorts are read from the relevant age columns, not stored twice.
#[derive(Traversable)]
pub struct UTXOCoreColumns<T: PcoVecValue, M: StorageMode = Rw> {
    /// Exact age ranges, youngest to oldest.
    pub age_range: ColumnarPerBlock<T, AgeRangeId, (), M>,
    /// Subsidy-halving epochs, earliest to latest.
    pub epoch: ColumnarPerBlock<T, EpochId, (), M>,
    /// Output-creation years, in chronological order.
    pub class: ColumnarPerBlock<T, ClassId, (), M>,
    /// Discount-entry followed by premium-entry UTXOs.
    pub entry: ColumnarPerBlock<T, EntryId, (), M>,
}

impl<T: PcoVecValue + AddAssign> UTXOCoreColumns<T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let version = version + Version::ONE;
        Ok(Self {
            age_range: ColumnarPerBlock::forced_import(
                db,
                &format!("utxos_{name}_by_age_range"),
                version,
                |_| (),
            )?,
            epoch: ColumnarPerBlock::forced_import(
                db,
                &format!("{name}_by_epoch"),
                version,
                |_| (),
            )?,
            class: ColumnarPerBlock::forced_import(
                db,
                &format!("{name}_by_class"),
                version,
                |_| (),
            )?,
            entry: ColumnarPerBlock::forced_import(
                db,
                &format!("{name}_by_entry"),
                version,
                |_| (),
            )?,
        })
    }

    pub fn additive_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        self.direct_source(cache, filter, name, version)
            .or_else(|| self.aggregate_source(cache, filter, name, version))
    }

    pub(crate) fn direct_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        match filter {
            Filter::Time(_) => AgeRangeId::matching(filter)
                .map(|id| self.age_range.cached_column(cache, name, version, id)),
            Filter::Epoch(_) => EpochId::matching(filter)
                .map(|id| self.epoch.cached_column(cache, name, version, id)),
            Filter::Class(_) => ClassId::matching(filter)
                .map(|id| self.class.cached_column(cache, name, version, id)),
            Filter::Entry(_) => EntryId::matching(filter)
                .map(|id| self.entry.cached_column(cache, name, version, id)),
            Filter::All | Filter::Term(_) | Filter::Amount(_) | Filter::Type(_) => None,
        }
    }

    pub(crate) fn aggregate_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        Some(self.age_range.cached_sum(
            cache,
            name,
            version,
            AgeRangeId::aggregate_columns(filter)?,
        ))
    }

    pub fn min_len(&self) -> usize {
        self.age_range
            .len()
            .min(self.epoch.len())
            .min(self.class.len())
            .min(self.entry.len())
    }

    #[inline(always)]
    pub fn push(&mut self, rows: impl Into<UTXOCoreRows<T>>) {
        let rows = rows.into();
        self.age_range.push(rows.age_range);
        self.epoch.push(rows.epoch);
        self.class.push(rows.class);
        self.entry.push(rows.entry);
    }

    pub fn collect_last(&self) -> Option<UTXOCoreRows<T>>
    where
        T: Default,
    {
        Some(UTXOCoreRows {
            age_range: self.age_range.height.collect_last()?,
            epoch: self.epoch.height.collect_last()?,
            class: self.class.height.collect_last()?,
            entry: self.entry.height.collect_last()?,
        })
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            self.age_range.stored_mut(),
            self.epoch.stored_mut(),
            self.class.stored_mut(),
            self.entry.stored_mut(),
        ]
    }
}
