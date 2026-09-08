use bitview_cohort::{
    AgeRange, AgeRangeId, ByEntry, ByEpoch, Class, ClassId, EntryId, EpochId, OverAge, OverAgeId,
    UTXOAggregate, UTXOAggregateId, UTXOGroupsWithoutAmountOrType, UnderAge, UnderAgeId,
};
use bitview_transforms::SoprRatio;
use bitview_traversable::Traversable;
use bitview_vecs::{ColumnarPerBlock, LazyColumnPerBlock};
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, StoredF32, Version};
use vecdb::{
    AnyStoredVec, BinaryTransform, CacheBudget, ColumnId, Database, PcoVec, ReadOnlyColumnarVec,
    Rw, StorageMode,
};

use super::Sopr24hInput;

/// Independently computed 24-hour SOPR columns and their resolution views.
#[derive(Traversable)]
pub struct Sopr24hColumns<M: StorageMode = Rw> {
    pub aggregate: ColumnarPerBlock<
        StoredF32,
        UTXOAggregateId,
        UTXOAggregate<LazyColumnPerBlock<StoredF32, UTXOAggregateId>>,
        M,
    >,
    pub age_range: ColumnarPerBlock<
        StoredF32,
        AgeRangeId,
        AgeRange<LazyColumnPerBlock<StoredF32, AgeRangeId>>,
        M,
    >,
    pub under_age: ColumnarPerBlock<
        StoredF32,
        UnderAgeId,
        UnderAge<LazyColumnPerBlock<StoredF32, UnderAgeId>>,
        M,
    >,
    pub over_age: ColumnarPerBlock<
        StoredF32,
        OverAgeId,
        OverAge<LazyColumnPerBlock<StoredF32, OverAgeId>>,
        M,
    >,
    pub epoch:
        ColumnarPerBlock<StoredF32, EpochId, ByEpoch<LazyColumnPerBlock<StoredF32, EpochId>>, M>,
    pub class:
        ColumnarPerBlock<StoredF32, ClassId, Class<LazyColumnPerBlock<StoredF32, ClassId>>, M>,
    pub entry:
        ColumnarPerBlock<StoredF32, EntryId, ByEntry<LazyColumnPerBlock<StoredF32, EntryId>>, M>,
}

impl Sopr24hColumns {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        let storage_version = version + Version::ONE;
        let aggregate = Self::import_columns(
            db,
            "sopr_24h_by_aggregate",
            storage_version,
            |name, source| UTXOAggregate {
                all: Self::column(
                    cache,
                    name,
                    storage_version,
                    source,
                    UTXOAggregateId::All,
                    mappings,
                ),
                sth: Self::column(
                    cache,
                    name,
                    storage_version,
                    source,
                    UTXOAggregateId::Sth,
                    mappings,
                ),
                lth: Self::column(
                    cache,
                    name,
                    storage_version,
                    source,
                    UTXOAggregateId::Lth,
                    mappings,
                ),
            },
        )?;
        let age_range = Self::import_columns(
            db,
            "utxos_sopr_24h_by_age_range",
            storage_version,
            |name, source| {
                AgeRange::from_fn(|column| {
                    Self::column(cache, name, storage_version, source, column, mappings)
                })
            },
        )?;
        let under_age = Self::import_columns(
            db,
            "utxos_sopr_24h_by_under_age",
            storage_version,
            |name, source| {
                UnderAge::from_fn(|column| {
                    Self::column(cache, name, storage_version, source, column, mappings)
                })
            },
        )?;
        let over_age = Self::import_columns(
            db,
            "utxos_sopr_24h_by_over_age",
            storage_version,
            |name, source| {
                OverAge::from_fn(|column| {
                    Self::column(cache, name, storage_version, source, column, mappings)
                })
            },
        )?;
        let epoch =
            Self::import_columns(db, "sopr_24h_by_epoch", storage_version, |name, source| {
                ByEpoch::from_fn(|column| {
                    Self::column(cache, name, storage_version, source, column, mappings)
                })
            })?;
        let class =
            Self::import_columns(db, "sopr_24h_by_class", storage_version, |name, source| {
                Class::from_fn(|column| {
                    Self::column(cache, name, storage_version, source, column, mappings)
                })
            })?;
        let entry =
            Self::import_columns(db, "sopr_24h_by_entry", storage_version, |name, source| {
                ByEntry::from_fn(|column| {
                    Self::column(cache, name, storage_version, source, column, mappings)
                })
            })?;

        Ok(Self {
            aggregate,
            age_range,
            under_age,
            over_age,
            epoch,
            class,
            entry,
        })
    }

    fn import_columns<C: ColumnId, S: Clone>(
        db: &Database,
        name: &str,
        version: Version,
        build_series: impl FnOnce(&str, &ReadOnlyColumnarVec<PcoVec<Height, StoredF32>, C>) -> S,
    ) -> Result<ColumnarPerBlock<StoredF32, C, S>> {
        ColumnarPerBlock::forced_import(db, name, version, |source| build_series(name, source))
    }

    fn column<C: ColumnId>(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, StoredF32>, C>,
        column: C,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> LazyColumnPerBlock<StoredF32, C> {
        LazyColumnPerBlock::new(
            cache,
            &format!("{name}_column_{}", column.index()),
            version,
            source,
            column,
            mappings,
        )
    }

    pub fn compute(
        &mut self,
        max_from: Height,
        inputs: &UTXOGroupsWithoutAmountOrType<Sopr24hInput>,
        exit: &Exit,
    ) -> Result<()> {
        Self::compute_columns(
            &mut self.aggregate,
            inputs,
            |column, inputs| match column {
                UTXOAggregateId::All => &inputs.all,
                UTXOAggregateId::Sth => &inputs.term.short,
                UTXOAggregateId::Lth => &inputs.term.long,
            },
            max_from,
            exit,
        )?;
        Self::compute_columns(
            &mut self.age_range,
            &inputs.age.range,
            |column, inputs| column.select(inputs),
            max_from,
            exit,
        )?;
        Self::compute_columns(
            &mut self.under_age,
            &inputs.age.under,
            |column, inputs| column.select(inputs),
            max_from,
            exit,
        )?;
        Self::compute_columns(
            &mut self.over_age,
            &inputs.age.over,
            |column, inputs| column.select(inputs),
            max_from,
            exit,
        )?;
        Self::compute_columns(
            &mut self.epoch,
            &inputs.epoch,
            |column, inputs| column.select(inputs),
            max_from,
            exit,
        )?;
        Self::compute_columns(
            &mut self.class,
            &inputs.class,
            |column, inputs| column.select(inputs),
            max_from,
            exit,
        )?;
        Self::compute_columns(
            &mut self.entry,
            &inputs.entry,
            |column, inputs| column.select(inputs),
            max_from,
            exit,
        )
    }

    fn compute_columns<C: ColumnId, S: Clone, I>(
        target: &mut ColumnarPerBlock<StoredF32, C, S>,
        inputs: &I,
        select: impl for<'a> Fn(C, &'a I) -> &'a Sopr24hInput,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        target.compute_columns2(
            max_from,
            |column| &select(column, inputs).transfer_volume,
            |column| &select(column, inputs).value_destroyed,
            |_, transfer_volume, value_destroyed| {
                SoprRatio::apply(transfer_volume, value_destroyed)
            },
            exit,
        )
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            self.aggregate.stored_mut(),
            self.age_range.stored_mut(),
            self.under_age.stored_mut(),
            self.over_age.stored_mut(),
            self.epoch.stored_mut(),
            self.class.stored_mut(),
            self.entry.stored_mut(),
        ]
    }
}
