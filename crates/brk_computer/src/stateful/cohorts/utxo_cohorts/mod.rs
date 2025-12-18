//! Container for all UTXO cohorts organized by filter type.

mod receive;
mod send;
mod tick_tock;

use std::path::Path;

use brk_error::Result;
use brk_grouper::{
    AmountFilter, ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount,
    ByMaxAge, ByMinAge, BySpendableType, ByTerm, ByYear, Filter, Filtered, StateLevel, Term,
    TimeFilter, UTXOGroups,
};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, HalvingEpoch, Height, OutputType, Sats, Version, Year};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{Database, Exit, IterableVec};

use crate::{
    Indexes,
    grouped::{PERCENTILES, PERCENTILES_LEN},
    indexes, price,
    stateful::DynCohortVecs,
};

use super::{CohortVecs, UTXOCohortVecs};

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

            year: ByYear {
                _2009: full(Filter::Year(Year::new(2009)))?,
                _2010: full(Filter::Year(Year::new(2010)))?,
                _2011: full(Filter::Year(Year::new(2011)))?,
                _2012: full(Filter::Year(Year::new(2012)))?,
                _2013: full(Filter::Year(Year::new(2013)))?,
                _2014: full(Filter::Year(Year::new(2014)))?,
                _2015: full(Filter::Year(Year::new(2015)))?,
                _2016: full(Filter::Year(Year::new(2016)))?,
                _2017: full(Filter::Year(Year::new(2017)))?,
                _2018: full(Filter::Year(Year::new(2018)))?,
                _2019: full(Filter::Year(Year::new(2019)))?,
                _2020: full(Filter::Year(Year::new(2020)))?,
                _2021: full(Filter::Year(Year::new(2021)))?,
                _2022: full(Filter::Year(Year::new(2022)))?,
                _2023: full(Filter::Year(Year::new(2023)))?,
                _2024: full(Filter::Year(Year::new(2024)))?,
                _2025: full(Filter::Year(Year::new(2025)))?,
                _2026: full(Filter::Year(Year::new(2026)))?,
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

    /// Write stateful vectors for separate and aggregate cohorts.
    pub fn write_stateful_vecs(&mut self, height: Height) -> Result<()> {
        // Flush separate cohorts (includes metrics + state)
        self.par_iter_separate_mut()
            .try_for_each(|v| v.write_stateful_vecs(height))?;

        // Write aggregate cohorts' metrics (including price_percentiles)
        // Note: aggregate cohorts no longer maintain price_to_amount state
        for v in self.0.iter_aggregate_mut() {
            v.metrics.write()?;
        }
        Ok(())
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

    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    /// Computes on-demand by merging age_range cohorts' price_to_amount data.
    /// This avoids maintaining redundant aggregate price_to_amount maps.
    pub fn truncate_push_aggregate_percentiles(&mut self, dateindex: DateIndex) -> Result<()> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        // Collect (filter, supply, price_to_amount as Vec) from age_range cohorts
        let age_range_data: Vec<_> = self
            .0
            .age_range
            .iter()
            .filter_map(|sub| {
                let state = sub.state.as_ref()?;
                let entries: Vec<(Dollars, Sats)> = state
                    .price_to_amount_iter()?
                    .map(|(&p, &a)| (p, a))
                    .collect();
                Some((sub.filter().clone(), state.supply.value, entries))
            })
            .collect();

        // Compute percentiles for each aggregate filter
        for aggregate in self.0.iter_aggregate_mut() {
            let filter = aggregate.filter().clone();

            // Get price_percentiles storage, skip if not configured
            let Some(pp) = aggregate
                .metrics
                .price_paid
                .as_mut()
                .and_then(|p| p.price_percentiles.as_mut())
            else {
                continue;
            };

            // Collect relevant cohort data for this aggregate
            let relevant: Vec<_> = age_range_data
                .iter()
                .filter(|(sub_filter, _, _)| filter.includes(sub_filter))
                .collect();

            // Calculate total supply
            let total_supply: u64 = relevant.iter().map(|(_, s, _)| u64::from(*s)).sum();

            if total_supply == 0 {
                pp.truncate_push(dateindex, &[Dollars::NAN; PERCENTILES_LEN])?;
                continue;
            }

            // K-way merge using min-heap: O(n log k) where k = number of cohorts
            // Each heap entry: (price, amount, cohort_idx, entry_idx)
            let mut heap: BinaryHeap<Reverse<(Dollars, usize, usize)>> = BinaryHeap::new();

            // Initialize heap with first entry from each cohort
            for (cohort_idx, (_, _, entries)) in relevant.iter().enumerate() {
                if !entries.is_empty() {
                    heap.push(Reverse((entries[0].0, cohort_idx, 0)));
                }
            }

            let targets = PERCENTILES.map(|p| total_supply * u64::from(p) / 100);
            let mut result = [Dollars::NAN; PERCENTILES_LEN];
            let mut accumulated = 0u64;
            let mut pct_idx = 0;
            let mut current_price: Option<Dollars> = None;
            let mut amount_at_price = 0u64;

            while let Some(Reverse((price, cohort_idx, entry_idx))) = heap.pop() {
                let (_, _, entries) = relevant[cohort_idx];
                let (_, amount) = entries[entry_idx];

                // If price changed, finalize previous price
                if let Some(current_price) = current_price
                    && current_price != price
                {
                    accumulated += amount_at_price;

                    while pct_idx < PERCENTILES_LEN && accumulated >= targets[pct_idx] {
                        result[pct_idx] = current_price;
                        pct_idx += 1;
                    }

                    if pct_idx >= PERCENTILES_LEN {
                        break;
                    }

                    amount_at_price = 0;
                }

                current_price = Some(price);
                amount_at_price += u64::from(amount);

                // Push next entry from this cohort
                let next_idx = entry_idx + 1;
                if next_idx < entries.len() {
                    heap.push(Reverse((entries[next_idx].0, cohort_idx, next_idx)));
                }
            }

            // Finalize last price
            if let Some(price) = current_price {
                accumulated += amount_at_price;
                while pct_idx < PERCENTILES_LEN && accumulated >= targets[pct_idx] {
                    result[pct_idx] = price;
                    pct_idx += 1;
                }
            }

            pp.truncate_push(dateindex, &result)?;
        }

        Ok(())
    }

    /// Validate computed versions for all cohorts (separate and aggregate).
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        // Validate separate cohorts
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;

        // Validate aggregate cohorts' price_percentiles
        for v in self.0.iter_aggregate_mut() {
            v.validate_computed_versions(base_version)?;
        }

        Ok(())
    }
}
