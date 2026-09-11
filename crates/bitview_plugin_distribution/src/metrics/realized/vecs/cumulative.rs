use bitview_cohort::{CohortContext, UTXOAndAddrGroups, UTXOGroups};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{LazyFiatPerBlockCumulativeWithSums, LazyWindowStartVec};
use brk_error::Result;
use brk_types::{Cents, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::{AmountSources, CumulativeUTXOSources};

#[derive(Traversable)]
pub struct CumulativeRealizedByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    /// Includes spends grouped by the address's pre-spend balance.
    pub cohorts: UTXOAndAddrGroups<
        LazyFiatPerBlockCumulativeWithSums<Cents>,
        AmountSources<Cents, LazyFiatPerBlockCumulativeWithSums<Cents>, M>,
    >,
    #[traversable(hidden)]
    pub stored: CumulativeUTXOSources<Cents, M>,
}

impl CumulativeRealizedByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &MappingsVecs,
        window_starts: &Windows<&LazyWindowStartVec>,
    ) -> Result<Self> {
        let stored = CumulativeUTXOSources::forced_import(
            cache,
            db,
            &format!("{metric}_cumulative_cents"),
            version,
        )?;
        let cohorts = UTXOGroups::new(|cohort_id| {
            let name = CohortContext::Utxo.metric_name(cohort_id, metric);
            let source = stored
                .stored
                .get(cohort_id)
                .expect("supported stored realized cohort");
            LazyFiatPerBlockCumulativeWithSums::from_cumulative_cents_source(
                &name,
                version,
                source,
                mappings,
                window_starts,
            )
        });
        let addr_version = version + Version::ONE;
        let addr_balance = AmountSources::forced_import(
            cache,
            db,
            &format!("addrs_{metric}_cumulative_cents_by_balance_range"),
            CohortContext::Addr,
            metric,
            addr_version,
            |name, source| {
                LazyFiatPerBlockCumulativeWithSums::from_cumulative_cents_source(
                    name,
                    addr_version,
                    source,
                    mappings,
                    window_starts,
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
