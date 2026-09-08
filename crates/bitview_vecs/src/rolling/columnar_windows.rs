use bitview_collections::{WindowId, Windows};
use bitview_compute::NumericValue;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::{ColumnarPerBlock, IndexSources, LazyColumnPerBlock};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ColumnarRollingWindows<T, M: StorageMode = Rw>(
    pub ColumnarPerBlock<T, WindowId, Windows<LazyColumnPerBlock<T, WindowId>>, M>,
)
where
    T: NumericValue + JsonSchema;

impl<T> ColumnarRollingWindows<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        Ok(Self(ColumnarPerBlock::forced_import(
            db,
            name,
            version,
            |source| {
                WindowId::series(|window| {
                    LazyColumnPerBlock::new(
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
