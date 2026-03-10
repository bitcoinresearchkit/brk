use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountPerBlock, RollingWindow24h, WindowStarts, Windows},
};

/// Single 24h rolling sum as amount (sats + btc + cents + usd).
///
/// Tree: `_24h.sats.height`, `_24h.btc.height`, etc.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindow24hAmountPerBlock<M: StorageMode = Rw>(
    pub RollingWindow24h<AmountPerBlock<M>>,
);

impl RollingWindow24hAmountPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(RollingWindow24h {
            _24h: AmountPerBlock::forced_import(db, &format!("{name}_24h"), version, indexes)?,
        }))
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        height_24h_ago: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self._24h
            .compute_rolling_sum(max_from, height_24h_ago, sats_source, cents_source, exit)
    }
}

/// Rolling sum only, window-first then unit.
///
/// Tree: `_24h.sats.height`, `_24h.btc.height`, etc.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingSumAmountPerBlock<M: StorageMode = Rw>(pub Windows<AmountPerBlock<M>>);

impl RollingSumAmountPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::<AmountPerBlock>::forced_import(
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
