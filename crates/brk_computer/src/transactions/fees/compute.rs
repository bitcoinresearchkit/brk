use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{FeeRate, Sats};
use vecdb::{Exit, unlikely};

use super::super::size;
use super::Vecs;
use crate::{ComputeIndexes, indexes, inputs};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        txins: &inputs::Vecs,
        size_vecs: &size::Vecs,
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

        self.fee.base.compute_transform2(
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

        self.fee_rate.txindex.compute_transform2(
            starting_indexes.txindex,
            &self.fee.base,
            &size_vecs.vsize.txindex,
            |(txindex, fee, vsize, ..)| (txindex, FeeRate::from((fee, vsize))),
            exit,
        )?;

        // Skip coinbase (first tx per block) since it has no fee
        self.fee
            .derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1)?;

        // Skip coinbase (first tx per block) since it has no feerate
        self.fee_rate
            .derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1)?;

        Ok(())
    }
}
