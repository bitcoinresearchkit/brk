use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Bitcoin, FeeRate, Sats};
use vecdb::{Exit, unlikely};

use super::super::size;
use super::Vecs;
use crate::{blocks, indexes, inputs, prices, ComputeIndexes};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        txins: &inputs::Vecs,
        size_vecs: &size::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.input_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.transactions.first_txinindex,
            &indexes.txindex.input_count,
            &txins.spent.value,
            exit,
        )?;

        self.output_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.transactions.first_txoutindex,
            &indexes.txindex.output_count,
            &indexer.vecs.outputs.value,
            exit,
        )?;

        self.fee_txindex.compute_transform2(
            starting_indexes.txindex,
            &self.input_value,
            &self.output_value,
            |(i, input, output, ..)| {
                let fee = if unlikely(input.is_max()) {
                    Sats::ZERO
                } else {
                    input - output
                };
                (i, fee)
            },
            exit,
        )?;

        self.fee_rate_txindex.compute_transform2(
            starting_indexes.txindex,
            &self.fee_txindex,
            &size_vecs.vsize.txindex,
            |(txindex, fee, vsize, ..)| (txindex, FeeRate::from((fee, vsize))),
            exit,
        )?;

        // Skip coinbase (first tx per block) since it has no fee
        self.fee.compute_with_skip(
            starting_indexes.height,
            &self.fee_txindex,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            exit,
            1,
        )?;

        // Skip coinbase (first tx per block) since it has no feerate
        self.fee_rate.compute_with_skip(
            starting_indexes.height,
            &self.fee_rate_txindex,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            exit,
            1,
        )?;

        // Compute fee USD sum per block: price * Bitcoin::from(sats)
        self.fee_usd_sum.compute_transform2(
            starting_indexes.height,
            self.fee.sum_cum.sum.inner(),
            &prices.usd.price,
            |(h, sats, price, ..)| (h, price * Bitcoin::from(sats)),
            exit,
        )?;

        // Rolling fee stats (from per-block sum)
        let window_starts = blocks.count.window_starts();
        self.fee_rolling.compute(
            starting_indexes.height,
            &window_starts,
            self.fee.sum_cum.sum.inner(),
            exit,
        )?;

        // Rolling fee rate distribution (from per-block average)
        self.fee_rate_rolling.compute_distribution(
            starting_indexes.height,
            &window_starts,
            &self.fee_rate.min_max_average.average.0,
            exit,
        )?;

        Ok(())
    }
}
