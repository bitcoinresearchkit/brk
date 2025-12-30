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
        let close = price.timeindexes_to_price_close.dateindex.u();

        // SMAs
        self.indexes_to_price_1w_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 7, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_8d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 8, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_13d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 13, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_21d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 21, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1m_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 30, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_34d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 34, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_55d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 55, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_89d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 89, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_144d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 144, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_200d_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 200, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1y_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 365, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_2y_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 2 * 365, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_200w_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 200 * 7, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_4y_sma
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, 4 * 365, exit)?;
                Ok(())
            })?;

        // EMAs
        self.indexes_to_price_1w_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 7, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_8d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 8, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_13d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 13, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_21d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 21, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1m_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 30, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_34d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 34, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_55d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 55, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_89d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 89, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_144d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 144, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_200d_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 200, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1y_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 365, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_2y_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 2 * 365, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_200w_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 200 * 7, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_4y_ema
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, 4 * 365, exit)?;
                Ok(())
            })?;

        Ok(())
    }
}
