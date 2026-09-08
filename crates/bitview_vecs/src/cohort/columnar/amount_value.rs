use crate::ColumnarValuePerBlockCumulativeRolling;
use bitview_cohort::{Amount, AmountRange, AmountRangeId, CohortContext};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Height, Sats, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, CacheBudget, Database, LazyVec, Rw, StorageMode};

#[derive(Deref, DerefMut, Traversable)]
pub struct ColumnarAmountValue<S: Clone, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub series: Amount<S>,
    /// Height-indexed matrix with one column per exact value range, ordered from
    /// smallest to largest.
    pub values: ColumnarValuePerBlockCumulativeRolling<AmountRangeId, (), M>,
}

impl<S: Clone> ColumnarAmountValue<S> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        matrix_name: &str,
        context: CohortContext,
        metric: &str,
        version: Version,
        mut build: impl FnMut(
            &str,
            LazyVec<Height, Sats, Height, StoredU64>,
            LazyVec<Height, Cents, Height, StoredU64>,
        ) -> S,
    ) -> Result<Self> {
        let values = ColumnarValuePerBlockCumulativeRolling::forced_import(
            db,
            matrix_name,
            version,
            |_, _| (),
        )?;

        let series = Amount::new(|filter, cohort_name| {
            let name = context.metric_name(&filter, cohort_name, metric);
            let amounts = AmountRangeId::matching(&filter);
            let (sats, cents) = match amounts {
                Some(amount) => {
                    values.sources(cache, &format!("{name}_cumulative"), version, [amount])
                }
                None => values.sources(
                    cache,
                    &format!("{name}_cumulative"),
                    version,
                    AmountRangeId::included_by(&filter),
                ),
            };
            build(&name, sats, cents)
        });

        Ok(Self { series, values })
    }

    #[inline(always)]
    pub fn push_cumulative(&mut self, sats: &AmountRange<Sats>, cents: &AmountRange<Cents>) {
        self.values.push_block(sats.clone(), cents.clone());
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.values.collect_vecs_mut()
    }
}
