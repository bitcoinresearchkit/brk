use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{price, utils::OptionExt, Indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = price.timeindexes_to_price_close.dateindex.u();

        self.price_1d_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 1, exit)?;
            Ok(())
        })?;

        self.price_1w_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 7, exit)?;
            Ok(())
        })?;

        self.price_1m_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 30, exit)?;
            Ok(())
        })?;

        self.price_3m_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 3 * 30, exit)?;
            Ok(())
        })?;

        self.price_6m_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 6 * 30, exit)?;
            Ok(())
        })?;

        self.price_1y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 365, exit)?;
            Ok(())
        })?;

        self.price_2y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 2 * 365, exit)?;
            Ok(())
        })?;

        self.price_3y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 3 * 365, exit)?;
            Ok(())
        })?;

        self.price_4y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 4 * 365, exit)?;
            Ok(())
        })?;

        self.price_5y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 5 * 365, exit)?;
            Ok(())
        })?;

        self.price_6y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 6 * 365, exit)?;
            Ok(())
        })?;

        self.price_8y_ago.compute_all(starting_indexes, exit, |v| {
            v.compute_previous_value(starting_indexes.dateindex, close, 8 * 365, exit)?;
            Ok(())
        })?;

        self.price_10y_ago
            .compute_all(starting_indexes, exit, |v| {
                v.compute_previous_value(starting_indexes.dateindex, close, 10 * 365, exit)?;
                Ok(())
            })?;

        // CAGR computed from returns
        self._2y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._2y_price_returns.dateindex.u(),
                2 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._3y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._3y_price_returns.dateindex.u(),
                3 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._4y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._4y_price_returns.dateindex.u(),
                4 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._5y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._5y_price_returns.dateindex.u(),
                5 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._6y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._6y_price_returns.dateindex.u(),
                6 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._8y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._8y_price_returns.dateindex.u(),
                8 * 365,
                exit,
            )?;
            Ok(())
        })?;
        self._10y_cagr.compute_all(starting_indexes, exit, |v| {
            v.compute_cagr(
                starting_indexes.dateindex,
                self._10y_price_returns.dateindex.u(),
                10 * 365,
                exit,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
