use crate::ColumnarValuePerBlockCumulativeRolling;
use bitview_cohort::{AgeRangeId, ClassId, EntryId, EpochId, Filter, UTXOCoreRows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Height, Sats, StoredU64, Version};
use vecdb::{AnyStoredVec, CacheBudget, ColumnId, Database, LazyVec, Rw, StorageMode};

#[derive(Traversable)]
pub struct CumulativeUTXOCoreValueColumns<M: StorageMode = Rw> {
    /// Height-indexed columns with one column per exact UTXO age range,
    /// ordered from youngest to oldest.
    pub age_range: ColumnarValuePerBlockCumulativeRolling<AgeRangeId, (), M>,
    /// Height-indexed columns with one column per subsidy-halving epoch,
    /// ordered from earliest to latest.
    pub epoch: ColumnarValuePerBlockCumulativeRolling<EpochId, (), M>,
    /// Height-indexed columns with one column per output-creation year, in
    /// chronological order.
    pub class: ColumnarValuePerBlockCumulativeRolling<ClassId, (), M>,
    /// Height-indexed columns with discount-entry followed by premium-entry
    /// UTXOs.
    pub entry: ColumnarValuePerBlockCumulativeRolling<EntryId, (), M>,
}

impl CumulativeUTXOCoreValueColumns {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        let version = version + Version::ONE;
        Ok(Self {
            age_range: Self::import(cache, db, &format!("utxos_{name}_by_age_range"), version)?,
            epoch: Self::import(cache, db, &format!("{name}_by_epoch"), version)?,
            class: Self::import(cache, db, &format!("{name}_by_class"), version)?,
            entry: Self::import(cache, db, &format!("{name}_by_entry"), version)?,
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
            .or_else(|| self.aggregate_sources(filter, name, version))
    }

    pub fn direct_sources(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<(
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )> {
        match filter {
            Filter::Time(_) => AgeRangeId::matching(filter)
                .map(|column| Self::column_sources(&self.age_range, name, version, [column])),
            Filter::Epoch(_) => EpochId::matching(filter)
                .map(|column| Self::column_sources(&self.epoch, name, version, [column])),
            Filter::Class(_) => ClassId::matching(filter)
                .map(|column| Self::column_sources(&self.class, name, version, [column])),
            Filter::Entry(_) => EntryId::matching(filter)
                .map(|column| Self::column_sources(&self.entry, name, version, [column])),
            Filter::All | Filter::Term(_) | Filter::Amount(_) | Filter::Type(_) => None,
        }
    }

    pub fn aggregate_sources(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<(
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )> {
        let columns = AgeRangeId::aggregate_columns(filter)?;
        Some(Self::column_sources(
            &self.age_range,
            name,
            version,
            columns,
        ))
    }

    pub fn column_sources<C>(
        source: &ColumnarValuePerBlockCumulativeRolling<C, ()>,
        name: &str,
        version: Version,
        columns: impl IntoIterator<Item = C>,
    ) -> (
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )
    where
        C: ColumnId,
    {
        source.sources(&format!("{name}_cumulative"), version, columns)
    }

    #[inline(always)]
    pub fn push_block(
        &mut self,
        sats: impl Into<UTXOCoreRows<Sats>>,
        cents: impl Into<UTXOCoreRows<Cents>>,
    ) {
        let sats = sats.into();
        let cents = cents.into();
        self.age_range.push_block(sats.age_range, cents.age_range);
        self.epoch.push_block(sats.epoch, cents.epoch);
        self.class.push_block(sats.class, cents.class);
        self.entry.push_block(sats.entry, cents.entry);
    }

    pub fn min_len(&self) -> usize {
        self.age_range
            .len()
            .min(self.epoch.len())
            .min(self.class.len())
            .min(self.entry.len())
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let Self {
            age_range,
            epoch,
            class,
            entry,
        } = self;
        let mut vecs = age_range.collect_vecs_mut();
        vecs.extend(epoch.collect_vecs_mut());
        vecs.extend(class.collect_vecs_mut());
        vecs.extend(entry.collect_vecs_mut());
        vecs
    }
}
