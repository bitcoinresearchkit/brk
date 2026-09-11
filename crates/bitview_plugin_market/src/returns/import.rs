use bitview_collections::{ByDcaCagr, ByDcaPeriod, ByLookbackPeriod, Windows};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use bitview_transforms::RatioDiffDollars;
use bitview_vecs::{LazyPercentPerBlock, LazyWindowVec, StdDevPerBlock};
use brk_error::{Error, Result};
use brk_types::{Dollars, Height, PartsPerMillionSigned64, Version};
use vecdb::{BinaryTransform, CacheBudget, Database, ReadableCloneableVec};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    window_starts: &ByLookbackPeriod<&impl ReadableCloneableVec<Height, Height>>,
    prices: &PriceVecs,
) -> Result<Vecs> {
    let periods =
        ByLookbackPeriod::try_from_period(window_starts, |name, _days, window_starts| {
            let metric_name = format!("price_return_{name}");
            let source = LazyWindowVec::<Height, Dollars, PartsPerMillionSigned64>::new(
                &format!("{metric_name}_ppm_source"),
                version,
                &prices.spot.usd.height,
                *window_starts,
                false,
                |current, past, _| {
                    RatioDiffDollars::<PartsPerMillionSigned64>::apply(current, past)
                },
            );
            Ok::<_, Error>(LazyPercentPerBlock::from_height_source(
                &metric_name,
                version,
                &source,
                mappings,
            ))
        })?;

    let dca_periods = ByDcaPeriod::from_lookback(&periods);
    let cagr = ByDcaCagr::try_new(&dca_periods, |name, days, source| {
        Ok::<_, Error>(LazyPercentPerBlock::from_lazy_cagr(
            &format!("price_cagr_{name}"),
            version,
            (days / 365) as u8,
            source,
        ))
    })?;

    let mut days_iter = Windows::<()>::DAYS.iter();
    let sd_24h = Windows::try_from_fn(|suffix| {
        let days = *days_iter.next().unwrap();
        StdDevPerBlock::forced_import(
            cache,
            db,
            "price_return_24h",
            suffix,
            days,
            version + Version::ONE,
            mappings,
        )
    })?;

    Ok(Vecs {
        periods,
        cagr,
        sd_24h,
    })
}
