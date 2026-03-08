use brk_traversable::Traversable;
use brk_types::Cents;
use rayon::prelude::*;
use serde::Serialize;

use super::CohortName;

/// Number of profitability range boundaries (24 boundaries → 25 buckets).
pub const PROFITABILITY_BOUNDARY_COUNT: usize = 24;

/// Compute 24 boundary prices from spot price for profitability bucketing.
///
/// Boundaries are returned in ascending price order (most profitable first → least profitable last).
/// Bucket assignment: prices ascending in k-way merge means we start at the most-profitable bucket
/// (lowest cost basis = highest profit) and advance the cursor as price crosses each boundary.
///
/// For P% profit: boundary = spot × 100 / (100 + P)
/// For L% loss:   boundary = spot × 100 / (100 - L)
///
/// Returns boundaries in ascending order:
/// [spot/11, spot/6, spot/4, spot/3, spot/2, spot×100/190, spot×100/180, ..., spot×100/10]
pub fn compute_profitability_boundaries(spot: Cents) -> [Cents; PROFITABILITY_BOUNDARY_COUNT] {
    let s = spot.as_u128();
    // Divisors in ascending boundary order (ascending price):
    // profit_over_1000: price < spot/11          → boundary at spot*100/1100 = spot/11
    // profit_500_to_1000: spot/11 ≤ p < spot/6   → boundary at spot*100/600  = spot/6
    // profit_300_to_500: spot/6 ≤ p < spot/4     → boundary at spot*100/400  = spot/4
    // profit_200_to_300: spot/4 ≤ p < spot/3     → boundary at spot*100/300  = spot/3
    // profit_100_to_200: spot/3 ≤ p < spot/2     → boundary at spot*100/200  = spot/2
    // profit_90_to_100: spot/2 ≤ p < spot*100/190 → boundary at spot*100/190
    // profit_80_to_90:                            → boundary at spot*100/180
    // profit_70_to_80:                            → boundary at spot*100/170
    // profit_60_to_70:                            → boundary at spot*100/160
    // profit_50_to_60:                            → boundary at spot*100/150
    // profit_40_to_50:                            → boundary at spot*100/140
    // profit_30_to_40:                            → boundary at spot*100/130
    // profit_20_to_30:                            → boundary at spot*100/120
    // profit_10_to_20:                            → boundary at spot*100/110
    // profit_0_to_10:                             → boundary at spot (= spot*100/100)
    // loss_0_to_10: spot ≤ p < spot*100/90        → boundary at spot*100/90
    // loss_10_to_20:                              → boundary at spot*100/80
    // loss_20_to_30:                              → boundary at spot*100/70
    // loss_30_to_40:                              → boundary at spot*100/60
    // loss_40_to_50:                              → boundary at spot*100/50 = spot*2
    // loss_50_to_60:                              → boundary at spot*100/40 = spot*5/2
    // loss_60_to_70:                              → boundary at spot*100/30 = spot*10/3
    // loss_70_to_80:                              → boundary at spot*100/20 = spot*5
    // loss_80_to_90:                              → boundary at spot*100/10 = spot*10
    // loss_90_to_100: spot*10 ≤ p                 (no upper boundary)
    let divisors: [u128; PROFITABILITY_BOUNDARY_COUNT] = [
        1100, // >1000% profit upper bound (spot/11)
        600,  // 500-1000% profit upper bound (spot/6)
        400,  // 300-500% profit upper bound (spot/4)
        300,  // 200-300% profit upper bound (spot/3)
        200,  // 100-200% profit upper bound (spot/2)
        190,  // 90-100% profit upper bound
        180,  // 80-90% profit upper bound
        170,  // 70-80% profit upper bound
        160,  // 60-70% profit upper bound
        150,  // 50-60% profit upper bound
        140,  // 40-50% profit upper bound
        130,  // 30-40% profit upper bound
        120,  // 20-30% profit upper bound
        110,  // 10-20% profit upper bound
        100,  // 0-10% profit upper bound (= spot)
        90,   // 0-10% loss upper bound
        80,   // 10-20% loss upper bound
        70,   // 20-30% loss upper bound
        60,   // 30-40% loss upper bound
        50,   // 40-50% loss upper bound
        40,   // 50-60% loss upper bound
        30,   // 60-70% loss upper bound
        20,   // 70-80% loss upper bound
        10,   // 80-90% loss upper bound
    ];

    let mut boundaries = [Cents::ZERO; PROFITABILITY_BOUNDARY_COUNT];
    for (i, &d) in divisors.iter().enumerate() {
        boundaries[i] = Cents::from(s * 100 / d);
    }
    boundaries
}

/// Profitability range names (25 ranges, from most profitable to most in loss)
pub const PROFITABILITY_RANGE_NAMES: ByProfitabilityRange<CohortName> = ByProfitabilityRange {
    profit_over_1000: CohortName::new("profit_over_1000pct", ">1000%", "Over 1000% Profit"),
    profit_500_to_1000: CohortName::new("profit_500_to_1000pct", "500-1000%", "500-1000% Profit"),
    profit_300_to_500: CohortName::new("profit_300_to_500pct", "300-500%", "300-500% Profit"),
    profit_200_to_300: CohortName::new("profit_200_to_300pct", "200-300%", "200-300% Profit"),
    profit_100_to_200: CohortName::new("profit_100_to_200pct", "100-200%", "100-200% Profit"),
    profit_90_to_100: CohortName::new("profit_90_to_100pct", "90-100%", "90-100% Profit"),
    profit_80_to_90: CohortName::new("profit_80_to_90pct", "80-90%", "80-90% Profit"),
    profit_70_to_80: CohortName::new("profit_70_to_80pct", "70-80%", "70-80% Profit"),
    profit_60_to_70: CohortName::new("profit_60_to_70pct", "60-70%", "60-70% Profit"),
    profit_50_to_60: CohortName::new("profit_50_to_60pct", "50-60%", "50-60% Profit"),
    profit_40_to_50: CohortName::new("profit_40_to_50pct", "40-50%", "40-50% Profit"),
    profit_30_to_40: CohortName::new("profit_30_to_40pct", "30-40%", "30-40% Profit"),
    profit_20_to_30: CohortName::new("profit_20_to_30pct", "20-30%", "20-30% Profit"),
    profit_10_to_20: CohortName::new("profit_10_to_20pct", "10-20%", "10-20% Profit"),
    profit_0_to_10: CohortName::new("profit_0_to_10pct", "0-10%", "0-10% Profit"),
    loss_0_to_10: CohortName::new("loss_0_to_10pct", "0-10%L", "0-10% Loss"),
    loss_10_to_20: CohortName::new("loss_10_to_20pct", "10-20%L", "10-20% Loss"),
    loss_20_to_30: CohortName::new("loss_20_to_30pct", "20-30%L", "20-30% Loss"),
    loss_30_to_40: CohortName::new("loss_30_to_40pct", "30-40%L", "30-40% Loss"),
    loss_40_to_50: CohortName::new("loss_40_to_50pct", "40-50%L", "40-50% Loss"),
    loss_50_to_60: CohortName::new("loss_50_to_60pct", "50-60%L", "50-60% Loss"),
    loss_60_to_70: CohortName::new("loss_60_to_70pct", "60-70%L", "60-70% Loss"),
    loss_70_to_80: CohortName::new("loss_70_to_80pct", "70-80%L", "70-80% Loss"),
    loss_80_to_90: CohortName::new("loss_80_to_90pct", "80-90%L", "80-90% Loss"),
    loss_90_to_100: CohortName::new("loss_90_to_100pct", "90-100%L", "90-100% Loss"),
};

impl ByProfitabilityRange<CohortName> {
    pub const fn names() -> &'static Self {
        &PROFITABILITY_RANGE_NAMES
    }
}

/// 25 profitability range buckets ordered from most profitable to most in loss.
///
/// During the k-way merge (ascending price order), the cursor starts at bucket 0
/// (profit_over_1000, lowest cost basis) and advances as price crosses each boundary.
#[derive(Default, Clone, Traversable, Serialize)]
pub struct ByProfitabilityRange<T> {
    pub profit_over_1000: T,
    pub profit_500_to_1000: T,
    pub profit_300_to_500: T,
    pub profit_200_to_300: T,
    pub profit_100_to_200: T,
    pub profit_90_to_100: T,
    pub profit_80_to_90: T,
    pub profit_70_to_80: T,
    pub profit_60_to_70: T,
    pub profit_50_to_60: T,
    pub profit_40_to_50: T,
    pub profit_30_to_40: T,
    pub profit_20_to_30: T,
    pub profit_10_to_20: T,
    pub profit_0_to_10: T,
    pub loss_0_to_10: T,
    pub loss_10_to_20: T,
    pub loss_20_to_30: T,
    pub loss_30_to_40: T,
    pub loss_40_to_50: T,
    pub loss_50_to_60: T,
    pub loss_60_to_70: T,
    pub loss_70_to_80: T,
    pub loss_80_to_90: T,
    pub loss_90_to_100: T,
}

/// Number of profitability range buckets.
pub const PROFITABILITY_RANGE_COUNT: usize = 25;

impl<T> ByProfitabilityRange<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(&'static str) -> T,
    {
        let n = &PROFITABILITY_RANGE_NAMES;
        Self {
            profit_over_1000: create(n.profit_over_1000.id),
            profit_500_to_1000: create(n.profit_500_to_1000.id),
            profit_300_to_500: create(n.profit_300_to_500.id),
            profit_200_to_300: create(n.profit_200_to_300.id),
            profit_100_to_200: create(n.profit_100_to_200.id),
            profit_90_to_100: create(n.profit_90_to_100.id),
            profit_80_to_90: create(n.profit_80_to_90.id),
            profit_70_to_80: create(n.profit_70_to_80.id),
            profit_60_to_70: create(n.profit_60_to_70.id),
            profit_50_to_60: create(n.profit_50_to_60.id),
            profit_40_to_50: create(n.profit_40_to_50.id),
            profit_30_to_40: create(n.profit_30_to_40.id),
            profit_20_to_30: create(n.profit_20_to_30.id),
            profit_10_to_20: create(n.profit_10_to_20.id),
            profit_0_to_10: create(n.profit_0_to_10.id),
            loss_0_to_10: create(n.loss_0_to_10.id),
            loss_10_to_20: create(n.loss_10_to_20.id),
            loss_20_to_30: create(n.loss_20_to_30.id),
            loss_30_to_40: create(n.loss_30_to_40.id),
            loss_40_to_50: create(n.loss_40_to_50.id),
            loss_50_to_60: create(n.loss_50_to_60.id),
            loss_60_to_70: create(n.loss_60_to_70.id),
            loss_70_to_80: create(n.loss_70_to_80.id),
            loss_80_to_90: create(n.loss_80_to_90.id),
            loss_90_to_100: create(n.loss_90_to_100.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str) -> Result<T, E>,
    {
        let n = &PROFITABILITY_RANGE_NAMES;
        Ok(Self {
            profit_over_1000: create(n.profit_over_1000.id)?,
            profit_500_to_1000: create(n.profit_500_to_1000.id)?,
            profit_300_to_500: create(n.profit_300_to_500.id)?,
            profit_200_to_300: create(n.profit_200_to_300.id)?,
            profit_100_to_200: create(n.profit_100_to_200.id)?,
            profit_90_to_100: create(n.profit_90_to_100.id)?,
            profit_80_to_90: create(n.profit_80_to_90.id)?,
            profit_70_to_80: create(n.profit_70_to_80.id)?,
            profit_60_to_70: create(n.profit_60_to_70.id)?,
            profit_50_to_60: create(n.profit_50_to_60.id)?,
            profit_40_to_50: create(n.profit_40_to_50.id)?,
            profit_30_to_40: create(n.profit_30_to_40.id)?,
            profit_20_to_30: create(n.profit_20_to_30.id)?,
            profit_10_to_20: create(n.profit_10_to_20.id)?,
            profit_0_to_10: create(n.profit_0_to_10.id)?,
            loss_0_to_10: create(n.loss_0_to_10.id)?,
            loss_10_to_20: create(n.loss_10_to_20.id)?,
            loss_20_to_30: create(n.loss_20_to_30.id)?,
            loss_30_to_40: create(n.loss_30_to_40.id)?,
            loss_40_to_50: create(n.loss_40_to_50.id)?,
            loss_50_to_60: create(n.loss_50_to_60.id)?,
            loss_60_to_70: create(n.loss_60_to_70.id)?,
            loss_70_to_80: create(n.loss_70_to_80.id)?,
            loss_80_to_90: create(n.loss_80_to_90.id)?,
            loss_90_to_100: create(n.loss_90_to_100.id)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self.profit_over_1000,
            &self.profit_500_to_1000,
            &self.profit_300_to_500,
            &self.profit_200_to_300,
            &self.profit_100_to_200,
            &self.profit_90_to_100,
            &self.profit_80_to_90,
            &self.profit_70_to_80,
            &self.profit_60_to_70,
            &self.profit_50_to_60,
            &self.profit_40_to_50,
            &self.profit_30_to_40,
            &self.profit_20_to_30,
            &self.profit_10_to_20,
            &self.profit_0_to_10,
            &self.loss_0_to_10,
            &self.loss_10_to_20,
            &self.loss_20_to_30,
            &self.loss_30_to_40,
            &self.loss_40_to_50,
            &self.loss_50_to_60,
            &self.loss_60_to_70,
            &self.loss_70_to_80,
            &self.loss_80_to_90,
            &self.loss_90_to_100,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self.profit_over_1000,
            &mut self.profit_500_to_1000,
            &mut self.profit_300_to_500,
            &mut self.profit_200_to_300,
            &mut self.profit_100_to_200,
            &mut self.profit_90_to_100,
            &mut self.profit_80_to_90,
            &mut self.profit_70_to_80,
            &mut self.profit_60_to_70,
            &mut self.profit_50_to_60,
            &mut self.profit_40_to_50,
            &mut self.profit_30_to_40,
            &mut self.profit_20_to_30,
            &mut self.profit_10_to_20,
            &mut self.profit_0_to_10,
            &mut self.loss_0_to_10,
            &mut self.loss_10_to_20,
            &mut self.loss_20_to_30,
            &mut self.loss_30_to_40,
            &mut self.loss_40_to_50,
            &mut self.loss_50_to_60,
            &mut self.loss_60_to_70,
            &mut self.loss_70_to_80,
            &mut self.loss_80_to_90,
            &mut self.loss_90_to_100,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self.profit_over_1000,
            &mut self.profit_500_to_1000,
            &mut self.profit_300_to_500,
            &mut self.profit_200_to_300,
            &mut self.profit_100_to_200,
            &mut self.profit_90_to_100,
            &mut self.profit_80_to_90,
            &mut self.profit_70_to_80,
            &mut self.profit_60_to_70,
            &mut self.profit_50_to_60,
            &mut self.profit_40_to_50,
            &mut self.profit_30_to_40,
            &mut self.profit_20_to_30,
            &mut self.profit_10_to_20,
            &mut self.profit_0_to_10,
            &mut self.loss_0_to_10,
            &mut self.loss_10_to_20,
            &mut self.loss_20_to_30,
            &mut self.loss_30_to_40,
            &mut self.loss_40_to_50,
            &mut self.loss_50_to_60,
            &mut self.loss_60_to_70,
            &mut self.loss_70_to_80,
            &mut self.loss_80_to_90,
            &mut self.loss_90_to_100,
        ]
        .into_par_iter()
    }

    /// Access as a fixed-size array of references (for indexed access during merge).
    pub fn as_array(&self) -> [&T; PROFITABILITY_RANGE_COUNT] {
        [
            &self.profit_over_1000,
            &self.profit_500_to_1000,
            &self.profit_300_to_500,
            &self.profit_200_to_300,
            &self.profit_100_to_200,
            &self.profit_90_to_100,
            &self.profit_80_to_90,
            &self.profit_70_to_80,
            &self.profit_60_to_70,
            &self.profit_50_to_60,
            &self.profit_40_to_50,
            &self.profit_30_to_40,
            &self.profit_20_to_30,
            &self.profit_10_to_20,
            &self.profit_0_to_10,
            &self.loss_0_to_10,
            &self.loss_10_to_20,
            &self.loss_20_to_30,
            &self.loss_30_to_40,
            &self.loss_40_to_50,
            &self.loss_50_to_60,
            &self.loss_60_to_70,
            &self.loss_70_to_80,
            &self.loss_80_to_90,
            &self.loss_90_to_100,
        ]
    }

    /// Access as a fixed-size array of mutable references (for indexed access during merge).
    pub fn as_array_mut(&mut self) -> [&mut T; PROFITABILITY_RANGE_COUNT] {
        [
            &mut self.profit_over_1000,
            &mut self.profit_500_to_1000,
            &mut self.profit_300_to_500,
            &mut self.profit_200_to_300,
            &mut self.profit_100_to_200,
            &mut self.profit_90_to_100,
            &mut self.profit_80_to_90,
            &mut self.profit_70_to_80,
            &mut self.profit_60_to_70,
            &mut self.profit_50_to_60,
            &mut self.profit_40_to_50,
            &mut self.profit_30_to_40,
            &mut self.profit_20_to_30,
            &mut self.profit_10_to_20,
            &mut self.profit_0_to_10,
            &mut self.loss_0_to_10,
            &mut self.loss_10_to_20,
            &mut self.loss_20_to_30,
            &mut self.loss_30_to_40,
            &mut self.loss_40_to_50,
            &mut self.loss_50_to_60,
            &mut self.loss_60_to_70,
            &mut self.loss_70_to_80,
            &mut self.loss_80_to_90,
            &mut self.loss_90_to_100,
        ]
    }
}
