use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{BasisPoints16, CheckedSub, Halving, Indexes, Sats};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{blocks, indexes, internal::RatioSatsBp16, prices, transactions};

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
        let window_starts = lookback.window_starts();

        self.coinbase.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |vec| {
                // Cursors avoid per-height PcoVec page decompression for the
                // tx-indexed lookups.  Coinbase txindex values are strictly
                // increasing, so the cursors only advance forward.
                let mut txout_cursor = indexer.vecs.transactions.first_txoutindex.cursor();
                let mut count_cursor = indexes.txindex.output_count.cursor();

                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    |(height, txindex, ..)| {
                        let ti = txindex.to_usize();

                        txout_cursor.advance(ti - txout_cursor.position());
                        let first_txoutindex = txout_cursor.next().unwrap().to_usize();

                        count_cursor.advance(ti - count_cursor.position());
                        let output_count: usize = count_cursor.next().unwrap().into();

                        let sats = indexer.vecs.outputs.value.fold_range_at(
                            first_txoutindex,
                            first_txoutindex + output_count,
                            Sats::ZERO,
                            |acc, v| acc + v,
                        );
                        (height, sats)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        // Coinbase fee is 0, so including it in the sum doesn't affect the result
        self.fees.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |vec| {
                vec.compute_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexes.height.txindex_count,
                    &transactions_fees.fee.txindex,
                    exit,
                )?;
                Ok(())
            },
        )?;

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

        self.unclaimed_rewards.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    &self.subsidy.base.sats.height,
                    |(height, subsidy, ..)| {
                        let halving = Halving::from(height);
                        let expected = Sats::FIFTY_BTC / 2_usize.pow(halving.to_usize() as u32);
                        (height, expected.checked_sub(subsidy).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        // All-time cumulative fee dominance
        self.fee_dominance
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                starting_indexes.height,
                &self.fees.cumulative.sats.height,
                &self.coinbase.cumulative.sats.height,
                exit,
            )?;

        // Rolling fee dominance = sum(fees) / sum(coinbase)
        for ((fee_dom, fees_w), coinbase_w) in self
            .fee_dominance_rolling
            .as_mut_array()
            .into_iter()
            .zip(self.fees.rolling.as_array())
            .zip(self.coinbase.sum.as_array())
        {
            fee_dom.compute_binary::<Sats, Sats, RatioSatsBp16>(
                starting_indexes.height,
                &fees_w.sum.sats.height,
                &coinbase_w.sats.height,
                exit,
            )?;
        }

        // All-time cumulative subsidy dominance
        self.subsidy_dominance
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                starting_indexes.height,
                &self.subsidy.cumulative.sats.height,
                &self.coinbase.cumulative.sats.height,
                exit,
            )?;

        // Rolling subsidy dominance = 1 - fee_dominance
        for (sub_dom, fee_dom) in self
            .subsidy_dominance_rolling
            .as_mut_array()
            .into_iter()
            .zip(self.fee_dominance_rolling.as_array())
        {
            sub_dom.bps.height.compute_transform(
                starting_indexes.height,
                &fee_dom.bps.height,
                |(height, fee, _)| (height, BasisPoints16::ONE - fee),
                exit,
            )?;
        }

        self.subsidy_sma_1y.cents.height.compute_rolling_average(
            starting_indexes.height,
            &lookback.height_1y_ago,
            &self.subsidy.base.cents.height,
            exit,
        )?;

        Ok(())
    }
}
