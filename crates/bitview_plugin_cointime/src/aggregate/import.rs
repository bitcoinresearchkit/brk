use std::ops::AddAssign;

use bitview_cohort::{TERM_NAMES, TermId, UTXOAggregateId};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::BoundedToF64;
use bitview_vecs::{
    LazyFiatPerBlock, LazyPerBlock, LazyPriceWithRatioPerBlock, LazySpotValuePerBlock, PerBlock,
};
use brk_error::Result;
use brk_types::{BoundedRatio, Cents, Height, Version};
use vecdb::{
    CacheBudget, CachedBoxedVec, Database, ImportOptions, ImportableVec, PcoVec, PcoVecValue,
    ReadOnlyClone, ReadOnlyColumnarVec, ReadableBoxedVec, ReadableCloneableVec,
    ReadableColumnarVec,
};

use super::{AwakeVecs, CohortVecs, DormantVecs, Sources, Vecs};

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    spot_price: &CachedBoxedVec<Height, Cents>,
    all_supply_in_loss_share: &PerBlock<BoundedRatio>,
) -> Result<Vecs> {
    let version = version + Version::ONE;
    let sources = Sources::forced_import(cache, db, version)?;
    let all_loss_share = all_supply_in_loss_share.height.read_only_boxed_clone();
    let term_loss_share = |term: TermId| {
        let name = term.select(&TERM_NAMES).id;
        sources
            .supply_in_loss_share
            .read_only_clone()
            .column(
                &format!("{name}_awake_supply_in_loss_share"),
                version + Version::ONE,
                term,
            )
            .read_only_boxed_clone()
    };
    let all = CohortVecs::new(
        UTXOAggregateId::All,
        version,
        &sources,
        all_loss_share,
        mappings,
        spot_price,
    );
    let sth = CohortVecs::new(
        UTXOAggregateId::Sth,
        version,
        &sources,
        term_loss_share(TermId::Short),
        mappings,
        spot_price,
    );
    let lth = CohortVecs::new(
        UTXOAggregateId::Lth,
        version,
        &sources,
        term_loss_share(TermId::Long),
        mappings,
        spot_price,
    );

    Ok(Vecs {
        all,
        sth,
        lth,
        sources,
    })
}

impl Sources {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            awake_supply: ImportableVec::forced_import_with(
                ImportOptions::new(db, "cointime_awake_supply_sats_by_term", version)
                    .with_cache_budget(cache),
            )?,
            dormant_supply: ImportableVec::forced_import_with(
                ImportOptions::new(db, "cointime_dormant_supply_sats_by_term", version)
                    .with_cache_budget(cache),
            )?,
            awake_cap: ImportableVec::forced_import_with(
                ImportOptions::new(db, "cointime_awake_cap_cents_by_term", version)
                    .with_cache_budget(cache),
            )?,
            awake_price: ImportableVec::forced_import_with(
                ImportOptions::new(db, "cointime_awake_price_cents_by_aggregate", version)
                    .with_cache_budget(cache),
            )?,
            supply_in_loss_share: ImportableVec::forced_import_with(
                ImportOptions::new(
                    db,
                    "cointime_awake_supply_in_loss_share_bounded_by_term",
                    version + Version::ONE,
                )
                .with_cache_budget(cache),
            )?,
        })
    }

    fn additive_source<T>(
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, TermId>,
        name: &str,
        version: Version,
        aggregate: UTXOAggregateId,
    ) -> ReadableBoxedVec<Height, T>
    where
        T: PcoVecValue + AddAssign,
    {
        match aggregate.term() {
            Some(term) => source.column(name, version, term).read_only_boxed_clone(),
            None => source
                .sum_columns(name, version, TermId::ALL.iter().copied())
                .read_only_boxed_clone(),
        }
    }
}

impl CohortVecs {
    fn new(
        aggregate: UTXOAggregateId,
        version: Version,
        sources: &Sources,
        supply_in_loss_share: ReadableBoxedVec<Height, BoundedRatio>,
        mappings: &MappingsVecs,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Self {
        let metric_name = |metric: &str| aggregate.metric_name(metric);
        let awake_supply = Sources::additive_source(
            &sources.awake_supply.read_only_clone(),
            &metric_name("awake_supply_sats"),
            version,
            aggregate,
        );
        let dormant_supply = Sources::additive_source(
            &sources.dormant_supply.read_only_clone(),
            &metric_name("dormant_supply_sats"),
            version,
            aggregate,
        );
        let awake_cap = Sources::additive_source(
            &sources.awake_cap.read_only_clone(),
            &metric_name("awake_cap_cents"),
            version,
            aggregate,
        );
        let awake_price = sources.awake_price.read_only_clone().column(
            &metric_name("awake_price_cents"),
            version,
            aggregate,
        );

        Self {
            awake: AwakeVecs {
                supply: LazySpotValuePerBlock::from_sats_source(
                    &metric_name("awake_supply"),
                    version,
                    &awake_supply,
                    mappings,
                    spot_price,
                ),
                supply_in_loss_share: LazyPerBlock::from_height_source::<BoundedToF64>(
                    &metric_name("awake_supply_in_loss_share"),
                    version,
                    &supply_in_loss_share,
                    mappings,
                ),
                cap: LazyFiatPerBlock::from_cents_source(
                    &metric_name("awake_cap"),
                    version,
                    &awake_cap,
                    mappings,
                ),
                price: LazyPriceWithRatioPerBlock::from_height_source(
                    &metric_name("awake_price"),
                    version,
                    &awake_price,
                    mappings,
                    spot_price,
                ),
            },
            dormant: DormantVecs {
                supply: LazySpotValuePerBlock::from_sats_source(
                    &metric_name("dormant_supply"),
                    version,
                    &dormant_supply,
                    mappings,
                    spot_price,
                ),
            },
        }
    }
}
