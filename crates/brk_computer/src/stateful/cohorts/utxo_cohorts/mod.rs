//! Container for all UTXO cohorts organized by filter type.

mod receive;
mod send;
mod tick_tock;

use std::path::Path;

use brk_error::Result;
use brk_grouper::{
    AmountFilter, ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount,
    ByMaxAge, ByMinAge, BySpendableType, ByTerm, Filter, Filtered, StateLevel, Term, TimeFilter,
    UTXOGroups,
};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, HalvingEpoch, Height, OutputType, Sats, Version};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{Database, Exit, IterableVec};

use crate::{Indexes, indexes, price, stateful::DynCohortVecs};

use super::{CohortVecs, HeightFlushable, UTXOCohortVecs};

const VERSION: Version = Version::new(0);

/// All UTXO cohorts organized by filter type.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct UTXOCohorts(pub(crate) UTXOGroups<UTXOCohortVecs>);

impl UTXOCohorts {
    /// Import all UTXO cohorts from database.
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
    ) -> Result<Self> {
        let v = version + VERSION + Version::ZERO;

        let create = |filter: Filter, state_level: StateLevel| -> Result<UTXOCohortVecs> {
            UTXOCohortVecs::forced_import(db, filter, v, indexes, price, states_path, state_level)
        };

        let full = |f: Filter| create(f, StateLevel::Full);
        let none = |f: Filter| create(f, StateLevel::None);

        Ok(Self(UTXOGroups {
            all: UTXOCohortVecs::forced_import(
                db,
                Filter::All,
                version + VERSION + Version::ONE,
                indexes,
                price,
                states_path,
                StateLevel::PriceOnly,
            )?,

            term: ByTerm {
                short: create(Filter::Term(Term::Sth), StateLevel::PriceOnly)?,
                long: create(Filter::Term(Term::Lth), StateLevel::PriceOnly)?,
            },

            epoch: ByEpoch {
                _0: full(Filter::Epoch(HalvingEpoch::new(0)))?,
                _1: full(Filter::Epoch(HalvingEpoch::new(1)))?,
                _2: full(Filter::Epoch(HalvingEpoch::new(2)))?,
                _3: full(Filter::Epoch(HalvingEpoch::new(3)))?,
                _4: full(Filter::Epoch(HalvingEpoch::new(4)))?,
            },

            type_: BySpendableType {
                p2pk65: full(Filter::Type(OutputType::P2PK65))?,
                p2pk33: full(Filter::Type(OutputType::P2PK33))?,
                p2pkh: full(Filter::Type(OutputType::P2PKH))?,
                p2sh: full(Filter::Type(OutputType::P2SH))?,
                p2wpkh: full(Filter::Type(OutputType::P2WPKH))?,
                p2wsh: full(Filter::Type(OutputType::P2WSH))?,
                p2tr: full(Filter::Type(OutputType::P2TR))?,
                p2a: full(Filter::Type(OutputType::P2A))?,
                p2ms: full(Filter::Type(OutputType::P2MS))?,
                empty: full(Filter::Type(OutputType::Empty))?,
                unknown: full(Filter::Type(OutputType::Unknown))?,
            },

            max_age: ByMaxAge {
                _1w: none(Filter::Time(TimeFilter::LowerThan(7)))?,
                _1m: none(Filter::Time(TimeFilter::LowerThan(30)))?,
                _2m: none(Filter::Time(TimeFilter::LowerThan(2 * 30)))?,
                _3m: none(Filter::Time(TimeFilter::LowerThan(3 * 30)))?,
                _4m: none(Filter::Time(TimeFilter::LowerThan(4 * 30)))?,
                _5m: none(Filter::Time(TimeFilter::LowerThan(5 * 30)))?,
                _6m: none(Filter::Time(TimeFilter::LowerThan(6 * 30)))?,
                _1y: none(Filter::Time(TimeFilter::LowerThan(365)))?,
                _2y: none(Filter::Time(TimeFilter::LowerThan(2 * 365)))?,
                _3y: none(Filter::Time(TimeFilter::LowerThan(3 * 365)))?,
                _4y: none(Filter::Time(TimeFilter::LowerThan(4 * 365)))?,
                _5y: none(Filter::Time(TimeFilter::LowerThan(5 * 365)))?,
                _6y: none(Filter::Time(TimeFilter::LowerThan(6 * 365)))?,
                _7y: none(Filter::Time(TimeFilter::LowerThan(7 * 365)))?,
                _8y: none(Filter::Time(TimeFilter::LowerThan(8 * 365)))?,
                _10y: none(Filter::Time(TimeFilter::LowerThan(10 * 365)))?,
                _12y: none(Filter::Time(TimeFilter::LowerThan(12 * 365)))?,
                _15y: none(Filter::Time(TimeFilter::LowerThan(15 * 365)))?,
            },

            min_age: ByMinAge {
                _1d: none(Filter::Time(TimeFilter::GreaterOrEqual(1)))?,
                _1w: none(Filter::Time(TimeFilter::GreaterOrEqual(7)))?,
                _1m: none(Filter::Time(TimeFilter::GreaterOrEqual(30)))?,
                _2m: none(Filter::Time(TimeFilter::GreaterOrEqual(2 * 30)))?,
                _3m: none(Filter::Time(TimeFilter::GreaterOrEqual(3 * 30)))?,
                _4m: none(Filter::Time(TimeFilter::GreaterOrEqual(4 * 30)))?,
                _5m: none(Filter::Time(TimeFilter::GreaterOrEqual(5 * 30)))?,
                _6m: none(Filter::Time(TimeFilter::GreaterOrEqual(6 * 30)))?,
                _1y: none(Filter::Time(TimeFilter::GreaterOrEqual(365)))?,
                _2y: none(Filter::Time(TimeFilter::GreaterOrEqual(2 * 365)))?,
                _3y: none(Filter::Time(TimeFilter::GreaterOrEqual(3 * 365)))?,
                _4y: none(Filter::Time(TimeFilter::GreaterOrEqual(4 * 365)))?,
                _5y: none(Filter::Time(TimeFilter::GreaterOrEqual(5 * 365)))?,
                _6y: none(Filter::Time(TimeFilter::GreaterOrEqual(6 * 365)))?,
                _7y: none(Filter::Time(TimeFilter::GreaterOrEqual(7 * 365)))?,
                _8y: none(Filter::Time(TimeFilter::GreaterOrEqual(8 * 365)))?,
                _10y: none(Filter::Time(TimeFilter::GreaterOrEqual(10 * 365)))?,
                _12y: none(Filter::Time(TimeFilter::GreaterOrEqual(12 * 365)))?,
            },

            age_range: ByAgeRange {
                up_to_1d: full(Filter::Time(TimeFilter::Range(0..1)))?,
                _1d_to_1w: full(Filter::Time(TimeFilter::Range(1..7)))?,
                _1w_to_1m: full(Filter::Time(TimeFilter::Range(7..30)))?,
                _1m_to_2m: full(Filter::Time(TimeFilter::Range(30..2 * 30)))?,
                _2m_to_3m: full(Filter::Time(TimeFilter::Range(2 * 30..3 * 30)))?,
                _3m_to_4m: full(Filter::Time(TimeFilter::Range(3 * 30..4 * 30)))?,
                _4m_to_5m: full(Filter::Time(TimeFilter::Range(4 * 30..5 * 30)))?,
                _5m_to_6m: full(Filter::Time(TimeFilter::Range(5 * 30..6 * 30)))?,
                _6m_to_1y: full(Filter::Time(TimeFilter::Range(6 * 30..365)))?,
                _1y_to_2y: full(Filter::Time(TimeFilter::Range(365..2 * 365)))?,
                _2y_to_3y: full(Filter::Time(TimeFilter::Range(2 * 365..3 * 365)))?,
                _3y_to_4y: full(Filter::Time(TimeFilter::Range(3 * 365..4 * 365)))?,
                _4y_to_5y: full(Filter::Time(TimeFilter::Range(4 * 365..5 * 365)))?,
                _5y_to_6y: full(Filter::Time(TimeFilter::Range(5 * 365..6 * 365)))?,
                _6y_to_7y: full(Filter::Time(TimeFilter::Range(6 * 365..7 * 365)))?,
                _7y_to_8y: full(Filter::Time(TimeFilter::Range(7 * 365..8 * 365)))?,
                _8y_to_10y: full(Filter::Time(TimeFilter::Range(8 * 365..10 * 365)))?,
                _10y_to_12y: full(Filter::Time(TimeFilter::Range(10 * 365..12 * 365)))?,
                _12y_to_15y: full(Filter::Time(TimeFilter::Range(12 * 365..15 * 365)))?,
                from_15y: full(Filter::Time(TimeFilter::GreaterOrEqual(15 * 365)))?,
            },

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

    /// Compute overlapping cohorts from component age/amount range cohorts.
    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_age_range = &self.0.age_range;
        let by_amount_range = &self.0.amount_range;

        [(&mut self.0.all, by_age_range.iter().collect::<Vec<_>>())]
            .into_par_iter()
            .chain(self.0.min_age.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_age_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.max_age.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_age_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.term.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_age_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.ge_amount.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_amount_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.lt_amount.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_amount_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
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
        self.par_iter_mut().try_for_each(|v| {
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

    /// Flush stateful vectors for separate cohorts.
    pub fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.safe_flush_stateful_vecs(height, exit))?;

        self.0
            .par_iter_aggregate_mut()
            .try_for_each(|v| v.price_to_amount.flush_at_height(height, exit))
    }

    /// Reset aggregate cohorts' price_to_amount for fresh start.
    pub fn reset_aggregate_price_to_amount(&mut self) -> Result<()> {
        self.0
            .iter_aggregate_mut()
            .try_for_each(|v| v.price_to_amount.reset())
    }

    /// Import aggregate cohorts' price_to_amount when resuming from checkpoint.
    pub fn import_aggregate_price_to_amount(&mut self, height: Height) -> Result<Height> {
        let Some(mut prev_height) = height.decremented() else {
            return Ok(Height::ZERO);
        };

        for v in self.0.iter_aggregate_mut() {
            prev_height = prev_height.min(v.price_to_amount.import_at_or_before(prev_height)?);
        }

        Ok(prev_height.incremented())
    }

    /// Get minimum height from all separate cohorts' height-indexed vectors.
    pub fn min_separate_height_vecs_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_height_vecs_len()))
            .min()
            .unwrap_or_default()
    }

    /// Import state for all separate cohorts at given height.
    pub fn import_separate_states(&mut self, height: Height) {
        self.par_iter_separate_mut().for_each(|v| {
            let _ = v.import_state(height);
        });
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

    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    /// Must be called after receive()/send() when price_to_amount is up to date.
    pub fn truncate_push_aggregate_percentiles(&mut self, height: Height) -> Result<()> {
        // Collect supply values from age_range cohorts
        let age_range_data: Vec<_> = self
            .0
            .age_range
            .iter()
            .map(|sub| {
                (
                    sub.filter().clone(),
                    sub.state
                        .as_ref()
                        .map(|s| s.supply.value)
                        .unwrap_or(Sats::ZERO),
                )
            })
            .collect();

        // Compute percentiles for each aggregate cohort in parallel
        let results: Vec<_> = self
            .0
            .par_iter_aggregate()
            .filter_map(|v| {
                v.price_to_amount.as_ref()?;
                let filter = v.filter().clone();
                let supply = age_range_data
                    .iter()
                    .filter(|(sub_filter, _)| filter.includes(sub_filter))
                    .map(|(_, value)| *value)
                    .fold(Sats::ZERO, |acc, v| acc + v);
                let percentiles = v.compute_percentile_prices_from_standalone(supply);
                Some((filter, percentiles))
            })
            .collect();

        // Push results sequentially (requires &mut)
        for (filter, percentiles) in results {
            let v = self
                .0
                .iter_aggregate_mut()
                .find(|v| v.filter() == &filter)
                .unwrap();

            if let Some(pp) = v
                .metrics
                .price_paid
                .as_mut()
                .and_then(|p| p.price_percentiles.as_mut())
            {
                pp.truncate_push(height, &percentiles)?;
            }
        }

        Ok(())
    }

    /// Validate computed versions for all separate cohorts.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))
    }
}
