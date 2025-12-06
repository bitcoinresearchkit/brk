use std::path::Path;

use brk_error::Result;
use brk_grouper::{
    AmountFilter, ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount,
    ByMaxAge, ByMinAge, BySpendableType, ByTerm, Filter, Filtered, StateLevel, Term, TimeFilter,
    UTXOGroups,
};
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, CheckedSub, DateIndex, Dollars, HalvingEpoch, Height, ONE_DAY_IN_SEC, OutputType,
    Sats, Timestamp, Version,
};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use vecdb::{Database, Exit, IterableVec, VecIndex};

use crate::{
    Indexes, indexes, price,
    stateful::{Flushable, HeightFlushable, r#trait::DynCohortVecs},
    states::{BlockState, Transacted},
    utils::OptionExt,
};

use super::{r#trait::CohortVecs, utxo_cohort};

const VERSION: Version = Version::new(0);

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct Vecs(UTXOGroups<utxo_cohort::Vecs>);

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
    ) -> Result<Self> {
        let v = version + VERSION + Version::ZERO;

        // Helper to create a cohort - booleans are now derived from filter
        let create = |filter: Filter, state_level: StateLevel| -> Result<utxo_cohort::Vecs> {
            utxo_cohort::Vecs::forced_import(
                db,
                filter,
                v,
                indexes,
                price,
                states_path,
                state_level,
            )
        };

        let full = |f: Filter| create(f, StateLevel::Full);
        let none = |f: Filter| create(f, StateLevel::None);

        Ok(Self(UTXOGroups {
            // Special case: all uses Version::ONE
            all: utxo_cohort::Vecs::forced_import(
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
                _1m_to_2m: full(Filter::Time(TimeFilter::Range(30..60)))?,
                _2m_to_3m: full(Filter::Time(TimeFilter::Range(60..90)))?,
                _3m_to_4m: full(Filter::Time(TimeFilter::Range(90..120)))?,
                _4m_to_5m: full(Filter::Time(TimeFilter::Range(120..150)))?,
                _5m_to_6m: full(Filter::Time(TimeFilter::Range(150..180)))?,
                _6m_to_1y: full(Filter::Time(TimeFilter::Range(180..365)))?,
                _1y_to_2y: full(Filter::Time(TimeFilter::Range(365..730)))?,
                _2y_to_3y: full(Filter::Time(TimeFilter::Range(730..1095)))?,
                _3y_to_4y: full(Filter::Time(TimeFilter::Range(1095..1460)))?,
                _4y_to_5y: full(Filter::Time(TimeFilter::Range(1460..1825)))?,
                _5y_to_6y: full(Filter::Time(TimeFilter::Range(1825..2190)))?,
                _6y_to_7y: full(Filter::Time(TimeFilter::Range(2190..2555)))?,
                _7y_to_8y: full(Filter::Time(TimeFilter::Range(2555..2920)))?,
                _8y_to_10y: full(Filter::Time(TimeFilter::Range(2920..3650)))?,
                _10y_to_12y: full(Filter::Time(TimeFilter::Range(3650..4380)))?,
                _12y_to_15y: full(Filter::Time(TimeFilter::Range(4380..5475)))?,
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

    pub fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;

        // Only blocks whose age % ONE_DAY >= threshold can cross a day boundary.
        // Saves 1 subtraction + 2 divisions per block vs computing days_old directly.
        let elapsed = (*timestamp).saturating_sub(*prev_timestamp);
        let threshold = ONE_DAY_IN_SEC.saturating_sub(elapsed);

        // Extract all mutable references upfront to avoid borrow checker issues
        // Use a single destructuring to get non-overlapping mutable borrows
        let UTXOGroups {
            all,
            term,
            age_range,
            ..
        } = &mut self.0;

        let mut vecs = age_range
            .iter_mut()
            .map(|v| (v.filter().clone(), &mut v.state))
            .collect::<Vec<_>>();

        // Collect aggregate cohorts' filter and p2a for age transitions
        let mut aggregate_p2a: Vec<(Filter, Option<&mut crate::PriceToAmount>)> = vec![
            (all.filter().clone(), all.price_to_amount.as_mut()),
            (
                term.short.filter().clone(),
                term.short.price_to_amount.as_mut(),
            ),
            (
                term.long.filter().clone(),
                term.long.price_to_amount.as_mut(),
            ),
        ];

        chain_state
            .iter()
            .filter(|block_state| {
                let age = (*prev_timestamp).saturating_sub(*block_state.timestamp);
                age % ONE_DAY_IN_SEC >= threshold
            })
            .for_each(|block_state| {
                let prev_days_old =
                    prev_timestamp.difference_in_days_between(block_state.timestamp);
                let days_old = timestamp.difference_in_days_between(block_state.timestamp);

                if prev_days_old == days_old {
                    return;
                }

                vecs.iter_mut().for_each(|(filter, state)| {
                    let is = filter.contains_time(days_old);
                    let was = filter.contains_time(prev_days_old);

                    if is && !was {
                        state
                            .as_mut()
                            .unwrap()
                            .increment(&block_state.supply, block_state.price);
                    } else if was && !is {
                        state
                            .as_mut()
                            .unwrap()
                            .decrement(&block_state.supply, block_state.price);
                    }
                });

                // Handle age transitions for aggregate cohorts' price_to_amount
                // Check which cohorts the UTXO was in vs is now in, and increment/decrement accordingly
                // Only process if there's remaining supply (like CohortState::increment/decrement do)
                if let Some(price) = block_state.price
                    && block_state.supply.value > Sats::ZERO
                {
                    aggregate_p2a.iter_mut().for_each(|(filter, p2a)| {
                        let is = filter.contains_time(days_old);
                        let was = filter.contains_time(prev_days_old);

                        if is && !was {
                            p2a.um().increment(price, &block_state.supply);
                        } else if was && !is {
                            p2a.um().decrement(price, &block_state.supply);
                        }
                    });
                }
            });
    }

    pub fn send(
        &mut self,
        height_to_sent: FxHashMap<Height, Transacted>,
        chain_state: &mut [BlockState],
    ) {
        // Extract all mutable references upfront to avoid borrow checker issues
        let UTXOGroups {
            all,
            term,
            age_range,
            epoch,
            type_,
            amount_range,
            ..
        } = &mut self.0;

        let mut time_based_vecs = age_range
            .iter_mut()
            .chain(epoch.iter_mut())
            .collect::<Vec<_>>();

        // Collect aggregate cohorts' filter and p2a for iteration
        let mut aggregate_p2a: Vec<(Filter, Option<&mut crate::PriceToAmount>)> = vec![
            (all.filter().clone(), all.price_to_amount.as_mut()),
            (
                term.short.filter().clone(),
                term.short.price_to_amount.as_mut(),
            ),
            (
                term.long.filter().clone(),
                term.long.price_to_amount.as_mut(),
            ),
        ];

        let last_block = chain_state.last().unwrap();
        let last_timestamp = last_block.timestamp;
        let current_price = last_block.price;

        let chain_state_len = chain_state.len();

        height_to_sent.into_iter().for_each(|(height, sent)| {
            chain_state[height.to_usize()].supply -= &sent.spendable_supply;

            let block_state = chain_state.get(height.to_usize()).unwrap();

            let prev_price = block_state.price;

            let blocks_old = chain_state_len - 1 - height.to_usize();

            let days_old = last_timestamp.difference_in_days_between(block_state.timestamp);
            let days_old_float =
                last_timestamp.difference_in_days_between_float(block_state.timestamp);

            let older_than_hour = last_timestamp
                .checked_sub(block_state.timestamp)
                .unwrap()
                .is_more_than_hour();

            time_based_vecs
                .iter_mut()
                .filter(|v| match v.filter() {
                    Filter::Time(TimeFilter::GreaterOrEqual(from)) => *from <= days_old,
                    Filter::Time(TimeFilter::LowerThan(to)) => *to > days_old,
                    Filter::Time(TimeFilter::Range(range)) => range.contains(&days_old),
                    Filter::Epoch(epoch) => *epoch == HalvingEpoch::from(height),
                    _ => unreachable!(),
                })
                .for_each(|vecs| {
                    vecs.state.um().send(
                        &sent.spendable_supply,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    );
                });

            sent.by_type
                .spendable
                .iter_typed()
                .for_each(|(output_type, supply_state)| {
                    type_.get_mut(output_type).state.um().send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    )
                });

            sent.by_size_group
                .iter_typed()
                .for_each(|(group, supply_state)| {
                    amount_range.get_mut(group).state.um().send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    );
                });

            // Update aggregate cohorts' price_to_amount using filter.contains_time()
            if let Some(prev_price) = prev_price {
                let supply_state = &sent.spendable_supply;
                if supply_state.value.is_not_zero() {
                    aggregate_p2a
                        .iter_mut()
                        .filter(|(f, _)| f.contains_time(days_old))
                        .map(|(_, p2a)| p2a)
                        .for_each(|p2a| {
                            p2a.um().decrement(prev_price, supply_state);
                        });
                }
            }
        });
    }

    pub fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>) {
        let supply_state = received.spendable_supply;

        [
            &mut self.0.age_range.up_to_1d,
            self.0.epoch.mut_vec_from_height(height),
        ]
        .into_iter()
        .for_each(|v| {
            v.state.um().receive(&supply_state, price);
        });

        // Update aggregate cohorts' price_to_amount
        // New UTXOs have days_old = 0, so use filter.contains_time(0) to check applicability
        if let Some(price) = price
            && supply_state.value.is_not_zero()
        {
            self.0
                .iter_aggregate_mut()
                .filter(|v| v.filter().contains_time(0))
                .for_each(|v| {
                    v.price_to_amount
                        .as_mut()
                        .unwrap()
                        .increment(price, &supply_state);
                });
        }

        self.type_.iter_mut().for_each(|vecs| {
            let output_type = match vecs.filter() {
                Filter::Type(output_type) => *output_type,
                _ => unreachable!(),
            };
            vecs.state
                .as_mut()
                .unwrap()
                .receive(received.by_type.get(output_type), price)
        });

        received
            .by_size_group
            .iter_typed()
            .for_each(|(group, supply_state)| {
                self.amount_range
                    .get_mut(group)
                    .state
                    .as_mut()
                    .unwrap()
                    .receive(supply_state, price);
            });
    }

    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_date_range = &self.0.age_range;
        let by_size_range = &self.0.amount_range;

        [(&mut self.0.all, by_date_range.iter().collect::<Vec<_>>())]
            .into_iter()
            .chain(self.0.min_age.iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_date_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.max_age.iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_date_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.term.iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_date_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.ge_amount.iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_size_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.lt_amount.iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_size_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .try_for_each(|(vecs, stateful)| {
                vecs.compute_from_stateful(starting_indexes, &stateful, exit)
            })
    }

    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.iter_mut()
            .try_for_each(|v| v.compute_rest_part1(indexes, price, starting_indexes, exit))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        height_to_realized_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.iter_mut().try_for_each(|v| {
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

    pub fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        // Flush stateful cohorts
        self.par_iter_separate_mut()
            .try_for_each(|v| v.safe_flush_stateful_vecs(height, exit))?;

        // Flush aggregate cohorts' price_to_amount and price_percentiles
        // Using traits ensures we can't forget to flush any field
        self.0.par_iter_aggregate_mut().try_for_each(|v| {
            v.price_to_amount.flush_at_height(height, exit)?;
            v.inner.price_percentiles.safe_write(exit)?;
            Ok(())
        })
    }

    /// Reset aggregate cohorts' price_to_amount when starting from scratch
    pub fn reset_aggregate_price_to_amount(&mut self) -> Result<()> {
        self.0.iter_aggregate_mut().try_for_each(|v| {
            v.price_to_amount.reset()
        })
    }

    /// Import aggregate cohorts' price_to_amount from disk when resuming from a checkpoint.
    /// Returns the height to start processing from (checkpoint_height + 1), matching the
    /// behavior of `common::import_state` for separate cohorts.
    ///
    /// Note: We don't check inner.min_height_vecs_len() for aggregate cohorts because their
    /// inner vecs (height_to_supply, etc.) are computed post-hoc by compute_overlapping_vecs,
    /// not maintained during the main processing loop.
    pub fn import_aggregate_price_to_amount(&mut self, height: Height) -> Result<Height> {
        // Match separate vecs behavior: decrement height to get prev_height
        let Some(mut prev_height) = height.decremented() else {
            // height is 0, return ZERO (caller will handle this)
            return Ok(Height::ZERO);
        };

        for v in self.0.iter_aggregate_mut() {
            // Using HeightFlushable trait - if price_to_amount is None, returns height unchanged
            prev_height = prev_height.min(v.price_to_amount.import_at_or_before(prev_height)?);
        }
        // Return prev_height + 1, matching separate vecs behavior
        Ok(prev_height.incremented())
    }

    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    /// Must be called after receive()/send() when price_to_amount is up to date.
    pub fn truncate_push_aggregate_percentiles(&mut self, height: Height) -> Result<()> {
        let age_range_data: Vec<_> = self
            .0
            .age_range
            .iter()
            .map(|sub| (sub.filter().clone(), sub.state.u().supply.value))
            .collect();

        let results: Vec<_> = self
            .0
            .par_iter_aggregate()
            .map(|v| {
                if v.price_to_amount.is_none() {
                    panic!();
                }
                let filter = v.filter().clone();
                let supply = age_range_data
                    .iter()
                    .filter(|(sub_filter, _)| filter.includes(sub_filter))
                    .map(|(_, value)| *value)
                    .fold(Sats::ZERO, |acc, v| acc + v);
                let percentiles = v.compute_percentile_prices_from_standalone(supply);
                (filter, percentiles)
            })
            .collect();

        // Push results sequentially (requires &mut)
        for (filter, percentiles) in results {
            let v = self
                .0
                .iter_aggregate_mut()
                .find(|v| v.filter() == &filter)
                .unwrap();

            if let Some(pp) = v.inner.price_percentiles.as_mut() {
                pp.truncate_push(height, &percentiles)?;
            }
        }

        Ok(())
    }
}
