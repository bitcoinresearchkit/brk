use bitview_cohort::{ByTerm, Term, UTXOAggregate, UTXOAggregateId};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::BoundedToF64;
use bitview_vecs::{
    LazyFiatPerBlock, LazyPerBlock, LazyPriceWithRatioPerBlock, LazySpotValuePerBlock, PerBlock,
    StoredSeries, import_stored,
};
use brk_error::Result;
use brk_types::{BoundedRatio, Cents, Height, Version};
use vecdb::{CacheBudget, CachedBoxedVec, Database, PcoVecValue, ReadableBoxedVec};

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
    let term_loss_share = |term: Term| {
        sources
            .supply_in_loss_share
            .get(term)
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
        term_loss_share(Term::Sth),
        mappings,
        spot_price,
    );
    let lth = CohortVecs::new(
        UTXOAggregateId::Lth,
        version,
        &sources,
        term_loss_share(Term::Lth),
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
        let version = version + Version::ONE;
        Ok(Self {
            awake_supply: import_aggregate(cache, db, "awake_supply_sats", version)?,
            dormant_supply: import_aggregate(cache, db, "dormant_supply_sats", version)?,
            awake_cap: import_aggregate(cache, db, "awake_cap_cents", version)?,
            awake_price: import_aggregate(cache, db, "awake_price_cents", version)?,
            supply_in_loss_share: ByTerm::try_new(|cohort_id| {
                let name = cohort_id.name();
                import_stored(
                    cache,
                    db,
                    &format!("{name}_awake_supply_in_loss_share_bounded"),
                    version,
                )
            })?,
        })
    }
}

fn import_aggregate<T: PcoVecValue>(
    cache: &'static CacheBudget,
    db: &Database,
    metric: &str,
    version: Version,
) -> Result<UTXOAggregate<StoredSeries<Height, T>>> {
    Ok(UTXOAggregate {
        all: import_stored(
            cache,
            db,
            &UTXOAggregateId::All.metric_name(metric),
            version,
        )?,
        sth: import_stored(
            cache,
            db,
            &UTXOAggregateId::Sth.metric_name(metric),
            version,
        )?,
        lth: import_stored(
            cache,
            db,
            &UTXOAggregateId::Lth.metric_name(metric),
            version,
        )?,
    })
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
        let awake_supply = aggregate.select(&sources.awake_supply);
        let dormant_supply = aggregate.select(&sources.dormant_supply);
        let awake_cap = aggregate.select(&sources.awake_cap);
        let awake_price = aggregate.select(&sources.awake_price);

        Self {
            awake: AwakeVecs {
                supply: LazySpotValuePerBlock::from_sats_source(
                    &metric_name("awake_supply"),
                    version,
                    awake_supply,
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
                    awake_cap,
                    mappings,
                ),
                price: LazyPriceWithRatioPerBlock::from_height_source(
                    &metric_name("awake_price"),
                    version,
                    awake_price,
                    mappings,
                    spot_price,
                ),
            },
            dormant: DormantVecs {
                supply: LazySpotValuePerBlock::from_sats_source(
                    &metric_name("dormant_supply"),
                    version,
                    dormant_supply,
                    mappings,
                    spot_price,
                ),
            },
        }
    }
}
