use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{FeeRate, Sats};
use vecdb::{Exit, unlikely};

use super::Vecs;
use super::super::size;
use crate::{indexes, inputs, ComputeIndexes};

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
        self.txindex_to_input_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txinindex,
            &indexes.transaction.txindex_to_input_count,
            &txins.spent.txinindex_to_value,
            exit,
        )?;

        self.txindex_to_output_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txoutindex,
            &indexes.transaction.txindex_to_output_count,
            &indexer.vecs.txout.txoutindex_to_value,
            exit,
        )?;

        self.txindex_to_fee.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_input_value,
            &self.txindex_to_output_value,
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

        self.txindex_to_fee_rate.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_fee,
            &size_vecs.txindex_to_vsize,
            |(txindex, fee, vsize, ..)| (txindex, FeeRate::from((fee, vsize))),
            exit,
        )?;

        self.indexes_to_fee.derive_from(
            indexer,
            indexes,
            starting_indexes,
            &self.txindex_to_fee,
            exit,
        )?;

        self.indexes_to_fee_rate.derive_from(
            indexer,
            indexes,
            starting_indexes,
            &self.txindex_to_fee_rate,
            exit,
        )?;

        Ok(())
    }
}
