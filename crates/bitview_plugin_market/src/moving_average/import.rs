use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{LazyPriceWithRatioPerBlock, import_stored};
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, Database, ReadableCloneableVec};

use super::{Vecs, sma::SmaVecs, vecs::EmaPeriodId};

const EMA_VERSION: Version = Version::TWO;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    blocks: &BlocksVecs,
    spot_price: &impl ReadableCloneableVec<Height, Cents>,
) -> Result<Vecs> {
    let sma = SmaVecs::new(version, mappings, &blocks.lookback, spot_price);
    let ema_version = version + EMA_VERSION;
    let ema_stored = EmaPeriodId::try_series(|period| {
        import_stored(
            cache,
            db,
            &format!("price_ema_{}_cents", period.suffix()),
            ema_version,
        )
    })?;
    let ema = EmaPeriodId::series(|period| {
        LazyPriceWithRatioPerBlock::from_height_source(
            &format!("price_ema_{}", period.suffix()),
            ema_version,
            period.select(&ema_stored),
            mappings,
            spot_price,
        )
    });
    Ok(Vecs {
        sma,
        ema,
        ema_stored,
    })
}
