use crate::{ColumnarPerBlockCumulativeRolling, FiatType, LazyFiatPerBlockCumulativeWithSums};
use bitview_cohort::{ByTerm, TermId, UTXOAggregate};
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyVec, CacheBudget, CachedReadableVec, Database, ReadableCloneableVec, ReadableColumnarVec,
    Rw, StorageMode,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct AdditiveAggregateFiatPerBlockCumulativeWithSums<C: FiatType, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub values: ColumnarPerBlockCumulativeRolling<
        C,
        TermId,
        UTXOAggregate<LazyFiatPerBlockCumulativeWithSums<C>>,
        M,
    >,
}

impl<C: FiatType> AdditiveAggregateFiatPerBlockCumulativeWithSums<C> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &crate::IndexSources,
        cached_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let values = ColumnarPerBlockCumulativeRolling::forced_import(
            db,
            &format!("{metric}_cumulative_cents_by_term"),
            version,
            |source| {
                UTXOAggregate::from_fn(|id| {
                    let name = id.metric_name(metric);
                    let cumulative = match id.term() {
                        Some(term) => cache
                            .wrap(source.column(&format!("{name}_cumulative_cents"), version, term))
                            .cached_boxed_clone(),
                        None => cache
                            .wrap(source.sum_columns(
                                &format!("{name}_cumulative_cents"),
                                version,
                                TermId::ALL.iter().copied(),
                            ))
                            .cached_boxed_clone(),
                    };
                    LazyFiatPerBlockCumulativeWithSums::from_cumulative_cents_source(
                        &name,
                        version,
                        &cumulative,
                        indexes,
                        cached_starts,
                    )
                })
            },
        )?;
        Ok(Self { values })
    }

    #[inline(always)]
    pub fn push_block(&mut self, row: UTXOAggregate<C>) {
        self.values.push_block(ByTerm {
            short: row.sth,
            long: row.lth,
        });
    }

    pub fn len(&self) -> usize {
        self.values.cumulative.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.cumulative.is_empty()
    }
}
