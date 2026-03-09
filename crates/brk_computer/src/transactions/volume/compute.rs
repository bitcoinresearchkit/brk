use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, Timestamp};
use vecdb::Exit;

use super::Vecs;
use crate::transactions::{count, fees};
use crate::{blocks, indexes, inputs, internal::PerSec, outputs, prices};

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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.lookback.window_starts();

        self.sent_sum.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |sats_vec| {
                Ok(sats_vec.compute_filtered_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexes.height.txindex_count,
                    &fees_vecs.input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?)
            },
        )?;

        self.received_sum.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |sats_vec| {
                Ok(sats_vec.compute_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexes.height.txindex_count,
                    &fees_vecs.output_value,
                    exit,
                )?)
            },
        )?;

        // Annualized volume: rolling 1y sum of per-block sent volume
        self.annualized_volume.sats.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_1y_ago,
            &self.sent_sum.sats,
            exit,
        )?;
        self.annualized_volume
            .compute(prices, starting_indexes.height, exit)?;

        self.tx_per_sec
            .height
            .compute_binary::<_, Timestamp, PerSec>(
                starting_indexes.height,
                &count_vecs.tx_count.height,
                &blocks.interval.height,
                exit,
            )?;
        self.inputs_per_sec
            .height
            .compute_binary::<_, Timestamp, PerSec>(
                starting_indexes.height,
                &inputs_count.full.sum,
                &blocks.interval.height,
                exit,
            )?;
        self.outputs_per_sec
            .height
            .compute_binary::<_, Timestamp, PerSec>(
                starting_indexes.height,
                &outputs_count.total_count.full.sum,
                &blocks.interval.height,
                exit,
            )?;

        Ok(())
    }
}
