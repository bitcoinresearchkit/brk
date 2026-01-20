use brk_traversable::Traversable;
use brk_types::{DateIndex, StoredF64};
use vecdb::{EagerVec, PcoVec};

use crate::internal::ComputedFromDateLast;

/// Reserve Risk metric components.
///
/// Reserve Risk = Price / HODL Bank
/// Where HODL Bank = Σ(Price - avg_VOCDD) over time
///
/// Low Reserve Risk = high long-term holder confidence = good buying opportunity.
#[derive(Clone, Traversable)]
pub struct Vecs {
    /// Moving average of VOCDD (Value-weighted CDD) over 365 days
    /// Used to smooth the VOCDD signal for HODL Bank calculation
    pub vocdd_365d_sma: EagerVec<PcoVec<DateIndex, StoredF64>>,

    /// HODL Bank = cumulative sum of (price - vocdd_365d_sma)
    /// Represents the opportunity cost of holding Bitcoin vs trading
    pub hodl_bank: EagerVec<PcoVec<DateIndex, StoredF64>>,

    /// Reserve Risk = price / hodl_bank
    /// A timing indicator for long-term Bitcoin accumulation
    pub reserve_risk: Option<ComputedFromDateLast<StoredF64>>,
}
