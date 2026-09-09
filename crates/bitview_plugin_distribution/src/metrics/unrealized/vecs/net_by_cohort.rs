use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmountOrType};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::LazyFiatPerBlock;
use brk_error::Result;
use brk_types::{CentsSigned, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::UTXOCoreColumns;

#[derive(Traversable)]
pub struct NetUnrealizedByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<LazyFiatPerBlock<CentsSigned>>,
    pub stored: UTXOCoreColumns<CentsSigned, M>,
}

impl NetUnrealizedByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
    ) -> Result<Self> {
        let metric = "net_unrealized_pnl";
        let stored =
            UTXOCoreColumns::forced_import(cache, db, "net_unrealized_pnl_cents", version)?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let source = stored
                .additive_source(&filter, &format!("{name}_cents"), version)
                .expect("supported net unrealized cohort");
            LazyFiatPerBlock::from_cents_source(&name, version, &source, mappings)
        });
        Ok(Self { cohorts, stored })
    }
}
