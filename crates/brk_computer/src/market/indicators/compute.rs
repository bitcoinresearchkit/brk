use brk_error::Result;
use brk_types::{BasisPoints16, Dollars};
use vecdb::Exit;

use super::{super::range, Vecs};
use crate::{
    ComputeIndexes, blocks, distribution,
    internal::{RatioDollarsBp32, Windows},
    mining, prices, transactions,
};

fn tf_multiplier(tf: &str) -> usize {
    match tf {
        "24h" => 1,
        "1w" => 7,
        "1m" => 30,
        "1y" => 365,
        _ => unreachable!(),
    }
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
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
        self.puell_multiple.bps.compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
            starting_indexes.height,
            &rewards.subsidy.base.usd.height,
            &rewards.subsidy_sma_1y.usd.height,
            exit,
        )?;

        // Stochastic Oscillator: K = (close - low_2w) / (high_2w - low_2w), stored as ratio (0–1)
        {
            let price = &prices.price.usd.height;
            self.stoch_k.bps.height.compute_transform3(
                starting_indexes.height,
                price,
                &range.price_min_2w.usd.height,
                &range.price_max_2w.usd.height,
                |(h, close, low, high, ..)| {
                    let range = *high - *low;
                    let stoch = if range == 0.0 {
                        BasisPoints16::ZERO
                    } else {
                        BasisPoints16::from(((*close - *low) / range) as f64)
                    };
                    (h, stoch)
                },
                exit,
            )?;

            self.stoch_d.bps.height.compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_3d_ago,
                &self.stoch_k.bps.height,
                exit,
            )?;
        }

        // RSI per timeframe
        for (tf, rsi_chain) in Windows::<()>::SUFFIXES.into_iter()
            .zip(self.rsi.as_mut_array())
        {
            let m = tf_multiplier(tf);
            let returns_source = match tf {
                "24h" => &returns.price_return._24h.ratio.height,
                "1w" => &returns.price_return._1w.ratio.height,
                "1m" => &returns.price_return._1m.ratio.height,
                "1y" => &returns.price_return._1y.ratio.height,
                _ => unreachable!(),
            };
            super::rsi::compute(
                rsi_chain,
                blocks,
                returns_source,
                14 * m,
                3 * m,
                starting_indexes,
                exit,
            )?;
        }

        // MACD per timeframe
        for (tf, macd_chain) in Windows::<()>::SUFFIXES.into_iter()
            .zip(self.macd.as_mut_array())
        {
            let m = tf_multiplier(tf);
            super::macd::compute(
                macd_chain,
                blocks,
                prices,
                12 * m,
                26 * m,
                9 * m,
                starting_indexes,
                exit,
            )?;
        }

        // Gini (per height)
        super::gini::compute(
            &mut self.gini,
            distribution,
            starting_indexes,
            exit,
        )?;

        // NVT: market_cap / tx_volume_24h
        self.nvt.bps.compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
            starting_indexes.height,
            &distribution.utxo_cohorts.all.metrics.supply.total.usd.height,
            &transactions.volume.sent_sum.rolling._24h.usd.height,
            exit,
        )?;

        // Pi Cycle: sma_111d / sma_350d_x2
        self.pi_cycle.bps.compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
            starting_indexes.height,
            &moving_average.price_sma_111d.price.usd.height,
            &moving_average.price_sma_350d_x2.usd.height,
            exit,
        )?;

        Ok(())
    }
}
