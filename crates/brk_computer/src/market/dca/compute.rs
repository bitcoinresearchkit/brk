use brk_error::Result;
use brk_types::{Date, DateIndex};
use vecdb::Exit;

use super::Vecs;
use crate::{
    price,
    traits::{ComputeDCAAveragePriceViaLen, ComputeDCAStackViaLen},
    utils::OptionExt,
    ComputeIndexes,
};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = price.timeindexes_to_price_close.dateindex.u();

        // DCA by period - stack and avg_price
        self._1w_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 7, exit)?;
                Ok(())
            })?;
        self._1w_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._1w_dca_stack.dateindex.u(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self._1m_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 30, exit)?;
                Ok(())
            })?;
        self._1m_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._1m_dca_stack.dateindex.u(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self._3m_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 3 * 30, exit)?;
                Ok(())
            })?;
        self._3m_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._3m_dca_stack.dateindex.u(),
                    3 * 30,
                    exit,
                )?;
                Ok(())
            })?;

        self._6m_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 6 * 30, exit)?;
                Ok(())
            })?;
        self._6m_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._6m_dca_stack.dateindex.u(),
                    6 * 30,
                    exit,
                )?;
                Ok(())
            })?;

        self._1y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 365, exit)?;
                Ok(())
            })?;
        self._1y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._1y_dca_stack.dateindex.u(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self._2y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 2 * 365, exit)?;
                Ok(())
            })?;
        self._2y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._2y_dca_stack.dateindex.u(),
                    2 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        self._3y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 3 * 365, exit)?;
                Ok(())
            })?;
        self._3y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._3y_dca_stack.dateindex.u(),
                    3 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        self._4y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 4 * 365, exit)?;
                Ok(())
            })?;
        self._4y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._4y_dca_stack.dateindex.u(),
                    4 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        self._5y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 5 * 365, exit)?;
                Ok(())
            })?;
        self._5y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._5y_dca_stack.dateindex.u(),
                    5 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        self._6y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 6 * 365, exit)?;
                Ok(())
            })?;
        self._6y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._6y_dca_stack.dateindex.u(),
                    6 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        self._8y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 8 * 365, exit)?;
                Ok(())
            })?;
        self._8y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._8y_dca_stack.dateindex.u(),
                    8 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        self._10y_dca_stack
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, 10 * 365, exit)?;
                Ok(())
            })?;
        self._10y_dca_avg_price
            .compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    self._10y_dca_stack.dateindex.u(),
                    10 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        // DCA by period - CAGR (computed from returns)
        self._2y_dca_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._2y_dca_returns.dateindex.u(),
                2 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._3y_dca_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._3y_dca_returns.dateindex.u(),
                3 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._4y_dca_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._4y_dca_returns.dateindex.u(),
                4 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._5y_dca_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._5y_dca_returns.dateindex.u(),
                5 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._6y_dca_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._6y_dca_returns.dateindex.u(),
                6 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._8y_dca_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._8y_dca_returns.dateindex.u(),
                8 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._10y_dca_cagr
            .compute_all(starting_indexes, exit, |v| {
                v.compute_cagr(
                    starting_indexes.dateindex,
                    self._10y_dca_returns.dateindex.u(),
                    10 * 365,
                    exit,
                )?;
                Ok(())
            })?;

        // DCA by year class - stack and avg_price
        // Each year class computes DCA from Jan 1 of that year
        [
            (
                2025,
                &mut self.dca_class_2025_stack,
                &mut self.dca_class_2025_avg_price,
            ),
            (
                2024,
                &mut self.dca_class_2024_stack,
                &mut self.dca_class_2024_avg_price,
            ),
            (
                2023,
                &mut self.dca_class_2023_stack,
                &mut self.dca_class_2023_avg_price,
            ),
            (
                2022,
                &mut self.dca_class_2022_stack,
                &mut self.dca_class_2022_avg_price,
            ),
            (
                2021,
                &mut self.dca_class_2021_stack,
                &mut self.dca_class_2021_avg_price,
            ),
            (
                2020,
                &mut self.dca_class_2020_stack,
                &mut self.dca_class_2020_avg_price,
            ),
            (
                2019,
                &mut self.dca_class_2019_stack,
                &mut self.dca_class_2019_avg_price,
            ),
            (
                2018,
                &mut self.dca_class_2018_stack,
                &mut self.dca_class_2018_avg_price,
            ),
            (
                2017,
                &mut self.dca_class_2017_stack,
                &mut self.dca_class_2017_avg_price,
            ),
            (
                2016,
                &mut self.dca_class_2016_stack,
                &mut self.dca_class_2016_avg_price,
            ),
            (
                2015,
                &mut self.dca_class_2015_stack,
                &mut self.dca_class_2015_avg_price,
            ),
        ]
        .into_iter()
        .try_for_each(|(year, stack, avg_price)| -> Result<()> {
            let dateindex = DateIndex::try_from(Date::new(year, 1, 1)).unwrap();

            stack.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_from(starting_indexes.dateindex, close, dateindex, exit)?;
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

            Ok(())
        })?;

        Ok(())
    }
}
