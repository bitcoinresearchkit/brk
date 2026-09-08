use bitview_collections::Windows;
use bitview_transforms::{BoundedOddsF64, BoundedToF64};
use bitview_vecs::{CachedWindowStartVec, LazyPerBlock, PerBlock, PerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::{DerivedVecs, Vecs};

impl DerivedVecs {
    fn forced_import_with_prefix(
        cache: &'static CacheBudget,
        db: &Database,
        prefix: &str,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        let name = |metric: &str| {
            if prefix.is_empty() {
                metric.to_owned()
            } else {
                format!("{prefix}_{metric}")
            }
        };
        let liveliness_name = name("liveliness");
        let version = version + Version::ONE;
        let liveliness_source = PerBlock::forced_import(
            cache,
            db,
            &name("liveliness_bounded_source"),
            version,
            mappings,
        )?;
        let liveliness = LazyPerBlock::from_resolutions::<BoundedToF64>(
            &liveliness_name,
            version,
            &liveliness_source,
        );
        let vaultedness = LazyPerBlock::from_resolutions::<BoundedToF64<true>>(
            &name("vaultedness"),
            version,
            &liveliness_source,
        );
        let ratio = LazyPerBlock::from_resolutions::<BoundedOddsF64>(
            &name("activity_to_vaultedness"),
            version + Version::ONE,
            &liveliness_source,
        );

        Ok(Self {
            liveliness_source,
            liveliness,
            vaultedness,
            ratio,
        })
    }
}

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        coinblocks_created: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "coinblocks_created",
            version,
            mappings,
            cached_starts,
        )?,
        coinblocks_stored: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "coinblocks_stored",
            version,
            mappings,
            cached_starts,
        )?,
        derived: DerivedVecs::forced_import_with_prefix(cache, db, "", version, mappings)?,
    })
}
