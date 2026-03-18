use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    Database, EagerVec, ImportableVec, LazyVecFrom1, PcoVec, ReadableCloneableVec, Rw, StorageMode,
};

use crate::internal::{BpsType, Percent};

/// Lightweight percent container: BPS height vec + lazy ratio + lazy percent.
/// No resolutions, no rolling stats.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
#[allow(clippy::type_complexity)]
pub struct PercentVec<B: BpsType, M: StorageMode = Rw>(
    pub Percent<M::Stored<EagerVec<PcoVec<Height, B>>>, LazyVecFrom1<Height, StoredF32, Height, B>>,
);

impl<B: BpsType> PercentVec<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
    ) -> brk_error::Result<Self> {
        let bps: EagerVec<PcoVec<Height, B>> =
            EagerVec::forced_import(db, &format!("{name}_bps"), version)?;
        let bps_clone = bps.read_only_boxed_clone();

        let ratio = LazyVecFrom1::transformed::<B::ToRatio>(
            &format!("{name}_ratio"),
            version,
            bps_clone.clone(),
        );

        let percent = LazyVecFrom1::transformed::<B::ToPercent>(name, version, bps_clone);

        Ok(Self(Percent {
            bps,
            ratio,
            percent,
        }))
    }
}
