use std::ops::AddAssign;

use bitview_cohort::{AmountRangeId, Filter, OVER_AMOUNT_FILTERS, UNDER_AMOUNT_FILTERS, UTXORows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, CachedBoxedVec, ColumnarVec, Database, EagerVec,
    ImportableVec, PcoVec, PcoVecValue, ReadOnlyClone, ReadableVec, Rw, StorageMode, WritableVec,
};

use super::{UTXOColumnarMetricWithoutAmount, UTXOColumnarMetricWithoutAmountOrType};

#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOColumnarMetric<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: UTXOColumnarMetricWithoutAmount<T, M>,
    /// Height-indexed matrix with one column per exact UTXO value range,
    /// ordered from smallest to largest.
    pub amount_range_matrix: M::Stored<EagerVec<ColumnarVec<PcoVec<Height, T>, AmountRangeId>>>,
}

impl<T> UTXOColumnarMetric<T>
where
    T: PcoVecValue + AddAssign,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            core: UTXOColumnarMetricWithoutAmount::forced_import(db, name, version)?,
            amount_range_matrix: EagerVec::forced_import(
                db,
                &format!("utxos_{name}_by_amount_range"),
                version + Version::ONE,
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
            .or_else(|| self.core.aggregate_source(cache, filter, name, version))
    }

    pub(crate) fn direct_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        match filter {
            Filter::Amount(_) => AmountRangeId::matching(filter).map(|id| {
                UTXOColumnarMetricWithoutAmountOrType::column(
                    cache,
                    &self.amount_range_matrix.read_only_clone(),
                    name,
                    version,
                    id,
                )
            }),
            _ => self.core.direct_source(cache, filter, name, version),
        }
    }

    fn aggregate_amount_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        let matrix = self.amount_range_matrix.read_only_clone();
        match filter {
            Filter::Amount(_) => UNDER_AMOUNT_FILTERS
                .iter()
                .chain(OVER_AMOUNT_FILTERS.iter())
                .find(|candidate| *candidate == filter)
                .map(|filter| {
                    UTXOColumnarMetricWithoutAmountOrType::sum(
                        cache,
                        &matrix,
                        name,
                        version,
                        AmountRangeId::included_by(filter),
                    )
                }),
            _ => None,
        }
    }

    pub fn min_len(&self) -> usize {
        self.core.min_len().min(self.amount_range_matrix.len())
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXORows<T>) {
        let UTXORows {
            core,
            amount_range,
            type_,
        } = rows;
        self.core.core.push(core);
        self.type_matrix.push(type_);
        self.amount_range_matrix.push(amount_range);
    }

    pub fn collect_last(&self) -> Option<UTXORows<T>>
    where
        T: Default,
    {
        Some(UTXORows {
            amount_range: self.amount_range_matrix.collect_last()?,
            type_: self.type_matrix.collect_last()?,
            core: self.core.core.collect_last()?,
        })
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.amount_range_matrix);
        vecs
    }
}
