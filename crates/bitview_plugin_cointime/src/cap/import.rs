use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::CentsUnsignedToDollars;
use bitview_vecs::{FiatPerBlock, LazyFiatPerBlock, LazyPerBlock, PerBlock, RatioPerBlock};
use brk_error::Result;
use brk_types::{Cents, Version};
use vecdb::{CacheBudget, Database, Ident};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    subsidy_cents: &PerBlock<Cents>,
) -> Result<Vecs> {
    let thermo_cents =
        LazyPerBlock::from_resolutions::<Ident>("thermo_cap_cents", version, subsidy_cents);
    let thermo_usd = LazyPerBlock::from_lazy::<CentsUnsignedToDollars, Cents>(
        "thermo_cap",
        version,
        &thermo_cents,
    );
    Ok(Vecs {
        thermo: LazyFiatPerBlock {
            usd: thermo_usd,
            cents: thermo_cents,
        },
        investor: FiatPerBlock::forced_import(cache, db, "investor_cap", version, mappings)?,
        vaulted: FiatPerBlock::forced_import(cache, db, "vaulted_cap", version, mappings)?,
        active: FiatPerBlock::forced_import(cache, db, "active_cap", version, mappings)?,
        cointime: FiatPerBlock::forced_import(
            cache,
            db,
            "cointime_cap",
            version + Version::ONE,
            mappings,
        )?,
        aviv: RatioPerBlock::forced_import(cache, db, "aviv", version, mappings)?,
    })
}
