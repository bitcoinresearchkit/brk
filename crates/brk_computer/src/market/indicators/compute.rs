use brk_error::Result;
use brk_types::{BasisPoints16, Dollars, Indexes};
use vecdb::Exit;

use super::{
    super::{moving_average, range, returns},
    Vecs, gini, macd, rsi,
};
use crate::{blocks, distribution, internal::RatioDollarsBp32, mining, prices, transactions};

const TF_MULTIPLIERS: [usize; 4] = [1, 7, 30, 365];

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        rewards: &mining::RewardsVecs,
        returns: &returns::Vecs,
        range: &range::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
        moving_average: &moving_average::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.puell_multiple
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
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
                &range.min._2w.usd.height,
                &range.max._2w.usd.height,
                |(h, close, low, high, ..)| {
                    let range = *high - *low;
                    let stoch = if range == 0.0 {
                        BasisPoints16::ZERO
                    } else {
                        BasisPoints16::from((*close - *low) / range)
                    };
                    (h, stoch)
                },
                exit,
            )?;

            self.stoch_d.bps.height.compute_rolling_average(
                starting_indexes.height,
                &blocks.lookback.height_3d_ago,
                &self.stoch_k.bps.height,
                exit,
            )?;
        }

        // RSI per timeframe
        let return_sources = [
            &returns.price_return._24h.ratio.height,
            &returns.price_return._1w.ratio.height,
            &returns.price_return._1m.ratio.height,
            &returns.price_return._1y.ratio.height,
        ];
        for ((rsi_chain, ret), &m) in self
            .rsi
            .as_mut_array()
            .into_iter()
            .zip(return_sources)
            .zip(&TF_MULTIPLIERS)
        {
            rsi::compute(
                rsi_chain,
                blocks,
                ret,
                14 * m,
                3 * m,
                starting_indexes,
                exit,
            )?;
        }

        // MACD per timeframe
        for (macd_chain, &m) in self.macd.as_mut_array().into_iter().zip(&TF_MULTIPLIERS) {
            macd::compute(
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
        gini::compute(&mut self.gini, distribution, starting_indexes, exit)?;

        // NVT: market_cap / tx_volume_24h
        let market_cap = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .usd
            .height;
        self.nvt
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                market_cap,
                &transactions.volume.sent_sum.rolling._24h.usd.height,
                exit,
            )?;

        // Pi Cycle: sma_111d / sma_350d_x2
        self.pi_cycle
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &moving_average.sma._111d.price.usd.height,
                &moving_average.sma._350d_x2.usd.height,
                exit,
            )?;

        Ok(())
    }
}
