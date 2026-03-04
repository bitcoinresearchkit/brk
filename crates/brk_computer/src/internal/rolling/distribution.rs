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
        Ok(Self(DistributionStats::try_from_fn(|suffix| {
            RollingWindows::forced_import(db, &format!("{name}_{suffix}"), v, indexes)
        })?))
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
        macro_rules! compute_window {
            ($w:ident) => {
                compute_rolling_distribution_from_starts(
                    max_from, windows.$w, source,
                    &mut self.0.average.$w.height, &mut self.0.min.$w.height,
                    &mut self.0.max.$w.height, &mut self.0.pct10.$w.height,
                    &mut self.0.pct25.$w.height, &mut self.0.median.$w.height,
                    &mut self.0.pct75.$w.height, &mut self.0.pct90.$w.height, exit,
                )?
            };
        }
        compute_window!(_24h);
        compute_window!(_1w);
        compute_window!(_1m);
        compute_window!(_1y);

        Ok(())
    }
}
