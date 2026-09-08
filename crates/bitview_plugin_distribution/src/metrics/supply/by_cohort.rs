use bitview_cohort::{CohortContext, Filter, UTXOGroupsWithoutAmount, UTXORows};
use bitview_traversable::Traversable;
use bitview_vecs::LazySpotValuePerBlock;
use brk_error::Result;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{AnyStoredVec, CacheBudget, CachedBoxedVec, Database, Rw, StorageMode};

use crate::metrics::UTXOTypedColumns;

#[derive(Traversable)]
pub struct SupplyByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmount<LazySpotValuePerBlock>,
    pub stored: UTXOTypedColumns<Sats, M>,
}

impl SupplyByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let stored = UTXOTypedColumns::forced_import(db, &format!("{metric}_sats"), version)?;
        let cohorts = UTXOGroupsWithoutAmount::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let source = stored
                .additive_source(cache, &filter, &format!("{name}_sats"), version)
                .expect("supported supply cohort");
            LazySpotValuePerBlock::from_sats_source(&name, version, &source, mappings, spot_price)
        });

        Ok(Self { cohorts, stored })
    }

    pub fn get(&self, filter: &Filter) -> Option<&LazySpotValuePerBlock> {
        self.cohorts.get(filter)
    }

    pub fn min_len(&self) -> usize {
        self.stored.min_len()
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXORows<Sats>) {
        self.stored.push(rows.core, rows.type_);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.stored.collect_vecs_mut()
    }
}
