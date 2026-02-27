//! Base generic struct with 8 type parameters — one per distribution statistic.
//!
//! Foundation for all distribution-style types (average, min, max, percentiles).

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct DistributionStats<A, B = A, C = A, D = A, E = A, F = A, G = A, H = A> {
    pub average: A,
    pub min: B,
    pub max: C,
    pub p10: D,
    pub p25: E,
    pub median: F,
    pub p75: G,
    pub p90: H,
}
