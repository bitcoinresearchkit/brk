use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, StoredF32, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode, UnaryTransform};

use crate::{
    indexes,
    internal::{Bp16ToFloat, Bp16ToPercent, Emas1w1m, NumericValue, PercentFromHeight},
};

const VERSION: Version = Version::ZERO;

/// 2 EMA vecs (1w, 1m) sourced from 24h rolling window,
/// each storing basis points with lazy ratio and percent float views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PercentRollingEmas1w1m<B, M: StorageMode = Rw>(pub Emas1w1m<PercentFromHeight<B, M>>)
where
    B: NumericValue + JsonSchema;

impl<B> PercentRollingEmas1w1m<B>
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
        let v = version + VERSION;
        Ok(Self(Emas1w1m::try_from_fn(|suffix| {
            PercentFromHeight::forced_import::<RatioTransform, PercentTransform>(
                db,
                &format!("{name}_{suffix}"),
                v,
                indexes,
            )
        })?))
    }

}

impl PercentRollingEmas1w1m<BasisPoints16> {
    pub(crate) fn forced_import_bp16(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import::<Bp16ToFloat, Bp16ToPercent>(db, name, version, indexes)
    }
}

impl<B> PercentRollingEmas1w1m<B>
where
    B: NumericValue + JsonSchema,
{
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
