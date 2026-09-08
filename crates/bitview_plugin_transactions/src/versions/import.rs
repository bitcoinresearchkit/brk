use bitview_collections::Windows;
use bitview_vecs::{
    CachedWindowStartVec, ColumnarPerBlockCumulativeRolling, LazyColumnPerBlockCumulativeRolling,
};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database, ReadOnlyClone};

use super::{Vecs, VersionId};

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    let source = ColumnarPerBlockCumulativeRolling::forced_import(
        db,
        "tx_version_count_cumulative",
        version,
        |_| (),
    )?;
    let counts = source.cumulative.read_only_clone();
    let import = |name, version_id| {
        LazyColumnPerBlockCumulativeRolling::new(
            cache,
            name,
            version,
            &counts,
            version_id,
            mappings,
            cached_starts,
        )
    };

    Ok(Vecs {
        v1: import("tx_v1", VersionId::V1),
        v2: import("tx_v2", VersionId::V2),
        v3: import("tx_v3", VersionId::V3),
        other: import("tx_other_version", VersionId::Other),
        source,
    })
}
