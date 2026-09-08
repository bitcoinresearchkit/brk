use bitview_collections::WindowsTo1m;
use bitview_vecs::{LazyPerBlock, PerBlock, RatioPerBlock};
use brk_error::Result;
use brk_types::{PartsPerMillionSigned64, StoredF32, Version};
use vecdb::{CacheBudget, Database};

use super::{MacdChain, Vecs, rsi_chain};

const VERSION: Version = Version::new(4);

fn forced_import_macd(
    cache: &'static CacheBudget,
    db: &Database,
    tf: &str,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
) -> Result<MacdChain> {
    let line = PerBlock::forced_import(cache, db, &format!("macd_line_{tf}"), version, mappings)?;
    let signal =
        PerBlock::forced_import(cache, db, &format!("macd_signal_{tf}"), version, mappings)?;

    let histogram = PerBlock::forced_import(
        cache,
        db,
        &format!("macd_histogram_{tf}"),
        version,
        mappings,
    )?;

    Ok(MacdChain {
        ema_fast: PerBlock::forced_import(
            cache,
            db,
            &format!("macd_ema_fast_{tf}"),
            version,
            mappings,
        )?,
        ema_slow: PerBlock::forced_import(
            cache,
            db,
            &format!("macd_ema_slow_{tf}"),
            version,
            mappings,
        )?,
        line,
        signal,
        histogram,
    })
}

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    returns: &LazyPerBlock<StoredF32, PartsPerMillionSigned64>,
) -> Result<Vecs> {
    let v = version + VERSION;

    let rsi = WindowsTo1m::try_from_fn(|tf| {
        rsi_chain::forced_import(cache, db, tf, v + Version::TWO, mappings, returns)
    })?;
    let macd = WindowsTo1m::try_from_fn(|tf| forced_import_macd(cache, db, tf, v, mappings))?;

    let pi_cycle = RatioPerBlock::forced_import_ppm(cache, db, "pi_cycle", v, mappings)?;

    Ok(Vecs {
        rsi,
        pi_cycle,
        macd,
    })
}
