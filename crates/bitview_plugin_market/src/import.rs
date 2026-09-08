use bitview_collections::ByLookbackPeriod;
use bitview_plugin::ImportContext;
use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use brk_error::{Error, Result};

use super::{STORAGE, Vecs};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        blocks: &BlocksVecs,
        prices: &PriceVecs,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 250_000)?;
        let version = STORAGE.schema_version();

        let spot_price = prices.spot.cents.resolutions.height_source();
        let ath =
            super::ath::forced_import(context.cache_budget(), &db, version, mappings, spot_price)?;
        let cached_starts = ByLookbackPeriod::try_new(|_, days| {
            Ok::<_, Error>(blocks.lookback.cached_start_vec(days as usize))
        })?;
        let lookback = super::lookback::forced_import(version, mappings, &cached_starts, prices)?;
        let returns = super::returns::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            &cached_starts,
            prices,
        )?;
        let volatility = super::volatility::forced_import(version, &returns)?;
        let range = super::range::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            spot_price,
        )?;
        let moving_average = super::moving_average::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            blocks,
            spot_price,
        )?;
        let technical = super::technical::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            &returns.periods._24h.ratio,
        )?;

        let this = Self {
            db,
            ath,
            lookback,
            returns,
            volatility,
            range,
            moving_average,
            technical,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
