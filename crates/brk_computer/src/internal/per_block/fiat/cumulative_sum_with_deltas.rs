use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{BpsType, LazyRollingDeltasFiatFromHeight, WindowStartVec, Windows},
};

use super::{FiatPerBlockCumulativeWithSums, FiatType};

#[derive(Deref, DerefMut, Traversable)]
pub struct FiatPerBlockCumulativeWithSumsAndDeltas<C, CS, B, M: StorageMode = Rw>
where
    C: FiatType + Into<f64>,
    CS: FiatType + From<f64>,
    B: BpsType + From<f64>,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: FiatPerBlockCumulativeWithSums<C, M>,
    pub delta: LazyRollingDeltasFiatFromHeight<C, CS, B>,
}

impl<C, CS, B> FiatPerBlockCumulativeWithSumsAndDeltas<C, CS, B>
where
    C: FiatType + Into<f64>,
    CS: FiatType + From<f64>,
    B: BpsType + From<f64>,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        delta_version_offset: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let inner = FiatPerBlockCumulativeWithSums::forced_import(
            db,
            name,
            version,
            indexes,
            cached_starts,
        )?;

        let delta = LazyRollingDeltasFiatFromHeight::new(
            &format!("{name}_delta"),
            version + delta_version_offset,
            &inner.cumulative.cents.height,
            cached_starts,
            indexes,
        );

        Ok(Self { inner, delta })
    }
}
