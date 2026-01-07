use brk_error::Result;
use brk_types::StoredF32;
use vecdb::Exit;

use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub fn compute(&mut self, starting_indexes: &ComputeIndexes, exit: &Exit) -> Result<()> {
        // CAGR computed from returns (2y+ periods only)
        let price_returns_dca = self.price_returns.as_dca_period();
        for (cagr, returns, days) in self.cagr.zip_mut_with_period(&price_returns_dca) {
            cagr.compute_all(starting_indexes, exit, |v| {
                // KISS: dateindex is no longer Option
                v.compute_cagr(
                    starting_indexes.dateindex,
                    &returns.dateindex,
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // KISS: dateindex is no longer Option
        let _1d_price_returns_dateindex = &self.price_returns._1d.dateindex;

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
