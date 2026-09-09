use bitview_cohort::{
    AgeRangeId, AmountRange, CohortContext, Filter, UTXOAndAddrGroups, UTXOGroups, UTXORows,
};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::SatsToCents;
use bitview_traversable::Traversable;
use bitview_vecs::{LazyIndexedVec, LazyPerBlock, LazySpotValuePerBlock};
use brk_error::Result;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{
    AnyStoredVec, BinaryTransform, CacheBudget, CachedBoxedVec, Database, Ident, ReadOnlyClone,
    ReadableBoxedVec, ReadableCloneableVec, ReadableColumnarVec, Rw, StorageMode,
};

use crate::metrics::{ColumnarAmount, UTXOColumns};

#[derive(Traversable)]
pub struct SupplyTotal<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts:
        UTXOAndAddrGroups<LazySpotValuePerBlock, ColumnarAmount<Sats, LazySpotValuePerBlock, M>>,
    pub stored: UTXOColumns<Sats, M>,
    #[traversable(skip)]
    all_supply: ReadableBoxedVec<Height, Sats>,
    #[traversable(skip)]
    all_market_cap: ReadableBoxedVec<Height, Cents>,
}

impl SupplyTotal {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let stored = UTXOColumns::forced_import(cache, db, "supply_sats", version)?;
        let age_ranges = stored.age_range.height.read_only_clone();
        let all_name = CohortContext::Utxo.metric_name(&Filter::All, "", "supply");
        let all_sats = age_ranges.sum_columns(
            &format!("{all_name}_sats"),
            version,
            AgeRangeId::ALL.iter().copied(),
        );
        let all_supply = all_sats.read_only_boxed_clone();
        let sats = LazyPerBlock::from_height_source::<Ident>(
            &format!("{all_name}_sats"),
            version,
            &all_sats,
            mappings,
        );
        let all_cents = LazyIndexedVec::new(
            &format!("{all_name}_cents_source"),
            version,
            &sats.height,
            spot_price,
            |_, sats, spot| SatsToCents::apply(sats, spot),
        );
        let all_market_cap = all_cents.read_only_boxed_clone();
        let all = LazySpotValuePerBlock::from_sats_and_cents(
            &all_name,
            version,
            sats,
            LazyPerBlock::from_height_source::<Ident>(
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
                        &age_ranges.column(&source_name, version, column),
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
                    stored
                        .additive_source(&filter, &source_name, version)
                        .expect("total-supply cohort source")
                };
                LazySpotValuePerBlock::from_sats_source(
                    &name, version, &source, mappings, spot_price,
                )
            }
        });
        let addr_balance = ColumnarAmount::forced_import(
            cache,
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
            cohorts: UTXOAndAddrGroups {
                utxo: cohorts,
                addr_balance,
            },
            stored,
            all_supply,
            all_market_cap,
        })
    }

    pub fn min_len(&self) -> usize {
        self.stored.min_len().min(self.cohorts.addr_balance.len())
    }

    pub fn get(&self, filter: &Filter) -> Option<&LazySpotValuePerBlock> {
        self.cohorts.utxo.get(filter)
    }

    pub fn all_supply(&self) -> &ReadableBoxedVec<Height, Sats> {
        &self.all_supply
    }

    pub fn all_market_cap(&self) -> &ReadableBoxedVec<Height, Cents> {
        &self.all_market_cap
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXORows<Sats>) {
        self.stored.push(rows);
    }

    #[inline(always)]
    pub fn push_addr_balance(&mut self, row: AmountRange<Sats>) {
        self.cohorts.addr_balance.push(row);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.stored.collect_vecs_mut();
        vecs.push(self.cohorts.addr_balance.stored_mut());
        vecs
    }
}
