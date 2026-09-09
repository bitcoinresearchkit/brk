use bitview_collections::Windows;
use bitview_plugin::ImportContext;
use bitview_plugin_distribution::{AllChainSources, Vecs as DistributionVecs};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use bitview_vecs::{CachedWindowStartVec, PerBlock};
use brk_error::Result;
use brk_types::{Cents, Version};

use super::{
    STORAGE, Vecs, activity, adjusted, age_range, aggregate, cap, prices, reserve_risk, supply,
    value,
};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
        prices: &PriceVecs,
        subsidy_cents: &PerBlock<Cents>,
        all_chain: &AllChainSources,
        distribution: &DistributionVecs,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 250_000)?;
        let version = STORAGE.schema_version();
        let v1 = version + Version::ONE;
        let spot_price = prices.spot.cents.height.read_only_cached_boxed_clone();
        let activity = activity::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let age_range = age_range::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
            &spot_price,
            distribution,
        )?;
        let supply = supply::forced_import(
            context.cache_budget(),
            &db,
            v1,
            mappings,
            &spot_price,
            &activity,
            all_chain,
        )?;
        let aggregate = aggregate::forced_import(
            context.cache_budget(),
            &db,
            version + Version::new(4),
            mappings,
            &spot_price,
            &supply.active_supply_in_loss_share.bounded,
        )?;
        let value = value::forced_import(context.cache_budget(), &db, v1, mappings, cached_starts)?;
        let cap = cap::forced_import(
            context.cache_budget(),
            &db,
            version + Version::TWO,
            mappings,
            subsidy_cents,
        )?;
        let prices = prices::forced_import(
            context.cache_budget(),
            &db,
            version + Version::new(3),
            mappings,
            &spot_price,
            all_chain,
            cap.cointime.cents.resolutions.height_source(),
        )?;
        let adjusted = adjusted::forced_import(context.cache_budget(), &db, version, mappings)?;
        let reserve_risk =
            reserve_risk::forced_import(context.cache_budget(), &db, v1, mappings, &spot_price)?;

        let this = Self {
            db,
            activity,
            age_range,
            aggregate,
            supply,
            value,
            cap,
            prices,
            adjusted,
            reserve_risk,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
