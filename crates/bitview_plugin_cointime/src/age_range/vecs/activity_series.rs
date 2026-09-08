use bitview_cohort::{AgeRange, AgeRangeId};
use bitview_traversable::Traversable;
use brk_types::{BoundedRatio, Height, StoredF64};
use vecdb::{Budgeted, CachedColumnarVec, PcoVec, ReadOnlyColumnarVec};

use bitview_vecs::LazyPerBlock;

#[derive(Clone, Traversable)]
pub struct ActivitySeries {
    #[traversable(skip)]
    pub cached: CachedColumnarVec<
        ReadOnlyColumnarVec<PcoVec<Height, BoundedRatio>, AgeRangeId>,
        AgeRangeId,
        Budgeted,
    >,
    pub wakefulness: AgeRange<LazyPerBlock<StoredF64, BoundedRatio>>,
    /// Dormancy for an exact UTXO age range: one minus wakefulness. Higher
    /// values mean more of the range's accumulated holding time remains stored
    /// rather than consumed by spending. Uses the exact encoded complement.
    pub dormancy: AgeRange<LazyPerBlock<StoredF64, BoundedRatio>>,
    /// Ratio of consumed to still-stored holding time in an exact UTXO age
    /// range: wakefulness divided by one minus wakefulness. Values above one
    /// mean consumed holding time exceeds stored holding time in the range.
    pub wakefulness_to_dormancy: AgeRange<LazyPerBlock<StoredF64, BoundedRatio>>,
}
