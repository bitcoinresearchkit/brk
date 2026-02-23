use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, HalvingEpoch, Sats, StoredF32};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, blocks, indexes, transactions};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        count_vecs: &blocks::CountVecs,
        transactions_fees: &transactions::FeesVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.coinbase.compute(starting_indexes, exit, |vec| {
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
        })?;

        self.coinbase_24h_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_24h_ago,
            &self.coinbase.sats.height,
            &self.coinbase.usd.height,
            exit,
        )?;
        self.coinbase_7d_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_1w_ago,
            &self.coinbase.sats.height,
            &self.coinbase.usd.height,
            exit,
        )?;
        self.coinbase_30d_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_1m_ago,
            &self.coinbase.sats.height,
            &self.coinbase.usd.height,
            exit,
        )?;
        self.coinbase_1y_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_1y_ago,
            &self.coinbase.sats.height,
            &self.coinbase.usd.height,
            exit,
        )?;

        let fee_sats_source = transactions_fees.fee.sats.height.sum_cum.sum.inner();
        let fee_usd_source = &transactions_fees.fee.usd.height.sum;
        self.fee_24h_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_24h_ago,
            fee_sats_source,
            fee_usd_source,
            exit,
        )?;
        self.fee_7d_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_1w_ago,
            fee_sats_source,
            fee_usd_source,
            exit,
        )?;
        self.fee_30d_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_1m_ago,
            fee_sats_source,
            fee_usd_source,
            exit,
        )?;
        self.fee_1y_sum.compute_rolling_sum(
            starting_indexes.height,
            &count_vecs.height_1y_ago,
            fee_sats_source,
            fee_usd_source,
            exit,
        )?;

        self.subsidy.compute(starting_indexes, exit, |vec| {
            vec.compute_transform2(
                starting_indexes.height,
                &self.coinbase.sats.height,
                transactions_fees.fee.sats.height.sum_cum.sum.inner(),
                |(height, coinbase, fees, ..)| {
                    (
                        height,
                        coinbase.checked_sub(fees).unwrap_or_else(|| {
                            dbg!(height, coinbase, fees);
                            panic!()
                        }),
                    )
                },
                exit,
            )?;
            Ok(())
        })?;

        self.unclaimed_rewards
            .compute(starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    &self.subsidy.sats.height,
                    |(height, subsidy, ..)| {
                        let halving = HalvingEpoch::from(height);
                        let expected = Sats::FIFTY_BTC / 2_usize.pow(halving.to_usize() as u32);
                        (height, expected.checked_sub(subsidy).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        // All-time cumulative fee dominance
        self.fee_dominance.height.compute_percentage(
            starting_indexes.height,
            transactions_fees.fee.sats.height.sum_cum.cumulative.inner(),
            self.coinbase.sats.rest.height_cumulative.inner(),
            exit,
        )?;

        // Rolling fee dominance = sum(fees) / sum(coinbase) * 100
        self.fee_dominance_24h.height.compute_percentage(
            starting_indexes.height,
            &self.fee_24h_sum.sats.height,
            &self.coinbase_24h_sum.sats.height,
            exit,
        )?;
        self.fee_dominance_7d.height.compute_percentage(
            starting_indexes.height,
            &self.fee_7d_sum.sats.height,
            &self.coinbase_7d_sum.sats.height,
            exit,
        )?;
        self.fee_dominance_30d.height.compute_percentage(
            starting_indexes.height,
            &self.fee_30d_sum.sats.height,
            &self.coinbase_30d_sum.sats.height,
            exit,
        )?;
        self.fee_dominance_1y.height.compute_percentage(
            starting_indexes.height,
            &self.fee_1y_sum.sats.height,
            &self.coinbase_1y_sum.sats.height,
            exit,
        )?;

        // All-time cumulative subsidy dominance
        self.subsidy_dominance.height.compute_percentage(
            starting_indexes.height,
            self.subsidy.sats.rest.height_cumulative.inner(),
            self.coinbase.sats.rest.height_cumulative.inner(),
            exit,
        )?;

        // Rolling subsidy dominance = 100 - fee_dominance
        let hundred = StoredF32::from(100u8);
        self.subsidy_dominance_24h.height.compute_transform(
            starting_indexes.height,
            &self.fee_dominance_24h.height,
            |(height, fee_dom, _)| (height, hundred - fee_dom),
            exit,
        )?;
        self.subsidy_dominance_7d.height.compute_transform(
            starting_indexes.height,
            &self.fee_dominance_7d.height,
            |(height, fee_dom, _)| (height, hundred - fee_dom),
            exit,
        )?;
        self.subsidy_dominance_30d.height.compute_transform(
            starting_indexes.height,
            &self.fee_dominance_30d.height,
            |(height, fee_dom, _)| (height, hundred - fee_dom),
            exit,
        )?;
        self.subsidy_dominance_1y.height.compute_transform(
            starting_indexes.height,
            &self.fee_dominance_1y.height,
            |(height, fee_dom, _)| (height, hundred - fee_dom),
            exit,
        )?;

        self.subsidy_usd_1y_sma.height.compute_rolling_average(
            starting_indexes.height,
            &count_vecs.height_1y_ago,
            &self.coinbase.usd.height,
            exit,
        )?;

        Ok(())
    }
}
