use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        BpsType, CachedWindowStarts, PerBlock, LazyRollingDeltasFromHeight, NumericValue,
    },
};

#[derive(Deref, DerefMut, Traversable)]
pub struct PerBlockWithDeltas<S, C, B, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema + Into<f64>,
    C: NumericValue + JsonSchema + From<f64>,
    B: BpsType + From<f64>,
{
    #[deref]
    #[deref_mut]
    pub base: PerBlock<S, M>,
    pub delta: LazyRollingDeltasFromHeight<S, C, B>,
}

impl<S, C, B> PerBlockWithDeltas<S, C, B>
where
    S: NumericValue + JsonSchema + Into<f64>,
    C: NumericValue + JsonSchema + From<f64>,
    B: BpsType + From<f64>,
{
    pub(crate) fn forced_import(
        db: &vecdb::Database,
        name: &str,
        version: Version,
        delta_version_offset: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let base = PerBlock::forced_import(db, name, version, indexes)?;

        let delta = LazyRollingDeltasFromHeight::new(
            &format!("{name}_delta"),
            version + delta_version_offset,
            &base.height,
            cached_starts,
            indexes,
        );

        Ok(Self { base, delta })
    }
}
