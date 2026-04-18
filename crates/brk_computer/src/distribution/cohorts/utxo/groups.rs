use std::path::Path;

use brk_cohort::{
    AgeRange, AmountRange, ByEpoch, Class, CohortContext, Filter, Filtered, OverAge, OverAmount,
    SpendableType, Term, UnderAge, UnderAmount,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSquaredSats, Dollars, Height, Indexes, Sats, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, ReadOnlyClone, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::{
        DynCohortVecs,
        metrics::{
            AllCohortMetrics, BasicCohortMetrics, CohortMetricsBase, CoreCohortMetrics,
            ExtendedAdjustedCohortMetrics, ExtendedCohortMetrics, ImportConfig,
            MinimalCohortMetrics, ProfitabilityMetrics, RealizedFullAccum, SupplyCore,
            TypeCohortMetrics,
        },
        state::UTXOCohortState,
    },
    indexes,
    internal::{AmountPerBlockCumulativeRolling, WindowStartVec, Windows},
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
    pub age_range: AgeRange<UTXOCohortVecs<BasicCohortMetrics<M>>>,
    pub under_age: UnderAge<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub over_age: OverAge<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub epoch: ByEpoch<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub class: Class<UTXOCohortVecs<CoreCohortMetrics<M>>>,
    pub over_amount: OverAmount<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    pub amount_range: AmountRange<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    pub under_amount: UnderAmount<UTXOCohortVecs<MinimalCohortMetrics<M>>>,
    #[traversable(rename = "type")]
    pub type_: SpendableType<UTXOCohortVecs<TypeCohortMetrics<M>>>,
    pub profitability: ProfitabilityMetrics<M>,
    pub matured: AgeRange<AmountPerBlockCumulativeRolling<M>>,
    #[traversable(skip)]
    pub(super) caches: UTXOCohortsTransientState,
}

/// In-memory state that does NOT survive rollback.
#[derive(Clone, Default)]
pub(crate) struct UTXOCohortsTransientState {
    pub(super) fenwick: CostBasisFenwick,
    /// Cached partition_point positions for tick_tock boundary searches.
    /// Avoids O(log n) binary search per boundary per block; scans forward
    /// from last known position (typically O(1) per boundary).
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
        cached_starts: &Windows<&WindowStartVec>,
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
            cached_starts,
        };
        let all_supply = SupplyCore::forced_import(&all_cfg)?;

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
                    cached_starts,
                };
                let state = Some(Box::new(UTXOCohortState::new(states_path, &full_name)));
                Ok(UTXOCohortVecs::new(
                    state,
                    BasicCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let age_range = AgeRange::try_new(&basic_separate)?;

        let core_separate =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<CoreCohortMetrics>> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                    cached_starts,
                };
                let state = Some(Box::new(UTXOCohortState::new(states_path, &full_name)));
                Ok(UTXOCohortVecs::new(
                    state,
                    CoreCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let epoch = ByEpoch::try_new(&core_separate)?;
        let class = Class::try_new(&core_separate)?;

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
                    cached_starts,
                };
                let state = Some(Box::new(UTXOCohortState::new(states_path, &full_name)));
                Ok(UTXOCohortVecs::new(
                    state,
                    MinimalCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let amount_range = AmountRange::try_new(&minimal_separate)?;

        let type_separate =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<TypeCohortMetrics>> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                    cached_starts,
                };
                let state = Some(Box::new(UTXOCohortState::new(states_path, &full_name)));
                Ok(UTXOCohortVecs::new(
                    state,
                    TypeCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let type_ = SpendableType::try_new(&type_separate)?;

        // Phase 3: Import "all" cohort with pre-imported supply.
        let all = UTXOCohortVecs::new(
            None,
            AllCohortMetrics::forced_import_with_supply(&all_cfg, all_supply)?,
        );

        // Phase 3b: Import profitability metrics (derived from "all" during k-way merge).
        let profitability = ProfitabilityMetrics::forced_import(db, v, indexes, cached_starts)?;

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
                cached_starts,
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
                cached_starts,
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
                    cached_starts,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    CoreCohortMetrics::forced_import(&cfg)?,
                ))
            };

        // under_age: CoreCohortMetrics (no state, aggregates from age_range)
        let under_age = UnderAge::try_new(&core_no_state)?;

        // over_age: CoreCohortMetrics (no state, aggregates from age_range)
        let over_age = OverAge::try_new(&core_no_state)?;

        let minimal_no_state =
            |f: Filter, name: &'static str| -> Result<UTXOCohortVecs<MinimalCohortMetrics>> {
                let full_name = CohortContext::Utxo.full_name(&f, name);
                let cfg = ImportConfig {
                    db,
                    filter: &f,
                    full_name: &full_name,
                    version: v,
                    indexes,
                    cached_starts,
                };
                Ok(UTXOCohortVecs::new(
                    None,
                    MinimalCohortMetrics::forced_import(&cfg)?,
                ))
            };

        let under_amount = UnderAmount::try_new(&minimal_no_state)?;
        let over_amount = OverAmount::try_new(&minimal_no_state)?;

        let prefix = CohortContext::Utxo.prefix();
        let matured = AgeRange::try_new(&|_f: Filter,
                                          name: &'static str|
         -> Result<AmountPerBlockCumulativeRolling> {
            AmountPerBlockCumulativeRolling::forced_import(
                db,
                &format!("{prefix}_{name}_matured_supply"),
                v,
                indexes,
                cached_starts,
            )
        })?;

        Ok(Self {
            all,
            sth,
            lth,
            epoch,
            class,
            type_,
            under_age,
            over_age,
            age_range,
            amount_range,
            under_amount,
            over_amount,
            profitability,
            matured,
            caches: UTXOCohortsTransientState::default(),
        })
    }

    /// Reset in-memory caches that become stale after rollback.
    pub(crate) fn reset_caches(&mut self) {
        self.caches = UTXOCohortsTransientState::default();
    }

    /// Initialize the Fenwick tree from all age-range BTreeMaps.
    /// Call after state import when all pending maps have been drained.
    pub(crate) fn init_fenwick_if_needed(&mut self) {
        if self.caches.fenwick.is_initialized() {
            return;
        }
        let Self {
            sth,
            caches,
            age_range,
            ..
        } = self;
        caches
            .fenwick
            .compute_is_sth(&sth.metrics.filter, age_range.iter().map(|v| v.filter()));

        let maps: Vec<_> = age_range
            .iter()
            .enumerate()
            .filter_map(|(i, sub)| {
                let state = sub.state.as_ref()?;
                let map = state.cost_basis_map();
                if map.is_empty() {
                    return None;
                }
                Some((map, caches.fenwick.is_sth_at(i)))
            })
            .collect();
        caches.fenwick.bulk_init(maps.into_iter());
    }

    /// Apply pending deltas from all age-range cohorts to the Fenwick tree.
    /// Call after receive/send, before push_cohort_states.
    pub(crate) fn update_fenwick_from_pending(&mut self) {
        if !self.caches.fenwick.is_initialized() {
            return;
        }
        // Destructure to get separate borrows on caches and age_range
        let Self {
            caches, age_range, ..
        } = self;
        for (i, sub) in age_range.iter().enumerate() {
            if let Some(state) = sub.state.as_ref() {
                let is_sth = caches.fenwick.is_sth_at(i);
                state.for_each_cost_basis_pending(|&price, delta| {
                    caches.fenwick.apply_delta(price, delta, is_sth);
                });
            }
        }
    }

    /// Push maturation sats to the matured vecs for the given height.
    #[inline(always)]
    pub(crate) fn push_maturation(&mut self, matured: &AgeRange<Sats>) {
        for (v, &sats) in self.matured.iter_mut().zip(matured.iter()) {
            v.block.sats.push(sats);
        }
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

    /// Sequential mutable iterator over all separate (stateful) cohorts.
    /// Use instead of `par_iter_separate_mut` when per-item work is trivial.
    pub(crate) fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut dyn DynCohortVecs> {
        let Self {
            age_range,
            epoch,
            class,
            amount_range,
            type_,
            ..
        } = self;
        age_range
            .iter_mut()
            .map(|x| x as &mut dyn DynCohortVecs)
            .chain(epoch.iter_mut().map(|x| x as &mut dyn DynCohortVecs))
            .chain(class.iter_mut().map(|x| x as &mut dyn DynCohortVecs))
            .chain(amount_range.iter_mut().map(|x| x as &mut dyn DynCohortVecs))
            .chain(type_.iter_mut().map(|x| x as &mut dyn DynCohortVecs))
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
            under_age,
            over_age,
            over_amount,
            amount_range,
            under_amount,
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
                over_age.par_iter_mut().try_for_each(|vecs| {
                    let sources = filter_sources_from(ar.iter(), Some(&vecs.metrics.filter));
                    vecs.metrics.compute_from_base_sources(si, &sources, exit)
                })
            }),
            Box::new(|| {
                under_age.par_iter_mut().try_for_each(|vecs| {
                    let sources = filter_sources_from(ar.iter(), Some(&vecs.metrics.filter));
                    vecs.metrics.compute_from_base_sources(si, &sources, exit)
                })
            }),
            Box::new(|| {
                over_amount
                    .par_iter_mut()
                    .chain(under_amount.par_iter_mut())
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
            all.extend(
                self.under_age
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(
                self.over_age
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(
                self.over_amount
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
                self.under_amount
                    .iter_mut()
                    .map(|x| x as &mut dyn DynCohortVecs),
            );
            all.extend(self.type_.iter_mut().map(|x| x as &mut dyn DynCohortVecs));
            all.into_par_iter()
                .try_for_each(|v| v.compute_rest_part1(prices, starting_indexes, exit))?;
        }

        // Compute matured cumulative + cents from sats × price
        self.matured
            .par_iter_mut()
            .try_for_each(|v| v.compute_rest(starting_indexes.height, prices, exit))?;

        // Compute profitability supply cents and realized price
        self.profitability.compute(prices, starting_indexes, exit)?;

        Ok(())
    }

    /// Second phase of post-processing: compute relative metrics.
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        // Get under_1h value sources for adjusted computation (cloned to avoid borrow conflicts).
        let under_1h_value_created = self
            .age_range
            .under_1h
            .metrics
            .activity
            .transfer_volume
            .block
            .cents
            .read_only_clone();
        let under_1h_value_destroyed = self
            .age_range
            .under_1h
            .metrics
            .realized
            .sopr
            .value_destroyed
            .block
            .read_only_clone();

        // "all" cohort computed first (no all_supply_sats needed).
        self.all.metrics.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_market_cap,
            &under_1h_value_created,
            &under_1h_value_destroyed,
            exit,
        )?;

        // Clone all_supply_sats and all_utxo_count for non-all cohorts.
        let all_supply_sats = self.all.metrics.supply.total.sats.height.read_only_clone();
        let all_utxo_count = self
            .all
            .metrics
            .outputs
            .unspent_count
            .height
            .read_only_clone();

        // Destructure to allow parallel mutable access to independent fields.
        let Self {
            sth,
            lth,
            age_range,
            under_age,
            over_age,
            over_amount,
            amount_range,
            under_amount,
            epoch,
            class,
            type_,
            ..
        } = self;

        // All remaining groups run in parallel. Each closure owns an exclusive &mut
        // to its field and shares read-only references to common data.
        let vc = &under_1h_value_created;
        let vd = &under_1h_value_destroyed;
        let ss = &all_supply_sats;
        let au = &all_utxo_count;

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
                    au,
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
                    au,
                    exit,
                )
            }),
            Box::new(|| {
                age_range.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                under_age.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                over_age.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                over_amount.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                epoch.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                class.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                amount_range.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                under_amount.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
            }),
            Box::new(|| {
                type_.par_iter_mut().try_for_each(|v| {
                    v.metrics
                        .compute_rest_part2(prices, starting_indexes, ss, au, exit)
                })
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
        for v in self.under_age.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.over_age.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.over_amount.iter_mut() {
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
        for v in self.under_amount.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        for v in self.type_.iter_mut() {
            vecs.extend(v.metrics.collect_all_vecs_mut());
        }
        vecs.extend(self.profitability.collect_all_vecs_mut());
        for v in self.matured.iter_mut() {
            let inner = &mut v.inner;
            vecs.push(&mut inner.block.sats);
            vecs.push(&mut inner.block.cents);
            vecs.push(&mut inner.cumulative.sats.height);
            vecs.push(&mut inner.cumulative.cents.height);
        }
        vecs.into_par_iter()
    }

    /// Commit all states to disk (separate from vec writes for parallelization).
    pub(crate) fn commit_all_states(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.write_state(height, cleanup))
    }

    pub(crate) fn min_stateful_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_stateful_len()))
            .chain(
                self.matured
                    .iter()
                    .map(|v| Height::from(v.block.sats.len())),
            )
            .min()
            .unwrap_or_default()
            .min(Height::from(self.profitability.min_stateful_len()))
            .min(Height::from(self.all.min_stateful_len()))
            .min(Height::from(self.sth.min_stateful_len()))
            .min(Height::from(self.lth.min_stateful_len()))
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
        self.iter_separate_mut()
            .for_each(|v| v.reset_state_starting_height());
    }

    /// Reset cost_basis_data for all separate cohorts (called during fresh start).
    pub(crate) fn reset_separate_cost_basis_data(&mut self) -> Result<()> {
        self.iter_separate_mut()
            .try_for_each(|v| v.reset_cost_basis_data_if_needed())
    }

    /// Validate computed versions for all cohorts.
    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        // Validate separate cohorts
        self.iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;

        // Validate aggregate cohorts
        self.all.metrics.validate_computed_versions(base_version)?;
        self.sth.metrics.validate_computed_versions(base_version)?;
        self.lth.metrics.validate_computed_versions(base_version)?;
        for v in self.over_age.iter_mut() {
            v.metrics.validate_computed_versions(base_version)?;
        }
        for v in self.under_age.iter_mut() {
            v.metrics.validate_computed_versions(base_version)?;
        }
        Ok(())
    }

    /// Aggregate RealizedFull fields from age_range states and push to all/sth/lth.
    /// Called during the block loop after separate cohorts' push_state but before reset.
    pub(crate) fn push_overlapping(&mut self, height_price: Cents) {
        let Self {
            all,
            sth,
            lth,
            age_range,
            ..
        } = self;

        let sth_filter = &sth.metrics.filter;

        let mut all_acc = RealizedFullAccum::default();
        let mut sth_acc = RealizedFullAccum::default();
        let mut lth_acc = RealizedFullAccum::default();

        let mut all_ccap = (0u128, 0u128);
        let mut sth_ccap = (0u128, 0u128);
        let mut lth_ccap = (0u128, 0u128);

        for ar in age_range.iter_mut() {
            if let Some(state) = ar.state.as_mut() {
                all_acc.add(&state.realized);

                let u = state.compute_unrealized_state(height_price);
                all_ccap.0 += u.capitalized_cap_in_profit_raw;
                all_ccap.1 += u.capitalized_cap_in_loss_raw;

                if sth_filter.includes(&ar.metrics.filter) {
                    sth_acc.add(&state.realized);
                    sth_ccap.0 += u.capitalized_cap_in_profit_raw;
                    sth_ccap.1 += u.capitalized_cap_in_loss_raw;
                } else {
                    lth_acc.add(&state.realized);
                    lth_ccap.0 += u.capitalized_cap_in_profit_raw;
                    lth_ccap.1 += u.capitalized_cap_in_loss_raw;
                }
            }
        }

        all.metrics.realized.push_accum(&all_acc);
        sth.metrics.realized.push_accum(&sth_acc);
        lth.metrics.realized.push_accum(&lth_acc);

        all.metrics
            .unrealized
            .capitalized_cap_in_profit_raw
            .push(CentsSquaredSats::new(all_ccap.0));
        all.metrics
            .unrealized
            .capitalized_cap_in_loss_raw
            .push(CentsSquaredSats::new(all_ccap.1));
        sth.metrics
            .unrealized
            .capitalized_cap_in_profit_raw
            .push(CentsSquaredSats::new(sth_ccap.0));
        sth.metrics
            .unrealized
            .capitalized_cap_in_loss_raw
            .push(CentsSquaredSats::new(sth_ccap.1));
        lth.metrics
            .unrealized
            .capitalized_cap_in_profit_raw
            .push(CentsSquaredSats::new(lth_ccap.0));
        lth.metrics
            .unrealized
            .capitalized_cap_in_loss_raw
            .push(CentsSquaredSats::new(lth_ccap.1));
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
