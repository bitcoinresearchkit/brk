use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints16, BasisPointsSigned16, BasisPointsSigned32, Height, StoredF32, Version,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, UnaryTransform, VecValue};

use crate::{
    indexes,
    internal::{
        Bp16ToFloat, Bp16ToPercent, Bps16ToFloat, Bps16ToPercent, Bps32ToFloat, Bps32ToPercent,
        NumericValue,
    },
    traits::ComputeDrawdown,
};

use super::{ComputedFromHeight, LazyFromHeight};

/// Basis-point storage with both ratio and percentage float views.
///
/// Stores integer basis points on disk (Pco-compressed),
/// exposes two lazy StoredF32 views:
/// - `ratio`: bps ÷ 10000 (e.g., 4523 bps → 0.4523)
/// - `percent`: bps ÷ 100 (e.g., 4523 bps → 45.23%)
///
/// Use for dominance, adoption, RSI, and other percentage-valued metrics.
#[derive(Traversable)]
pub struct PercentFromHeight<B, M: StorageMode = Rw>
where
    B: NumericValue + JsonSchema,
{
    pub bps: ComputedFromHeight<B, M>,
    pub ratio: LazyFromHeight<StoredF32, B>,
    pub percent: LazyFromHeight<StoredF32, B>,
}

impl<B> PercentFromHeight<B>
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
        let bps = ComputedFromHeight::forced_import(db, &format!("{name}_bps"), version, indexes)?;

        let ratio = LazyFromHeight::from_computed::<RatioTransform>(
            &format!("{name}_ratio"),
            version,
            bps.height.read_only_boxed_clone(),
            &bps,
        );

        let percent = LazyFromHeight::from_computed::<PercentTransform>(
            name,
            version,
            bps.height.read_only_boxed_clone(),
            &bps,
        );

        Ok(Self { bps, ratio, percent })
    }

}

impl PercentFromHeight<BasisPoints16> {
    pub(crate) fn forced_import_bp16(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import::<Bp16ToFloat, Bp16ToPercent>(db, name, version, indexes)
    }
}

impl PercentFromHeight<BasisPointsSigned16> {
    pub(crate) fn forced_import_bps16(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import::<Bps16ToFloat, Bps16ToPercent>(db, name, version, indexes)
    }
}

impl PercentFromHeight<BasisPointsSigned32> {
    pub(crate) fn forced_import_bps32(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import::<Bps32ToFloat, Bps32ToPercent>(db, name, version, indexes)
    }
}

impl<B> PercentFromHeight<B>
where
    B: NumericValue + JsonSchema,
{
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

    pub(crate) fn compute_drawdown<C, A>(
        &mut self,
        max_from: Height,
        current: &impl ReadableVec<Height, C>,
        ath: &impl ReadableVec<Height, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        C: VecValue,
        A: VecValue,
        f64: From<C> + From<A>,
        vecdb::EagerVec<vecdb::PcoVec<Height, B>>: ComputeDrawdown<Height>,
    {
        self.bps.height.compute_drawdown(max_from, current, ath, exit)
    }
}
