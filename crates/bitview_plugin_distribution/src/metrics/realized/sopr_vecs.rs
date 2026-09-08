use bitview_cohort::{
    AGE_RANGE_FILTERS, AgeRangeId, CLASS_FILTERS, ClassId, CohortContext, ENTRY_FILTERS,
    EPOCH_FILTERS, EntryId, EpochId, Filter, OVER_AGE_FILTERS, OverAgeId, Term, UNDER_AGE_FILTERS,
    UTXOGroupsWithoutAmountOrType, UnderAgeId,
};
use bitview_traversable::Traversable;
use bitview_vecs::{LazyColumnPerBlock, LazyPerBlock};
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, StoredF32, Version};
use vecdb::{AnyStoredVec, CacheBudget, ColumnId, Database, Ident, Rw, StorageMode};

use super::{Sopr24hColumns, Sopr24hInput};

const VERSION: Version = Version::ONE;

#[derive(Traversable)]
pub struct Sopr24hVecs<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<LazyPerBlock<StoredF32>>,
    pub stored: Sopr24hColumns<M>,
}

impl Sopr24hVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        let version = version + VERSION;
        let stored = Sopr24hColumns::forced_import(cache, db, version, mappings)?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "sopr_24h");
            let version = Self::cohort_version(version, &filter);
            match &filter {
                Filter::All => Self::logical_source(&stored.aggregate.series.all, &name, version),
                Filter::Term(Term::Sth) => {
                    Self::logical_source(&stored.aggregate.series.sth, &name, version)
                }
                Filter::Term(Term::Lth) => {
                    Self::logical_source(&stored.aggregate.series.lth, &name, version)
                }
                Filter::Time(_) => AgeRangeId::ALL
                    .iter()
                    .copied()
                    .find(|id| id.select(&AGE_RANGE_FILTERS) == &filter)
                    .map(|id| {
                        Self::logical_source(id.select(&stored.age_range.series), &name, version)
                    })
                    .or_else(|| {
                        UnderAgeId::ALL
                            .iter()
                            .copied()
                            .find(|id| id.select(&UNDER_AGE_FILTERS) == &filter)
                            .map(|id| {
                                Self::logical_source(
                                    id.select(&stored.under_age.series),
                                    &name,
                                    version,
                                )
                            })
                    })
                    .or_else(|| {
                        OverAgeId::ALL
                            .iter()
                            .copied()
                            .find(|id| id.select(&OVER_AGE_FILTERS) == &filter)
                            .map(|id| {
                                Self::logical_source(
                                    id.select(&stored.over_age.series),
                                    &name,
                                    version,
                                )
                            })
                    })
                    .expect("supported SOPR time cohort"),
                Filter::Epoch(_) => EpochId::ALL
                    .iter()
                    .copied()
                    .find(|id| id.select(&EPOCH_FILTERS) == &filter)
                    .map(|id| Self::logical_source(id.select(&stored.epoch.series), &name, version))
                    .expect("supported SOPR epoch cohort"),
                Filter::Class(_) => ClassId::ALL
                    .iter()
                    .copied()
                    .find(|id| id.select(&CLASS_FILTERS) == &filter)
                    .map(|id| Self::logical_source(id.select(&stored.class.series), &name, version))
                    .expect("supported SOPR class cohort"),
                Filter::Entry(_) => EntryId::ALL
                    .iter()
                    .copied()
                    .find(|id| id.select(&ENTRY_FILTERS) == &filter)
                    .map(|id| Self::logical_source(id.select(&stored.entry.series), &name, version))
                    .expect("supported SOPR entry cohort"),
                Filter::Amount(_) | Filter::Type(_) => unreachable!("unsupported SOPR cohort"),
            }
        });

        Ok(Self { cohorts, stored })
    }

    fn logical_source<C: ColumnId>(
        source: &LazyColumnPerBlock<StoredF32, C>,
        name: &str,
        version: Version,
    ) -> LazyPerBlock<StoredF32> {
        LazyPerBlock::from_resolutions::<Ident>(name, version, &source.resolutions)
    }

    fn cohort_version(base: Version, filter: &Filter) -> Version {
        base + Version::ONE
            + if matches!(filter, Filter::All) {
                Version::ONE
            } else {
                Version::ZERO
            }
    }

    pub fn compute(
        &mut self,
        max_from: Height,
        inputs: &UTXOGroupsWithoutAmountOrType<Sopr24hInput>,
        exit: &Exit,
    ) -> Result<()> {
        self.stored.compute(max_from, inputs, exit)
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.stored.collect_vecs_mut()
    }
}
