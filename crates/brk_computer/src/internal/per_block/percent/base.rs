use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    BinaryTransform, Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, VecValue,
};

use crate::{
    indexes,
    internal::{BpsType, Percent, algo::ComputeDrawdown},
};

use crate::internal::{LazyPerBlock, PerBlock};

/// Basis-point storage with both ratio and percentage float views.
///
/// Stores integer basis points on disk (Pco-compressed),
/// exposes two lazy StoredF32 views:
/// - `ratio`: bps / 10000 (e.g., 4523 bps -> 0.4523)
/// - `percent`: bps / 100 (e.g., 4523 bps -> 45.23%)
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PercentPerBlock<B: BpsType, M: StorageMode = Rw>(
    pub Percent<PerBlock<B, M>, LazyPerBlock<StoredF32, B>>,
);

impl<B: BpsType> PercentPerBlock<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let bps = PerBlock::forced_import(db, &format!("{name}_bps"), version, indexes)?;
        let bps_clone = bps.height.read_only_boxed_clone();

        let ratio = LazyPerBlock::from_computed::<B::ToRatio>(
            &format!("{name}_ratio"),
            version,
            bps_clone.clone(),
            &bps,
        );

        let percent = LazyPerBlock::from_computed::<B::ToPercent>(name, version, bps_clone, &bps);

        Ok(Self(Percent {
            bps,
            ratio,
            percent,
        }))
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
        self.bps
            .compute_binary::<S1T, S2T, F>(max_from, source1, source2, exit)
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
        self.bps
            .height
            .compute_drawdown(max_from, current, ath, exit)
    }
}
