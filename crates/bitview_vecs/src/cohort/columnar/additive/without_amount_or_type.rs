use bitview_cohort::UTXOCoreRows;
use std::ops::AddAssign;

use bitview_cohort::{AgeRangeId, ClassId, EntryId, EpochId, Filter};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, CachedBoxedVec, CachedReadableVec, ColumnId, ColumnarVec,
    Database, EagerVec, ImportableVec, PcoVec, PcoVecValue, ReadOnlyClone, ReadOnlyColumnarVec,
    ReadableColumnarVec, ReadableVec, Rw, StorageMode, WritableVec,
};

#[derive(Traversable)]
pub struct UTXOColumnarMetricWithoutAmountOrType<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    /// Height-indexed matrix with one column per exact UTXO age range, ordered
    /// from youngest to oldest.
    pub age_range_matrix: M::Stored<EagerVec<ColumnarVec<PcoVec<Height, T>, AgeRangeId>>>,
    /// Height-indexed matrix with one column per subsidy-halving epoch, ordered
    /// from earliest to latest.
    pub epoch_matrix: M::Stored<EagerVec<ColumnarVec<PcoVec<Height, T>, EpochId>>>,
    /// Height-indexed matrix with one column per output-creation year, in
    /// chronological order.
    pub class_matrix: M::Stored<EagerVec<ColumnarVec<PcoVec<Height, T>, ClassId>>>,
    /// Height-indexed matrix with discount-entry followed by premium-entry
    /// UTXOs.
    pub entry_matrix: M::Stored<EagerVec<ColumnarVec<PcoVec<Height, T>, EntryId>>>,
}

impl<T> UTXOColumnarMetricWithoutAmountOrType<T>
where
    T: PcoVecValue + AddAssign,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let version = version + Version::ONE;
        Ok(Self {
            age_range_matrix: EagerVec::forced_import(
                db,
                &format!("utxos_{name}_by_age_range"),
                version,
            )?,
            epoch_matrix: EagerVec::forced_import(db, &format!("{name}_by_epoch"), version)?,
            class_matrix: EagerVec::forced_import(db, &format!("{name}_by_class"), version)?,
            entry_matrix: EagerVec::forced_import(db, &format!("{name}_by_entry"), version)?,
        })
    }

    pub fn additive_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        self.direct_source(cache, filter, name, version)
            .or_else(|| self.aggregate_source(cache, filter, name, version))
    }

    pub(crate) fn direct_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        match filter {
            Filter::Time(_) => AgeRangeId::matching(filter).map(|id| {
                Self::column(
                    cache,
                    &self.age_range_matrix.read_only_clone(),
                    name,
                    version,
                    id,
                )
            }),
            Filter::Epoch(_) => EpochId::matching(filter).map(|id| {
                Self::column(
                    cache,
                    &self.epoch_matrix.read_only_clone(),
                    name,
                    version,
                    id,
                )
            }),
            Filter::Class(_) => ClassId::matching(filter).map(|id| {
                Self::column(
                    cache,
                    &self.class_matrix.read_only_clone(),
                    name,
                    version,
                    id,
                )
            }),
            Filter::Entry(_) => EntryId::matching(filter).map(|id| {
                Self::column(
                    cache,
                    &self.entry_matrix.read_only_clone(),
                    name,
                    version,
                    id,
                )
            }),
            Filter::All | Filter::Term(_) | Filter::Amount(_) | Filter::Type(_) => None,
        }
    }

    pub(crate) fn aggregate_source(
        &self,
        cache: &'static CacheBudget,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<CachedBoxedVec<Height, T>> {
        let columns = AgeRangeId::aggregate_columns(filter)?;
        Some(Self::sum(
            cache,
            &self.age_range_matrix.read_only_clone(),
            name,
            version,
            columns,
        ))
    }

    pub(crate) fn column<C>(
        cache: &'static CacheBudget,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, C>,
        name: &str,
        version: Version,
        column: C,
    ) -> CachedBoxedVec<Height, T>
    where
        C: ColumnId,
    {
        cache
            .wrap(source.column(name, version, column))
            .cached_boxed_clone()
    }

    pub(crate) fn sum<C>(
        cache: &'static CacheBudget,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, C>,
        name: &str,
        version: Version,
        columns: impl IntoIterator<Item = C>,
    ) -> CachedBoxedVec<Height, T>
    where
        C: ColumnId,
    {
        cache
            .wrap(source.sum_columns(name, version, columns))
            .cached_boxed_clone()
    }

    pub fn min_len(&self) -> usize {
        self.age_range_matrix
            .len()
            .min(self.epoch_matrix.len())
            .min(self.class_matrix.len())
            .min(self.entry_matrix.len())
    }

    #[inline(always)]
    pub fn push(&mut self, rows: impl Into<UTXOCoreRows<T>>) {
        let rows = rows.into();
        self.age_range_matrix.push(rows.age_range);
        self.epoch_matrix.push(rows.epoch);
        self.class_matrix.push(rows.class);
        self.entry_matrix.push(rows.entry);
    }

    pub fn collect_last(&self) -> Option<UTXOCoreRows<T>>
    where
        T: Default,
    {
        Some(UTXOCoreRows {
            age_range: self.age_range_matrix.collect_last()?,
            epoch: self.epoch_matrix.collect_last()?,
            class: self.class_matrix.collect_last()?,
            entry: self.entry_matrix.collect_last()?,
        })
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.age_range_matrix,
            &mut self.epoch_matrix,
            &mut self.class_matrix,
            &mut self.entry_matrix,
        ]
    }
}
