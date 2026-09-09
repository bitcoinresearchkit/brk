use bitview_cohort::{
    Filter, OVER_AGE_FILTERS, OVER_AMOUNT_FILTERS, OverAgeId, OverAmountId, UNDER_AGE_FILTERS,
    UNDER_AMOUNT_FILTERS, UTXO_AGGREGATE_FILTERS, UTXOAggregateId, UTXOAggregateRows, UnderAgeId,
    UnderAmountId,
};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, PcoVecValue, ReadableBoxedVec, Rw, StorageMode};

use crate::ColumnarPerBlock;

/// Stored all/STH/LTH and age/amount threshold results for non-additive metrics.
#[derive(Traversable)]
pub struct UTXOOverlappingColumns<T: PcoVecValue, M: StorageMode = Rw> {
    pub aggregate: ColumnarPerBlock<T, UTXOAggregateId, (), M>,
    pub under_age: ColumnarPerBlock<T, UnderAgeId, (), M>,
    pub over_age: ColumnarPerBlock<T, OverAgeId, (), M>,
    pub under_amount: ColumnarPerBlock<T, UnderAmountId, (), M>,
    pub over_amount: ColumnarPerBlock<T, OverAmountId, (), M>,
}

impl<T: PcoVecValue> UTXOOverlappingColumns<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        let version = version + Version::ONE;
        Ok(Self {
            aggregate: ColumnarPerBlock::forced_import(
                cache,
                db,
                &format!("{name}_by_aggregate"),
                version,
                |_| (),
            )?,
            under_age: ColumnarPerBlock::forced_import(
                cache,
                db,
                &format!("utxos_{name}_by_under_age"),
                version,
                |_| (),
            )?,
            over_age: ColumnarPerBlock::forced_import(
                cache,
                db,
                &format!("utxos_{name}_by_over_age"),
                version,
                |_| (),
            )?,
            under_amount: ColumnarPerBlock::forced_import(
                cache,
                db,
                &format!("utxos_{name}_by_under_amount"),
                version,
                |_| (),
            )?,
            over_amount: ColumnarPerBlock::forced_import(
                cache,
                db,
                &format!("utxos_{name}_by_over_amount"),
                version,
                |_| (),
            )?,
        })
    }

    pub fn source(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<ReadableBoxedVec<Height, T>> {
        match filter {
            Filter::All | Filter::Term(_) => UTXOAggregateId::ALL
                .iter()
                .copied()
                .find(|id| id.select(&UTXO_AGGREGATE_FILTERS) == filter)
                .map(|id| self.aggregate.column_source(name, version, id)),
            Filter::Time(_) => UnderAgeId::ALL
                .iter()
                .copied()
                .find(|id| id.select(&UNDER_AGE_FILTERS) == filter)
                .map(|id| self.under_age.column_source(name, version, id))
                .or_else(|| {
                    OverAgeId::ALL
                        .iter()
                        .copied()
                        .find(|id| id.select(&OVER_AGE_FILTERS) == filter)
                        .map(|id| self.over_age.column_source(name, version, id))
                }),
            Filter::Amount(_) => UnderAmountId::ALL
                .iter()
                .copied()
                .find(|id| id.select(&UNDER_AMOUNT_FILTERS) == filter)
                .map(|id| self.under_amount.column_source(name, version, id))
                .or_else(|| {
                    OverAmountId::ALL
                        .iter()
                        .copied()
                        .find(|id| id.select(&OVER_AMOUNT_FILTERS) == filter)
                        .map(|id| self.over_amount.column_source(name, version, id))
                }),
            Filter::Epoch(_) | Filter::Class(_) | Filter::Entry(_) | Filter::Type(_) => None,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, rows: UTXOAggregateRows<T>) {
        self.aggregate.push(rows.aggregate);
        self.under_age.push(rows.under_age);
        self.over_age.push(rows.over_age);
        self.under_amount.push(rows.under_amount);
        self.over_amount.push(rows.over_amount);
    }

    pub fn min_len(&self) -> usize {
        self.aggregate
            .len()
            .min(self.under_age.len())
            .min(self.over_age.len())
            .min(self.under_amount.len())
            .min(self.over_amount.len())
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            self.aggregate.stored_mut(),
            self.under_age.stored_mut(),
            self.over_age.stored_mut(),
            self.under_amount.stored_mut(),
            self.over_amount.stored_mut(),
        ]
    }
}
