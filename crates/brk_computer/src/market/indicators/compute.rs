use brk_error::Result;
use brk_types::{StoredF32, Version};
use vecdb::{AnyVec, Exit, TypedVecIterator};

use super::{
    super::{moving_average, range, returns::Vecs as ReturnsVecs},
    Vecs,
};
use crate::{ComputeIndexes, blocks, distribution, price, utils::OptionExt};

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
            self.indexes_to_puell_multiple.as_mut(),
            rewards.indexes_to_subsidy_usd_1y_sma.as_ref(),
            rewards.indexes_to_coinbase.dollars.as_ref(),
        ) {
            let date_to_coinbase_usd_sum = coinbase_dollars.dateindex.unwrap_sum();

            puell.compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    date_to_coinbase_usd_sum,
                    sma.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;
        }

        let returns_dateindex = returns._1d_price_returns.dateindex.u();

        self.dateindex_to_rsi_gains.compute_transform(
            starting_indexes.dateindex,
            returns_dateindex,
            |(i, ret, ..)| (i, StoredF32::from((*ret).max(0.0))),
            exit,
        )?;

        self.dateindex_to_rsi_losses.compute_transform(
            starting_indexes.dateindex,
            returns_dateindex,
            |(i, ret, ..)| (i, StoredF32::from((-*ret).max(0.0))),
            exit,
        )?;

        self.dateindex_to_rsi_avg_gain_14d.compute_sma(
            starting_indexes.dateindex,
            &self.dateindex_to_rsi_gains,
            14,
            exit,
        )?;

        self.dateindex_to_rsi_avg_loss_14d.compute_sma(
            starting_indexes.dateindex,
            &self.dateindex_to_rsi_losses,
            14,
            exit,
        )?;

        let ema12 = moving_average
            .indexes_to_price_12d_ema
            .price
            .u()
            .dateindex
            .u();
        let ema26 = moving_average
            .indexes_to_price_26d_ema
            .price
            .u()
            .dateindex
            .u();

        self.dateindex_to_macd_line.compute_transform2(
            starting_indexes.dateindex,
            ema12,
            ema26,
            |(i, a, b, _)| (i, StoredF32::from(*a - *b)),
            exit,
        )?;

        self.dateindex_to_macd_signal.compute_ema(
            starting_indexes.dateindex,
            &self.dateindex_to_macd_line,
            9,
            exit,
        )?;

        // Stochastic RSI: StochRSI = (RSI - min) / (max - min) * 100
        self.dateindex_to_rsi_14d_min.compute_min(
            starting_indexes.dateindex,
            &self.dateindex_to_rsi_14d,
            14,
            exit,
        )?;

        self.dateindex_to_rsi_14d_max.compute_max(
            starting_indexes.dateindex,
            &self.dateindex_to_rsi_14d,
            14,
            exit,
        )?;

        self.dateindex_to_stoch_rsi.compute_transform3(
            starting_indexes.dateindex,
            &self.dateindex_to_rsi_14d,
            &self.dateindex_to_rsi_14d_min,
            &self.dateindex_to_rsi_14d_max,
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

        self.dateindex_to_stoch_rsi_k.compute_sma(
            starting_indexes.dateindex,
            &self.dateindex_to_stoch_rsi,
            3,
            exit,
        )?;

        self.dateindex_to_stoch_rsi_d.compute_sma(
            starting_indexes.dateindex,
            &self.dateindex_to_stoch_rsi_k,
            3,
            exit,
        )?;

        // Stochastic Oscillator: K = (close - low_14) / (high_14 - low_14) * 100
        if let (Some(close), Some(low_2w), Some(high_2w)) = (
            price.usd.timeindexes_to_price_close.dateindex.as_ref(),
            range.indexes_to_price_2w_min.dateindex.as_ref(),
            range.indexes_to_price_2w_max.dateindex.as_ref(),
        ) {
            self.dateindex_to_stoch_k.compute_transform3(
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

            self.dateindex_to_stoch_d.compute_sma(
                starting_indexes.dateindex,
                &self.dateindex_to_stoch_k,
                3,
                exit,
            )?;
        }

        let amount_range = &distribution.utxo_cohorts.amount_range;
        let supply_vecs: Vec<_> = amount_range
            .iter()
            .filter_map(|c| c.metrics.supply.indexes_to_supply.sats.dateindex.as_ref())
            .collect();
        let count_vecs: Vec<_> = amount_range
            .iter()
            .filter_map(|c| {
                c.metrics
                    .supply
                    .indexes_to_utxo_count
                    .dateindex
                    .last
                    .as_ref()
            })
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

            self.dateindex_to_gini.compute_to(
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
