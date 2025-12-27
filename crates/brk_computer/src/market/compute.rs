use std::thread;

use brk_error::Result;
use brk_types::{Date, DateIndex, Dollars, StoredF32, StoredU16};
use vecdb::{EagerVec, Exit, GenericStoredVec, PcoVec, TypedVecIterator, VecIndex};

use crate::{
    price,
    traits::{ComputeDCAAveragePriceViaLen, ComputeDCAStackViaLen, ComputeDrawdown},
    utils::OptionExt,
    Indexes,
};

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(price, starting_indexes, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_price_ath.compute_all_time_high(
            starting_indexes.height,
            &price.chainindexes_to_price_high.height,
            exit,
        )?;
        self.height_to_price_drawdown.compute_drawdown(
            starting_indexes.height,
            &price.chainindexes_to_price_close.height,
            &self.height_to_price_ath,
            exit,
        )?;

        self.indexes_to_price_ath
            .compute_all(starting_indexes, exit, |v| {
                v.compute_all_time_high(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_drawdown
            .compute_all(starting_indexes, exit, |v| {
                v.compute_drawdown(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.u(),
                    self.indexes_to_price_ath.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_since_price_ath
            .compute_all(starting_indexes, exit, |v| {
                let mut high_iter = price.timeindexes_to_price_high.dateindex.u().into_iter();
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_ath.dateindex.u(),
                    |(i, ath, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
                            } else {
                                StoredU16::default()
                            });
                        }
                        let days = if *high_iter.get_unwrap(i) == ath {
                            StoredU16::default()
                        } else {
                            prev.unwrap() + StoredU16::new(1)
                        };
                        prev.replace(days);
                        (i, days)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_max_days_between_price_aths
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_days_since_price_ath.dateindex.u(),
                    |(i, days, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
                            } else {
                                StoredU16::ZERO
                            });
                        }
                        let max = prev.unwrap().max(days);
                        prev.replace(max);
                        (i, max)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_max_years_between_price_aths
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_max_days_between_price_aths.dateindex.u(),
                    |(i, max, ..)| (i, StoredF32::from(*max as f64 / 365.0)),
                    exit,
                )?;
                Ok(())
            })?;

        [
            (1, &mut self.price_1d_ago, &mut self._1d_price_returns, None),
            (7, &mut self.price_1w_ago, &mut self._1w_price_returns, None),
            (
                30,
                &mut self.price_1m_ago,
                &mut self._1m_price_returns,
                None,
            ),
            (
                3 * 30,
                &mut self.price_3m_ago,
                &mut self._3m_price_returns,
                None,
            ),
            (
                6 * 30,
                &mut self.price_6m_ago,
                &mut self._6m_price_returns,
                None,
            ),
            (
                365,
                &mut self.price_1y_ago,
                &mut self._1y_price_returns,
                None,
            ),
            (
                2 * 365,
                &mut self.price_2y_ago,
                &mut self._2y_price_returns,
                Some(&mut self._2y_cagr),
            ),
            (
                3 * 365,
                &mut self.price_3y_ago,
                &mut self._3y_price_returns,
                Some(&mut self._3y_cagr),
            ),
            (
                4 * 365,
                &mut self.price_4y_ago,
                &mut self._4y_price_returns,
                Some(&mut self._4y_cagr),
            ),
            (
                5 * 365,
                &mut self.price_5y_ago,
                &mut self._5y_price_returns,
                Some(&mut self._5y_cagr),
            ),
            (
                6 * 365,
                &mut self.price_6y_ago,
                &mut self._6y_price_returns,
                Some(&mut self._6y_cagr),
            ),
            (
                8 * 365,
                &mut self.price_8y_ago,
                &mut self._8y_price_returns,
                Some(&mut self._8y_cagr),
            ),
            (
                10 * 365,
                &mut self.price_10y_ago,
                &mut self._10y_price_returns,
                Some(&mut self._10y_cagr),
            ),
        ]
        .into_iter()
        .try_for_each(|(days, ago, returns, cagr)| -> Result<()> {
            ago.compute_all(starting_indexes, exit, |v| {
                v.compute_previous_value(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.u(),
                    days,
                    exit,
                )?;
                Ok(())
            })?;

            returns.compute_all(starting_indexes, exit, |v| {
                v.compute_percentage_change(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.u(),
                    days,
                    exit,
                )?;
                Ok(())
            })?;

            if let Some(cagr) = cagr {
                cagr.compute_all(starting_indexes, exit, |v| {
                    v.compute_cagr(
                        starting_indexes.dateindex,
                        returns.dateindex.u(),
                        days,
                        exit,
                    )?;
                    Ok(())
                })?;
            }

            Ok(())
        })?;

        [
            (
                7,
                &mut self._1w_dca_stack,
                &mut self._1w_dca_avg_price,
                &mut self._1w_dca_returns,
                None,
            ),
            (
                30,
                &mut self._1m_dca_stack,
                &mut self._1m_dca_avg_price,
                &mut self._1m_dca_returns,
                None,
            ),
            (
                3 * 30,
                &mut self._3m_dca_stack,
                &mut self._3m_dca_avg_price,
                &mut self._3m_dca_returns,
                None,
            ),
            (
                6 * 30,
                &mut self._6m_dca_stack,
                &mut self._6m_dca_avg_price,
                &mut self._6m_dca_returns,
                None,
            ),
            (
                365,
                &mut self._1y_dca_stack,
                &mut self._1y_dca_avg_price,
                &mut self._1y_dca_returns,
                None,
            ),
            (
                2 * 365,
                &mut self._2y_dca_stack,
                &mut self._2y_dca_avg_price,
                &mut self._2y_dca_returns,
                Some(&mut self._2y_dca_cagr),
            ),
            (
                3 * 365,
                &mut self._3y_dca_stack,
                &mut self._3y_dca_avg_price,
                &mut self._3y_dca_returns,
                Some(&mut self._3y_dca_cagr),
            ),
            (
                4 * 365,
                &mut self._4y_dca_stack,
                &mut self._4y_dca_avg_price,
                &mut self._4y_dca_returns,
                Some(&mut self._4y_dca_cagr),
            ),
            (
                5 * 365,
                &mut self._5y_dca_stack,
                &mut self._5y_dca_avg_price,
                &mut self._5y_dca_returns,
                Some(&mut self._5y_dca_cagr),
            ),
            (
                6 * 365,
                &mut self._6y_dca_stack,
                &mut self._6y_dca_avg_price,
                &mut self._6y_dca_returns,
                Some(&mut self._6y_dca_cagr),
            ),
            (
                8 * 365,
                &mut self._8y_dca_stack,
                &mut self._8y_dca_avg_price,
                &mut self._8y_dca_returns,
                Some(&mut self._8y_dca_cagr),
            ),
            (
                10 * 365,
                &mut self._10y_dca_stack,
                &mut self._10y_dca_avg_price,
                &mut self._10y_dca_returns,
                Some(&mut self._10y_dca_cagr),
            ),
        ]
        .into_iter()
        .try_for_each(
            |(days, dca_stack, dca_avg_price, dca_returns, dca_cagr)| -> Result<()> {
                dca_stack.compute_all(starting_indexes, exit, |v| {
                    v.compute_dca_stack_via_len(
                        starting_indexes.dateindex,
                        price.timeindexes_to_price_close.dateindex.u(),
                        days,
                        exit,
                    )?;
                    Ok(())
                })?;

                dca_avg_price.compute_all(starting_indexes, exit, |v| {
                    v.compute_dca_avg_price_via_len(
                        starting_indexes.dateindex,
                        dca_stack.dateindex.u(),
                        days,
                        exit,
                    )?;
                    Ok(())
                })?;

                dca_returns.compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage_difference(
                        starting_indexes.dateindex,
                        price.timeindexes_to_price_close.dateindex.u(),
                        dca_avg_price.dateindex.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

                if let Some(dca_cagr) = dca_cagr {
                    dca_cagr.compute_all(starting_indexes, exit, |v| {
                        v.compute_cagr(
                            starting_indexes.dateindex,
                            dca_returns.dateindex.u(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    })?;
                }

                Ok(())
            },
        )?;

        [
            (
                2015,
                &mut self.dca_class_2015_avg_price,
                &mut self.dca_class_2015_returns,
                &mut self.dca_class_2015_stack,
            ),
            (
                2016,
                &mut self.dca_class_2016_avg_price,
                &mut self.dca_class_2016_returns,
                &mut self.dca_class_2016_stack,
            ),
            (
                2017,
                &mut self.dca_class_2017_avg_price,
                &mut self.dca_class_2017_returns,
                &mut self.dca_class_2017_stack,
            ),
            (
                2018,
                &mut self.dca_class_2018_avg_price,
                &mut self.dca_class_2018_returns,
                &mut self.dca_class_2018_stack,
            ),
            (
                2019,
                &mut self.dca_class_2019_avg_price,
                &mut self.dca_class_2019_returns,
                &mut self.dca_class_2019_stack,
            ),
            (
                2020,
                &mut self.dca_class_2020_avg_price,
                &mut self.dca_class_2020_returns,
                &mut self.dca_class_2020_stack,
            ),
            (
                2021,
                &mut self.dca_class_2021_avg_price,
                &mut self.dca_class_2021_returns,
                &mut self.dca_class_2021_stack,
            ),
            (
                2022,
                &mut self.dca_class_2022_avg_price,
                &mut self.dca_class_2022_returns,
                &mut self.dca_class_2022_stack,
            ),
            (
                2023,
                &mut self.dca_class_2023_avg_price,
                &mut self.dca_class_2023_returns,
                &mut self.dca_class_2023_stack,
            ),
            (
                2024,
                &mut self.dca_class_2024_avg_price,
                &mut self.dca_class_2024_returns,
                &mut self.dca_class_2024_stack,
            ),
            (
                2025,
                &mut self.dca_class_2025_avg_price,
                &mut self.dca_class_2025_returns,
                &mut self.dca_class_2025_stack,
            ),
        ]
        .into_iter()
        .try_for_each(|(year, avg_price, returns, stack)| -> Result<()> {
            let dateindex = DateIndex::try_from(Date::new(year, 1, 1)).unwrap();

            stack.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_from(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.u(),
                    dateindex,
                    exit,
                )?;
                Ok(())
            })?;

            avg_price.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_from(
                    starting_indexes.dateindex,
                    stack.dateindex.u(),
                    dateindex,
                    exit,
                )?;
                Ok(())
            })?;

            returns.compute_all(starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.u(),
                    avg_price.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

            Ok(())
        })?;

        thread::scope(|s| -> Result<()> {
            [
                (
                    &mut self.indexes_to_price_1w_sma,
                    &mut self.indexes_to_price_1w_ema,
                    7,
                ),
                (
                    &mut self.indexes_to_price_8d_sma,
                    &mut self.indexes_to_price_8d_ema,
                    8,
                ),
                (
                    &mut self.indexes_to_price_13d_sma,
                    &mut self.indexes_to_price_13d_ema,
                    13,
                ),
                (
                    &mut self.indexes_to_price_21d_sma,
                    &mut self.indexes_to_price_21d_ema,
                    21,
                ),
                (
                    &mut self.indexes_to_price_1m_sma,
                    &mut self.indexes_to_price_1m_ema,
                    30,
                ),
                (
                    &mut self.indexes_to_price_34d_sma,
                    &mut self.indexes_to_price_34d_ema,
                    34,
                ),
                (
                    &mut self.indexes_to_price_55d_sma,
                    &mut self.indexes_to_price_55d_ema,
                    55,
                ),
                (
                    &mut self.indexes_to_price_89d_sma,
                    &mut self.indexes_to_price_89d_ema,
                    89,
                ),
                (
                    &mut self.indexes_to_price_144d_sma,
                    &mut self.indexes_to_price_144d_ema,
                    144,
                ),
                (
                    &mut self.indexes_to_price_200d_sma,
                    &mut self.indexes_to_price_200d_ema,
                    200,
                ),
                (
                    &mut self.indexes_to_price_1y_sma,
                    &mut self.indexes_to_price_1y_ema,
                    365,
                ),
                (
                    &mut self.indexes_to_price_2y_sma,
                    &mut self.indexes_to_price_2y_ema,
                    2 * 365,
                ),
                (
                    &mut self.indexes_to_price_200w_sma,
                    &mut self.indexes_to_price_200w_ema,
                    200 * 7,
                ),
                (
                    &mut self.indexes_to_price_4y_sma,
                    &mut self.indexes_to_price_4y_ema,
                    4 * 365,
                ),
            ]
            .into_iter()
            .for_each(|(sma, ema, days)| {
                s.spawn(move || -> Result<()> {
                    sma.compute_all(price, starting_indexes, exit, |v| {
                        v.compute_sma(
                            starting_indexes.dateindex,
                            price.timeindexes_to_price_close.dateindex.u(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    })?;

                    ema.compute_all(price, starting_indexes, exit, |v| {
                        v.compute_ema(
                            starting_indexes.dateindex,
                            price.timeindexes_to_price_close.dateindex.u(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    })
                });
            });
            Ok(())
        })?;

        self.indexes_to_price_200d_sma_x0_8
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_200d_sma
                        .price
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, v * 0.8),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_200d_sma_x2_4
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_200d_sma
                        .price
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, v * 2.4),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1d_returns_1w_sd.compute_all(
            starting_indexes,
            exit,
            self._1d_price_returns.dateindex.u(),
            None as Option<&EagerVec<PcoVec<DateIndex, Dollars>>>,
        )?;
        self.indexes_to_1d_returns_1m_sd.compute_all(
            starting_indexes,
            exit,
            self._1d_price_returns.dateindex.u(),
            None as Option<&EagerVec<PcoVec<DateIndex, Dollars>>>,
        )?;
        self.indexes_to_1d_returns_1y_sd.compute_all(
            starting_indexes,
            exit,
            self._1d_price_returns.dateindex.u(),
            None as Option<&EagerVec<PcoVec<DateIndex, Dollars>>>,
        )?;

        self.indexes_to_price_1w_volatility
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_1d_returns_1w_sd
                        .sd
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, (*v * 7.0_f32.sqrt()).into()),
                    exit,
                )?;
                Ok(())
            })?;
        self.indexes_to_price_1m_volatility
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_1d_returns_1m_sd
                        .sd
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, (*v * 30.0_f32.sqrt()).into()),
                    exit,
                )?;
                Ok(())
            })?;
        self.indexes_to_price_1y_volatility
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_1d_returns_1y_sd
                        .sd
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, (*v * 365.0_f32.sqrt()).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.dateindex_to_price_true_range.compute_transform3(
            starting_indexes.dateindex,
            price.timeindexes_to_price_open.dateindex.u(),
            price.timeindexes_to_price_high.dateindex.u(),
            price.timeindexes_to_price_low.dateindex.u(),
            |(i, open, high, low, ..)| {
                let high_min_low = **high - **low;
                let high_min_open = (**high - **open).abs();
                let low_min_open = (**low - **open).abs();
                (i, high_min_low.max(high_min_open).max(low_min_open).into())
            },
            exit,
        )?;

        self.dateindex_to_price_true_range_2w_sum.compute_sum(
            starting_indexes.dateindex,
            &self.dateindex_to_price_true_range,
            14,
            exit,
        )?;

        self.indexes_to_price_1w_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.u(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1w_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.u(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_2w_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.u(),
                    14,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_2w_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.u(),
                    14,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1m_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.u(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1m_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.u(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1y_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.u(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1y_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.u(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_2w_choppiness_index
            .compute_all(starting_indexes, exit, |v| {
                let n = 14;
                let log10n = (n as f32).log10();
                v.compute_transform3(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_true_range_2w_sum,
                    self.indexes_to_price_2w_max.dateindex.u(),
                    self.indexes_to_price_2w_min.dateindex.u(),
                    |(i, tr_sum, max, min, ..)| {
                        (
                            i,
                            StoredF32::from(
                                100.0 * (*tr_sum / (*max - *min) as f32).log10() / log10n,
                            ),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
