use bitview_cohort::{AgeRange, AgeRangeId};
use bitview_traversable::Traversable;
use brk_types::{BoundedRatio, Height, StoredF64};
use derive_more::{Deref, DerefMut};
use vecdb::{Budgeted, CachedColumnarVec, PcoVec, ReadOnlyColumnarVec};

use bitview_compute::{LazyColumnPerBlock, LazyPerBlock};

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct SpendingExposureSeries {
    #[traversable(skip)]
    pub cached_mobility: CachedColumnarVec<
        ReadOnlyColumnarVec<PcoVec<Height, BoundedRatio>, AgeRangeId>,
        AgeRangeId,
        Budgeted,
    >,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub age_range: AgeRange<LazyColumnPerBlock<StoredF64, AgeRangeId>>,
    /// Estimated probability that supply in a UTXO age range will ever be
    /// spent: one minus exp of negative spending exposure. Nonpositive or NaN
    /// exposure returns zero; positive results are capped just below one. A
    /// value near zero identifies supply unlikely to move, while a value near
    /// one identifies supply likely to move eventually. Floored at bounded
    /// scale 4,294,967,294 and shared with weighted supply/cap calculations.
    pub mobility: AgeRange<LazyPerBlock<StoredF64, BoundedRatio>>,
}
