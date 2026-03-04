use std::path::Path;

use brk_cohort::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByYear, CohortContext, Filter, Term,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Indexes, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, ReadOnlyClone, ReadableVec, Rw, StorageMode};

use crate::{blocks, distribution::DynCohortVecs, indexes, prices};

use crate::distribution::metrics::{
    AdjustedCohortMetrics, AllCohortMetrics, BasicCohortMetrics, CohortMetricsBase,
    ExtendedAdjustedCohortMetrics, ExtendedCohortMetrics, ImportConfig, SupplyMetrics,
};

use super::vecs::UTXOCohortVecs;

use crate::distribution::state::UTXOCohortState;

const VERSION: Version = Version::new(0);

/// All UTXO cohorts organized by filter type.
///
/// Each group uses a concrete metrics type matching its required features:
/// - age_range: extended realized + extended cost basis
/// - epoch/year/amount/type: basic metrics with relative
/// - all: extended + adjusted (no rel_to_all)
/// - sth: extended + adjusted
/// - lth: extended
/// - max_age: adjusted
/// - min_age: basic
#[derive(Traversable)]
pub struct UTXOCohorts<M: StorageMode = Rw> {
    pub all: UTXOCohortVecs<AllCohortMetrics<M>>,
    pub sth: UTXOCohortVecs<ExtendedAdjustedCohortMetrics<M>>,
    pub lth: UTXOCohortVecs<ExtendedCohortMetrics<M>>,
    pub age_range: ByAgeRange<UTXOCohortVecs<ExtendedCohortMetrics<M>>>,
    pub max_age: ByMaxAge<UTXOCohortVecs<AdjustedCohortMetrics<M>>>,
    pub min_age: ByMinAge<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub ge_amount: ByGreatEqualAmount<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub amount_range: ByAmountRange<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub lt_amount: ByLowerThanAmount<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub epoch: ByEpoch<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub year: ByYear<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub type_: BySpendableType<UTXOCohortVecs<BasicCohortMetrics<M>>>,
}

macro_rules! collect_separate {
    ($self:expr, $method:ident, $trait_ref:ty) => {{
        let mut v: Vec<$trait_ref> = Vec::with_capacity(UTXOCohorts::SEPARATE_COHORT_CAPACITY);
        v.extend($self.age_range.$method().map(|x| x as $trait_ref));
        v.extend($self.epoch.$method().map(|x| x as $trait_ref));
        v.extend($self.year.$method().map(|x| x as $trait_ref));
        v.extend($self.amount_range.$method().map(|x| x as $trait_ref));
        v.extend($self.type_.$method().map(|x| x as $trait_ref));
        v
    }};
}

impl UTXOCohorts<Rw> {
    /// Import all UTXO cohorts from database.
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        states_path: &Path,
    ) -> Result<Self> {
        let v = version + VERSION;

        // Phase 1: Import "all" supply first so it can be referenced by all cohorts' relative metrics.
        let all_full_name = CohortContext::Utxo.full_name(&Filter::All, "");
        let all_cfg = ImportConfig {
            db,
            filter: &Filter::All,
            full_name: &all_full_name,
            version: v + Version::ONE,
            indexes,
        };
        let all_supply = SupplyMetrics::forced_import(&all_cfg)?;

        // Phase 2: Import separate (stateful) cohorts.

        // age_range: ExtendedCohortMetrics with full state
        let age_range = {
            ByAgeRange::try_new(&|f: Filter, name: &'static str| -> Result<_> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                };
                let state = Some(Box::new(UTXOCohortState::new(states_path, &full_name)));
                Ok(UTXOCohortVecs::new(
                    state,
                    ExtendedCohortMetrics::forced_import(&cfg)?,
                ))
            })?
        };

        // Helper for separate cohorts with BasicCohortMetrics + full state
        let basic_separate =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<BasicCohortMetrics>> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                };
                let state = Some(Box::new(UTXOCohortState::new(states_path, &full_name)));
                Ok(UTXOCohortVecs::new(
                    state,
                    BasicCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let amount_range = ByAmountRange::try_new(&basic_separate)?;
        let epoch = ByEpoch::try_new(&basic_separate)?;
        let year = ByYear::try_new(&basic_separate)?;
        let type_ = BySpendableType::try_new(&basic_separate)?;

        // Phase 3: Import "all" cohort with pre-imported supply.
        let all = UTXOCohortVecs::new(
            None,
            AllCohortMetrics::forced_import_with_supply(&all_cfg, all_supply)?,
        );

        // Phase 4: Import aggregate cohorts.

        // sth: ExtendedAdjustedCohortMetrics
        let sth = {
            let f = Filter::Term(Term::Sth);
            let full_name = CohortContext::Utxo.full_name(&f, "sth");
            let cfg = ImportConfig {
                db,
                filter: &f,
                full_name: &full_name,
                version: v,
                indexes,
            };
            UTXOCohortVecs::new(None, ExtendedAdjustedCohortMetrics::forced_import(&cfg)?)
        };

        // lth: ExtendedCohortMetrics
        let lth = {
            let f = Filter::Term(Term::Lth);
            let full_name = CohortContext::Utxo.full_name(&f, "lth");
            let cfg = ImportConfig {
                db,
                filter: &f,
                full_name: &full_name,
                version: v,
                indexes,
            };
            UTXOCohortVecs::new(None, ExtendedCohortMetrics::forced_import(&cfg)?)
        };

        // max_age: AdjustedCohortMetrics (adjusted + peak_regret)
        let max_age = {
            ByMaxAge::try_new(&|f: Filter, name: &'static str| -> Result<_> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    AdjustedCohortMetrics::forced_import(&cfg)?,
                ))
            })?
        };

        // min_age: BasicCohortMetrics
        let min_age = {
            ByMinAge::try_new(&|f: Filter, name: &'static str| -> Result<_> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    BasicCohortMetrics::forced_import(&cfg)?,
                ))
            })?
        };

        // ge_amount, lt_amount: BasicCohortMetrics (no state)
        let basic_no_state =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<BasicCohortMetrics>> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    BasicCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let lt_amount = ByLowerThanAmount::try_new(&basic_no_state)?;
        let ge_amount = ByGreatEqualAmount::try_new(&basic_no_state)?;

        Ok(Self {
            all,
            sth,
            lth,
            epoch,
            year,
            type_,
            max_age,
            min_age,
            age_range,
            amount_range,
            lt_amount,
            ge_amount,
        })
    }

    /// ~71 separate cohorts (21 age + 5 epoch + 18 year + 15 amount + 12 type)
    const SEPARATE_COHORT_CAPACITY: usize = 80;

    pub(crate) fn par_iter_separate_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn DynCohortVecs> {
        collect_separate!(self, iter_mut, &mut dyn DynCohortVecs).into_par_iter()
    }

    /// Immutable iterator over all separate (stateful) cohorts.
    pub(crate) fn iter_separate(&self) -> impl Iterator<Item = &dyn DynCohortVecs> {
        collect_separate!(self, iter, &dyn DynCohortVecs).into_iter()
    }

    /// Mutable iterator over all separate cohorts (non-parallel).
    pub(crate) fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut dyn DynCohortVecs> {
        collect_separate!(self, iter_mut, &mut dyn DynCohortVecs).into_iter()
    }

    pub(crate) fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let age_range = &self.age_range;
        let amount_range = &self.amount_range;

        // all: aggregate of all age_range
        // Note: realized.extended rolling sums are computed from base in compute_rest_part2.
        // Note: cost_basis.extended percentiles are computed in truncate_push_aggregate_percentiles.
        {
            let sources_dyn: Vec<&dyn CohortMetricsBase> = age_range
                .iter()
                .map(|v| &v.metrics as &dyn CohortMetricsBase)
                .collect();
            self.all
                .metrics
                .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;
        }

        // sth: aggregate of matching age_range
        {
            let sth_filter = self.sth.metrics.filter().clone();
            let sources_dyn: Vec<&dyn CohortMetricsBase> = age_range
                .iter()
                .filter(|v| sth_filter.includes(v.metrics.filter()))
                .map(|v| &v.metrics as &dyn CohortMetricsBase)
                .collect();
            self.sth
                .metrics
                .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;
        }

        // lth: aggregate of matching age_range
        {
            let lth_filter = self.lth.metrics.filter().clone();
            let sources_dyn: Vec<&dyn CohortMetricsBase> = age_range
                .iter()
                .filter(|v| lth_filter.includes(v.metrics.filter()))
                .map(|v| &v.metrics as &dyn CohortMetricsBase)
                .collect();
            self.lth
                .metrics
                .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;
        }

        // min_age: base from matching age_range
        self.min_age
            .par_iter_mut()
            .try_for_each(|vecs| -> Result<()> {
                let filter = vecs.metrics.filter().clone();
                let sources_dyn: Vec<&dyn CohortMetricsBase> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics
                    .compute_base_from_others(starting_indexes, &sources_dyn, exit)
            })?;

        // max_age: base + peak_regret from matching age_range
        self.max_age
            .par_iter_mut()
            .try_for_each(|vecs| -> Result<()> {
                let filter = vecs.metrics.filter().clone();
                let sources_dyn: Vec<&dyn CohortMetricsBase> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics
                    .compute_base_from_others(starting_indexes, &sources_dyn, exit)
            })?;

        // ge_amount, lt_amount: base only from matching amount_range
        self.ge_amount
            .par_iter_mut()
            .chain(self.lt_amount.par_iter_mut())
            .try_for_each(|vecs| {
                let filter = vecs.metrics.filter().clone();
                let sources_dyn: Vec<&dyn CohortMetricsBase> = amount_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics
                    .compute_base_from_others(starting_indexes, &sources_dyn, exit)
            })?;

        Ok(())
    }

    /// First phase of post-processing: compute index transforms.
    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute all metrics except net_sentiment (all cohorts via DynCohortVecs)
        {
            let mut all: Vec<&mut dyn DynCohortVecs> = Vec::with_capacity(Self::SEPARATE_COHORT_CAPACITY + 3);
            all.push(&mut self.all);
            all.push(&mut self.sth);
            all.push(&mut self.lth);
            all.extend(self.max_age.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
            all.extend(self.min_age.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
            all.extend(
                self.ge_amount
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(
                self.age_range
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(self.epoch.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
            all.extend(self.year.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
            all.extend(
                self.amount_range
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(
                self.lt_amount
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(self.type_.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
            all.into_par_iter()
                .try_for_each(|v| v.compute_rest_part1(blocks, prices, starting_indexes, exit))?;
        }

        // 2. Compute net_sentiment.height for separate cohorts (greed - pain)
        self.par_iter_separate_mut()
            .try_for_each(|v| v.compute_net_sentiment_height(starting_indexes, exit))?;

        // 3. Compute net_sentiment.height for aggregate cohorts (weighted average)
        {
            let age_range = &self.age_range;
            let amount_range = &self.amount_range;

            // all
            {
                let sources: Vec<_> = age_range
                    .iter()
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                self.all.metrics.compute_net_sentiment_from_others_dyn(
                    starting_indexes,
                    &sources,
                    exit,
                )?;
            }

            // sth
            {
                let filter = self.sth.metrics.filter().clone();
                let sources: Vec<_> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                self.sth.metrics.compute_net_sentiment_from_others_dyn(
                    starting_indexes,
                    &sources,
                    exit,
                )?;
            }

            // lth
            {
                let filter = self.lth.metrics.filter().clone();
                let sources: Vec<_> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                self.lth.metrics.compute_net_sentiment_from_others_dyn(
                    starting_indexes,
                    &sources,
                    exit,
                )?;
            }

            // min_age, max_age from age_range
            for vecs in self.min_age.iter_mut() {
                let filter = vecs.metrics.filter().clone();
                let sources: Vec<_> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics.compute_net_sentiment_from_others_dyn(
                    starting_indexes,
                    &sources,
                    exit,
                )?;
            }
            for vecs in self.max_age.iter_mut() {
                let filter = vecs.metrics.filter().clone();
                let sources: Vec<_> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics.compute_net_sentiment_from_others_dyn(
                    starting_indexes,
                    &sources,
                    exit,
                )?;
            }

            // ge_amount, lt_amount from amount_range
            for vecs in self.ge_amount.iter_mut().chain(self.lt_amount.iter_mut()) {
                let filter = vecs.metrics.filter().clone();
                let sources: Vec<_> = amount_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics.compute_net_sentiment_from_others_dyn(
                    starting_indexes,
                    &sources,
                    exit,
                )?;
            }
        }

        Ok(())
    }

    /// Second phase of post-processing: compute relative metrics.
    pub(crate) fn compute_rest_part2<HM>(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_market_cap: &HM,
        exit: &Exit,
    ) -> Result<()>
    where
        HM: ReadableVec<Height, Dollars> + Sync,
    {
        // Get up_to_1h value sources for adjusted computation (cloned to avoid borrow conflicts).
        let up_to_1h_value_created = self
            .age_range
            .up_to_1h
            .metrics
            .realized
            .value_created
            .height
            .read_only_clone();
        let up_to_1h_value_destroyed = self
            .age_range
            .up_to_1h
            .metrics
            .realized
            .value_destroyed
            .height
            .read_only_clone();

        // "all" cohort computed first (no all_supply_sats needed).
        self.all.metrics.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_market_cap,
            &up_to_1h_value_created,
            &up_to_1h_value_destroyed,
            exit,
        )?;

        // Clone all_supply_sats for non-all cohorts.
        let all_supply_sats = self.all.metrics.supply.total.sats.height.read_only_clone();

        self.sth.metrics.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_market_cap,
            &up_to_1h_value_created,
            &up_to_1h_value_destroyed,
            &all_supply_sats,
            exit,
        )?;
        self.lth.metrics.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_market_cap,
            &all_supply_sats,
            exit,
        )?;
        self.age_range.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.max_age.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &up_to_1h_value_created,
                &up_to_1h_value_destroyed,
                &all_supply_sats,
                exit,
            )
        })?;
        self.min_age.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.ge_amount.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.epoch.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.year.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.amount_range.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.lt_amount.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        self.type_.par_iter_mut().try_for_each(|v| {
            v.metrics.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                &all_supply_sats,
                exit,
            )
        })?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_vecs_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::with_capacity(2048);
        vecs.extend(self.all.metrics.collect_all_vecs_mut());
        vecs.extend(self.sth.metrics.collect_all_vecs_mut());
        vecs.extend(self.lth.metrics.collect_all_vecs_mut());
        for v in self.age_range.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.max_age.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.min_age.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.ge_amount.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.epoch.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.year.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.amount_range.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.lt_amount.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.type_.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        vecs.into_par_iter()
    }

    /// Commit all states to disk (separate from vec writes for parallelization).
    pub(crate) fn commit_all_states(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.write_state(height, cleanup))
    }

    /// Get minimum height from all separate cohorts' height-indexed vectors.
    pub(crate) fn min_separate_stateful_height_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_stateful_height_len()))
            .min()
            .unwrap_or_default()
    }

    /// Import state for all separate cohorts at or before given height.
    /// Returns true if all imports succeeded and returned the expected height.
    pub(crate) fn import_separate_states(&mut self, height: Height) -> bool {
        self.par_iter_separate_mut()
            .map(|v| v.import_state(height).unwrap_or_default())
            .all(|h| h == height)
    }

    /// Reset state heights for all separate cohorts.
    pub(crate) fn reset_separate_state_heights(&mut self) {
        self.par_iter_separate_mut().for_each(|v| {
            v.reset_state_starting_height();
        });
    }

    /// Reset cost_basis_data for all separate cohorts (called during fresh start).
    pub(crate) fn reset_separate_cost_basis_data(&mut self) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.reset_cost_basis_data_if_needed())
    }

    /// Validate computed versions for all cohorts.
    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        // Validate separate cohorts
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;

        // Validate aggregate cohorts
        self.all.metrics.validate_computed_versions(base_version)?;
        self.sth.metrics.validate_computed_versions(base_version)?;
        self.lth.metrics.validate_computed_versions(base_version)?;
        for v in self.min_age.iter_mut() {
            v.metrics.validate_computed_versions(base_version)?;
        }
        for v in self.max_age.iter_mut() {
            v.metrics.validate_computed_versions(base_version)?;
        }
        for v in self.ge_amount.iter_mut() {
            v.metrics.validate_computed_versions(base_version)?;
        }
        for v in self.lt_amount.iter_mut() {
            v.metrics.validate_computed_versions(base_version)?;
        }

        Ok(())
    }
}
