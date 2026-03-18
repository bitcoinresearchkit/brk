use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Dollars, Halving, Indexes, Sats};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{blocks, indexes, internal::{RatioDollarsBp32, RatioSatsBp16}, prices, transactions};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        lookback: &blocks::LookbackVecs,
        transactions_fees: &transactions::FeesVecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // coinbase and fees are independent — parallelize
        let window_starts = lookback.window_starts();
        let (r_coinbase, r_fees) = rayon::join(
            || {
                self.coinbase.compute(
                    starting_indexes.height,
                    prices,
                    exit,
                    |vec| {
                        let mut txout_cursor =
                            indexer.vecs.transactions.first_txout_index.cursor();
                        let mut count_cursor = indexes.tx_index.output_count.cursor();

                        vec.compute_transform(
                            starting_indexes.height,
                            &indexer.vecs.transactions.first_tx_index,
                            |(height, tx_index, ..)| {
                                let ti = tx_index.to_usize();

                                txout_cursor.advance(ti - txout_cursor.position());
                                let first_txout_index =
                                    txout_cursor.next().unwrap().to_usize();

                                count_cursor.advance(ti - count_cursor.position());
                                let output_count: usize =
                                    count_cursor.next().unwrap().into();

                                let sats = indexer.vecs.outputs.value.fold_range_at(
                                    first_txout_index,
                                    first_txout_index + output_count,
                                    Sats::ZERO,
                                    |acc, v| acc + v,
                                );
                                (height, sats)
                            },
                            exit,
                        )?;
                        Ok(())
                    },
                )
            },
            || {
                self.fees.compute(
                    starting_indexes.height,
                    &window_starts,
                    prices,
                    exit,
                    |vec| {
                        vec.compute_sum_from_indexes(
                            starting_indexes.height,
                            &indexer.vecs.transactions.first_tx_index,
                            &indexes.height.tx_index_count,
                            &transactions_fees.fee.tx_index,
                            exit,
                        )?;
                        Ok(())
                    },
                )
            },
        );
        r_coinbase?;
        r_fees?;

        self.subsidy.base.sats.height.compute_transform2(
            starting_indexes.height,
            &self.coinbase.base.sats.height,
            &self.fees.base.sats.height,
            |(height, coinbase, fees, ..)| {
                (
                    height,
                    coinbase.checked_sub(fees).unwrap_or_else(|| {
                        panic!("coinbase {coinbase:?} < fees {fees:?} at {height:?}")
                    }),
                )
            },
            exit,
        )?;
        self.subsidy.compute(prices, starting_indexes.height, exit)?;

        self.unclaimed.base.sats.height.compute_transform(
            starting_indexes.height,
            &self.subsidy.base.sats.height,
            |(height, subsidy, ..)| {
                let halving = Halving::from(height);
                let expected = Sats::FIFTY_BTC / 2_usize.pow(halving.to_usize() as u32);
                (height, expected.checked_sub(subsidy).unwrap())
            },
            exit,
        )?;
        self.unclaimed.compute(prices, starting_indexes.height, exit)?;

        self.fee_dominance
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                starting_indexes.height,
                &self.fees.cumulative.sats.height,
                &self.coinbase.cumulative.sats.height,
                exit,
            )?;

        self.fee_dominance_rolling
            .compute_binary::<Sats, Sats, RatioSatsBp16, _, _>(
                starting_indexes.height,
                self.fees.sum.as_array().map(|w| &w.sats.height),
                self.coinbase.sum.as_array().map(|w| &w.sats.height),
                exit,
            )?;

        self.subsidy_dominance
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                starting_indexes.height,
                &self.subsidy.cumulative.sats.height,
                &self.coinbase.cumulative.sats.height,
                exit,
            )?;

        self.subsidy_sma_1y.cents.height.compute_rolling_average(
            starting_indexes.height,
            &lookback._1y,
            &self.subsidy.base.cents.height,
            exit,
        )?;

        self.fee_to_subsidy_ratio
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32, _, _>(
                starting_indexes.height,
                self.coinbase.sum.as_array().map(|w| &w.usd.height),
                self.fees.sum.as_array().map(|w| &w.usd.height),
                exit,
            )?;

        Ok(())
    }
}
