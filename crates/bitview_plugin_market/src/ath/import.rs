use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::{DaysToYears, RatioDiffCents};
use bitview_vecs::{LazyIndexedVec, LazyPerBlock, LazyPercentPerBlock, PerBlock, Price};
use brk_error::Result;
use brk_types::{Cents, Height, PartsPerMillionSigned32, Version};
use vecdb::{BinaryTransform, CacheBudget, Database, ReadableCloneableVec};

use super::Vecs;

const VERSION: Version = Version::ONE;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    spot_price: &impl ReadableCloneableVec<Height, Cents>,
) -> Result<Vecs> {
    let v = version + VERSION;

    let high = Price::forced_import(cache, db, "price_ath", v, mappings)?;

    let max_days_between =
        PerBlock::forced_import(cache, db, "max_days_between_price_ath", v, mappings)?;

    let max_years_between = LazyPerBlock::from_resolutions::<DaysToYears>(
        "max_years_between_price_ath",
        v,
        &max_days_between,
    );

    let days_since = PerBlock::forced_import(cache, db, "days_since_price_ath", v, mappings)?;

    let years_since =
        LazyPerBlock::from_resolutions::<DaysToYears>("years_since_price_ath", v, &days_since);

    let drawdown_source = LazyIndexedVec::new(
        "price_drawdown_ppm_source",
        v,
        high.cents.resolutions.height_source(),
        spot_price,
        |_, high, spot| RatioDiffCents::<PartsPerMillionSigned32>::apply(spot, high),
    );
    let drawdown =
        LazyPercentPerBlock::from_height_source("price_drawdown", v, &drawdown_source, mappings);

    Ok(Vecs {
        high,
        drawdown,
        days_since,
        years_since,
        max_days_between,
        max_years_between,
    })
}
