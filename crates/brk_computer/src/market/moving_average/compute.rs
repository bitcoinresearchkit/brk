use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{price, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = &price.usd.timeindexes_to_price_close.dateindex;

        for (sma, period) in [
            (&mut self.indexes_to_price_1w_sma, 7),
            (&mut self.indexes_to_price_8d_sma, 8),
            (&mut self.indexes_to_price_13d_sma, 13),
            (&mut self.indexes_to_price_21d_sma, 21),
            (&mut self.indexes_to_price_1m_sma, 30),
            (&mut self.indexes_to_price_34d_sma, 34),
            (&mut self.indexes_to_price_55d_sma, 55),
            (&mut self.indexes_to_price_89d_sma, 89),
            (&mut self.indexes_to_price_111d_sma, 111),
            (&mut self.indexes_to_price_144d_sma, 144),
            (&mut self.indexes_to_price_200d_sma, 200),
            (&mut self.indexes_to_price_350d_sma, 350),
            (&mut self.indexes_to_price_1y_sma, 365),
            (&mut self.indexes_to_price_2y_sma, 2 * 365),
            (&mut self.indexes_to_price_200w_sma, 200 * 7),
            (&mut self.indexes_to_price_4y_sma, 4 * 365),
        ] {
            sma.compute_all(price, starting_indexes, exit, |v| {
                v.compute_sma(starting_indexes.dateindex, close, period, exit)?;
                Ok(())
            })?;
        }

        for (ema, period) in [
            (&mut self.indexes_to_price_1w_ema, 7),
            (&mut self.indexes_to_price_8d_ema, 8),
            (&mut self.indexes_to_price_12d_ema, 12),
            (&mut self.indexes_to_price_13d_ema, 13),
            (&mut self.indexes_to_price_21d_ema, 21),
            (&mut self.indexes_to_price_26d_ema, 26),
            (&mut self.indexes_to_price_1m_ema, 30),
            (&mut self.indexes_to_price_34d_ema, 34),
            (&mut self.indexes_to_price_55d_ema, 55),
            (&mut self.indexes_to_price_89d_ema, 89),
            (&mut self.indexes_to_price_144d_ema, 144),
            (&mut self.indexes_to_price_200d_ema, 200),
            (&mut self.indexes_to_price_1y_ema, 365),
            (&mut self.indexes_to_price_2y_ema, 2 * 365),
            (&mut self.indexes_to_price_200w_ema, 200 * 7),
            (&mut self.indexes_to_price_4y_ema, 4 * 365),
        ] {
            ema.compute_all(price, starting_indexes, exit, |v| {
                v.compute_ema(starting_indexes.dateindex, close, period, exit)?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
