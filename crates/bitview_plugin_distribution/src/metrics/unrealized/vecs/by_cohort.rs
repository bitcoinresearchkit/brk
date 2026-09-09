use std::ops::AddAssign;

use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmount};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{FiatType, LazyFiatPerBlock};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database, PcoVecValue, Rw, StorageMode};

use crate::metrics::UTXOTypedColumns;

#[derive(Traversable)]
pub struct UnrealizedByCohort<C, M: StorageMode = Rw>
where
    C: FiatType + PcoVecValue,
{
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmount<LazyFiatPerBlock<C>>,
    pub stored: UTXOTypedColumns<C, M>,
}

impl<C> UnrealizedByCohort<C>
where
    C: FiatType + PcoVecValue + AddAssign,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &MappingsVecs,
    ) -> Result<Self> {
        let stored =
            UTXOTypedColumns::forced_import(cache, db, &format!("{metric}_cents"), version)?;
        let cohorts = UTXOGroupsWithoutAmount::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let source = stored
                .additive_source(&filter, &format!("{name}_cents"), version)
                .expect("supported unrealized cohort");
            LazyFiatPerBlock::from_cents_source(&name, version, &source, mappings)
        });
        Ok(Self { cohorts, stored })
    }
}
