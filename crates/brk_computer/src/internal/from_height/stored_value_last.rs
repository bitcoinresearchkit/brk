use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::ByUnit,
};

const VERSION: Version = Version::ZERO;

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct StoredValueFromHeightLast<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    pub base: ByUnit<M>,
}

impl StoredValueFromHeightLast {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            base: ByUnit::forced_import(db, name, v, indexes)?,
        })
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        usd_source: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .sats
            .height
            .compute_rolling_sum(max_from, window_starts, sats_source, exit)?;
        self.base
            .usd
            .height
            .compute_rolling_sum(max_from, window_starts, usd_source, exit)?;
        Ok(())
    }
}
