use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        AmountPerBlock, DistributionStats, WindowStarts, Windows,
        algo::compute_rolling_distribution_from_starts,
    },
};

/// One window slot: 8 distribution stats, each a AmountPerBlock.
///
/// Tree: `average.sats.height`, `min.sats.height`, etc.
#[derive(Traversable)]
pub struct RollingDistributionSlot<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub distribution: DistributionStats<AmountPerBlock<M>>,
}

impl RollingDistributionSlot {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            distribution: DistributionStats::try_from_fn(|suffix| {
                AmountPerBlock::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
            })?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        let d = &mut self.distribution;

        macro_rules! compute_unit {
            ($unit:ident, $source:expr) => {
                compute_rolling_distribution_from_starts(
                    max_from,
                    starts,
                    $source,
                    &mut d.average.$unit.height,
                    &mut d.min.$unit.height,
                    &mut d.max.$unit.height,
                    &mut d.pct10.$unit.height,
                    &mut d.pct25.$unit.height,
                    &mut d.median.$unit.height,
                    &mut d.pct75.$unit.height,
                    &mut d.pct90.$unit.height,
                    exit,
                )?
            };
        }
        compute_unit!(sats, sats_source);
        compute_unit!(cents, cents_source);

        Ok(())
    }
}

/// Rolling distribution across 4 windows, window-first.
///
/// Tree: `_24h.average.sats.height`, `_24h.min.sats.height`, etc.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingDistributionAmountPerBlock<M: StorageMode = Rw>(
    pub Windows<RollingDistributionSlot<M>>,
);

impl RollingDistributionAmountPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::try_from_fn(|suffix| {
            RollingDistributionSlot::forced_import(
                db,
                &format!("{name}_{suffix}"),
                version,
                indexes,
            )
        })?))
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        for (slot, starts) in self.0.as_mut_array().into_iter().zip(windows.as_array()) {
            slot.compute(max_from, *starts, sats_source, cents_source, exit)?;
        }
        Ok(())
    }
}
