use brk_error::Result;

use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    IndexSources, LazyRollingAvgsAmountFromHeight, LazyRollingSumsAmountFromHeight,
    ValuePerBlockCumulative, Windows,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct ValuePerBlockCumulativeRolling<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ValuePerBlockCumulative<M>,
    pub sum: LazyRollingSumsAmountFromHeight,
    pub average: LazyRollingAvgsAmountFromHeight,
}

const VERSION: Version = Version::TWO;

impl ValuePerBlockCumulativeRolling {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let inner = ValuePerBlockCumulative::forced_import(db, name, v, indexes)?;
        let sum = LazyRollingSumsAmountFromHeight::new(
            &format!("{name}_sum"),
            v,
            inner.cumulative.sats.resolutions.height_source(),
            inner.cumulative.cents.resolutions.height_source(),
            window_starts,
            indexes,
        );
        let average = LazyRollingAvgsAmountFromHeight::new(
            &format!("{name}_average"),
            v,
            inner.cumulative.sats.resolutions.height_source(),
            inner.cumulative.cents.resolutions.height_source(),
            window_starts,
            indexes,
        );

        Ok(Self {
            inner,
            sum,
            average,
        })
    }
}
