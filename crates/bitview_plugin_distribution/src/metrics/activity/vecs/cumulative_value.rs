use bitview_cohort::{AmountRange, CohortContext, UTXOAndAddrGroups, UTXOGroups, UTXORows};
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyValuePerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{Cents, Sats, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, Rw, StorageMode};

use crate::metrics::{ColumnarAmountValue, CumulativeUTXOValueColumnarMetric};

#[derive(Traversable)]
pub struct CumulativeValueByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    /// UTXO groups and spent output value grouped by the spending address's
    /// balance immediately before the spend.
    pub cohorts: UTXOAndAddrGroups<
        LazyValuePerBlockCumulativeRolling,
        ColumnarAmountValue<LazyValuePerBlockCumulativeRolling, M>,
    >,
    pub cumulative: CumulativeUTXOValueColumnarMetric<M>,
}

impl CumulativeValueByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let cumulative = CumulativeUTXOValueColumnarMetric::forced_import(
            db,
            &format!("{metric}_cumulative"),
            version,
        )?;
        let cohorts = UTXOGroups::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let (sats, cents) = cumulative
                .sources(cache, &filter, &name, version)
                .expect("supported cumulative value cohort");
            LazyValuePerBlockCumulativeRolling::from_cumulative_sources(
                &name,
                version,
                &sats,
                &cents,
                mappings,
                cached_starts,
            )
        });
        let addr_version = version + Version::ONE;
        let addr_balance = ColumnarAmountValue::forced_import(
            cache,
            db,
            &format!("addrs_{metric}_cumulative_by_balance_range"),
            CohortContext::Addr,
            metric,
            addr_version,
            |name, sats, cents| {
                LazyValuePerBlockCumulativeRolling::from_cumulative_sources(
                    name,
                    addr_version,
                    &sats,
                    &cents,
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
            cumulative,
        })
    }

    #[inline(always)]
    pub fn push_block(&mut self, sats: UTXORows<Sats>, cents: UTXORows<Cents>) {
        self.cumulative.push_block(sats, cents);
    }

    #[inline(always)]
    pub fn push_addr_balance(&mut self, sats: &AmountRange<Sats>, cents: &AmountRange<Cents>) {
        self.cohorts.addr_balance.push_cumulative(sats, cents);
    }

    pub fn min_len(&self) -> usize {
        self.cumulative
            .min_len()
            .min(self.cohorts.addr_balance.len())
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.cumulative.collect_vecs_mut();
        vecs.extend(self.cohorts.addr_balance.collect_vecs_mut());
        vecs
    }
}
