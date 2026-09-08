use bitview_plugin_mappings::Vecs as MappingsVecs;
use brk_error::Result;

use bitview_cohort::{AgeRangeId, AmountRange, CohortContext, Filter, UTXOGroups};
use bitview_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{
    AnyStoredVec, BinaryTransform, Budgeted, CachedBoxedVec, CachedColumnarVec, CachedReadableVec,
    Database, PcoVec, PinnedCachedVec, ReadOnlyClone, ReadOnlyColumnarVec, ReadableColumnarVec, Rw,
    StorageMode,
};

use crate::metrics::{ColumnarAmount, UTXOColumnarMetric, UTXORows};
use bitview_compute::{
    CACHE_BUDGET, Identity, LazyIndexedVec, LazyPerBlock, LazySpotValuePerBlock, SatsToCents,
};

#[derive(Traversable)]
pub struct SupplyTotal<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroups<LazySpotValuePerBlock>,
    #[traversable(flatten)]
    pub matrices: UTXOColumnarMetric<Sats, M>,
    /// Groups funded addresses by their balance at the represented block.
    pub addr_balance: ColumnarAmount<Sats, LazySpotValuePerBlock, M>,
    #[traversable(skip)]
    all_supply: CachedBoxedVec<Height, Sats>,
    #[traversable(skip)]
    all_market_cap: CachedBoxedVec<Height, Cents>,
    /// Shared decoded age inputs for raw sums and weighted consumers.
    #[traversable(skip)]
    pub age_ranges: CachedColumnarVec<
        ReadOnlyColumnarVec<PcoVec<Height, Sats>, AgeRangeId>,
        AgeRangeId,
        Budgeted,
    >,
}

impl SupplyTotal {
    pub fn forced_import(
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let matrices = UTXOColumnarMetric::forced_import(db, "supply_sats", version)?;
        let age_ranges = CachedColumnarVec::new(
            matrices.age_range_matrix.read_only_clone(),
            version,
            |column| CACHE_BUDGET.wrap(column),
        );
        let all_name = CohortContext::Utxo.metric_name(&Filter::All, "", "supply");
        // These two frequently reused roots are pinned. The public series and
        // all downstream consumers share them; no second series owner is kept.
        let all_sats = PinnedCachedVec::wrap(age_ranges.sum_columns(
            &format!("{all_name}_sats"),
            version,
            AgeRangeId::ALL.iter().copied(),
        ));
        let all_supply = all_sats.cached_boxed_clone();
        let sats = LazyPerBlock::from_height_source::<Identity<Sats>>(
            &format!("{all_name}_sats"),
            version,
            &all_sats,
            mappings,
        );
        let all_cents = PinnedCachedVec::wrap(LazyIndexedVec::new(
            &format!("{all_name}_cents_source"),
            version,
            &sats.height,
            spot_price,
            |_, sats, spot| SatsToCents::apply(sats, spot),
        ));
        let all_market_cap = all_cents.cached_boxed_clone();
        let all = LazySpotValuePerBlock::from_sats_and_cents(
            &all_name,
            version,
            sats,
            LazyPerBlock::from_height_source::<Identity<Cents>>(
                &format!("{all_name}_cents"),
                version,
                &all_cents,
                mappings,
            ),
        );
        let cohorts = UTXOGroups::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "supply");
            if matches!(filter, Filter::All) {
                all.clone()
            } else {
                let source_name = format!("{name}_sats");
                let source = if let Some(column) = AgeRangeId::matching(&filter) {
                    return LazySpotValuePerBlock::from_sats_source(
                        &name,
                        version,
                        age_ranges.cached_column(column),
                        mappings,
                        spot_price,
                    );
                } else if let Some(columns) = AgeRangeId::aggregate_columns(&filter) {
                    return LazySpotValuePerBlock::from_sats_source(
                        &name,
                        version,
                        &age_ranges.sum_columns(&source_name, version, columns),
                        mappings,
                        spot_price,
                    );
                } else {
                    matrices
                        .additive_source(&filter, &source_name, version)
                        .expect("total-supply cohort source")
                };
                LazySpotValuePerBlock::from_sats_source(
                    &name, version, &source, mappings, spot_price,
                )
            }
        });
        let addr_balance = ColumnarAmount::forced_import(
            db,
            "addrs_supply_sats_by_balance_range",
            CohortContext::Addr,
            "supply",
            version + Version::ONE,
            |name, source| {
                LazySpotValuePerBlock::from_sats_source(
                    name,
                    version + Version::ONE,
                    source,
                    mappings,
                    spot_price,
                )
            },
        )?;

        Ok(Self {
            cohorts,
            matrices,
            addr_balance,
            all_supply,
            all_market_cap,
            age_ranges,
        })
    }

    pub fn min_len(&self) -> usize {
        self.matrices.min_len().min(self.addr_balance.len())
    }

    pub fn get(&self, filter: &Filter) -> Option<&LazySpotValuePerBlock> {
        self.cohorts.get(filter)
    }

    pub fn all_supply(&self) -> &CachedBoxedVec<Height, Sats> {
        &self.all_supply
    }

    pub fn all_market_cap(&self) -> &CachedBoxedVec<Height, Cents> {
        &self.all_market_cap
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXORows<Sats>) {
        self.matrices.push(rows);
    }

    #[inline(always)]
    pub fn push_addr_balance(&mut self, row: AmountRange<Sats>) {
        self.addr_balance.push(row);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.matrices.collect_vecs_mut();
        vecs.push(self.addr_balance.stored_mut());
        vecs
    }
}
