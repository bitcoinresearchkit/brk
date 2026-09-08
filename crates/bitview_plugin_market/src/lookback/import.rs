use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use brk_error::Result;
use vecdb::ReadableCloneableVec;

use brk_error::Error;
use brk_types::Version;
use brk_types::{Cents, Height};

use super::Vecs;
use bitview_compute::{ByLookbackPeriod, LazyWindowVec, Price};

pub fn forced_import(
    version: Version,
    mappings: &MappingsVecs,
    cached_starts: &ByLookbackPeriod<&impl ReadableCloneableVec<Height, Height>>,
    prices: &PriceVecs,
) -> Result<Vecs> {
    let price_past =
        ByLookbackPeriod::try_from_period(cached_starts, |name, _days, window_starts| {
            let metric_name = format!("price_past_{name}");
            let source = LazyWindowVec::<Height, Cents, Cents>::new(
                &format!("{metric_name}_cents_source"),
                version,
                prices.spot.cents.height.read_only_boxed_clone(),
                window_starts.read_only_boxed_clone(),
                false,
                |_, past, _| past,
            );
            Ok::<_, Error>(Price::from_height_source(
                &metric_name,
                version,
                &source,
                mappings,
            ))
        })?;

    Ok(Vecs { price_past })
}
