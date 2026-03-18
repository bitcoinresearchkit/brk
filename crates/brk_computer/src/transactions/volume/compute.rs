use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredF32};
use vecdb::Exit;

use super::Vecs;
use crate::transactions::{count, fees};
use crate::{indexes, inputs, outputs, prices};

const WINDOW_SECS: [f64; 4] = [86400.0, 7.0 * 86400.0, 30.0 * 86400.0, 365.0 * 86400.0];

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        count_vecs: &count::Vecs,
        fees_vecs: &fees::Vecs,
        inputs_count: &inputs::CountVecs,
        outputs_count: &outputs::CountVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.transfer_volume
            .compute(starting_indexes.height, prices, exit, |sats_vec| {
                Ok(sats_vec.compute_filtered_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_tx_index,
                    &indexes.height.tx_index_count,
                    &fees_vecs.input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?)
            })?;

        let h = starting_indexes.height;
        let tx_sums = count_vecs.total.rolling.sum.0.as_array();
        let input_sums = inputs_count.rolling.sum.0.as_array();
        let output_sums = outputs_count.total.rolling.sum.0.as_array();

        for (i, &secs) in WINDOW_SECS.iter().enumerate() {
            self.tx_per_sec.as_mut_array()[i].height.compute_transform(
                h,
                &tx_sums[i].height,
                |(h, sum, ..)| (h, StoredF32::from(*sum as f64 / secs)),
                exit,
            )?;
            self.inputs_per_sec.as_mut_array()[i]
                .height
                .compute_transform(
                    h,
                    &input_sums[i].height,
                    |(h, sum, ..)| (h, StoredF32::from(*sum as f64 / secs)),
                    exit,
                )?;
            self.outputs_per_sec.as_mut_array()[i]
                .height
                .compute_transform(
                    h,
                    &output_sums[i].height,
                    |(h, sum, ..)| (h, StoredF32::from(*sum as f64 / secs)),
                    exit,
                )?;
        }

        Ok(())
    }
}
