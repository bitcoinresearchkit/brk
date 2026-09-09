use bitview_cohort::{
    AmountRange, CohortContext, CohortId, UTXOAndAddrGroups, UTXOGroups, UTXOValues,
};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::SatsToCents;
use bitview_traversable::Traversable;
use bitview_vecs::{LazyIndexedVec, LazyPerBlock, LazySpotValuePerBlock};
use brk_error::Result;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{
    AnyStoredVec, BinaryTransform, CacheBudget, CachedBoxedVec, Database, Ident, ReadableBoxedVec,
    ReadableCloneableVec, Rw, StorageMode,
};

use crate::metrics::{AmountSources, UTXOSources};

#[derive(Traversable)]
pub struct SupplyTotal<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts:
        UTXOAndAddrGroups<LazySpotValuePerBlock, AmountSources<Sats, LazySpotValuePerBlock, M>>,
    #[traversable(hidden)]
    pub stored: UTXOSources<Sats, M>,
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
        let stored = UTXOSources::forced_import(cache, db, "supply_sats", version)?;
        let all_name = CohortContext::Utxo.metric_name(CohortId::All, "supply");
        let all_sats = stored.get(CohortId::All).expect("all supply source");
        let all_supply = all_sats.read_only_boxed_clone();
        let sats = LazyPerBlock::from_height_source::<Ident>(
            &format!("{all_name}_sats"),
            version,
            all_sats,
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
        let cohorts = UTXOGroups::new(|cohort_id| {
            let name = CohortContext::Utxo.metric_name(cohort_id, "supply");
            if matches!(cohort_id, CohortId::All) {
                all.clone()
            } else {
                let source = stored.get(cohort_id).expect("total-supply cohort source");
                LazySpotValuePerBlock::from_sats_source(
                    &name, version, source, mappings, spot_price,
                )
            }
        });
        let addr_balance = AmountSources::forced_import(
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

    pub fn get(&self, cohort_id: CohortId) -> Option<&LazySpotValuePerBlock> {
        self.cohorts.utxo.get(cohort_id)
    }

    pub fn all_supply(&self) -> &ReadableBoxedVec<Height, Sats> {
        &self.all_supply
    }

    pub fn all_market_cap(&self) -> &ReadableBoxedVec<Height, Cents> {
        &self.all_market_cap
    }

    #[inline(always)]
    pub fn push(&mut self, cohort_values: UTXOValues<Sats>) {
        self.stored.push(cohort_values);
    }

    #[inline(always)]
    pub fn push_addr_balance(&mut self, values: AmountRange<Sats>) {
        self.cohorts.addr_balance.push(values);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.stored.collect_vecs_mut();
        vecs.extend(self.cohorts.addr_balance.collect_vecs_mut());
        vecs
    }
}
