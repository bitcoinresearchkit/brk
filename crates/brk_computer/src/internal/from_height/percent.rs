use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use vecdb::{BinaryTransform, Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, VecValue};

use crate::{
    indexes,
    internal::BpsType,
    traits::ComputeDrawdown,
};

use super::{ComputedFromHeight, LazyFromHeight};

/// Basis-point storage with both ratio and percentage float views.
///
/// Stores integer basis points on disk (Pco-compressed),
/// exposes two lazy StoredF32 views:
/// - `ratio`: bps / 10000 (e.g., 4523 bps -> 0.4523)
/// - `percent`: bps / 100 (e.g., 4523 bps -> 45.23%)
#[derive(Traversable)]
pub struct PercentFromHeight<B: BpsType, M: StorageMode = Rw> {
    pub bps: ComputedFromHeight<B, M>,
    pub ratio: LazyFromHeight<StoredF32, B>,
    pub percent: LazyFromHeight<StoredF32, B>,
}

impl<B: BpsType> PercentFromHeight<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let bps = ComputedFromHeight::forced_import(db, &format!("{name}_bps"), version, indexes)?;

        let ratio = LazyFromHeight::from_computed::<B::ToRatio>(
            &format!("{name}_ratio"),
            version,
            bps.height.read_only_boxed_clone(),
            &bps,
        );

        let percent = LazyFromHeight::from_computed::<B::ToPercent>(
            name,
            version,
            bps.height.read_only_boxed_clone(),
            &bps,
        );

        Ok(Self { bps, ratio, percent })
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
