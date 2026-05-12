use crate::NUM_BINS;

/// Per-block oracle histogram: count of eligible outputs per bin. Wraps
/// the raw `[u32; NUM_BINS]` so callers can't pass arbitrary bin-indexed
/// arrays to `Oracle::process_histogram`. Deref to the underlying array
/// gives indexing for read paths.
#[derive(Clone)]
pub struct Histogram([u32; NUM_BINS]);

impl Histogram {
    #[inline]
    pub fn zeros() -> Self {
        Self([0; NUM_BINS])
    }

    #[inline]
    pub fn increment(&mut self, bin: usize) {
        self.0[bin] += 1;
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::zeros()
    }
}

impl std::ops::Deref for Histogram {
    type Target = [u32; NUM_BINS];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Histogram {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
