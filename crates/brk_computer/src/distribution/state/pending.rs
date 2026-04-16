use brk_types::{CentsSats, CentsSquaredSats, Sats};

#[derive(Clone, Debug, Default)]
pub(crate) struct PendingCapDelta {
    pub inc: CentsSats,
    pub dec: CentsSats,
}

impl PendingCapDelta {
    pub fn is_zero(&self) -> bool {
        self.inc == CentsSats::ZERO && self.dec == CentsSats::ZERO
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PendingCapitalizedCapRawDelta {
    pub inc: CentsSquaredSats,
    pub dec: CentsSquaredSats,
}

/// Pending increments and decrements for a single price bucket.
#[derive(Clone, Copy, Debug, Default)]
pub struct PendingDelta {
    pub inc: Sats,
    pub dec: Sats,
}
