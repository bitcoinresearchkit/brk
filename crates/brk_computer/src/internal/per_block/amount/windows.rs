//! AmountPerBlockWindows - window-first ordering.
//!
//! Access pattern: `coinbase_sum._24h.sats.height`
//! Each window (24h, 7d, 30d, 1y) contains sats (stored) + btc (lazy) + usd (stored).

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use brk_types::{Cents, Sats};

use crate::{
    indexes,
    internal::{AmountPerBlock, WindowStarts, Windows},
};

/// Value rolling windows — window-first, currency-last.
///
/// Each window contains `AmountPerBlock` (sats + btc lazy + usd).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct AmountPerBlockWindows<M: StorageMode = Rw>(pub Windows<AmountPerBlock<M>>);

impl AmountPerBlockWindows {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::try_from_fn(|suffix| {
            AmountPerBlock::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
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
            w.compute_rolling_sum(max_from, *starts, sats_source, cents_source, exit)?;
        }
        Ok(())
    }
}
