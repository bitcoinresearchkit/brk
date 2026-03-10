use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, VecValue};

use crate::{
    indexes,
    internal::{BpsType, RatioPerBlock, Windows},
};

/// 4 rolling window vecs (24h, 1w, 1m, 1y), each storing basis points
/// with a lazy ratio float view.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RatioRollingWindows<B: BpsType, M: StorageMode = Rw>(
    pub Windows<RatioPerBlock<B, M>>,
);

impl<B: BpsType> RatioRollingWindows<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::try_from_fn(|suffix| {
            RatioPerBlock::forced_import_raw(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
    }

    pub(crate) fn compute_binary<S1T, S2T, F, R1, R2>(
        &mut self,
        max_from: Height,
        sources1: [&R1; 4],
        sources2: [&R2; 4],
        exit: &Exit,
    ) -> Result<()>
    where
        S1T: VecValue,
        S2T: VecValue,
        R1: ReadableVec<Height, S1T>,
        R2: ReadableVec<Height, S2T>,
        F: BinaryTransform<S1T, S2T, B>,
    {
        for ((target, s1), s2) in self
            .0
            .as_mut_array()
            .into_iter()
            .zip(sources1)
            .zip(sources2)
        {
            target.bps.compute_binary::<S1T, S2T, F>(max_from, s1, s2, exit)?;
        }
        Ok(())
    }
}
