use crate::{ColumnarPerBlock, FiatType, LazyFiatPerBlock};
use bitview_cohort::{ByTerm, TermId, UTXOAggregate};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, CachedReadableVec, Database, ReadableColumnarVec, Rw, StorageMode};

#[derive(Deref, DerefMut, Traversable)]
pub struct AdditiveAggregateFiatPerBlock<C: FiatType, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub values: ColumnarPerBlock<C, TermId, UTXOAggregate<LazyFiatPerBlock<C>>, M>,
}

impl<C: FiatType> AdditiveAggregateFiatPerBlock<C> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &crate::IndexSources,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            db,
            &format!("{metric}_cents_by_term"),
            version,
            |source| {
                UTXOAggregate::from_fn(|aggregate| {
                    let name = aggregate.metric_name(metric);
                    let cents = match aggregate.term() {
                        Some(term) => cache
                            .wrap(source.column(&format!("{name}_cents"), version, term))
                            .cached_boxed_clone(),
                        None => cache
                            .wrap(source.sum_columns(
                                &format!("{name}_cents"),
                                version,
                                TermId::ALL.iter().copied(),
                            ))
                            .cached_boxed_clone(),
                    };
                    LazyFiatPerBlock::from_cents_source(&name, version, &cents, indexes)
                })
            },
        )?;
        Ok(Self { values })
    }

    #[inline(always)]
    pub fn push(&mut self, row: UTXOAggregate<C>) {
        self.values.push(ByTerm {
            short: row.sth,
            long: row.lth,
        });
    }
}
