use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountFromHeight, WindowStarts, Windows},
};

/// Rolling sum only, window-first then unit.
///
/// Tree: `_24h.sats.height`, `_24h.btc.height`, etc.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingSumAmountFromHeight<M: StorageMode = Rw>(pub Windows<AmountFromHeight<M>>);

impl RollingSumAmountFromHeight {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::<AmountFromHeight>::forced_import(
            db,
            &format!("{name}_sum"),
            version,
            indexes,
        )?))
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        for (w, starts) in self.0.as_mut_array().into_iter().zip(windows.as_array()) {
            w.sats
                .height
                .compute_rolling_sum(max_from, *starts, sats_source, exit)?;
            w.cents
                .height
                .compute_rolling_sum(max_from, *starts, cents_source, exit)?;
        }
        Ok(())
    }
}
