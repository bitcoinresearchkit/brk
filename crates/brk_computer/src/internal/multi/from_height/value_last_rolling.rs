//! Value type for Height + Rolling pattern.
//!
//! Combines ValueFromHeight (sats/btc/usd per height, no period views) with
//! StoredValueRollingWindows (rolling sums across 4 windows).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{StoredValueRollingWindows, ValueFromHeight, WindowStarts},
    prices,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct ValueFromHeightLastRolling<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub value: ValueFromHeight<M>,
    pub rolling: StoredValueRollingWindows<M>,
}

const VERSION: Version = Version::ZERO;

impl ValueFromHeightLastRolling {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            value: ValueFromHeight::forced_import(db, name, v)?,
            rolling: StoredValueRollingWindows::forced_import(db, name, v, indexes)?,
        })
    }

    /// Compute sats height via closure, then USD from price, then rolling windows.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        prices: &prices::Vecs,
        exit: &Exit,
        compute_sats: impl FnOnce(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        compute_sats(&mut self.value.sats)?;
        self.value.compute_usd(prices, max_from, exit)?;
        self.rolling.compute_rolling_sum(
            max_from,
            windows,
            &self.value.sats,
            &self.value.usd,
            exit,
        )?;
        Ok(())
    }
}
