use brk_error::Result;
use brk_types::{StoredF32, Version};
use vecdb::{AnyVec, Exit, TypedVecIterator};

use super::{
    super::{moving_average, range, returns::Vecs as ReturnsVecs},
    Vecs,
};
use crate::{ComputeIndexes, blocks, distribution, price};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        rewards: &blocks::RewardsVecs,
        returns: &ReturnsVecs,
        moving_average: &moving_average::Vecs,
        range: &range::Vecs,
        price: &price::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        if let (Some(puell), Some(sma), Some(coinbase_dollars)) = (
            self.puell_multiple.as_mut(),
            rewards.subsidy_usd_1y_sma.as_ref(),
            rewards.coinbase.dollars.as_ref(),
        ) {
            let date_to_coinbase_usd_sum = &coinbase_dollars.dateindex.sum_cum.sum.0;

            puell.compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    date_to_coinbase_usd_sum,
                    &sma.dateindex,
                    exit,
                )?;
                Ok(())
            })?;
        }

        let returns_dateindex = &returns.price_returns._1d.dateindex;

        self.rsi_gains.compute_transform(
            starting_indexes.dateindex,
            returns_dateindex,
            |(i, ret, ..)| (i, StoredF32::from((*ret).max(0.0))),
            exit,
        )?;

        self.rsi_losses.compute_transform(
            starting_indexes.dateindex,
            returns_dateindex,
            |(i, ret, ..)| (i, StoredF32::from((-*ret).max(0.0))),
            exit,
        )?;

        self.rsi_average_gain_14d.compute_rma(
            starting_indexes.dateindex,
            &self.rsi_gains,
            14,
            exit,
        )?;

        self.rsi_average_loss_14d.compute_rma(
            starting_indexes.dateindex,
            &self.rsi_losses,
            14,
            exit,
        )?;

        let ema12 = &moving_average
            .price_12d_ema
            .price
            .as_ref()
            .unwrap()
            .dateindex;
        let ema26 = &moving_average
            .price_26d_ema
            .price
            .as_ref()
            .unwrap()
            .dateindex;

        self.macd_line.compute_transform2(
            starting_indexes.dateindex,
            ema12,
            ema26,
            |(i, a, b, _)| (i, StoredF32::from(*a - *b)),
            exit,
        )?;

        self.macd_signal
            .compute_ema(starting_indexes.dateindex, &self.macd_line, 9, exit)?;

        // Stochastic RSI: StochRSI = (RSI - min) / (max - min) * 100
        self.rsi_14d_min
            .compute_min(starting_indexes.dateindex, &self.rsi_14d, 14, exit)?;

        self.rsi_14d_max
            .compute_max(starting_indexes.dateindex, &self.rsi_14d, 14, exit)?;

        self.stoch_rsi.compute_transform3(
            starting_indexes.dateindex,
            &self.rsi_14d,
            &self.rsi_14d_min,
            &self.rsi_14d_max,
            |(i, rsi, min, max, ..)| {
                let range = *max - *min;
                let stoch = if range == 0.0 {
                    StoredF32::from(50.0)
                } else {
                    StoredF32::from((*rsi - *min) / range * 100.0)
                };
                (i, stoch)
            },
            exit,
        )?;

        self.stoch_rsi_k
            .compute_sma(starting_indexes.dateindex, &self.stoch_rsi, 3, exit)?;

        self.stoch_rsi_d
            .compute_sma(starting_indexes.dateindex, &self.stoch_rsi_k, 3, exit)?;

        // Stochastic Oscillator: K = (close - low_14) / (high_14 - low_14) * 100
        {
            let close = &price.usd.split.close.dateindex;
            let low_2w = &range.price_2w_min.dateindex;
            let high_2w = &range.price_2w_max.dateindex;
            self.stoch_k.compute_transform3(
                starting_indexes.dateindex,
                close,
                low_2w,
                high_2w,
                |(i, close, low, high, ..)| {
                    let range = *high - *low;
                    let stoch = if range == 0.0 {
                        StoredF32::from(50.0)
                    } else {
                        StoredF32::from((**close - *low) / range * 100.0)
                    };
                    (i, stoch)
                },
                exit,
            )?;

            self.stoch_d
                .compute_sma(starting_indexes.dateindex, &self.stoch_k, 3, exit)?;
        }

        let amount_range = &distribution.utxo_cohorts.amount_range;

        let supply_vecs: Vec<_> = amount_range
            .iter()
            .map(|c| &c.metrics.supply.total.sats.dateindex.0)
            .collect();
        let count_vecs: Vec<_> = amount_range
            .iter()
            .map(|c| &c.metrics.outputs.utxo_count.dateindex)
            .collect();

        if let Some(first_supply) = supply_vecs.first()
            && supply_vecs.len() == count_vecs.len()
        {
            let version = supply_vecs
                .iter()
                .fold(Version::ZERO, |acc, v| acc + v.version())
                + count_vecs
                    .iter()
                    .fold(Version::ZERO, |acc, v| acc + v.version());
            let mut supply_iters: Vec<_> = supply_vecs.iter().map(|v| v.into_iter()).collect();
            let mut count_iters: Vec<_> = count_vecs.iter().map(|v| v.into_iter()).collect();

            self.gini.compute_to(
                starting_indexes.dateindex,
                first_supply.len(),
                version,
                |dateindex| {
                    let buckets: Vec<(u64, u64)> = supply_iters
                        .iter_mut()
                        .zip(count_iters.iter_mut())
                        .map(|(s, c)| {
                            let count: u64 = *c.get_unwrap(dateindex);
                            let supply: u64 = *s.get_unwrap(dateindex);
                            (count, supply)
                        })
                        .collect();
                    (dateindex, StoredF32::from(gini_from_lorenz(&buckets)))
                },
                exit,
            )?;
        }

        Ok(())
    }
}

fn gini_from_lorenz(buckets: &[(u64, u64)]) -> f32 {
    let total_count: u64 = buckets.iter().map(|(c, _)| c).sum();
    let total_supply: u64 = buckets.iter().map(|(_, s)| s).sum();

    if total_count == 0 || total_supply == 0 {
        return 0.0;
    }

    let (mut cum_count, mut cum_supply, mut area) = (0u64, 0u64, 0.0f64);
    let (tc, ts) = (total_count as f64, total_supply as f64);

    for &(count, supply) in buckets {
        let (p0, w0) = (cum_count as f64 / tc, cum_supply as f64 / ts);
        cum_count += count;
        cum_supply += supply;
        let (p1, w1) = (cum_count as f64 / tc, cum_supply as f64 / ts);
        area += (p1 - p0) * (w0 + w1) / 2.0;
    }

    (1.0 - 2.0 * area) as f32
}
