use brk_error::Result;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::{AnyExportableVec, Database, ReadOnlyClone, Ro, Rw, StorageMode, WritableVec};

use crate::indexes;
use crate::internal::{ComputedFromHeightLast, Price};

pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

/// Compute spot percentile rank by interpolating within percentile bands.
/// Returns a value between 0 and 100 indicating where spot sits in the distribution.
pub(crate) fn compute_spot_percentile_rank(
    percentile_prices: &[Dollars; PERCENTILES_LEN],
    spot: Dollars,
) -> StoredF32 {
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

pub struct PercentilesVecs<M: StorageMode = Rw> {
    pub vecs: [Price<ComputedFromHeightLast<Dollars, M>>; PERCENTILES_LEN],
}

const VERSION: Version = Version::ONE;

impl PercentilesVecs {
    pub(crate) fn forced_import(
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let vecs = PERCENTILES
            .into_iter()
            .map(|p| {
                let metric_name = format!("{prefix}_pct{p:02}");
                Price::forced_import(db, &metric_name, version + VERSION, indexes)
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .ok()
            .expect("PERCENTILES length mismatch");

        Ok(Self { vecs })
    }

    /// Push percentile prices at this height.
    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        percentile_prices: &[Dollars; PERCENTILES_LEN],
    ) -> Result<()> {
        for (i, v) in self.vecs.iter_mut().enumerate() {
            v.usd.height.truncate_push(height, percentile_prices[i])?;
        }
        Ok(())
    }

    /// Validate computed versions or reset if mismatched.
    pub(crate) fn validate_computed_version_or_reset(&mut self, version: Version) -> Result<()> {
        for vec in self.vecs.iter_mut() {
            vec.usd.height.validate_computed_version_or_reset(version)?;
        }
        Ok(())
    }
}

impl ReadOnlyClone for PercentilesVecs {
    type ReadOnly = PercentilesVecs<Ro>;

    fn read_only_clone(&self) -> Self::ReadOnly {
        PercentilesVecs {
            vecs: self
                .vecs
                .each_ref()
                .map(|v| v.read_only_clone()),
        }
    }
}

impl<M: StorageMode> Traversable for PercentilesVecs<M>
where
    Price<ComputedFromHeightLast<Dollars, M>>: Traversable,
{
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            PERCENTILES
                .iter()
                .zip(self.vecs.iter())
                .map(|(p, v)| (format!("pct{p:02}"), v.to_tree_node()))
                .collect(),
        )
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        self.vecs
            .iter()
            .flat_map(|p| p.iter_any_exportable())
    }
}
