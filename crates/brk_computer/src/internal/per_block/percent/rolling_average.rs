use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{BpsType, WindowStarts},
};

use crate::internal::{ComputedPerBlockRollingAverage, LazyPerBlock};

/// Like PercentPerBlock but with rolling average stats on the bps data.
#[derive(Traversable)]
pub struct PercentPerBlockRollingAverage<B: BpsType, M: StorageMode = Rw> {
    pub bps: ComputedPerBlockRollingAverage<B, M>,
    pub ratio: LazyPerBlock<StoredF32, B>,
    pub percent: LazyPerBlock<StoredF32, B>,
}

impl<B: BpsType> PercentPerBlockRollingAverage<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let bps = ComputedPerBlockRollingAverage::forced_import(
            db,
            &format!("{name}_bps"),
            version,
            indexes,
        )?;

        let ratio = LazyPerBlock::from_height_source::<B::ToRatio>(
            &format!("{name}_ratio"),
            version,
            bps.height.read_only_boxed_clone(),
            indexes,
        );

        let percent = LazyPerBlock::from_height_source::<B::ToPercent>(
            name,
            version,
            bps.height.read_only_boxed_clone(),
            indexes,
        );

        Ok(Self {
            bps,
            ratio,
            percent,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, B>>) -> Result<()>,
    ) -> Result<()>
    where
        B: Default,
        f64: From<B>,
    {
        self.bps.compute(max_from, windows, exit, compute_height)
    }
}
