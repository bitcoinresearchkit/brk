//! RollingDistribution - 8 distribution stats, each a RollingWindows.
//!
//! Computes average, min, max, p10, p25, median, p75, p90 rolling windows
//! from a single source vec in a single sorted-vec pass per window.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedVecValue, DistributionStats, NumericValue, RollingWindows, WindowStarts},
    traits::compute_rolling_distribution_from_starts,
};

/// 8 distribution stats × 4 windows = 32 stored height vecs, each with 17 index views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingDistribution<T, M: StorageMode = Rw>(pub DistributionStats<RollingWindows<T, M>>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

const VERSION: Version = Version::ZERO;

impl<T> RollingDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self(DistributionStats {
            average: RollingWindows::forced_import(db, &format!("{name}_average"), v, indexes)?,
            min: RollingWindows::forced_import(db, &format!("{name}_min"), v, indexes)?,
            max: RollingWindows::forced_import(db, &format!("{name}_max"), v, indexes)?,
            pct10: RollingWindows::forced_import(db, &format!("{name}_p10"), v, indexes)?,
            pct25: RollingWindows::forced_import(db, &format!("{name}_p25"), v, indexes)?,
            median: RollingWindows::forced_import(db, &format!("{name}_median"), v, indexes)?,
            pct75: RollingWindows::forced_import(db, &format!("{name}_p75"), v, indexes)?,
            pct90: RollingWindows::forced_import(db, &format!("{name}_p90"), v, indexes)?,
        }))
    }

    /// Compute all 8 distribution stats across all 4 windows from a single source.
    ///
    /// Uses a single sorted-vec pass per window that extracts all 8 stats:
    /// - average: running sum / count
    /// - min/max: first/last of sorted vec
    /// - p10/p25/median/p75/p90: percentile interpolation from sorted vec
    pub(crate) fn compute_distribution(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
    {
        compute_rolling_distribution_from_starts(
            max_from, windows._24h, source,
            &mut self.0.average._24h.height, &mut self.0.min._24h.height,
            &mut self.0.max._24h.height, &mut self.0.pct10._24h.height,
            &mut self.0.pct25._24h.height, &mut self.0.median._24h.height,
            &mut self.0.pct75._24h.height, &mut self.0.pct90._24h.height, exit,
        )?;
        compute_rolling_distribution_from_starts(
            max_from, windows._7d, source,
            &mut self.0.average._7d.height, &mut self.0.min._7d.height,
            &mut self.0.max._7d.height, &mut self.0.pct10._7d.height,
            &mut self.0.pct25._7d.height, &mut self.0.median._7d.height,
            &mut self.0.pct75._7d.height, &mut self.0.pct90._7d.height, exit,
        )?;
        compute_rolling_distribution_from_starts(
            max_from, windows._30d, source,
            &mut self.0.average._30d.height, &mut self.0.min._30d.height,
            &mut self.0.max._30d.height, &mut self.0.pct10._30d.height,
            &mut self.0.pct25._30d.height, &mut self.0.median._30d.height,
            &mut self.0.pct75._30d.height, &mut self.0.pct90._30d.height, exit,
        )?;
        compute_rolling_distribution_from_starts(
            max_from, windows._1y, source,
            &mut self.0.average._1y.height, &mut self.0.min._1y.height,
            &mut self.0.max._1y.height, &mut self.0.pct10._1y.height,
            &mut self.0.pct25._1y.height, &mut self.0.median._1y.height,
            &mut self.0.pct75._1y.height, &mut self.0.pct90._1y.height, exit,
        )?;

        Ok(())
    }
}
