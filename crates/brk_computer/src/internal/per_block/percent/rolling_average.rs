use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{BpsType, CachedWindowStarts},
};

use crate::internal::{PerBlockRollingAverage, LazyPerBlock};

/// Like PercentPerBlock but with rolling average stats on the bps data.
#[derive(Traversable)]
pub struct PercentPerBlockRollingAverage<B: BpsType, M: StorageMode = Rw> {
    pub bps: PerBlockRollingAverage<B, M>,
    pub ratio: LazyPerBlock<StoredF32, B>,
    pub percent: LazyPerBlock<StoredF32, B>,
}

impl<B: BpsType> PercentPerBlockRollingAverage<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let bps = PerBlockRollingAverage::forced_import(
            db,
            &format!("{name}_bps"),
            version,
            indexes,
            cached_starts,
        )?;

        let ratio = LazyPerBlock::from_height_source::<B::ToRatio>(
            &format!("{name}_ratio"),
            version,
            bps.base.read_only_boxed_clone(),
            indexes,
        );

        let percent = LazyPerBlock::from_height_source::<B::ToPercent>(
            name,
            version,
            bps.base.read_only_boxed_clone(),
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
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, B>>) -> Result<()>,
    ) -> Result<()> {
        self.bps.compute(max_from, exit, compute_height)
    }
}
