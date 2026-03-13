use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{BpsType, CachedWindowStarts, LazyRollingDeltasFiatFromHeight},
};

use super::{CentsType, FiatPerBlock};

#[derive(Deref, DerefMut, Traversable)]
pub struct FiatPerBlockWithDeltas<C, CS, B, M: StorageMode = Rw>
where
    C: CentsType + Into<f64>,
    CS: CentsType + From<f64>,
    B: BpsType + From<f64>,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: FiatPerBlock<C, M>,
    pub delta: LazyRollingDeltasFiatFromHeight<C, CS, B>,
}

impl<C, CS, B> FiatPerBlockWithDeltas<C, CS, B>
where
    C: CentsType + JsonSchema + Into<f64>,
    CS: CentsType + From<f64>,
    B: BpsType + From<f64>,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        delta_version_offset: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let inner = FiatPerBlock::forced_import(db, name, version, indexes)?;

        let delta = LazyRollingDeltasFiatFromHeight::new(
            &format!("{name}_delta"),
            version + delta_version_offset,
            &inner.cents.height,
            cached_starts,
            indexes,
        );

        Ok(Self { inner, delta })
    }
}
