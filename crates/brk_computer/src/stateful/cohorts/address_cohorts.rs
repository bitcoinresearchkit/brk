//! Container for all Address cohorts organized by filter type.

use std::path::Path;

use brk_error::Result;
use brk_grouper::{
    AddressGroups, AmountFilter, ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount, Filter,
    Filtered,
};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Sats, Version};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{Database, Exit, IterableVec};

use crate::{Indexes, indexes, price, stateful::DynCohortVecs};

use super::{AddressCohortVecs, CohortVecs};

const VERSION: Version = Version::new(0);

/// All Address cohorts organized by filter type.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct AddressCohorts(AddressGroups<AddressCohortVecs>);

impl AddressCohorts {
    /// Import all Address cohorts from database.
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
    ) -> Result<Self> {
        let v = version + VERSION + Version::ZERO;

        // Helper to create a cohort - only amount_range cohorts have state
        let create = |filter: Filter, has_state: bool| -> Result<AddressCohortVecs> {
            let states_path = if has_state { Some(states_path) } else { None };
            AddressCohortVecs::forced_import(db, filter, v, indexes, price, states_path)
        };

        let full = |f: Filter| create(f, true);
        let none = |f: Filter| create(f, false);

        Ok(Self(AddressGroups {
            amount_range: ByAmountRange {
                _0sats: full(Filter::Amount(AmountFilter::LowerThan(Sats::_1)))?,
                _1sat_to_10sats: full(Filter::Amount(AmountFilter::Range(Sats::_1..Sats::_10)))?,
                _10sats_to_100sats: full(Filter::Amount(AmountFilter::Range(
                    Sats::_10..Sats::_100,
                )))?,
                _100sats_to_1k_sats: full(Filter::Amount(AmountFilter::Range(
                    Sats::_100..Sats::_1K,
                )))?,
                _1k_sats_to_10k_sats: full(Filter::Amount(AmountFilter::Range(
                    Sats::_1K..Sats::_10K,
                )))?,
                _10k_sats_to_100k_sats: full(Filter::Amount(AmountFilter::Range(
                    Sats::_10K..Sats::_100K,
                )))?,
                _100k_sats_to_1m_sats: full(Filter::Amount(AmountFilter::Range(
                    Sats::_100K..Sats::_1M,
                )))?,
                _1m_sats_to_10m_sats: full(Filter::Amount(AmountFilter::Range(
                    Sats::_1M..Sats::_10M,
                )))?,
                _10m_sats_to_1btc: full(Filter::Amount(AmountFilter::Range(
                    Sats::_10M..Sats::_1BTC,
                )))?,
                _1btc_to_10btc: full(Filter::Amount(AmountFilter::Range(
                    Sats::_1BTC..Sats::_10BTC,
                )))?,
                _10btc_to_100btc: full(Filter::Amount(AmountFilter::Range(
                    Sats::_10BTC..Sats::_100BTC,
                )))?,
                _100btc_to_1k_btc: full(Filter::Amount(AmountFilter::Range(
                    Sats::_100BTC..Sats::_1K_BTC,
                )))?,
                _1k_btc_to_10k_btc: full(Filter::Amount(AmountFilter::Range(
                    Sats::_1K_BTC..Sats::_10K_BTC,
                )))?,
                _10k_btc_to_100k_btc: full(Filter::Amount(AmountFilter::Range(
                    Sats::_10K_BTC..Sats::_100K_BTC,
                )))?,
                _100k_btc_or_more: full(Filter::Amount(AmountFilter::GreaterOrEqual(
                    Sats::_100K_BTC,
                )))?,
            },

            lt_amount: ByLowerThanAmount {
                _10sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_10)))?,
                _100sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_100)))?,
                _1k_sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_1K)))?,
                _10k_sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_10K)))?,
                _100k_sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_100K)))?,
                _1m_sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_1M)))?,
                _10m_sats: none(Filter::Amount(AmountFilter::LowerThan(Sats::_10M)))?,
                _1btc: none(Filter::Amount(AmountFilter::LowerThan(Sats::_1BTC)))?,
                _10btc: none(Filter::Amount(AmountFilter::LowerThan(Sats::_10BTC)))?,
                _100btc: none(Filter::Amount(AmountFilter::LowerThan(Sats::_100BTC)))?,
                _1k_btc: none(Filter::Amount(AmountFilter::LowerThan(Sats::_1K_BTC)))?,
                _10k_btc: none(Filter::Amount(AmountFilter::LowerThan(Sats::_10K_BTC)))?,
                _100k_btc: none(Filter::Amount(AmountFilter::LowerThan(Sats::_100K_BTC)))?,
            },

            ge_amount: ByGreatEqualAmount {
                _1sat: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_1)))?,
                _10sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_10)))?,
                _100sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_100)))?,
                _1k_sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_1K)))?,
                _10k_sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_10K)))?,
                _100k_sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_100K)))?,
                _1m_sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_1M)))?,
                _10m_sats: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_10M)))?,
                _1btc: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_1BTC)))?,
                _10btc: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_10BTC)))?,
                _100btc: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_100BTC)))?,
                _1k_btc: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_1K_BTC)))?,
                _10k_btc: none(Filter::Amount(AmountFilter::GreaterOrEqual(Sats::_10K_BTC)))?,
            },
        }))
    }

    /// Compute overlapping cohorts from component amount_range cohorts.
    ///
    /// For example, ">=1 BTC" cohort is computed from sum of amount_range cohorts that match.
    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_amount_range = &self.0.amount_range;

        // ge_amount cohorts computed from matching amount_range cohorts
        [
            self.0
                .ge_amount
                .par_iter_mut()
                .map(|vecs| {
                    let filter = vecs.filter().clone();
                    (
                        vecs,
                        by_amount_range
                            .iter()
                            .filter(|other| filter.includes(other.filter()))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            // lt_amount cohorts computed from matching amount_range cohorts
            self.0
                .lt_amount
                .par_iter_mut()
                .map(|vecs| {
                    let filter = vecs.filter().clone();
                    (
                        vecs,
                        by_amount_range
                            .iter()
                            .filter(|other| filter.includes(other.filter()))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        ]
        .into_iter()
        .flatten()
        .try_for_each(|(vecs, components)| {
            vecs.compute_from_stateful(starting_indexes, &components, exit)
        })
    }

    /// First phase of post-processing: compute index transforms.
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.par_iter_mut()
            .try_for_each(|v| v.compute_rest_part1(indexes, price, starting_indexes, exit))
    }

    /// Second phase of post-processing: compute relative metrics.
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2<S, D, HM, DM, HR, DR>(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &S,
        dateindex_to_supply: &D,
        height_to_market_cap: Option<&HM>,
        dateindex_to_market_cap: Option<&DM>,
        height_to_realized_cap: Option<&HR>,
        dateindex_to_realized_cap: Option<&DR>,
        exit: &Exit,
    ) -> Result<()>
    where
        S: IterableVec<Height, Bitcoin> + Sync,
        D: IterableVec<DateIndex, Bitcoin> + Sync,
        HM: IterableVec<Height, Dollars> + Sync,
        DM: IterableVec<DateIndex, Dollars> + Sync,
        HR: IterableVec<Height, Dollars> + Sync,
        DR: IterableVec<DateIndex, Dollars> + Sync,
    {
        self.0.par_iter_mut().try_for_each(|v| {
            v.compute_rest_part2(
                indexes,
                price,
                starting_indexes,
                height_to_supply,
                dateindex_to_supply,
                height_to_market_cap,
                dateindex_to_market_cap,
                height_to_realized_cap,
                dateindex_to_realized_cap,
                exit,
            )
        })
    }

    /// Write stateful vectors for separate cohorts.
    pub fn safe_write_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.safe_write_stateful_vecs(height, exit))
    }

    /// Get minimum height from all separate cohorts' height-indexed vectors.
    pub fn min_separate_height_vecs_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_height_vecs_len()))
            .min()
            .unwrap_or_default()
    }

    /// Import state for all separate cohorts at or before given height.
    /// Returns true if all imports succeeded and returned the expected height.
    pub fn import_separate_states(&mut self, height: Height) -> bool {
        self.par_iter_separate_mut()
            .map(|v| v.import_state(height).unwrap_or_default())
            .all(|h| h == height)
    }

    /// Reset state heights for all separate cohorts.
    pub fn reset_separate_state_heights(&mut self) {
        self.par_iter_separate_mut().for_each(|v| {
            v.reset_state_starting_height();
        });
    }

    /// Reset price_to_amount for all separate cohorts (called during fresh start).
    pub fn reset_separate_price_to_amount(&mut self) -> Result<()> {
        self.par_iter_separate_mut().try_for_each(|v| {
            if let Some(state) = v.state.as_mut() {
                state.reset_price_to_amount_if_needed()?;
            }
            Ok(())
        })
    }

    /// Validate computed versions for all separate cohorts.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))
    }
}
