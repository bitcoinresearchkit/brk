use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = &prices.price.cents.height;

        for (sma, period) in [
            (&mut self.price_sma_1w, 7),
            (&mut self.price_sma_8d, 8),
            (&mut self.price_sma_13d, 13),
            (&mut self.price_sma_21d, 21),
            (&mut self.price_sma_1m, 30),
            (&mut self.price_sma_34d, 34),
            (&mut self.price_sma_55d, 55),
            (&mut self.price_sma_89d, 89),
            (&mut self.price_sma_111d, 111),
            (&mut self.price_sma_144d, 144),
            (&mut self.price_sma_200d, 200),
            (&mut self.price_sma_350d, 350),
            (&mut self.price_sma_1y, 365),
            (&mut self.price_sma_2y, 2 * 365),
            (&mut self.price_sma_200w, 200 * 7),
            (&mut self.price_sma_4y, 4 * 365),
        ] {
            let window_starts = blocks.count.start_vec(period);
            sma.compute_all(prices, starting_indexes, exit, |v| {
                v.compute_rolling_average(starting_indexes.height, window_starts, close, exit)?;
                Ok(())
            })?;
        }

        for (ema, period) in [
            (&mut self.price_ema_1w, 7),
            (&mut self.price_ema_8d, 8),
            (&mut self.price_ema_12d, 12),
            (&mut self.price_ema_13d, 13),
            (&mut self.price_ema_21d, 21),
            (&mut self.price_ema_26d, 26),
            (&mut self.price_ema_1m, 30),
            (&mut self.price_ema_34d, 34),
            (&mut self.price_ema_55d, 55),
            (&mut self.price_ema_89d, 89),
            (&mut self.price_ema_144d, 144),
            (&mut self.price_ema_200d, 200),
            (&mut self.price_ema_1y, 365),
            (&mut self.price_ema_2y, 2 * 365),
            (&mut self.price_ema_200w, 200 * 7),
            (&mut self.price_ema_4y, 4 * 365),
        ] {
            let window_starts = blocks.count.start_vec(period);
            ema.compute_all(prices, starting_indexes, exit, |v| {
                v.compute_rolling_ema(starting_indexes.height, window_starts, close, exit)?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
