//! Value type for Height + Rolling pattern.
//!
//! Combines Value (sats/btc/usd per height, no period views) with
//! AmountPerBlockWindows (rolling sums across 4 windows).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{Amount, AmountPerBlockWindows, WindowStarts},
    prices,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct AmountPerBlockRolling<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub amount: Amount<Height, M>,
    #[traversable(flatten)]
    pub rolling: AmountPerBlockWindows<M>,
}

impl AmountPerBlockRolling {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            amount: Amount::forced_import(db, name, version)?,
            rolling: AmountPerBlockWindows::forced_import(db, name, version, indexes)?,
        })
    }

    /// Compute sats height via closure, then cents from price, then rolling windows.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        prices: &prices::Vecs,
        exit: &Exit,
        compute_sats: impl FnOnce(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        compute_sats(&mut self.amount.sats)?;
        self.amount.compute_cents(prices, max_from, exit)?;
        self.rolling.compute_rolling_sum(
            max_from,
            windows,
            &self.amount.sats,
            &self.amount.cents,
            exit,
        )?;
        Ok(())
    }
}
