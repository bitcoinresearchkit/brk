use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ByUnit, DistributionStats, WindowStarts, Windows},
    traits::compute_rolling_distribution_from_starts,
};

/// One window slot: sum + 8 distribution stats, each a ByUnit.
///
/// Tree: `sum.sats.height`, `average.sats.height`, etc.
#[derive(Traversable)]
pub struct RollingFullSlot<M: StorageMode = Rw> {
    pub sum: ByUnit<M>,
    #[traversable(flatten)]
    pub distribution: DistributionStats<ByUnit<M>>,
}

impl RollingFullSlot {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            sum: ByUnit::forced_import(db, &format!("{name}_sum"), version, indexes)?,
            distribution: DistributionStats {
                average: ByUnit::forced_import(db, &format!("{name}_average"), version, indexes)?,
                min: ByUnit::forced_import(db, &format!("{name}_min"), version, indexes)?,
                max: ByUnit::forced_import(db, &format!("{name}_max"), version, indexes)?,
                pct10: ByUnit::forced_import(db, &format!("{name}_p10"), version, indexes)?,
                pct25: ByUnit::forced_import(db, &format!("{name}_p25"), version, indexes)?,
                median: ByUnit::forced_import(db, &format!("{name}_median"), version, indexes)?,
                pct75: ByUnit::forced_import(db, &format!("{name}_p75"), version, indexes)?,
                pct90: ByUnit::forced_import(db, &format!("{name}_p90"), version, indexes)?,
            },
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        usd_source: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.sum.sats.height.compute_rolling_sum(max_from, starts, sats_source, exit)?;
        self.sum.usd.height.compute_rolling_sum(max_from, starts, usd_source, exit)?;

        let d = &mut self.distribution;

        compute_rolling_distribution_from_starts(
            max_from, starts, sats_source,
            &mut d.average.sats.height, &mut d.min.sats.height,
            &mut d.max.sats.height, &mut d.pct10.sats.height,
            &mut d.pct25.sats.height, &mut d.median.sats.height,
            &mut d.pct75.sats.height, &mut d.pct90.sats.height, exit,
        )?;

        compute_rolling_distribution_from_starts(
            max_from, starts, usd_source,
            &mut d.average.usd.height, &mut d.min.usd.height,
            &mut d.max.usd.height, &mut d.pct10.usd.height,
            &mut d.pct25.usd.height, &mut d.median.usd.height,
            &mut d.pct75.usd.height, &mut d.pct90.usd.height, exit,
        )?;

        Ok(())
    }
}

/// Rolling sum + distribution across 4 windows, window-first.
///
/// Tree: `_24h.sum.sats.height`, `_24h.average.sats.height`, etc.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingFullByUnit<M: StorageMode = Rw>(pub Windows<RollingFullSlot<M>>);

const VERSION: Version = Version::ZERO;

impl RollingFullByUnit {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self(Windows {
            _24h: RollingFullSlot::forced_import(db, &format!("{name}_24h"), v, indexes)?,
            _7d: RollingFullSlot::forced_import(db, &format!("{name}_7d"), v, indexes)?,
            _30d: RollingFullSlot::forced_import(db, &format!("{name}_30d"), v, indexes)?,
            _1y: RollingFullSlot::forced_import(db, &format!("{name}_1y"), v, indexes)?,
        }))
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        sats_source: &impl ReadableVec<Height, Sats>,
        usd_source: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        for (slot, starts) in self.0.as_mut_array().into_iter().zip(windows.as_array()) {
            slot.compute(max_from, starts, sats_source, usd_source, exit)?;
        }
        Ok(())
    }
}
