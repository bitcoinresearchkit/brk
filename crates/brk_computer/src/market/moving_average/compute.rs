use brk_error::Result;
use brk_types::Dollars;
use vecdb::{Exit, ReadableOptionVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, blocks, indexes, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = &prices.usd.price;

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

        let h2d = &indexes.height.day1;
        let closes: Vec<Dollars> = prices.usd.split.close.day1.collect_or_default();

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
            let k = 2.0f64 / (period as f64 + 1.0);

            // Compute date-level EMA, then expand to height level
            let date_ema = compute_date_ema(&closes, k);

            ema.compute_all(blocks, prices, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    h2d,
                    |(h, date, ..)| (h, Dollars::from(date_ema[date.to_usize()])),
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }
}

fn compute_date_ema(closes: &[Dollars], k: f64) -> Vec<f64> {
    let mut date_ema: Vec<f64> = Vec::with_capacity(closes.len());
    let mut ema_val = 0.0f64;
    for (d, close) in closes.iter().enumerate() {
        let close = f64::from(*close);
        if d == 0 {
            ema_val = close;
        } else {
            ema_val = close * k + ema_val * (1.0 - k);
        }
        date_ema.push(ema_val);
    }
    date_ema
}
