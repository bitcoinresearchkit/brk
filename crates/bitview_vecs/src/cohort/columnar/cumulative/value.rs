use crate::ColumnarValuePerBlockCumulativeRolling;
use bitview_cohort::{
    AmountRangeId, Filter, OVER_AMOUNT_FILTERS, SPENDABLE_TYPE_FILTERS, SpendableTypeId,
    UNDER_AMOUNT_FILTERS, UTXORows,
};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Height, Sats, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, CacheBudget, ColumnId, Database, LazyVec, Rw, StorageMode};

use super::CumulativeUTXOCoreValueColumns;

#[derive(Deref, DerefMut, Traversable)]
pub struct CumulativeUTXOValueColumns<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: CumulativeUTXOCoreValueColumns<M>,
    /// Height-indexed columns with one column per exact UTXO value range,
    /// ordered from smallest to largest.
    pub amount_range: ColumnarValuePerBlockCumulativeRolling<AmountRangeId, (), M>,
    #[traversable(rename = "type")]
    /// Height-indexed columns with one column per spendable BRK output type, in
    /// canonical `SpendableTypeId` order.
    pub type_: ColumnarValuePerBlockCumulativeRolling<SpendableTypeId, (), M>,
}

impl CumulativeUTXOValueColumns {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        let core = CumulativeUTXOCoreValueColumns::forced_import(cache, db, name, version)?;
        let version = version + Version::ONE;
        Ok(Self {
            core,
            amount_range: Self::import(
                cache,
                db,
                &format!("utxos_{name}_by_amount_range"),
                version,
            )?,
            type_: Self::import(cache, db, &format!("{name}_by_type"), version)?,
        })
    }

    fn import<C>(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<ColumnarValuePerBlockCumulativeRolling<C, ()>>
    where
        C: ColumnId,
    {
        ColumnarValuePerBlockCumulativeRolling::forced_import(cache, db, name, version, |_, _| ())
    }

    pub fn sources(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<(
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )> {
        self.direct_sources(filter, name, version)
            .or_else(|| self.amount_aggregate_sources(filter, name, version))
            .or_else(|| self.core.aggregate_sources(filter, name, version))
    }

    fn direct_sources(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<(
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )> {
        match filter {
            Filter::Amount(_) => AmountRangeId::matching(filter).map(|column| {
                CumulativeUTXOCoreValueColumns::column_sources(
                    &self.amount_range,
                    name,
                    version,
                    [column],
                )
            }),
            Filter::Type(_) => SpendableTypeId::ALL
                .iter()
                .copied()
                .find(|column| column.select(&SPENDABLE_TYPE_FILTERS) == filter)
                .map(|column| {
                    CumulativeUTXOCoreValueColumns::column_sources(
                        &self.type_,
                        name,
                        version,
                        [column],
                    )
                }),
            _ => self.core.direct_sources(filter, name, version),
        }
    }

    fn amount_aggregate_sources(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<(
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )> {
        let filter = UNDER_AMOUNT_FILTERS
            .iter()
            .chain(OVER_AMOUNT_FILTERS.iter())
            .find(|candidate| *candidate == filter)?;
        Some(CumulativeUTXOCoreValueColumns::column_sources(
            &self.amount_range,
            name,
            version,
            AmountRangeId::included_by(filter),
        ))
    }

    #[inline(always)]
    pub fn push_block(&mut self, sats: UTXORows<Sats>, cents: UTXORows<Cents>) {
        self.core.push_block(sats.core, cents.core);
        self.amount_range
            .push_block(sats.amount_range, cents.amount_range);
        self.type_.push_block(sats.type_, cents.type_);
    }

    pub fn min_len(&self) -> usize {
        self.core
            .min_len()
            .min(self.amount_range.len())
            .min(self.type_.len())
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let Self {
            core,
            amount_range,
            type_,
        } = self;
        let mut vecs = core.collect_vecs_mut();
        vecs.extend(amount_range.collect_vecs_mut());
        vecs.extend(type_.collect_vecs_mut());
        vecs
    }
}
