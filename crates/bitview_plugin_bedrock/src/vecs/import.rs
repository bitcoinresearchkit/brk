use bitview_plugin::ImportContext;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::BoundedToF64;
use bitview_vecs::{ColumnarDailyMetric, DailyMappings, LazyColumnDailyPrice, LazyDailyMetric};
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, CachedBoxedVec, Database, ReadableCloneableVec, ReadableColumnarVec};

use super::Vecs;
use crate::{
    CapitalizedPriceVecs, CostBasisVecs, LossPercentileId, ModeVecs, Modes, PriceBandId, STORAGE,
};

impl ModeVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        let loss_threshold = ColumnarDailyMetric::forced_import(
            cache,
            db,
            &format!("{name}_loss_thresholds_bounded"),
            version + Version::ONE,
            |source| {
                LossPercentileId::series(|percentile| {
                    LazyDailyMetric::from_source::<BoundedToF64>(
                        &format!("{name}_loss_threshold_{}", percentile.suffix()),
                        version + Version::ONE,
                        source
                            .column(
                                &format!("{name}_loss_threshold_{}_bounded", percentile.suffix()),
                                version + Version::ONE,
                                percentile,
                            )
                            .read_only_boxed_clone(),
                        mappings,
                    )
                })
            },
        )?;

        let prices = ColumnarDailyMetric::forced_import(
            cache,
            db,
            &format!("{name}_price_bands"),
            version,
            |source| {
                PriceBandId::series(|band| {
                    LazyColumnDailyPrice::new(
                        &format!("{name}_{}", band.suffix()),
                        version,
                        source,
                        band,
                        mappings,
                    )
                })
            },
        )?;

        Ok(Self {
            loss_threshold,
            prices,
        })
    }
}

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        spot: &CachedBoxedVec<Height, Cents>,
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
