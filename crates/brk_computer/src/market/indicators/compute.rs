use brk_error::Result;
use brk_types::{Day1, StoredF32};
use vecdb::{Exit, ReadableVec};

use super::{super::range, Vecs};
use crate::{ComputeIndexes, blocks, distribution, indexes, mining, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        rewards: &mining::RewardsVecs,
        returns: &super::super::returns::Vecs,
        range: &range::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.puell_multiple.height.compute_divide(
            starting_indexes.height,
            &rewards.coinbase.usd.height,
            &rewards.subsidy_usd_1y_sma.height,
            exit,
        )?;

        // Stochastic Oscillator: K = (close - low_2w) / (high_2w - low_2w) * 100
        {
            let price = &prices.usd.price;
            self.stoch_k.height.compute_transform3(
                starting_indexes.height,
                price,
                &range.price_2w_min.height,
                &range.price_2w_max.height,
                |(h, close, low, high, ..)| {
                    let range = *high - *low;
                    let stoch = if range == 0.0 {
                        StoredF32::from(50.0)
                    } else {
                        StoredF32::from(((*close - *low) / range * 100.0) as f32)
                    };
                    (h, stoch)
                },
                exit,
            )?;

            self.stoch_d.height.compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_3d_ago,
                &self.stoch_k.height,
                exit,
            )?;
        }

        // Pre-collect Height→Day1 mapping
        let h2d: Vec<Day1> = indexes.height.day1.collect();
        let total_heights = h2d.len();

        // RSI per timeframe
        for (tf, rsi_chain) in self.rsi.iter_mut() {
            super::rsi::compute(
                rsi_chain,
                tf,
                returns,
                &h2d,
                total_heights,
                starting_indexes,
                exit,
            )?;
        }

        // MACD per timeframe
        for (tf, macd_chain) in self.macd.iter_mut() {
            super::macd::compute(
                macd_chain,
                tf,
                prices,
                &h2d,
                total_heights,
                starting_indexes,
                exit,
            )?;
        }

        // Gini (daily only, expanded to Height)
        super::gini::compute(
            &mut self.gini,
            distribution,
            &h2d,
            total_heights,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }
}
