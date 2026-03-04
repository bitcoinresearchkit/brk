use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredF32, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Rw, StorageMode, UnaryTransform};

use crate::{
    indexes,
    internal::{Bp16ToFloat, Bp16ToPercent, NumericValue, PercentFromHeight, Windows},
};

const VERSION: Version = Version::ZERO;

/// 4 rolling window vecs (24h, 1w, 1m, 1y), each storing basis points
/// with lazy ratio and percent float views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PercentRollingWindows<B, M: StorageMode = Rw>(pub Windows<PercentFromHeight<B, M>>)
where
    B: NumericValue + JsonSchema;

impl<B> PercentRollingWindows<B>
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
        Ok(Self(Windows::try_from_fn(|suffix| {
            PercentFromHeight::forced_import::<RatioTransform, PercentTransform>(
                db,
                &format!("{name}_{suffix}"),
                v,
                indexes,
            )
        })?))
    }
}

impl PercentRollingWindows<BasisPoints16> {
    pub(crate) fn forced_import_bp16(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import::<Bp16ToFloat, Bp16ToPercent>(db, name, version, indexes)
    }
}
