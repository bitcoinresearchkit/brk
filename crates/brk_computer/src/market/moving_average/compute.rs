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
        let close = &prices.cached_spot_cents;

        for (sma, period) in [
            (&mut self.sma._1w, 7),
            (&mut self.sma._8d, 8),
            (&mut self.sma._13d, 13),
            (&mut self.sma._21d, 21),
            (&mut self.sma._1m, 30),
            (&mut self.sma._34d, 34),
            (&mut self.sma._55d, 55),
            (&mut self.sma._89d, 89),
            (&mut self.sma._111d, 111),
            (&mut self.sma._144d, 144),
            (&mut self.sma._200d, 200),
            (&mut self.sma._350d, 350),
            (&mut self.sma._1y, 365),
            (&mut self.sma._2y, 2 * 365),
            (&mut self.sma._200w, 200 * 7),
            (&mut self.sma._4y, 4 * 365),
        ] {
            let window_starts = blocks.lookback.start_vec(period);
            sma.compute_all(prices, starting_indexes, exit, |v| {
                v.compute_rolling_average(starting_indexes.height, window_starts, close, exit)?;
                Ok(())
            })?;
        }

        for (ema, period) in [
            (&mut self.ema._1w, 7),
            (&mut self.ema._8d, 8),
            (&mut self.ema._12d, 12),
            (&mut self.ema._13d, 13),
            (&mut self.ema._21d, 21),
            (&mut self.ema._26d, 26),
            (&mut self.ema._1m, 30),
            (&mut self.ema._34d, 34),
            (&mut self.ema._55d, 55),
            (&mut self.ema._89d, 89),
            (&mut self.ema._144d, 144),
            (&mut self.ema._200d, 200),
            (&mut self.ema._1y, 365),
            (&mut self.ema._2y, 2 * 365),
            (&mut self.ema._200w, 200 * 7),
            (&mut self.ema._4y, 4 * 365),
        ] {
            let window_starts = blocks.lookback.start_vec(period);
            ema.compute_all(prices, starting_indexes, exit, |v| {
                v.compute_rolling_ema(starting_indexes.height, window_starts, close, exit)?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
