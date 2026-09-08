use bitview_plugin_mappings::Vecs as MappingsVecs;
use brk_error::Result;

use brk_types::{Cents, Dollars, Height, StoredF64, Version};
use vecdb::{CachedBoxedVec, Database, EagerVec, ImportableVec};

use super::Vecs;
use bitview_compute::{CACHE_BUDGET, Identity, LazyIndexedVec, LazyPerBlock};

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    spot_price: &CachedBoxedVec<Height, Cents>,
) -> Result<Vecs> {
    let v1 = version + Version::ONE;
    let hodl_bank = CACHE_BUDGET.wrap(EagerVec::forced_import(db, "hodl_bank", v1)?);
    let value_source = LazyIndexedVec::new(
        "reserve_risk_source",
        v1,
        &hodl_bank,
        spot_price,
        |_, hodl_bank, spot| StoredF64::from(Dollars::from(spot)) / hodl_bank,
    );
    Ok(Vecs {
        vocdd_median_1y: EagerVec::forced_import(db, "vocdd_median_1y", v1)?,
        hodl_bank,
        value: LazyPerBlock::from_height_source::<Identity<StoredF64>>(
            "reserve_risk",
            v1,
            &value_source,
            mappings,
        ),
    })
}
