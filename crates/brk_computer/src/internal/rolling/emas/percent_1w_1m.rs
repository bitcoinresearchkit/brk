use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{BpsType, Emas1w1m, PercentFromHeight},
};

/// 2 EMA vecs (1w, 1m) sourced from 24h rolling window,
/// each storing basis points with lazy ratio and percent float views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PercentRollingEmas1w1m<B: BpsType, M: StorageMode = Rw>(
    pub Emas1w1m<PercentFromHeight<B, M>>,
);

impl<B: BpsType> PercentRollingEmas1w1m<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Emas1w1m::try_from_fn(|suffix| {
            PercentFromHeight::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
    }

    pub(crate) fn compute_from_24h(
        &mut self,
        max_from: Height,
        height_1w_ago: &impl ReadableVec<Height, Height>,
        height_1m_ago: &impl ReadableVec<Height, Height>,
        source: &impl ReadableVec<Height, B>,
        exit: &Exit,
    ) -> Result<()>
    where
        f64: From<B>,
        B: From<f64> + Default,
    {
        self._1w
            .bps
            .height
            .compute_rolling_ema(max_from, height_1w_ago, source, exit)?;
        self._1m
            .bps
            .height
            .compute_rolling_ema(max_from, height_1m_ago, source, exit)?;
        Ok(())
    }
}
