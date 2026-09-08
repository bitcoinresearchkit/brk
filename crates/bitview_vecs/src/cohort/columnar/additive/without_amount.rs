use std::ops::AddAssign;

use bitview_cohort::{Filter, SPENDABLE_TYPE_FILTERS, SpendableTypeId, UTXORows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, CachedBoxedVec, ColumnarVec, Database, EagerVec,
    ImportableVec, PcoVec, PcoVecValue, ReadOnlyClone, Rw, StorageMode, WritableVec,
};

use super::UTXOColumnarMetricWithoutAmountOrType;

#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOColumnarMetricWithoutAmount<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: UTXOColumnarMetricWithoutAmountOrType<T, M>,
    /// Height-indexed matrix with one column per spendable BRK output type, in
    /// canonical `SpendableTypeId` order.
    pub type_matrix: M::Stored<EagerVec<ColumnarVec<PcoVec<Height, T>, SpendableTypeId>>>,
}

impl<T> UTXOColumnarMetricWithoutAmount<T>
where
    T: PcoVecValue + AddAssign,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            core: UTXOColumnarMetricWithoutAmountOrType::forced_import(db, name, version)?,
            type_matrix: EagerVec::forced_import(
                db,
                &format!("{name}_by_type"),
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
            Filter::Type(output_type) => {
                SpendableTypeId::from_output_type(*output_type).map(|id| {
                    debug_assert_eq!(id.select(&SPENDABLE_TYPE_FILTERS), filter);
                    UTXOColumnarMetricWithoutAmountOrType::column(
                        cache,
                        &self.type_matrix.read_only_clone(),
                        name,
                        version,
                        id,
                    )
                })
            }
            _ => self.core.direct_source(cache, filter, name, version),
        }
    }

    pub fn min_len(&self) -> usize {
        self.core.min_len().min(self.type_matrix.len())
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXORows<T>) {
        let UTXORows { core, type_, .. } = rows;
        self.core.push(core);
        self.type_matrix.push(type_);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.type_matrix);
        vecs
    }
}
