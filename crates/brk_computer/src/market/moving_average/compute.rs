use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = &prices.price.cents.height;

        for (sma, period) in [
            (&mut self.price_1w_sma, 7),
            (&mut self.price_8d_sma, 8),
            (&mut self.price_13d_sma, 13),
            (&mut self.price_21d_sma, 21),
            (&mut self.price_1m_sma, 30),
            (&mut self.price_34d_sma, 34),
            (&mut self.price_55d_sma, 55),
            (&mut self.price_89d_sma, 89),
            (&mut self.price_111d_sma, 111),
            (&mut self.price_144d_sma, 144),
            (&mut self.price_200d_sma, 200),
            (&mut self.price_350d_sma, 350),
            (&mut self.price_1y_sma, 365),
            (&mut self.price_2y_sma, 2 * 365),
            (&mut self.price_200w_sma, 200 * 7),
            (&mut self.price_4y_sma, 4 * 365),
        ] {
            let window_starts = blocks.count.start_vec(period);
            sma.compute_all(blocks, prices, starting_indexes, exit, |v| {
                v.compute_rolling_average(starting_indexes.height, window_starts, close, exit)?;
                Ok(())
            })?;
        }

        for (ema, period) in [
            (&mut self.price_1w_ema, 7),
            (&mut self.price_8d_ema, 8),
            (&mut self.price_12d_ema, 12),
            (&mut self.price_13d_ema, 13),
            (&mut self.price_21d_ema, 21),
            (&mut self.price_26d_ema, 26),
            (&mut self.price_1m_ema, 30),
            (&mut self.price_34d_ema, 34),
            (&mut self.price_55d_ema, 55),
            (&mut self.price_89d_ema, 89),
            (&mut self.price_144d_ema, 144),
            (&mut self.price_200d_ema, 200),
            (&mut self.price_1y_ema, 365),
            (&mut self.price_2y_ema, 2 * 365),
            (&mut self.price_200w_ema, 200 * 7),
            (&mut self.price_4y_ema, 4 * 365),
        ] {
            let window_starts = blocks.count.start_vec(period);
            ema.compute_all(blocks, prices, starting_indexes, exit, |v| {
                v.compute_rolling_ema(starting_indexes.height, window_starts, close, exit)?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
