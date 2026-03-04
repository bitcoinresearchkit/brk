use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{BpsType, PercentFromHeight, Windows},
};

/// 4 rolling window vecs (24h, 1w, 1m, 1y), each storing basis points
/// with lazy ratio and percent float views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PercentRollingWindows<B: BpsType, M: StorageMode = Rw>(
    pub Windows<PercentFromHeight<B, M>>,
);

impl<B: BpsType> PercentRollingWindows<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::try_from_fn(|suffix| {
            PercentFromHeight::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
    }
}
