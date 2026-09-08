use bitview_collections::{WindowId, Windows};
use bitview_compute::FixedRatio;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::{ColumnarPerBlock, IndexSources, LazyColumnPercentPerBlock};

/// Four named fixed-point percentage views backed by one columnar source.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ColumnarPercentRollingWindows<B: FixedRatio, M: StorageMode = Rw>(
    pub ColumnarPerBlock<B, WindowId, Windows<LazyColumnPercentPerBlock<B, WindowId>>, M>,
);

impl<B: FixedRatio> ColumnarPercentRollingWindows<B> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        Ok(Self(ColumnarPerBlock::forced_import(
            db,
            &format!("{name}_{}", B::SUFFIX),
            version,
            |source| {
                WindowId::series(|window| {
                    LazyColumnPercentPerBlock::new(
                        cache,
                        &format!("{name}_{}", window.suffix()),
                        version,
                        source,
                        window,
                        indexes,
                    )
                })
            },
        )?))
    }
}
