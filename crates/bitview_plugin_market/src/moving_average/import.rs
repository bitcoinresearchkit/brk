use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{ColumnarPerBlock, LazyColumnPriceWithRatioPerBlock};
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, Database, ReadableCloneableVec};

use super::{Vecs, sma::SmaVecs, vecs::EmaPeriodId};

const EMA_VERSION: Version = Version::ONE;

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
    let ema = ColumnarPerBlock::forced_import(db, "price_ema_cents", ema_version, |source| {
        EmaPeriodId::series(|period| {
            LazyColumnPriceWithRatioPerBlock::new(
                cache,
                &format!("price_ema_{}", period.suffix()),
                ema_version,
                source,
                period,
                mappings,
                spot_price,
            )
        })
    })?;

    Ok(Vecs { sma, ema })
}
