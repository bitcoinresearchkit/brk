use std::{cmp::Reverse, collections::BinaryHeap, fs, path::Path};

use brk_cohort::{
    AGE_BOUNDARIES, ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount,
    ByMaxAge, ByMinAge, BySpendableType, ByYear, CohortContext, Filter, Filtered, TERM_NAMES, Term,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Cents, CentsCompact, CostBasisDistribution, Date, Day1, Dollars, Height, ONE_HOUR_IN_SEC, Sats,
    StoredF32, Timestamp, Version,
};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, ReadOnlyClone, ReadableVec, Rw, StorageMode, VecIndex, WritableVec};

use crate::{
    ComputeIndexes, blocks,
    distribution::{DynCohortVecs, compute::PriceRangeMax, state::BlockState},
    indexes,
    internal::{PERCENTILES, PERCENTILES_LEN, compute_spot_percentile_rank},
    prices,
};

use crate::distribution::metrics::{
    AdjustedCohortMetrics, AllCohortMetrics, BasicCohortMetrics, CohortMetricsBase,
    ExtendedAdjustedCohortMetrics, ExtendedCohortMetrics, ImportConfig, PeakRegretCohortMetrics,
    SupplyMetrics,
};

use super::vecs::UTXOCohortVecs;

use crate::distribution::state::UTXOCohortState;

const VERSION: Version = Version::new(0);

/// Significant digits for cost basis prices (after rounding to dollars).
const COST_BASIS_PRICE_DIGITS: i32 = 5;

/// All UTXO cohorts organized by filter type.
///
/// Each group uses a concrete metrics type matching its required features:
/// - age_range: extended realized + extended cost basis + peak regret
/// - epoch/year/amount/type: basic metrics with relative
/// - all: extended + adjusted + peak regret (no rel_to_all)
/// - sth: extended + adjusted + peak regret
/// - lth: extended + peak regret
/// - max_age: adjusted + peak regret
/// - min_age: peak regret
#[derive(Traversable)]
pub struct UTXOCohorts<M: StorageMode = Rw> {
    pub all: UTXOCohortVecs<AllCohortMetrics<M>>,
    pub sth: UTXOCohortVecs<ExtendedAdjustedCohortMetrics<M>>,
    pub lth: UTXOCohortVecs<ExtendedCohortMetrics<M>>,
    pub age_range: ByAgeRange<UTXOCohortVecs<ExtendedCohortMetrics<M>>>,
    pub max_age: ByMaxAge<UTXOCohortVecs<AdjustedCohortMetrics<M>>>,
    pub min_age: ByMinAge<UTXOCohortVecs<PeakRegretCohortMetrics<M>>>,
    pub ge_amount: ByGreatEqualAmount<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub amount_range: ByAmountRange<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub lt_amount: ByLowerThanAmount<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub epoch: ByEpoch<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub year: ByYear<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub type_: BySpendableType<UTXOCohortVecs<BasicCohortMetrics<M>>>,
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
            filter: Filter::All,
            full_name: &all_full_name,
            context: CohortContext::Utxo,
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
                    filter: f,
                    full_name: &full_name,
                    context: CohortContext::Utxo,
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
                    filter: f,
                    full_name: &full_name,
                    context: CohortContext::Utxo,
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
                filter: f,
                full_name: &full_name,
                context: CohortContext::Utxo,
                version: v,
                indexes,
            };
            UTXOCohortVecs::new(
                None,
                ExtendedAdjustedCohortMetrics::forced_import(
                    &cfg,
                )?,
            )
        };

        // lth: ExtendedCohortMetrics
        let lth = {
            let f = Filter::Term(Term::Lth);
            let full_name = CohortContext::Utxo.full_name(&f, "lth");
            let cfg = ImportConfig {
                db,
                filter: f,
                full_name: &full_name,
                context: CohortContext::Utxo,
                version: v,
                indexes,
            };
            UTXOCohortVecs::new(
                None,
                ExtendedCohortMetrics::forced_import(&cfg)?,
            )
        };

        // max_age: AdjustedCohortMetrics (adjusted + peak_regret)
        let max_age = {
            ByMaxAge::try_new(&|f: Filter, name: &'static str| -> Result<_> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: f,
                    full_name: &full_name,
                    context: CohortContext::Utxo,
                    version: v,
                    indexes,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    AdjustedCohortMetrics::forced_import(&cfg)?,
                ))
            })?
        };

        // min_age: PeakRegretCohortMetrics
        let min_age = {
            ByMinAge::try_new(&|f: Filter, name: &'static str| -> Result<_> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: f,
                    full_name: &full_name,
                    context: CohortContext::Utxo,
                    version: v,
                    indexes,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    PeakRegretCohortMetrics::forced_import(&cfg)?,
                ))
            })?
        };

        // ge_amount, lt_amount: BasicCohortMetrics (no state)
        let basic_no_state =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<BasicCohortMetrics>> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: f,
                    full_name: &full_name,
                    context: CohortContext::Utxo,
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

    // === Iteration helpers ===

    /// Parallel iterator over all separate (stateful) cohorts.
    pub(crate) fn par_iter_separate_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn DynCohortVecs> {
        let mut v: Vec<&mut dyn DynCohortVecs> = Vec::new();
        v.extend(
            self.age_range
                .iter_mut()
                .map(|x| x as &mut dyn DynCohortVecs),
        );
        v.extend(self.epoch.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
        v.extend(self.year.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
        v.extend(
            self.amount_range
                .iter_mut()
                .map(|x| x as &mut dyn DynCohortVecs),
        );
        v.extend(self.type_.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
        v.into_par_iter()
    }

    /// Immutable iterator over all separate (stateful) cohorts.
    pub(crate) fn iter_separate(&self) -> impl Iterator<Item = &dyn DynCohortVecs> {
        let mut v: Vec<&dyn DynCohortVecs> = Vec::new();
        v.extend(self.age_range.iter().map(|x| x as &dyn DynCohortVecs));
        v.extend(self.epoch.iter().map(|x| x as &dyn DynCohortVecs));
        v.extend(self.year.iter().map(|x| x as &dyn DynCohortVecs));
        v.extend(self.amount_range.iter().map(|x| x as &dyn DynCohortVecs));
        v.extend(self.type_.iter().map(|x| x as &dyn DynCohortVecs));
        v.into_iter()
    }

    /// Mutable iterator over all separate cohorts (non-parallel).
    pub(crate) fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut dyn DynCohortVecs> {
        let mut v: Vec<&mut dyn DynCohortVecs> = Vec::new();
        v.extend(
            self.age_range
                .iter_mut()
                .map(|x| x as &mut dyn DynCohortVecs),
        );
        v.extend(self.epoch.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
        v.extend(self.year.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
        v.extend(
            self.amount_range
                .iter_mut()
                .map(|x| x as &mut dyn DynCohortVecs),
        );
        v.extend(self.type_.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
        v.into_iter()
    }

    // === Computation methods ===

    /// Compute overlapping cohorts from component age/amount range cohorts.
    pub(crate) fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let age_range = &self.age_range;
        let amount_range = &self.amount_range;

        // all: aggregate of all age_range (base + peak_regret)
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

            let pr_sources: Vec<_> = age_range
                .iter()
                .map(|v| &v.metrics.unrealized.peak_regret_ext)
                .collect();
            self.all
                .metrics
                .unrealized
                .peak_regret_ext
                .compute_from_stateful(starting_indexes, &pr_sources, exit)?;
        }

        // sth: aggregate of matching age_range (base + peak_regret)
        {
            let sth_filter = self.sth.metrics.filter().clone();
            let matching: Vec<_> = age_range
                .iter()
                .filter(|v| sth_filter.includes(v.metrics.filter()))
                .collect();

            let sources_dyn: Vec<&dyn CohortMetricsBase> = matching
                .iter()
                .map(|v| &v.metrics as &dyn CohortMetricsBase)
                .collect();
            self.sth
                .metrics
                .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;

            let pr_sources: Vec<_> = matching
                .iter()
                .map(|v| &v.metrics.unrealized.peak_regret_ext)
                .collect();
            self.sth
                .metrics
                .unrealized
                .peak_regret_ext
                .compute_from_stateful(starting_indexes, &pr_sources, exit)?;
        }

        // lth: aggregate of matching age_range (base + peak_regret)
        {
            let lth_filter = self.lth.metrics.filter().clone();
            let matching: Vec<_> = age_range
                .iter()
                .filter(|v| lth_filter.includes(v.metrics.filter()))
                .collect();

            let sources_dyn: Vec<&dyn CohortMetricsBase> = matching
                .iter()
                .map(|v| &v.metrics as &dyn CohortMetricsBase)
                .collect();
            self.lth
                .metrics
                .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;

            let pr_sources: Vec<_> = matching
                .iter()
                .map(|v| &v.metrics.unrealized.peak_regret_ext)
                .collect();
            self.lth
                .metrics
                .unrealized
                .peak_regret_ext
                .compute_from_stateful(starting_indexes, &pr_sources, exit)?;
        }

        // min_age: base + peak_regret from matching age_range
        self.min_age
            .iter_mut()
            .collect::<Vec<_>>()
            .into_par_iter()
            .try_for_each(|vecs| -> Result<()> {
                let filter = vecs.metrics.filter().clone();
                let matching: Vec<_> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .collect();

                let sources_dyn: Vec<&dyn CohortMetricsBase> = matching
                    .iter()
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics
                    .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;

                let pr_sources: Vec<_> = matching
                    .iter()
                    .map(|v| &v.metrics.unrealized.peak_regret_ext)
                    .collect();
                vecs.metrics
                    .unrealized
                    .peak_regret_ext
                    .compute_from_stateful(starting_indexes, &pr_sources, exit)?;

                Ok(())
            })?;

        // max_age: base + peak_regret from matching age_range
        self.max_age
            .iter_mut()
            .collect::<Vec<_>>()
            .into_par_iter()
            .try_for_each(|vecs| -> Result<()> {
                let filter = vecs.metrics.filter().clone();
                let matching: Vec<_> = age_range
                    .iter()
                    .filter(|v| filter.includes(v.metrics.filter()))
                    .collect();

                let sources_dyn: Vec<&dyn CohortMetricsBase> = matching
                    .iter()
                    .map(|v| &v.metrics as &dyn CohortMetricsBase)
                    .collect();
                vecs.metrics
                    .compute_base_from_others(starting_indexes, &sources_dyn, exit)?;

                let pr_sources: Vec<_> = matching
                    .iter()
                    .map(|v| &v.metrics.unrealized.peak_regret_ext)
                    .collect();
                vecs.metrics
                    .unrealized
                    .peak_regret_ext
                    .compute_from_stateful(starting_indexes, &pr_sources, exit)?;

                Ok(())
            })?;

        // ge_amount, lt_amount: base only from matching amount_range
        self.ge_amount
            .iter_mut()
            .chain(self.lt_amount.iter_mut())
            .collect::<Vec<_>>()
            .into_par_iter()
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
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute all metrics except net_sentiment (all cohorts via DynCohortVecs)
        {
            let mut all: Vec<&mut dyn DynCohortVecs> = Vec::new();
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
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &HM,
        exit: &Exit,
    ) -> Result<()>
    where
        HM: ReadableVec<Height, Dollars> + Sync,
    {
        // Get up_to_1h value sources for adjusted computation (cloned to avoid borrow conflicts).
        let up_to_1h_value_created = self.age_range.up_to_1h.metrics.realized.value_created.height.read_only_clone();
        let up_to_1h_value_destroyed = self.age_range.up_to_1h.metrics.realized.value_destroyed.height.read_only_clone();

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
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
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

    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    pub(crate) fn truncate_push_aggregate_percentiles(
        &mut self,
        height: Height,
        spot: Dollars,
        day1_opt: Option<Day1>,
        states_path: &Path,
    ) -> Result<()> {
        // Collect (filter, entries, total_sats, total_usd) from age_range cohorts.
        let age_range_data: Vec<_> = self
            .age_range
            .iter()
            .filter_map(|sub| {
                let state = sub.state.as_ref()?;
                let mut total_sats: u64 = 0;
                let mut total_usd: u128 = 0;
                let entries: Vec<(Cents, Sats)> = state
                    .cost_basis_data_iter()
                    .map(|(price, &sats)| {
                        let sats_u64 = u64::from(sats);
                        let price_u128 = price.as_u128();
                        total_sats += sats_u64;
                        total_usd += price_u128 * sats_u64 as u128;
                        (price, sats)
                    })
                    .collect();
                Some((sub.filter().clone(), entries, total_sats, total_usd))
            })
            .collect();

        // Build list of (filter, cost_basis_extended, cohort_name) for aggregate cohorts
        struct AggregateTarget<'a> {
            filter: Filter,
            extended: &'a mut crate::distribution::metrics::CostBasisExtended,
            cohort_name: Option<&'static str>,
        }

        let mut targets = [
            AggregateTarget {
                filter: self.all.metrics.filter().clone(),
                extended: &mut self.all.metrics.cost_basis.extended,
                cohort_name: Some("all"),
            },
            AggregateTarget {
                filter: self.sth.metrics.filter().clone(),
                extended: &mut self.sth.metrics.cost_basis.extended,
                cohort_name: Some(TERM_NAMES.short.id),
            },
            AggregateTarget {
                filter: self.lth.metrics.filter().clone(),
                extended: &mut self.lth.metrics.cost_basis.extended,
                cohort_name: Some(TERM_NAMES.long.id),
            },
        ];

        for target in targets.iter_mut() {
            let filter = &target.filter;

            let mut total_sats: u64 = 0;
            let mut total_usd: u128 = 0;
            let relevant: Vec<_> = age_range_data
                .iter()
                .filter(|(sub_filter, _, _, _)| filter.includes(sub_filter))
                .map(|(_, entries, cohort_sats, cohort_usd)| {
                    total_sats += cohort_sats;
                    total_usd += cohort_usd;
                    entries
                })
                .collect();

            if total_sats == 0 {
                let nan_prices = [Dollars::NAN; PERCENTILES_LEN];
                target
                    .extended
                    .percentiles
                    .truncate_push(height, &nan_prices)?;
                target
                    .extended
                    .invested_capital
                    .truncate_push(height, &nan_prices)?;
                target
                    .extended
                    .spot_cost_basis_percentile
                    .height
                    .truncate_push(height, StoredF32::NAN)?;
                target
                    .extended
                    .spot_invested_capital_percentile
                    .height
                    .truncate_push(height, StoredF32::NAN)?;
                continue;
            }

            // K-way merge using min-heap
            let mut heap: BinaryHeap<Reverse<(Cents, usize, usize)>> = BinaryHeap::new();
            for (cohort_idx, entries) in relevant.iter().enumerate() {
                if !entries.is_empty() {
                    heap.push(Reverse((entries[0].0, cohort_idx, 0)));
                }
            }

            let sat_targets = PERCENTILES.map(|p| total_sats * u64::from(p) / 100);
            let usd_targets = PERCENTILES.map(|p| total_usd * u128::from(p) / 100);

            let mut sat_result = [Dollars::NAN; PERCENTILES_LEN];
            let mut usd_result = [Dollars::NAN; PERCENTILES_LEN];

            let mut cumsum_sats: u64 = 0;
            let mut cumsum_usd: u128 = 0;
            let mut sat_idx = 0;
            let mut usd_idx = 0;

            let mut current_price: Option<Cents> = None;
            let mut sats_at_price: u64 = 0;
            let mut usd_at_price: u128 = 0;

            let collect_merged = day1_opt.is_some();
            let max_unique_prices = if collect_merged {
                relevant.iter().map(|e| e.len()).max().unwrap_or(0)
            } else {
                0
            };
            let mut merged: Vec<(CentsCompact, Sats)> = Vec::with_capacity(max_unique_prices);

            let mut finalize_price = |price: Cents, sats: u64, usd: u128| {
                cumsum_sats += sats;
                cumsum_usd += usd;

                if sat_idx < PERCENTILES_LEN || usd_idx < PERCENTILES_LEN {
                    let dollars = price.to_dollars();
                    while sat_idx < PERCENTILES_LEN && cumsum_sats >= sat_targets[sat_idx] {
                        sat_result[sat_idx] = dollars;
                        sat_idx += 1;
                    }
                    while usd_idx < PERCENTILES_LEN && cumsum_usd >= usd_targets[usd_idx] {
                        usd_result[usd_idx] = dollars;
                        usd_idx += 1;
                    }
                }

                if collect_merged {
                    let rounded: CentsCompact =
                        price.round_to_dollar(COST_BASIS_PRICE_DIGITS).into();
                    if let Some((last_price, last_sats)) = merged.last_mut()
                        && *last_price == rounded
                    {
                        *last_sats += Sats::from(sats);
                    } else {
                        merged.push((rounded, Sats::from(sats)));
                    }
                }
            };

            while let Some(Reverse((price, cohort_idx, entry_idx))) = heap.pop() {
                let entries = relevant[cohort_idx];
                let (_, amount) = entries[entry_idx];
                let amount_u64 = u64::from(amount);
                let price_u128 = price.as_u128();

                if let Some(prev_price) = current_price
                    && prev_price != price
                {
                    finalize_price(prev_price, sats_at_price, usd_at_price);
                    sats_at_price = 0;
                    usd_at_price = 0;
                }

                current_price = Some(price);
                sats_at_price += amount_u64;
                usd_at_price += price_u128 * amount_u64 as u128;

                let next_idx = entry_idx + 1;
                if next_idx < entries.len() {
                    heap.push(Reverse((entries[next_idx].0, cohort_idx, next_idx)));
                }
            }

            if let Some(price) = current_price {
                finalize_price(price, sats_at_price, usd_at_price);
            }

            target
                .extended
                .percentiles
                .truncate_push(height, &sat_result)?;
            target
                .extended
                .invested_capital
                .truncate_push(height, &usd_result)?;

            let rank = compute_spot_percentile_rank(&sat_result, spot);
            target
                .extended
                .spot_cost_basis_percentile
                .height
                .truncate_push(height, rank)?;
            let rank = compute_spot_percentile_rank(&usd_result, spot);
            target
                .extended
                .spot_invested_capital_percentile
                .height
                .truncate_push(height, rank)?;

            // Write daily cost basis snapshot
            if let Some(day1) = day1_opt
                && let Some(cohort_name) = target.cohort_name
            {
                let date = Date::from(day1);
                let dir = states_path.join(format!("utxo_{cohort_name}_cost_basis/by_date"));
                fs::create_dir_all(&dir)?;
                let path = dir.join(date.to_string());
                fs::write(
                    path,
                    CostBasisDistribution::serialize_iter(merged.into_iter())?,
                )?;
            }
        }

        Ok(())
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

    /// Compute and push peak regret for all age_range cohorts.
    pub(crate) fn compute_and_push_peak_regret(
        &mut self,
        chain_state: &[BlockState],
        current_height: Height,
        current_timestamp: Timestamp,
        spot: Cents,
        price_range_max: &PriceRangeMax,
    ) -> Result<()> {
        const FIRST_PRICE_HEIGHT: usize = 68_195;

        let start_height = FIRST_PRICE_HEIGHT;
        let end_height = current_height.to_usize() + 1;

        if end_height <= start_height {
            for cohort in self.age_range.iter_mut() {
                cohort
                    .metrics
                    .unrealized
                    .peak_regret_ext
                    .peak_regret
                    .height
                    .truncate_push(current_height, Dollars::ZERO)?;
            }
            return Ok(());
        }

        let spot_u128 = spot.as_u128();
        let current_ts = *current_timestamp;

        let splits: [usize; 20] = std::array::from_fn(|k| {
            let boundary_seconds = (AGE_BOUNDARIES[k] as u32) * ONE_HOUR_IN_SEC;
            let threshold_ts = current_ts.saturating_sub(boundary_seconds);
            chain_state[..end_height].partition_point(|b| *b.timestamp <= threshold_ts)
        });

        let ranges: [(usize, usize); 21] = std::array::from_fn(|i| {
            if i == 0 {
                (splits[0], end_height)
            } else if i < 20 {
                (splits[i], splits[i - 1])
            } else {
                (start_height, splits[19])
            }
        });

        let regrets: [Dollars; 21] = ranges
            .into_par_iter()
            .map(|(range_start, range_end)| {
                let effective_start = range_start.max(start_height);
                if effective_start >= range_end {
                    return Dollars::ZERO;
                }

                let mut regret: u128 = 0;
                for (i, block) in chain_state[effective_start..range_end].iter().enumerate() {
                    let supply = block.supply.value;

                    if supply.is_zero() {
                        continue;
                    }

                    let cost_basis = block.price;
                    let receive_height = Height::from(effective_start + i);
                    let peak = price_range_max.max_between(receive_height, current_height);
                    let peak_u128 = peak.as_u128();
                    let cost_u128 = cost_basis.as_u128();
                    let supply_u128 = supply.as_u128();

                    regret += if spot_u128 >= cost_u128 {
                        (peak_u128 - spot_u128) * supply_u128
                    } else {
                        (peak_u128 - cost_u128) * supply_u128
                    };
                }

                Cents::new((regret / Sats::ONE_BTC_U128) as u64).to_dollars()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for (cohort, regret) in self.age_range.iter_mut().zip(regrets) {
            cohort
                .metrics
                .unrealized
                .peak_regret_ext
                .peak_regret
                .height
                .truncate_push(current_height, regret)?;
        }

        Ok(())
    }
}
