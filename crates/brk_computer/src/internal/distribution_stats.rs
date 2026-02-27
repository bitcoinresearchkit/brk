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

impl<A> DistributionStats<A> {
    /// Apply a fallible operation to each of the 8 fields.
    pub fn try_for_each_mut(&mut self, mut f: impl FnMut(&mut A) -> brk_error::Result<()>) -> brk_error::Result<()> {
        f(&mut self.average)?;
        f(&mut self.min)?;
        f(&mut self.max)?;
        f(&mut self.p10)?;
        f(&mut self.p25)?;
        f(&mut self.median)?;
        f(&mut self.p75)?;
        f(&mut self.p90)?;
        Ok(())
    }

    /// Get minimum value by applying a function to each field.
    pub fn min_by(&self, mut f: impl FnMut(&A) -> usize) -> usize {
        f(&self.average)
            .min(f(&self.min))
            .min(f(&self.max))
            .min(f(&self.p10))
            .min(f(&self.p25))
            .min(f(&self.median))
            .min(f(&self.p75))
            .min(f(&self.p90))
    }

}
