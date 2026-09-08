use bitview_cohort::{AmountRange, CohortContext, UTXOAndAddrGroups, UTXOGroups};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyPerBlockWithDeltas};
use brk_error::Result;
use brk_types::{PartsPerMillionSigned64, StoredI64, StoredU64, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::{ColumnarAmount, UTXOColumnarMetric};

#[derive(Traversable)]
pub struct UnspentOutputCount<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOAndAddrGroups<
        LazyPerBlockWithDeltas<StoredU64, StoredI64, PartsPerMillionSigned64>,
        ColumnarAmount<
            StoredU64,
            LazyPerBlockWithDeltas<StoredU64, StoredI64, PartsPerMillionSigned64>,
            M,
        >,
    >,
    #[traversable(flatten)]
    pub matrices: UTXOColumnarMetric<StoredU64, M>,
}

impl UnspentOutputCount {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let matrices = UTXOColumnarMetric::forced_import(db, "utxo_count", version)?;
        let cohorts = UTXOGroups::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "utxo_count");
            LazyPerBlockWithDeltas::from_height_source(
                &name,
                version,
                &matrices
                    .additive_source(cache, &filter, &name, version)
                    .expect("unspent-output cohort source"),
                Version::TWO,
                mappings,
                cached_starts,
            )
        });
        let addr_balance = ColumnarAmount::forced_import(
            cache,
            db,
            "addrs_utxo_count_by_balance_range",
            CohortContext::Addr,
            "utxo_count",
            version + Version::ONE,
            |name, source| {
                LazyPerBlockWithDeltas::from_height_source(
                    name,
                    version + Version::ONE,
                    source,
                    Version::TWO,
                    mappings,
                    cached_starts,
                )
            },
        )?;
        Ok(Self {
            cohorts: UTXOAndAddrGroups {
                utxo: cohorts,
                addr_balance,
            },
            matrices,
        })
    }

    #[inline(always)]
    pub fn push_addr_balance(&mut self, row: AmountRange<StoredU64>) {
        self.cohorts.addr_balance.push(row);
    }
}
