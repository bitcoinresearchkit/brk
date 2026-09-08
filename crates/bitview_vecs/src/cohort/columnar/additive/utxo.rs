use std::ops::AddAssign;

use bitview_cohort::{AmountRangeId, Filter, OVER_AMOUNT_FILTERS, UNDER_AMOUNT_FILTERS, UTXORows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, CacheBudget, CachedBoxedVec, Database, PcoVecValue, ReadableVec, Rw, StorageMode,
};

use super::UTXOTypedColumns;
use crate::ColumnarPerBlock;

/// All disjoint UTXO axes: core, output type, and individual output amount.
#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOColumns<T: PcoVecValue, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub typed: UTXOTypedColumns<T, M>,
    pub amount_range: ColumnarPerBlock<T, AmountRangeId, (), M>,
}

impl<T: PcoVecValue + AddAssign> UTXOColumns<T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            typed: UTXOTypedColumns::forced_import(db, name, version)?,
            amount_range: ColumnarPerBlock::forced_import(
                db,
                &format!("utxos_{name}_by_amount_range"),
                version + Version::ONE,
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
            .or_else(|| self.aggregate_amount_source(cache, filter, name, version))
            .or_else(|| self.typed.aggregate_source(cache, filter, name, version))
    }

    pub(crate) fn direct_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        match filter {
            Filter::Amount(_) => AmountRangeId::matching(filter)
                .map(|id| self.amount_range.cached_column(cache, name, version, id)),
            _ => self.typed.direct_source(cache, filter, name, version),
        }
    }

    fn aggregate_amount_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        let filter = UNDER_AMOUNT_FILTERS
            .iter()
            .chain(OVER_AMOUNT_FILTERS.iter())
            .find(|candidate| *candidate == filter)?;
        Some(
            self.amount_range
                .cached_sum(cache, name, version, AmountRangeId::included_by(filter)),
        )
    }

    pub fn min_len(&self) -> usize {
        self.typed.min_len().min(self.amount_range.len())
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXORows<T>) {
        self.typed.push(rows.core, rows.type_);
        self.amount_range.push(rows.amount_range);
    }

    pub fn collect_last(&self) -> Option<UTXORows<T>>
    where
        T: Default,
    {
        Some(UTXORows {
            amount_range: self.amount_range.height.collect_last()?,
            type_: self.type_.height.collect_last()?,
            core: self.typed.core.collect_last()?,
        })
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.typed.collect_vecs_mut();
        vecs.push(self.amount_range.stored_mut());
        vecs
    }
}
