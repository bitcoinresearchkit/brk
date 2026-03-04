use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, StoredF32, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableCloneableVec, Rw, StorageMode, UnaryTransform};

use crate::{indexes, internal::{Bp16ToFloat, Bp16ToPercent, NumericValue, WindowStarts}};

use super::{ComputedFromHeightDistribution, LazyFromHeight};

/// Like PercentFromHeight but with rolling distribution stats on the bps data.
#[derive(Traversable)]
pub struct PercentFromHeightDistribution<B, M: StorageMode = Rw>
where
    B: NumericValue + JsonSchema,
{
    pub bps: ComputedFromHeightDistribution<B, M>,
    pub ratio: LazyFromHeight<StoredF32, B>,
    pub percent: LazyFromHeight<StoredF32, B>,
}

impl<B> PercentFromHeightDistribution<B>
where
    B: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import<RatioTransform, PercentTransform>(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self>
    where
        RatioTransform: UnaryTransform<B, StoredF32>,
        PercentTransform: UnaryTransform<B, StoredF32>,
    {
        let bps = ComputedFromHeightDistribution::forced_import(db, &format!("{name}_bps"), version, indexes)?;

        let ratio = LazyFromHeight::from_height_source::<RatioTransform>(
            &format!("{name}_ratio"),
            version,
            bps.height.read_only_boxed_clone(),
            indexes,
        );

        let percent = LazyFromHeight::from_height_source::<PercentTransform>(
            name,
            version,
            bps.height.read_only_boxed_clone(),
            indexes,
        );

        Ok(Self { bps, ratio, percent })
    }

}

impl PercentFromHeightDistribution<BasisPoints16> {
    pub(crate) fn forced_import_bp16(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import::<Bp16ToFloat, Bp16ToPercent>(db, name, version, indexes)
    }
}

impl<B> PercentFromHeightDistribution<B>
where
    B: NumericValue + JsonSchema,
{
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, B>>) -> Result<()>,
    ) -> Result<()>
    where
        B: Copy + Ord + From<f64> + Default,
        f64: From<B>,
    {
        self.bps.compute(max_from, windows, exit, compute_height)
    }
}
