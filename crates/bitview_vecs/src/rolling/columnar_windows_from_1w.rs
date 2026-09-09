use bitview_collections::{WindowFrom1wId, WindowsFrom1w};
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
pub struct ColumnarRollingWindowsFrom1w<T, M: StorageMode = Rw>(
    pub ColumnarPerBlock<T, WindowFrom1wId, WindowsFrom1w<LazyColumnPerBlock<T, WindowFrom1wId>>, M>,
)
where
    T: NumericValue + JsonSchema;

impl<T> ColumnarRollingWindowsFrom1w<T>
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
            cache,
            db,
            name,
            version,
            |source| {
                WindowFrom1wId::series(|window| {
                    LazyColumnPerBlock::new(
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
