use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, UnaryTransform, VecValue};

use crate::indexes;

use super::{ComputedFromHeight, LazyFromHeight};
use crate::internal::NumericValue;

/// Basis-point storage with lazy ratio float view (÷10000).
///
/// Stores integer basis points on disk (Pco-compressed),
/// exposes a lazy StoredF32 ratio (e.g., 25000 bps → 2.5).
#[derive(Traversable)]
pub struct Float32FromHeight<B, M: StorageMode = Rw>
where
    B: NumericValue + JsonSchema,
{
    pub bps: ComputedFromHeight<B, M>,
    pub float: LazyFromHeight<StoredF32, B>,
}

impl<B> Float32FromHeight<B>
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

    pub(crate) fn compute_binary<S1T, S2T, F>(
        &mut self,
        max_from: Height,
        source1: &impl ReadableVec<Height, S1T>,
        source2: &impl ReadableVec<Height, S2T>,
        exit: &Exit,
    ) -> Result<()>
    where
        S1T: VecValue,
        S2T: VecValue,
        F: BinaryTransform<S1T, S2T, B>,
    {
        self.bps.compute_binary::<S1T, S2T, F>(max_from, source1, source2, exit)
    }
}
