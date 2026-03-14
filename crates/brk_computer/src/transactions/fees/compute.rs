use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{FeeRate, Indexes, Sats};
use vecdb::{Exit, unlikely};

use super::super::size;
use super::Vecs;
use crate::{indexes, inputs};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        txins: &inputs::Vecs,
        size_vecs: &size::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // input_value and output_value are independent — parallelize
        let (r1, r2) = rayon::join(
            || {
                self.input_value.compute_sum_from_indexes(
                    starting_indexes.tx_index,
                    &indexer.vecs.transactions.first_txin_index,
                    &indexes.tx_index.input_count,
                    &txins.spent.value,
                    exit,
                )
            },
            || {
                self.output_value.compute_sum_from_indexes(
                    starting_indexes.tx_index,
                    &indexer.vecs.transactions.first_txout_index,
                    &indexes.tx_index.output_count,
                    &indexer.vecs.outputs.value,
                    exit,
                )
            },
        );
        r1?;
        r2?;

        self.fee.tx_index.compute_transform2(
            starting_indexes.tx_index,
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

        self.fee_rate.tx_index.compute_transform2(
            starting_indexes.tx_index,
            &self.fee.tx_index,
            &size_vecs.vsize.tx_index,
            |(tx_index, fee, vsize, ..)| (tx_index, FeeRate::from((fee, vsize))),
            exit,
        )?;

        // Skip coinbase (first tx per block) since it has fee=0
        self.fee
            .derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1)?;

        // Skip coinbase (first tx per block) since it has no feerate
        self.fee_rate
            .derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1)?;

        Ok(())
    }
}
