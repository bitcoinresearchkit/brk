use bitview_cohort::{CohortContext, UTXOAndAddrGroups, UTXOGroups};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyFiatPerBlockWithDeltas};
use brk_error::Result;
use brk_types::{Cents, CentsSigned, PartsPerMillionSigned64, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::{ColumnarAmount, UTXOColumns};

#[derive(Traversable)]
pub struct RealizedCapByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOAndAddrGroups<
        LazyFiatPerBlockWithDeltas<Cents, CentsSigned, PartsPerMillionSigned64>,
        ColumnarAmount<
            Cents,
            LazyFiatPerBlockWithDeltas<Cents, CentsSigned, PartsPerMillionSigned64>,
            M,
        >,
    >,
    pub stored: UTXOColumns<Cents, M>,
}

impl RealizedCapByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let stored = UTXOColumns::forced_import(cache, db, "realized_cap_cents", version)?;
        let cohorts = UTXOGroups::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "realized_cap");
            LazyFiatPerBlockWithDeltas::from_cents_source(
                &name,
                version,
                &stored
                    .additive_source(&filter, &format!("{name}_cents"), version)
                    .expect("realized-cap cohort source"),
                Version::TWO,
                mappings,
                cached_starts,
            )
        });
        let addr_version = version + Version::ONE;
        let addr_balance = ColumnarAmount::forced_import(
            cache,
            db,
            "addrs_realized_cap_cents_by_balance_range",
            CohortContext::Addr,
            "realized_cap",
            addr_version,
            |name, source| {
                LazyFiatPerBlockWithDeltas::from_cents_source(
                    name,
                    addr_version,
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
            stored,
        })
    }
}
