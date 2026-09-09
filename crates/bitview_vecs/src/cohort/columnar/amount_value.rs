use bitview_cohort::{Amount, AmountRange, AmountRangeId, CohortContext};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Height, Sats, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, LazyVec, Rw, StorageMode};

use crate::ColumnarValuePerBlockCumulativeRolling;

/// Cumulative sats/cents columns with shared amount-cohort views.
#[derive(Deref, DerefMut, Traversable)]
pub struct ColumnarAmountValue<S: Clone, M: StorageMode = Rw>(
    #[traversable(flatten)] pub ColumnarValuePerBlockCumulativeRolling<AmountRangeId, Amount<S>, M>,
);

impl<S: Clone> ColumnarAmountValue<S> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        storage_name: &str,
        context: CohortContext,
        metric: &str,
        version: Version,
        mut build: impl FnMut(
            &str,
            LazyVec<Height, Sats, Height, StoredU64>,
            LazyVec<Height, Cents, Height, StoredU64>,
        ) -> S,
    ) -> Result<Self> {
        Ok(Self(ColumnarValuePerBlockCumulativeRolling::forced_import(
            cache,
            db,
            storage_name,
            version,
            |sats, cents| {
                Amount::new(|filter, cohort_name| {
                    let name = context.metric_name(&filter, cohort_name, metric);
                    let source_name = format!("{name}_cumulative");
                    let (sats, cents) = match AmountRangeId::matching(&filter) {
                        Some(column) => ColumnarValuePerBlockCumulativeRolling::<AmountRangeId, ()>::sources_from(sats, cents, &source_name, version, [column]),
                        None => ColumnarValuePerBlockCumulativeRolling::<AmountRangeId, ()>::sources_from(sats, cents, &source_name, version, AmountRangeId::included_by(&filter)),
                    };
                    build(&name, sats, cents)
                })
            },
        )?))
    }

    #[inline(always)]
    pub fn push_cumulative(&mut self, sats: &AmountRange<Sats>, cents: &AmountRange<Cents>) {
        self.0.push_block(sats.clone(), cents.clone());
    }
}
