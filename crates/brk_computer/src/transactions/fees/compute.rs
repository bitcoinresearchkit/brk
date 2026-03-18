use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{FeeRate, Indexes, Sats};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec, unlikely};

use super::super::size;
use super::Vecs;
use crate::{indexes, inputs};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        spent: &inputs::SpentVecs,
        size_vecs: &size::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.input_value.compute_sum_from_indexes(
            starting_indexes.tx_index,
            &indexer.vecs.transactions.first_txin_index,
            &indexes.tx_index.input_count,
            &spent.value,
            exit,
        )?;
        self.output_value.compute_sum_from_indexes(
            starting_indexes.tx_index,
            &indexer.vecs.transactions.first_txout_index,
            &indexes.tx_index.output_count,
            &indexer.vecs.outputs.value,
            exit,
        )?;

        self.compute_fee_and_fee_rate(size_vecs, starting_indexes, exit)?;

        let (r3, r4) = rayon::join(
            || self.fee.derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1),
            || self.fee_rate.derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1),
        );
        r3?;
        r4?;

        Ok(())
    }

    fn compute_fee_and_fee_rate(
        &mut self,
        size_vecs: &size::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = self.input_value.version()
            + self.output_value.version()
            + size_vecs.vsize.tx_index.version();

        self.fee
            .tx_index
            .validate_computed_version_or_reset(dep_version)?;
        self.fee_rate
            .tx_index
            .validate_computed_version_or_reset(dep_version)?;

        let target = self
            .input_value
            .len()
            .min(self.output_value.len())
            .min(size_vecs.vsize.tx_index.len());
        let min = self
            .fee
            .tx_index
            .len()
            .min(self.fee_rate.tx_index.len())
            .min(starting_indexes.tx_index.to_usize());

        if min >= target {
            return Ok(());
        }

        self.fee.tx_index.truncate_if_needed(starting_indexes.tx_index)?;
        self.fee_rate.tx_index.truncate_if_needed(starting_indexes.tx_index)?;

        loop {
            let skip = self.fee.tx_index.len();
            let end = self.fee.tx_index.batch_end(target);
            if skip >= end {
                break;
            }

            let input_batch = self.input_value.collect_range_at(skip, end);
            let output_batch = self.output_value.collect_range_at(skip, end);
            let vsize_batch = size_vecs.vsize.tx_index.collect_range_at(skip, end);

            for j in 0..input_batch.len() {
                let fee = if unlikely(input_batch[j].is_max()) {
                    Sats::ZERO
                } else {
                    input_batch[j] - output_batch[j]
                };
                self.fee.tx_index.push(fee);
                self.fee_rate
                    .tx_index
                    .push(FeeRate::from((fee, vsize_batch[j])));
            }

            let _lock = exit.lock();
            let (r1, r2) = rayon::join(
                || self.fee.tx_index.write(),
                || self.fee_rate.tx_index.write(),
            );
            r1?;
            r2?;
        }

        Ok(())
    }
}
