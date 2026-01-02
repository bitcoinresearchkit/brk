use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{price, utils::OptionExt, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = price.usd.timeindexes_to_price_close.dateindex.u();

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

        Ok(())
    }
}
