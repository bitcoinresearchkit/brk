use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmountOrType};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyFiatPerBlockCumulativeWithSumsAndDeltas};
use brk_error::Result;
use brk_types::{CentsSigned, PartsPerMillionSigned64, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOCoreColumns;

#[derive(Traversable)]
pub struct CumulativeNetRealizedByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<
        LazyFiatPerBlockCumulativeWithSumsAndDeltas<
            CentsSigned,
            CentsSigned,
            PartsPerMillionSigned64,
        >,
    >,
    pub stored: CumulativeUTXOCoreColumns<CentsSigned, M>,
}

impl CumulativeNetRealizedByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let version = version + Version::ONE;
        let stored = CumulativeUTXOCoreColumns::forced_import(
            cache,
            db,
            "net_realized_pnl_cumulative_cents",
            version,
        )?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "net_realized_pnl");
            let source = stored
                .columns
                .additive_source(&filter, &format!("{name}_cumulative_cents"), version)
                .expect("supported net realized cohort");
            LazyFiatPerBlockCumulativeWithSumsAndDeltas::from_cumulative_cents_source(
                &name,
                version,
                &source,
                Version::new(5),
                mappings,
                cached_starts,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
