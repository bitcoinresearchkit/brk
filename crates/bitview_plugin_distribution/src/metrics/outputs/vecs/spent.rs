use bitview_cohort::{CohortContext, UTXOGroups};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyPerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOColumns;

#[derive(Traversable)]
pub struct SpentOutputCount<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroups<LazyPerBlockCumulativeRolling<StoredU64>>,
    pub stored: CumulativeUTXOColumns<StoredU64, M>,
}

impl SpentOutputCount {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let version = version + Version::ONE;
        let stored = CumulativeUTXOColumns::forced_import(
            cache,
            db,
            "spent_utxo_count_cumulative",
            version,
        )?;
        let cohorts = UTXOGroups::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "spent_utxo_count");
            LazyPerBlockCumulativeRolling::from_cumulative_source(
                &name,
                version,
                &stored
                    .columns
                    .additive_source(&filter, &format!("{name}_cumulative"), version)
                    .expect("spent-output cohort source"),
                cached_starts,
                mappings,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
