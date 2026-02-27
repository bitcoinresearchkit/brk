use brk_error::Result;
use brk_types::{Day1, Dollars, StoredF32};
use vecdb::{Exit, ReadableVec};

use super::{super::range, Vecs};
use crate::{
    ComputeIndexes, blocks, distribution, indexes,
    internal::Ratio32,
    mining, prices, transactions,
};

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
        transactions: &transactions::Vecs,
        moving_average: &super::super::moving_average::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.puell_multiple.height.compute_divide(
            starting_indexes.height,
            &rewards.coinbase.base.usd.height,
            &rewards.subsidy_usd_1y_sma.height,
            exit,
        )?;

        // Stochastic Oscillator: K = (close - low_2w) / (high_2w - low_2w) * 100
        {
            let price = &prices.price.usd;
            self.stoch_k.height.compute_transform3(
                starting_indexes.height,
                price,
                &range.price_2w_min.usd.height,
                &range.price_2w_max.usd.height,
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

        // NVT: realized_cap / tx_volume_24h
        self.nvt.compute_binary::<Dollars, Dollars, Ratio32>(
            starting_indexes.height,
            &distribution.utxo_cohorts.all.metrics.supply.total.usd.height,
            &transactions.volume.sent_sum.usd,
            exit,
        )?;

        // Pi Cycle: sma_111d / sma_350d_x2
        self.pi_cycle.compute_binary::<Dollars, Dollars, Ratio32>(
            starting_indexes.height,
            &moving_average.price_111d_sma.price.usd.height,
            &moving_average.price_350d_sma_x2.usd.height,
            exit,
        )?;

        Ok(())
    }
}
