//! Compute methods for Vecs.
//!
//! This module contains methods for post-processing computations:
//! - `compute_from_stateful`: Compute aggregate cohort values from separate cohorts
//! - `compute_rest_part1`: First phase of computed metrics
//! - `compute_rest_part2`: Second phase of computed metrics

use brk_error::Result;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, StoredF64, StoredU64};
use vecdb::{Exit, IterableVec, TypedVecIterator};

use crate::{Indexes, indexes, price, utils::OptionExt};

use super::Vecs;

impl Vecs {
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_supply.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_supply)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_utxo_count.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_utxo_count)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_sent.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_sent)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_satblocks_destroyed.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_satblocks_destroyed)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_satdays_destroyed.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_satdays_destroyed)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;

        if let Some(height_to_realized_cap) = &mut self.height_to_realized_cap {
            height_to_realized_cap.compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_realized_cap.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;

            self.height_to_min_price_paid.um().compute_min_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_min_price_paid.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_max_price_paid.um().compute_max_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_max_price_paid.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_realized_profit.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_realized_profit.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_realized_loss.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_realized_loss.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_value_created.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_value_created.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_value_destroyed.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_value_destroyed.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_supply_in_profit.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_supply_in_profit.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_supply_in_loss.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_supply_in_loss.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_unrealized_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_unrealized_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_unrealized_loss.um().compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_unrealized_loss.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.dateindex_to_supply_in_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_in_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_supply_in_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_in_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_unrealized_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_unrealized_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_unrealized_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_unrealized_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_min_price_paid.um().compute_min_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_min_price_paid.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;
            self.height_to_max_price_paid.um().compute_max_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_max_price_paid.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .um()
                    .compute_sum_of_others(
                        starting_indexes.height,
                        others
                            .iter()
                            .map(|v| {
                                v.height_to_adjusted_value_created
                                    .as_ref()
                                    .unwrap_or(v.height_to_value_created.u())
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        exit,
                    )?;
                self.height_to_adjusted_value_destroyed
                    .um()
                    .compute_sum_of_others(
                        starting_indexes.height,
                        others
                            .iter()
                            .map(|v| {
                                v.height_to_adjusted_value_destroyed
                                    .as_ref()
                                    .unwrap_or(v.height_to_value_destroyed.u())
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        exit,
                    )?;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_supply_value.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_supply),
        )?;

        self.indexes_to_supply
            .compute_all(price, starting_indexes, exit, |v| {
                let mut dateindex_to_height_count_iter =
                    indexes.dateindex_to_height_count.into_iter();
                let mut height_to_supply_iter = self.height_to_supply.into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(i, height, ..)| {
                        let count = dateindex_to_height_count_iter.get_unwrap(i);
                        if count == StoredU64::default() {
                            unreachable!()
                        }
                        let supply = height_to_supply_iter.get_unwrap(height + (*count - 1));
                        (i, supply)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_utxo_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_utxo_count),
        )?;

        self.height_to_supply_half_value
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_supply,
                    |(h, v, ..)| (h, v / 2),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_supply_half
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_supply.sats.dateindex.u(),
                    |(i, sats, ..)| (i, sats / 2),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_sent.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_sent),
        )?;

        self.indexes_to_coinblocks_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_satblocks_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_coindays_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_satdays_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
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
        if let Some(v) = self.indexes_to_supply_rel_to_circulating_supply.as_mut() {
            v.compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage(
                    starting_indexes.height,
                    &self.height_to_supply_value.bitcoin,
                    height_to_supply,
                    exit,
                )?;
                Ok(())
            })?;
        }

        if let Some(indexes_to_realized_cap) = self.indexes_to_realized_cap.as_mut() {
            let height_to_market_cap = height_to_market_cap.unwrap();
            let dateindex_to_market_cap = dateindex_to_market_cap.unwrap();

            indexes_to_realized_cap.compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_cap.u()),
            )?;

            self.indexes_to_realized_price.um().compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.height_to_realized_cap.u(),
                        &self.height_to_supply_value.bitcoin,
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_realized_price_extra.um().compute_rest(
                price.u(),
                starting_indexes,
                exit,
                Some(self.indexes_to_realized_price.u().dateindex.unwrap_last()),
            )?;

            self.indexes_to_realized_profit.um().compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_profit.u()),
            )?;

            self.indexes_to_realized_loss.um().compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_loss.u()),
            )?;

            self.indexes_to_neg_realized_loss.um().compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_transform(
                        starting_indexes.height,
                        self.height_to_realized_loss.u(),
                        |(i, v, ..)| (i, v * -1_i64),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_value_created.um().compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_value_created.u()),
            )?;

            self.indexes_to_value_destroyed.um().compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_value_destroyed.u()),
            )?;

            self.indexes_to_realized_cap_30d_delta.um().compute_all(
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_change(
                        starting_indexes.dateindex,
                        self.indexes_to_realized_cap.u().dateindex.unwrap_last(),
                        30,
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_net_realized_pnl.um().compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_subtract(
                        starting_indexes.height,
                        self.height_to_realized_profit.u(),
                        self.height_to_realized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_realized_value.um().compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_add(
                        starting_indexes.height,
                        self.height_to_realized_profit.u(),
                        self.height_to_realized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.dateindex_to_sopr.um().compute_divide(
                starting_indexes.dateindex,
                self.indexes_to_value_created.u().dateindex.unwrap_sum(),
                self.indexes_to_value_destroyed.u().dateindex.unwrap_sum(),
                exit,
            )?;

            self.dateindex_to_sopr_7d_ema.um().compute_ema(
                starting_indexes.dateindex,
                self.dateindex_to_sopr.u(),
                7,
                exit,
            )?;

            self.dateindex_to_sopr_30d_ema.um().compute_ema(
                starting_indexes.dateindex,
                self.dateindex_to_sopr.u(),
                30,
                exit,
            )?;

            self.dateindex_to_sell_side_risk_ratio
                .um()
                .compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_value.u().dateindex.unwrap_sum(),
                    self.indexes_to_realized_cap.u().dateindex.unwrap_last(),
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio_7d_ema
                .um()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sell_side_risk_ratio.u(),
                    7,
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio_30d_ema
                .um()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sell_side_risk_ratio.u(),
                    30,
                    exit,
                )?;

            self.indexes_to_supply_in_profit.um().compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.dateindex_to_supply_in_profit.u()),
            )?;
            self.indexes_to_supply_in_loss.um().compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.dateindex_to_supply_in_loss.u()),
            )?;
            self.indexes_to_unrealized_profit.um().compute_rest(
                starting_indexes,
                exit,
                Some(self.dateindex_to_unrealized_profit.u()),
            )?;
            self.indexes_to_unrealized_loss.um().compute_rest(
                starting_indexes,
                exit,
                Some(self.dateindex_to_unrealized_loss.u()),
            )?;
            self.height_to_total_unrealized_pnl.um().compute_add(
                starting_indexes.height,
                self.height_to_unrealized_profit.u(),
                self.height_to_unrealized_loss.u(),
                exit,
            )?;
            self.indexes_to_total_unrealized_pnl.um().compute_all(
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_add(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.u(),
                        self.dateindex_to_unrealized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;
            self.height_to_total_realized_pnl.um().compute_add(
                starting_indexes.height,
                self.height_to_realized_profit.u(),
                self.height_to_realized_loss.u(),
                exit,
            )?;
            self.indexes_to_total_realized_pnl
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.dateindex,
                        self.indexes_to_realized_profit.u().dateindex.unwrap_sum(),
                        self.indexes_to_realized_loss.u().dateindex.unwrap_sum(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_min_price_paid.um().compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_min_price_paid.u()),
            )?;
            self.indexes_to_max_price_paid.um().compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_max_price_paid.u()),
            )?;

            self.height_to_neg_unrealized_loss.um().compute_transform(
                starting_indexes.height,
                self.height_to_unrealized_loss.u(),
                |(h, v, ..)| (h, v * -1_i64),
                exit,
            )?;
            self.indexes_to_neg_unrealized_loss
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_transform(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_loss.u(),
                        |(h, v, ..)| (h, v * -1_i64),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_net_unrealized_pnl.um().compute_subtract(
                starting_indexes.height,
                self.height_to_unrealized_profit.u(),
                self.height_to_unrealized_loss.u(),
                exit,
            )?;

            self.indexes_to_net_unrealized_pnl
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_subtract(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.u(),
                        self.dateindex_to_unrealized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_unrealized_profit_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_unrealized_loss_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_unrealized_loss.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_neg_unrealized_loss_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_neg_unrealized_loss.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_net_unrealized_pnl_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_net_unrealized_pnl.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.indexes_to_unrealized_profit_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_unrealized_loss_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_loss.u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_neg_unrealized_loss_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_neg_unrealized_loss.u().dateindex.u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_net_unrealized_pnl_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_unrealized_pnl.u().dateindex.u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;

            if self
                .height_to_unrealized_profit_rel_to_own_market_cap
                .is_some()
            {
                self.height_to_unrealized_profit_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_profit.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.height_to_unrealized_loss_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_loss.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.height_to_neg_unrealized_loss_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_neg_unrealized_loss.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.height_to_net_unrealized_pnl_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_net_unrealized_pnl.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.indexes_to_unrealized_profit_rel_to_own_market_cap
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.u(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_unrealized_loss_rel_to_own_market_cap
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.u(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_neg_unrealized_loss_rel_to_own_market_cap
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_neg_unrealized_loss
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_net_unrealized_pnl_rel_to_own_market_cap
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_net_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
            }

            if self
                .height_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                .is_some()
            {
                self.height_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_profit.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.height_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_loss.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_neg_unrealized_loss.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_net_unrealized_pnl.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.u(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.u(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_neg_unrealized_loss
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_net_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
            }

            self.indexes_to_realized_profit_rel_to_realized_cap
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.height_to_realized_profit.u(),
                        *height_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_loss_rel_to_realized_cap
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.height_to_realized_loss.u(),
                        *height_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_rel_to_realized_cap
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.indexes_to_net_realized_pnl.u().height.u(),
                        *height_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.height_to_supply_in_loss_value.um().compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.height_to_supply_in_loss.u()),
            )?;
            self.height_to_supply_in_profit_value.um().compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.height_to_supply_in_profit.u()),
            )?;
            self.height_to_supply_in_loss_rel_to_own_supply
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    &self.height_to_supply_in_loss_value.u().bitcoin,
                    &self.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.height_to_supply_in_profit_rel_to_own_supply
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    &self.height_to_supply_in_profit_value.u().bitcoin,
                    &self.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.indexes_to_supply_in_loss_rel_to_own_supply
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_supply_in_loss.u().bitcoin.dateindex.u(),
                        self.indexes_to_supply.bitcoin.dateindex.u(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_supply_in_profit_rel_to_own_supply
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_supply_in_profit.u().bitcoin.dateindex.u(),
                        self.indexes_to_supply.bitcoin.dateindex.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_change(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl
                            .u()
                            .dateindex
                            .unwrap_cumulative(),
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .u()
                            .dateindex
                            .u(),
                        *dateindex_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .u()
                            .dateindex
                            .u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;

            if self
                .height_to_supply_in_profit_rel_to_circulating_supply
                .as_mut()
                .is_some()
            {
                self.height_to_supply_in_loss_rel_to_circulating_supply
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        &self.height_to_supply_in_loss_value.u().bitcoin,
                        height_to_supply,
                        exit,
                    )?;
                self.height_to_supply_in_profit_rel_to_circulating_supply
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        &self.height_to_supply_in_profit_value.u().bitcoin,
                        height_to_supply,
                        exit,
                    )?;
                self.indexes_to_supply_in_loss_rel_to_circulating_supply
                    .um()
                    .compute_all(starting_indexes, exit, |v| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_supply_in_loss
                                .as_ref()
                                .unwrap()
                                .bitcoin
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            dateindex_to_supply,
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_supply_in_profit_rel_to_circulating_supply
                    .um()
                    .compute_all(starting_indexes, exit, |v| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_supply_in_profit
                                .as_ref()
                                .unwrap()
                                .bitcoin
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            dateindex_to_supply,
                            exit,
                        )?;
                        Ok(())
                    })?;
            }

            if self.indexes_to_adjusted_value_created.is_some() {
                self.indexes_to_adjusted_value_created.um().compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_adjusted_value_created.u()),
                )?;

                self.indexes_to_adjusted_value_destroyed.um().compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_adjusted_value_destroyed.u()),
                )?;

                self.dateindex_to_adjusted_sopr.um().compute_divide(
                    starting_indexes.dateindex,
                    self.indexes_to_adjusted_value_created
                        .u()
                        .dateindex
                        .unwrap_sum(),
                    self.indexes_to_adjusted_value_destroyed
                        .u()
                        .dateindex
                        .unwrap_sum(),
                    exit,
                )?;

                self.dateindex_to_adjusted_sopr_7d_ema.um().compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_adjusted_sopr.u(),
                    7,
                    exit,
                )?;

                self.dateindex_to_adjusted_sopr_30d_ema.um().compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_adjusted_sopr.u(),
                    30,
                    exit,
                )?;
            }

            if let Some(indexes_to_realized_cap_rel_to_own_market_cap) =
                self.indexes_to_realized_cap_rel_to_own_market_cap.as_mut()
            {
                indexes_to_realized_cap_rel_to_own_market_cap.compute_all(
                    indexes,
                    starting_indexes,
                    exit,
                    |v| {
                        v.compute_percentage(
                            starting_indexes.height,
                            self.height_to_realized_cap.u(),
                            self.height_to_supply_value.dollars.u(),
                            exit,
                        )?;
                        Ok(())
                    },
                )?;
            }
        }

        if let Some(dateindex_to_realized_profit_to_loss_ratio) =
            self.dateindex_to_realized_profit_to_loss_ratio.as_mut()
        {
            dateindex_to_realized_profit_to_loss_ratio.compute_divide(
                starting_indexes.dateindex,
                self.indexes_to_realized_profit.u().dateindex.unwrap_sum(),
                self.indexes_to_realized_loss.u().dateindex.unwrap_sum(),
                exit,
            )?;
        }

        Ok(())
    }
}
