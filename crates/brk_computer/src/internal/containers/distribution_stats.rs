use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct DistributionStats<A> {
    pub average: A,
    pub min: A,
    pub max: A,
    pub pct10: A,
    pub pct25: A,
    pub median: A,
    pub pct75: A,
    pub pct90: A,
}

impl<A> DistributionStats<A> {
    pub const SUFFIXES: [&'static str; 8] = [
        "average", "min", "max", "pct10", "pct25", "median", "pct75", "pct90",
    ];

    pub fn try_from_fn<E>(
        mut f: impl FnMut(&str) -> std::result::Result<A, E>,
    ) -> std::result::Result<Self, E> {
        Ok(Self {
            average: f(Self::SUFFIXES[0])?,
            min: f(Self::SUFFIXES[1])?,
            max: f(Self::SUFFIXES[2])?,
            pct10: f(Self::SUFFIXES[3])?,
            pct25: f(Self::SUFFIXES[4])?,
            median: f(Self::SUFFIXES[5])?,
            pct75: f(Self::SUFFIXES[6])?,
            pct90: f(Self::SUFFIXES[7])?,
        })
    }

    /// Apply a fallible operation to each of the 8 fields.
    pub fn try_for_each_mut(
        &mut self,
        mut f: impl FnMut(&mut A) -> brk_error::Result<()>,
    ) -> brk_error::Result<()> {
        f(&mut self.average)?;
        f(&mut self.min)?;
        f(&mut self.max)?;
        f(&mut self.pct10)?;
        f(&mut self.pct25)?;
        f(&mut self.median)?;
        f(&mut self.pct75)?;
        f(&mut self.pct90)?;
        Ok(())
    }

    /// Get minimum value by applying a function to each field.
    pub fn min_by(&self, mut f: impl FnMut(&A) -> usize) -> usize {
        f(&self.average)
            .min(f(&self.min))
            .min(f(&self.max))
            .min(f(&self.pct10))
            .min(f(&self.pct25))
            .min(f(&self.median))
            .min(f(&self.pct75))
            .min(f(&self.pct90))
    }
}
