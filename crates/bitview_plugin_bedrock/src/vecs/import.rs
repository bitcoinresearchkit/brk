use bitview_plugin::ImportContext;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::BoundedToF64;
use bitview_vecs::{DailyMappings, LazyDailyMetric, LazyDailyPrice, import_stored};
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, Database, ReadableBoxedVec};

use super::Vecs;
use crate::{
    CapitalizedPriceVecs, CostBasisVecs, ModeVecs, Modes, Percentiles, PriceBands, STORAGE,
};

impl ModeVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        let version = version + Version::TWO;
        let loss_threshold_stored = Percentiles::try_from_fn(|id| {
            import_stored(
                cache,
                db,
                &format!("{name}_loss_threshold_{}_bounded", id.suffix()),
                version,
            )
        })?;
        let loss_threshold = Percentiles::from_fn(|id| {
            LazyDailyMetric::from_source::<BoundedToF64>(
                &format!("{name}_loss_threshold_{}", id.suffix()),
                version,
                id.select(&loss_threshold_stored),
                mappings,
            )
        });
        let prices_stored = PriceBands::try_from_fn(|id| {
            import_stored(cache, db, &format!("{name}_{}_cents", id.suffix()), version)
        })?;
        let prices = PriceBands::from_fn(|id| {
            LazyDailyPrice::from_day1_source(
                &format!("{name}_{}", id.suffix()),
                version,
                id.select(&prices_stored),
                mappings,
            )
        });
        Ok(Self {
            loss_threshold,
            prices,
            loss_threshold_stored,
            prices_stored,
        })
    }
}

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        spot: &ReadableBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 100_000)?;
        let states_path = STORAGE.path(context).join("states");
        let version = STORAGE.schema_version();
        let indexes = mappings;
        let mappings = DailyMappings::new(indexes);

        let modes = Modes::try_from_fn(|mode| {
            let name = mode.name();
            ModeVecs::forced_import(
                context.cache_budget(),
                &db,
                &format!("bedrock_{name}"),
                version,
                &mappings,
            )
        })?;
        let cost_basis =
            CostBasisVecs::forced_import(context.cache_budget(), &db, version, &mappings)?;
        let capitalized_price = CapitalizedPriceVecs::forced_import(
            context.cache_budget(),
            &db,
            version,
            indexes,
            &mappings,
            spot,
        )?;
        let this = Self {
            db,
            states_path,
            cost_basis,
            capitalized_price,
            modes,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
