use brk_error::Result;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{DateIndex, Dollars, StoredF32, Version};
use rayon::prelude::*;
use vecdb::{
    AnyExportableVec, AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, PcoVec,
};

use crate::{ComputeIndexes, indexes};

use super::Price;

pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

/// Compute spot percentile rank by interpolating within percentile bands.
/// Returns a value between 0 and 100 indicating where spot sits in the distribution.
pub fn compute_spot_percentile_rank(percentile_prices: &[Dollars; PERCENTILES_LEN], spot: Dollars) -> StoredF32 {
    if spot.is_nan() || percentile_prices[0].is_nan() {
        return StoredF32::NAN;
    }

    let spot_f64 = f64::from(spot);

    // Below lowest percentile (p5) - extrapolate towards 0
    let p5 = f64::from(percentile_prices[0]);
    if spot_f64 <= p5 {
        if p5 == 0.0 {
            return StoredF32::from(0.0);
        }
        // Linear extrapolation: rank = 5 * (spot / p5)
        return StoredF32::from((5.0 * spot_f64 / p5).max(0.0));
    }

    // Above highest percentile (p95) - extrapolate towards 100
    let p95 = f64::from(percentile_prices[PERCENTILES_LEN - 1]);
    let p90 = f64::from(percentile_prices[PERCENTILES_LEN - 2]);
    if spot_f64 >= p95 {
        if p95 == p90 {
            return StoredF32::from(100.0);
        }
        // Linear extrapolation using p90-p95 slope
        let slope = 5.0 / (p95 - p90);
        return StoredF32::from((95.0 + (spot_f64 - p95) * slope).min(100.0));
    }

    // Find the band containing spot and interpolate
    for i in 0..PERCENTILES_LEN - 1 {
        let lower = f64::from(percentile_prices[i]);
        let upper = f64::from(percentile_prices[i + 1]);

        if spot_f64 >= lower && spot_f64 <= upper {
            let lower_pct = f64::from(PERCENTILES[i]);
            let upper_pct = f64::from(PERCENTILES[i + 1]);

            if upper == lower {
                return StoredF32::from(lower_pct);
            }

            // Linear interpolation
            let ratio = (spot_f64 - lower) / (upper - lower);
            return StoredF32::from(lower_pct + ratio * (upper_pct - lower_pct));
        }
    }

    StoredF32::NAN
}

#[derive(Clone)]
pub struct PercentilesVecs {
    pub vecs: [Option<Price>; PERCENTILES_LEN],
}

const VERSION: Version = Version::ZERO;

impl PercentilesVecs {
    pub fn forced_import(
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &indexes::Vecs,
        compute: bool,
    ) -> Result<Self> {
        let vecs = PERCENTILES.map(|p| {
            compute.then(|| {
                let metric_name = format!("{prefix}_pct{p:02}");
                Price::forced_import(db, &metric_name, version + VERSION, indexes).unwrap()
            })
        });

        Ok(Self { vecs })
    }

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    pub fn min_stateful_dateindex_len(&self) -> usize {
        self.vecs
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.dateindex.len())
            .min()
            .unwrap_or(usize::MAX)
    }

    /// Push percentile prices at date boundary.
    /// Only called when dateindex is Some (last height of the day).
    pub fn truncate_push(
        &mut self,
        dateindex: DateIndex,
        percentile_prices: &[Dollars; PERCENTILES_LEN],
    ) -> Result<()> {
        for (i, vec) in self.vecs.iter_mut().enumerate() {
            if let Some(v) = vec {
                v.dateindex.truncate_push(dateindex, percentile_prices[i])?;
            }
        }
        Ok(())
    }

    pub fn compute_rest(&mut self, starting_indexes: &ComputeIndexes, exit: &Exit) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.compute_rest(
                starting_indexes,
                exit,
                None::<&EagerVec<PcoVec<DateIndex, Dollars>>>,
            )?;
        }
        Ok(())
    }

    pub fn get(&self, percentile: u8) -> Option<&Price> {
        PERCENTILES
            .iter()
            .position(|&p| p == percentile)
            .and_then(|i| self.vecs[i].as_ref())
    }
}

impl PercentilesVecs {
    pub fn write(&mut self) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.dateindex.write()?;
        }
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.vecs
            .iter_mut()
            .flatten()
            .map(|v| &mut v.dateindex as &mut dyn AnyStoredVec)
            .collect::<Vec<_>>()
            .into_par_iter()
    }

    /// Validate computed versions or reset if mismatched.
    pub fn validate_computed_version_or_reset(&mut self, version: Version) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.dateindex.validate_computed_version_or_reset(version)?;
        }
        Ok(())
    }
}

impl Traversable for PercentilesVecs {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            PERCENTILES
                .iter()
                .zip(self.vecs.iter())
                .filter_map(|(p, v)| v.as_ref().map(|v| (format!("pct{p:02}"), v.to_tree_node())))
                .collect(),
        )
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        self.vecs
            .iter()
            .flatten()
            .flat_map(|p| p.iter_any_exportable())
    }
}
