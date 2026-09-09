use std::ops::AddAssign;

use bitview_cohort::{Filter, SpendableType, SpendableTypeId, UTXOCoreRows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, CacheBudget, Database, PcoVecValue, ReadableBoxedVec, Rw, StorageMode};

use super::UTXOCoreColumns;
use crate::ColumnarPerBlock;

/// Core UTXO columns extended with the spendable output-type axis.
#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOTypedColumns<T: PcoVecValue, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: UTXOCoreColumns<T, M>,
    #[traversable(rename = "type")]
    pub type_: ColumnarPerBlock<T, SpendableTypeId, (), M>,
}

impl<T: PcoVecValue + AddAssign> UTXOTypedColumns<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            core: UTXOCoreColumns::forced_import(cache, db, name, version)?,
            type_: ColumnarPerBlock::forced_import(
                cache,
                db,
                &format!("{name}_by_type"),
                version + Version::ONE,
                |_| (),
            )?,
        })
    }

    pub fn additive_source(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<ReadableBoxedVec<Height, T>> {
        self.direct_source(filter, name, version)
            .or_else(|| self.core.aggregate_source(filter, name, version))
    }

    pub(crate) fn direct_source(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<ReadableBoxedVec<Height, T>> {
        match filter {
            Filter::Type(output_type) => SpendableTypeId::from_output_type(*output_type)
                .map(|id| self.type_.column_source(name, version, id)),
            _ => self.core.direct_source(filter, name, version),
        }
    }

    pub fn min_len(&self) -> usize {
        self.core.min_len().min(self.type_.len())
    }

    #[inline(always)]
    pub fn push(&mut self, core: UTXOCoreRows<T>, type_: SpendableType<T>) {
        self.core.push(core);
        self.type_.push(type_);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(self.type_.stored_mut());
        vecs
    }
}
