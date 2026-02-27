use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ByUnit, WindowStarts, Windows},
};

/// Rolling sum only, window-first then unit.
///
/// Tree: `_24h.sats.height`, `_24h.btc.height`, etc.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingSumByUnit<M: StorageMode = Rw>(pub Windows<ByUnit<M>>);

const VERSION: Version = Version::ZERO;

impl RollingSumByUnit {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self(Windows::<ByUnit>::forced_import(db, &format!("{name}_sum"), v, indexes)?))
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        sats_source: &impl ReadableVec<Height, Sats>,
        usd_source: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        for (w, starts) in self.0.as_mut_array().into_iter().zip(windows.as_array()) {
            w.sats.height.compute_rolling_sum(max_from, starts, sats_source, exit)?;
            w.usd.height.compute_rolling_sum(max_from, starts, usd_source, exit)?;
        }
        Ok(())
    }
}
