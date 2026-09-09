use std::ops::AddAssign;

use bitview_cohort::{
    CohortContext, Filter, SpendableType, SpendableTypeId, UTXOCoreValues, UTXOOverlappingValues,
};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, Database, PcoVecValue, Rw, StorageMode, WritableVec,
};

use super::UTXOCoreSources;
use crate::{StoredSeries, import_stored};

#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOTypedSources<T: PcoVecValue, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: UTXOCoreSources<T, M>,
    #[traversable(rename = "type")]
    pub type_: SpendableType<StoredSeries<Height, T, M>>,
}

impl<T: PcoVecValue + AddAssign> UTXOTypedSources<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            core: UTXOCoreSources::forced_import(cache, db, name, version)?,
            type_: SpendableType::try_new(|filter, cohort| {
                import_stored(
                    cache,
                    db,
                    &CohortContext::Utxo.metric_name(&filter, cohort, name),
                    version + Version::TWO,
                )
            })?,
        })
    }

    pub fn get(&self, filter: &Filter) -> Option<&StoredSeries<Height, T>> {
        match filter {
            Filter::Type(output_type) => {
                SpendableTypeId::from_output_type(*output_type).map(|id| id.select(&self.type_))
            }
            _ => self.core.get(filter),
        }
    }

    pub fn min_len(&self) -> usize {
        self.type_
            .iter()
            .map(AnyVec::len)
            .fold(self.core.min_len(), usize::min)
    }

    pub fn push(&mut self, core: UTXOCoreValues<T>, type_: SpendableType<T>) {
        self.push_with_overlapping(core, type_, None);
    }

    pub(crate) fn push_with_overlapping(
        &mut self,
        core: UTXOCoreValues<T>,
        type_: SpendableType<T>,
        overlapping: Option<&UTXOOverlappingValues<T>>,
    ) {
        self.core.push_with_overlapping(core, overlapping);
        for (target, &value) in self.type_.iter_mut().zip(type_.iter()) {
            target.push(value);
        }
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.extend(self.type_.iter_mut().map(|v| v as &mut dyn AnyStoredVec));
        vecs
    }
}
