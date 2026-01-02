use brk_error::Result;
use brk_types::StoredF32;
use vecdb::Exit;

use super::Vecs;
use crate::{utils::OptionExt, ComputeIndexes};

impl Vecs {
    pub fn compute(&mut self, starting_indexes: &ComputeIndexes, exit: &Exit) -> Result<()> {
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

        // Returns standard deviation (computed from 1d returns)
        let _1d_price_returns_dateindex = self._1d_price_returns.dateindex.u();

        self.indexes_to_1d_returns_1w_sd.compute_all(
            starting_indexes,
            exit,
            _1d_price_returns_dateindex,
        )?;
        self.indexes_to_1d_returns_1m_sd.compute_all(
            starting_indexes,
            exit,
            _1d_price_returns_dateindex,
        )?;
        self.indexes_to_1d_returns_1y_sd.compute_all(
            starting_indexes,
            exit,
            _1d_price_returns_dateindex,
        )?;

        // Downside returns: min(return, 0)
        self.dateindex_to_downside_returns.compute_transform(
            starting_indexes.dateindex,
            _1d_price_returns_dateindex,
            |(i, ret, ..)| (i, StoredF32::from((*ret).min(0.0))),
            exit,
        )?;

        // Downside deviation (SD of downside returns)
        self.indexes_to_downside_1w_sd.compute_all(
            starting_indexes,
            exit,
            &self.dateindex_to_downside_returns,
        )?;
        self.indexes_to_downside_1m_sd.compute_all(
            starting_indexes,
            exit,
            &self.dateindex_to_downside_returns,
        )?;
        self.indexes_to_downside_1y_sd.compute_all(
            starting_indexes,
            exit,
            &self.dateindex_to_downside_returns,
        )?;

        Ok(())
    }
}
