use bitview_vecs::{PerBlock, PercentPerBlock};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
) -> Result<Vecs> {
    Ok(Vecs {
        inflation_rate: PercentPerBlock::forced_import(
            cache,
            db,
            "cointime_adj_inflation_rate",
            version + Version::new(3),
            mappings,
        )?,
        tx_velocity_native: PerBlock::forced_import(
            cache,
            db,
            "cointime_adj_tx_velocity_btc",
            version,
            mappings,
        )?,
        tx_velocity_fiat: PerBlock::forced_import(
            cache,
            db,
            "cointime_adj_tx_velocity_usd",
            version,
            mappings,
        )?,
    })
}
