use bitview_cohort::{ByTerm, TermId, UTXOAggregate};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, ReadableCloneableVec, ReadableColumnarVec, Rw, StorageMode};

use crate::{ColumnarPerBlock, FiatType, IndexSources, LazyFiatPerBlock};

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
        indexes: &IndexSources,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            cache,
            db,
            &format!("{metric}_cents_by_term"),
            version,
            |source| {
                UTXOAggregate::from_fn(|aggregate| {
                    let name = aggregate.metric_name(metric);
                    let cents = match aggregate.term() {
                        Some(term) => source
                            .column(&format!("{name}_cents"), version, term)
                            .read_only_boxed_clone(),
                        None => source
                            .sum_columns(
                                &format!("{name}_cents"),
                                version,
                                TermId::ALL.iter().copied(),
                            )
                            .read_only_boxed_clone(),
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
