use std::path::Path;

use brk_cohort::{
    ByAgeRange, ByAmountRange, ByClass, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge,
    ByMinAge, BySpendableType, CohortContext, Filter, Filtered, Term,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Indexes, Sats, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, ReadOnlyClone, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::{
        DynCohortVecs,
        metrics::{
            AllCohortMetrics, BasicCohortMetrics, CohortMetricsBase, CoreCohortMetrics,
            ExtendedAdjustedCohortMetrics, ExtendedCohortMetrics, ImportConfig,
            MinimalCohortMetrics, ProfitabilityMetrics, SupplyMetrics,
        },
        state::UTXOCohortState,
    },
    indexes,
    internal::ValueFromHeight,
    prices,
};

use super::{fenwick::CostBasisFenwick, vecs::UTXOCohortVecs};

const VERSION: Version = Version::new(0);

/// All UTXO cohorts organized by filter type.
#[derive(Traversable)]
pub struct UTXOCohorts<M: StorageMode = Rw> {
    pub all: UTXOCohortVecs<AllCohortMetrics<M>>,
    pub sth: UTXOCohortVecs<ExtendedAdjustedCohortMetrics<M>>,
    pub lth: UTXOCohortVecs<ExtendedCohortMetrics<M>>,
    pub age_range: ByAgeRange<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub max_age: ByMaxAge<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub min_age: ByMinAge<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub epoch: ByEpoch<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub class: ByClass<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub ge_amount: ByGreatEqualAmount<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    pub amount_range: ByAmountRange<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    pub lt_amount: ByLowerThanAmount<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    pub type_: BySpendableType<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    pub profitability: ProfitabilityMetrics<M>,
    pub matured: ByAgeRange<ValueFromHeight<M>>,
    #[traversable(skip)]
    pub(super) fenwick: CostBasisFenwick,
    /// Cached partition_point positions for tick_tock boundary searches.
    /// Avoids O(log n) binary search per boundary per block; scans forward
    /// from last known position (typically O(1) per boundary).
    #[traversable(skip)]
    pub(super) tick_tock_cached_positions: [usize; 20],
}

impl UTXOCohorts<Rw> {
    /// ~71 separate cohorts (21 age + 5 epoch + 18 class + 15 amount + 12 type)
    const SEPARATE_COHORT_CAPACITY: usize = 80;

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

        let age_range = ByAgeRange::try_new(&basic_separate)?;

        let core_separate =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<CoreCohortMetrics>> {
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
                    CoreCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let epoch = ByEpoch::try_new(&core_separate)?;
        let class = ByClass::try_new(&core_separate)?;

        // Helper for separate cohorts with MinimalCohortMetrics + MinimalRealizedState
        let minimal_separate =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<MinimalCohortMetrics>> {
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
                    MinimalCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let amount_range = ByAmountRange::try_new(&minimal_separate)?;
        let type_ = BySpendableType::try_new(&minimal_separate)?;

        // Phase 3: Import "all" cohort with pre-imported supply.
        let all = UTXOCohortVecs::new(
            None,
            AllCohortMetrics::forced_import_with_supply(&all_cfg, all_supply)?,
        );

        // Phase 3b: Import profitability metrics (derived from "all" during k-way merge).
        let profitability = ProfitabilityMetrics::forced_import(db, v, indexes)?;

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

        // CoreCohortMetrics without state (no state, for aggregate cohorts)
        let core_no_state =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<CoreCohortMetrics>> {
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
                    CoreCohortMetrics::forced_import(&cfg)?,
                ))
            };

        // max_age: CoreCohortMetrics (no state, aggregates from age_range)
        let max_age = ByMaxAge::try_new(&core_no_state)?;

        // min_age: CoreCohortMetrics (no state, aggregates from age_range)
        let min_age = ByMinAge::try_new(&core_no_state)?;

        // MinimalCohortMetrics without state (for aggregate amount cohorts)
        let minimal_no_state =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<MinimalCohortMetrics>> {
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
                    MinimalCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let lt_amount = ByLowerThanAmount::try_new(&minimal_no_state)?;
        let ge_amount = ByGreatEqualAmount::try_new(&minimal_no_state)?;

        let matured = ByAgeRange::try_new(&|_f: Filter, name: &'static str| -> Result<ValueFromHeight> {
            ValueFromHeight::forced_import(db, &format!("utxo_{name}_matured"), v, indexes)
        })?;

        Ok(Self {
            all,
            sth,
            lth,
            epoch,
            class,
            type_,
            max_age,
            min_age,
            age_range,
            amount_range,
            lt_amount,
            ge_amount,
            profitability,
            matured,
            fenwick: CostBasisFenwick::new(),
            tick_tock_cached_positions: [0; 20],
        })
    }

    /// Initialize the Fenwick tree from all age-range BTreeMaps.
    /// Call after state import when all pending maps have been drained.
    pub(crate) fn init_fenwick_if_needed(&mut self) {
        if self.fenwick.is_initialized() {
            return;
        }
        let Self {
            sth, fenwick, age_range, ..
        } = self;
        fenwick.compute_is_sth(&sth.metrics.filter, age_range.iter().map(|v| v.filter()));

        let maps: Vec<_> = age_range
            .iter()
            .enumerate()
            .filter_map(|(i, sub)| {
                let state = sub.state.as_ref()?;
                let map = state.cost_basis_map();
                if map.is_empty() {
                    return None;
                }
                Some((map, fenwick.is_sth_at(i)))
            })
            .collect();
        fenwick.bulk_init(maps.into_iter());
    }

    /// Apply pending deltas from all age-range cohorts to the Fenwick tree.
    /// Call after receive/send, before push_cohort_states.
    pub(crate) fn update_fenwick_from_pending(&mut self) {
        if !self.fenwick.is_initialized() {
            return;
        }
        // Destructure to get separate borrows on fenwick and age_range
        let Self {
            fenwick, age_range, ..
        } = self;
        for (i, sub) in age_range.iter().enumerate() {
            if let Some(state) = sub.state.as_ref() {
                let is_sth = fenwick.is_sth_at(i);
                state.for_each_cost_basis_pending(|&price, delta| {
                    fenwick.apply_delta(price, delta, is_sth);
                });
            }
        }
    }

    /// Push maturation sats to the matured vecs for the given height.
    pub(crate) fn push_maturation(
        &mut self,
        height: Height,
        matured: &ByAgeRange<Sats>,
    ) -> Result<()> {
        for (v, &sats) in self.matured.iter_mut().zip(matured.iter()) {
            v.sats.height.truncate_push(height, sats)?;
        }
        Ok(())
    }

    pub(crate) fn par_iter_separate_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn DynCohortVecs> {
        let Self {
            age_range,
            epoch,
            class,
            amount_range,
            type_,
            ..
        } = self;
        age_range
            .par_iter_mut()
            .map(|x| x as &mut dyn DynCohortVecs)
            .chain(epoch.par_iter_mut().map(|x| x as &mut dyn DynCohortVecs))
            .chain(class.par_iter_mut().map(|x| x as &mut dyn DynCohortVecs))
            .chain(
                amount_range
                    .par_iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            )
            .chain(type_.par_iter_mut().map(|x| x as &mut dyn DynCohortVecs))
    }

    /// Immutable iterator over all separate (stateful) cohorts.
    pub(crate) fn iter_separate(&self) -> impl Iterator<Item = &dyn DynCohortVecs> {
        self.age_range
            .iter()
            .map(|x| x as &dyn DynCohortVecs)
            .chain(self.epoch.iter().map(|x| x as &dyn DynCohortVecs))
            .chain(self.class.iter().map(|x| x as &dyn DynCohortVecs))
            .chain(self.amount_range.iter().map(|x| x as &dyn DynCohortVecs))
            .chain(self.type_.iter().map(|x| x as &dyn DynCohortVecs))
    }

    pub(crate) fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let Self {
            all,
            sth,
            lth,
            age_range,
            max_age,
            min_age,
            ge_amount,
            amount_range,
            lt_amount,
            ..
        } = self;

        let ar = &*age_range;
        let amr = &*amount_range;
        let si = starting_indexes;

        let tasks: Vec<Box<dyn FnOnce() -> Result<()> + Send + '_>> = vec![
            Box::new(|| {
                let sources = filter_sources_from(ar.iter(), None);
                all.metrics.compute_base_from_others(si, &sources, exit)
            }),
            Box::new(|| {
                let sources = filter_sources_from(ar.iter(), Some(sth.metrics.filter()));
                sth.metrics.compute_base_from_others(si, &sources, exit)
            }),
            Box::new(|| {
                let sources = filter_sources_from(ar.iter(), Some(lth.metrics.filter()));
                lth.metrics.compute_base_from_others(si, &sources, exit)
            }),
            Box::new(|| {
                min_age.par_iter_mut().try_for_each(|vecs| {
                    let sources = filter_sources_from(ar.iter(), Some(&vecs.metrics.filter));
                    vecs.metrics.compute_from_base_sources(si, &sources, exit)
                })
            }),
            Box::new(|| {
                max_age.par_iter_mut().try_for_each(|vecs| {
                    let sources = filter_sources_from(ar.iter(), Some(&vecs.metrics.filter));
                    vecs.metrics.compute_from_base_sources(si, &sources, exit)
                })
            }),
            Box::new(|| {
                ge_amount
                    .par_iter_mut()
                    .chain(lt_amount.par_iter_mut())
                    .try_for_each(|vecs| {
                        let sources =
                            filter_minimal_sources_from(amr.iter(), Some(&vecs.metrics.filter));
                        vecs.metrics.compute_from_sources(si, &sources, exit)
                    })
            }),
        ];

        tasks
            .into_par_iter()
            .map(|f| f())
            .collect::<Result<Vec<_>>>()?;

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
            let mut all: Vec<&mut dyn DynCohortVecs> =
                Vec::with_capacity(Self::SEPARATE_COHORT_CAPACITY + 3);
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
            all.extend(self.class.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
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

        // Compute matured cents from sats × price
        self.matured
            .par_iter_mut()
            .try_for_each(|v| v.compute(prices, starting_indexes.height, exit))?;

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

        // Destructure to allow parallel mutable access to independent fields.
        let Self {
            sth,
            lth,
            age_range,
            max_age,
            min_age,
            ge_amount,
            amount_range,
            lt_amount,
            epoch,
            class,
            type_,
            ..
        } = self;

        // All remaining groups run in parallel. Each closure owns an exclusive &mut
        // to its field and shares read-only references to common data.
        let vc = &up_to_1h_value_created;
        let vd = &up_to_1h_value_destroyed;
        let ss = &all_supply_sats;

        let tasks: Vec<Box<dyn FnOnce() -> Result<()> + Send + '_>> = vec![
            Box::new(|| {
                sth.metrics.compute_rest_part2(
                    blocks,
                    prices,
                    starting_indexes,
                    height_to_market_cap,
                    vc,
                    vd,
                    ss,
                    exit,
                )
            }),
            Box::new(|| {
                lth.metrics.compute_rest_part2(
                    blocks,
                    prices,
                    starting_indexes,
                    height_to_market_cap,
                    ss,
                    exit,
                )
            }),
            Box::new(|| {
                age_range.par_iter_mut().try_for_each(|v| {
                    v.metrics.compute_rest_part2(
                        blocks,
                        prices,
                        starting_indexes,
                        ss,
                        exit,
                    )
                })
            }),
            Box::new(|| {
                max_age.par_iter_mut().try_for_each(|v| {
                    v.metrics.compute_rest_part2(
                        blocks,
                        prices,
                        starting_indexes,
                        ss,
                        exit,
                    )
                })
            }),
            Box::new(|| {
                min_age.par_iter_mut().try_for_each(|v| {
                    v.metrics.compute_rest_part2(
                        blocks,
                        prices,
                        starting_indexes,
                        ss,
                        exit,
                    )
                })
            }),
            Box::new(|| {
                ge_amount
                    .par_iter_mut()
                    .try_for_each(|v| v.metrics.compute_rest_part2(prices, starting_indexes, exit))
            }),
            Box::new(|| {
                epoch.par_iter_mut().try_for_each(|v| {
                    v.metrics.compute_rest_part2(
                        blocks,
                        prices,
                        starting_indexes,
                        ss,
                        exit,
                    )
                })
            }),
            Box::new(|| {
                class.par_iter_mut().try_for_each(|v| {
                    v.metrics.compute_rest_part2(
                        blocks,
                        prices,
                        starting_indexes,
                        ss,
                        exit,
                    )
                })
            }),
            Box::new(|| {
                amount_range
                    .par_iter_mut()
                    .try_for_each(|v| v.metrics.compute_rest_part2(prices, starting_indexes, exit))
            }),
            Box::new(|| {
                lt_amount
                    .par_iter_mut()
                    .try_for_each(|v| v.metrics.compute_rest_part2(prices, starting_indexes, exit))
            }),
            Box::new(|| {
                type_
                    .par_iter_mut()
                    .try_for_each(|v| v.metrics.compute_rest_part2(prices, starting_indexes, exit))
            }),
        ];

        tasks
            .into_par_iter()
            .map(|f| f())
            .collect::<Result<Vec<_>>>()?;

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
        for v in self.class.iter_mut() {
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
        vecs.extend(self.profitability.collect_all_vecs_mut());
        for v in self.matured.iter_mut() {
            let base = &mut v.base;
            vecs.push(&mut base.sats.height);
            vecs.push(&mut base.cents.height);
        }
        vecs.into_par_iter()
    }

    /// Commit all states to disk (separate from vec writes for parallelization).
    pub(crate) fn commit_all_states(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.write_state(height, cleanup))
    }

    /// Get minimum height from all separate cohorts' + profitability height-indexed vectors.
    pub(crate) fn min_separate_stateful_height_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_stateful_height_len()))
            .chain(self.matured.iter().map(|v| Height::from(v.min_stateful_len())))
            .min()
            .unwrap_or_default()
            .min(Height::from(self.profitability.min_stateful_height_len()))
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

/// Filter source cohorts by an optional filter.
/// If filter is None, returns all sources (used for "all" aggregate).
fn filter_sources_from<'a, M: CohortMetricsBase + 'a>(
    sources: impl Iterator<Item = &'a UTXOCohortVecs<M>>,
    filter: Option<&Filter>,
) -> Vec<&'a M> {
    match filter {
        Some(f) => sources
            .filter(|v| f.includes(v.metrics.filter()))
            .map(|v| &v.metrics)
            .collect(),
        None => sources.map(|v| &v.metrics).collect(),
    }
}

/// Filter MinimalCohortMetrics source cohorts by an optional filter.
fn filter_minimal_sources_from<'a>(
    sources: impl Iterator<Item = &'a UTXOCohortVecs<MinimalCohortMetrics>>,
    filter: Option<&Filter>,
) -> Vec<&'a MinimalCohortMetrics> {
    match filter {
        Some(f) => sources
            .filter(|v| f.includes(&v.metrics.filter))
            .map(|v| &v.metrics)
            .collect(),
        None => sources.map(|v| &v.metrics).collect(),
    }
}
