use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredF32;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, ComputeIndexes, indexes, inputs, outputs, prices};
use crate::transactions::{count, fees};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        count_vecs: &count::Vecs,
        fees_vecs: &fees::Vecs,
        inputs_count: &inputs::CountVecs,
        outputs_count: &outputs::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent_sum.sats.height.compute_filtered_sum_from_indexes(
            starting_indexes.height,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            &fees_vecs.input_value,
            |sats| !sats.is_max(),
            exit,
        )?;

        self.received_sum.sats.height.compute_sum_from_indexes(
            starting_indexes.height,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            &fees_vecs.output_value,
            exit,
        )?;

        // Compute USD from sats × price
        self.sent_sum
            .compute(prices, starting_indexes.height, exit)?;
        self.received_sum
            .compute(prices, starting_indexes.height, exit)?;

        // Annualized volume: rolling 1y sum of per-block sent volume
        self.annualized_volume.sats.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &self.sent_sum.sats.height,
            exit,
        )?;
        self.annualized_volume
            .compute(prices, starting_indexes.height, exit)?;

        // Rolling sums for sent and received
        let window_starts = blocks.count.window_starts();
        self.sent_sum_rolling.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.sent_sum.sats.height,
            &self.sent_sum.usd.height,
            exit,
        )?;
        self.received_sum_rolling.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.received_sum.sats.height,
            &self.received_sum.usd.height,
            exit,
        )?;

        // tx_per_sec: per-block tx count / block interval
        self.tx_per_sec.height.compute_transform2(
            starting_indexes.height,
            &count_vecs.tx_count.height,
            &blocks.interval.interval.height,
            |(h, tx_count, interval, ..)| {
                let interval_f64 = f64::from(*interval);
                let per_sec = if interval_f64 > 0.0 {
                    StoredF32::from(*tx_count as f64 / interval_f64)
                } else {
                    StoredF32::NAN
                };
                (h, per_sec)
            },
            exit,
        )?;

        // inputs_per_sec: per-block input count / block interval
        self.inputs_per_sec.height.compute_transform2(
            starting_indexes.height,
            &inputs_count.height.sum_cumulative.sum.0,
            &blocks.interval.interval.height,
            |(h, input_count, interval, ..)| {
                let interval_f64 = f64::from(*interval);
                let per_sec = if interval_f64 > 0.0 {
                    StoredF32::from(*input_count as f64 / interval_f64)
                } else {
                    StoredF32::NAN
                };
                (h, per_sec)
            },
            exit,
        )?;

        // outputs_per_sec: per-block output count / block interval
        self.outputs_per_sec.height.compute_transform2(
            starting_indexes.height,
            &outputs_count.total_count.sum_cumulative.sum.0,
            &blocks.interval.interval.height,
            |(h, output_count, interval, ..)| {
                let interval_f64 = f64::from(*interval);
                let per_sec = if interval_f64 > 0.0 {
                    StoredF32::from(*output_count as f64 / interval_f64)
                } else {
                    StoredF32::NAN
                };
                (h, per_sec)
            },
            exit,
        )?;

        Ok(())
    }
}
