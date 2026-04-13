use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredF32};
use vecdb::Exit;

use super::Vecs;
use crate::transactions::{count, fees};
use crate::{indexes, internal::Windows, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        count_vecs: &count::Vecs,
        fees_vecs: &fees::Vecs,
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
        let tx_per_sec = self.tx_per_sec.as_mut_array();
        for (i, &secs) in Windows::<()>::SECS.iter().enumerate() {
            tx_per_sec[i].height.compute_transform(
                h,
                &tx_sums[i].height,
                |(h, sum, ..)| (h, StoredF32::from(*sum as f64 / secs)),
                exit,
            )?;
        }

        Ok(())
    }
}
