use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{StoredF32, Version};
use schemars::JsonSchema;
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode, UnaryTransform};

use crate::indexes;

use super::{ComputedFromHeight, LazyFromHeight};
use crate::internal::NumericValue;

/// Basis-point storage with lazy float view.
///
/// Stores integer basis points on disk (Pco-compressed),
/// exposes a lazy StoredF32 view (bps / 100).
#[derive(Traversable)]
pub struct BpsFromHeight<B, M: StorageMode = Rw>
where
    B: NumericValue + JsonSchema,
{
    pub bps: ComputedFromHeight<B, M>,
    pub float: LazyFromHeight<StoredF32, B>,
}

impl<B> BpsFromHeight<B>
where
    B: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import<F: UnaryTransform<B, StoredF32>>(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let bps = ComputedFromHeight::forced_import(db, name, version, indexes)?;

        let float = LazyFromHeight::from_computed::<F>(
            &format!("{name}_float"),
            version,
            bps.height.read_only_boxed_clone(),
            &bps,
        );

        Ok(Self { bps, float })
    }
}
