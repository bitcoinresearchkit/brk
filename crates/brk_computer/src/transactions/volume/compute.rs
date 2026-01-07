use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredF32, ONE_DAY_IN_SEC_F64};
use vecdb::Exit;

use super::Vecs;
use super::super::{count, fees};
use crate::{indexes, inputs, outputs, price, ComputeIndexes};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        count_vecs: &count::Vecs,
        fees_vecs: &fees::Vecs,
        inputs_count: &inputs::CountVecs,
        outputs_count: &outputs::CountVecs,
        starting_indexes: &ComputeIndexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_sent_sum
            .compute_all(indexes, price, starting_indexes, exit, |v| {
                v.compute_filtered_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.tx.height_to_first_txindex,
                    &indexes.block.height_to_txindex_count,
                    &fees_vecs.txindex_to_input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume.compute_all(starting_indexes, exit, |v| {
            v.compute_sum(
                starting_indexes.dateindex,
                &self.indexes_to_sent_sum.sats.dateindex.0,
                365,
                exit,
            )?;
            Ok(())
        })?;

        self.indexes_to_annualized_volume_btc.compute_all(starting_indexes, exit, |v| {
            v.compute_sum(
                starting_indexes.dateindex,
                &*self.indexes_to_sent_sum.bitcoin.dateindex,
                365,
                exit,
            )?;
            Ok(())
        })?;

        if let Some(indexes_to_sent_sum) = self.indexes_to_sent_sum.dollars.as_ref() {
            self.indexes_to_annualized_volume_usd.compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    &indexes_to_sent_sum.dateindex.0,
                    365,
                    exit,
                )?;
                Ok(())
            })?;
        }

        self.indexes_to_tx_per_sec.compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    &count_vecs.indexes_to_tx_count.dateindex.sum_cum.sum.0,
                    &indexes.time.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        let completion = date.completion();
                        let per_sec = if completion == 0.0 {
                            StoredF32::NAN
                        } else {
                            StoredF32::from(*tx_count as f64 / (completion * ONE_DAY_IN_SEC_F64))
                        };
                        (i, per_sec)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_inputs_per_sec.compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    &inputs_count.indexes_to_count.dateindex.sum_cum.sum.0,
                    &indexes.time.dateindex_to_date,
                    |(i, input_count, date, ..)| {
                        let completion = date.completion();
                        let per_sec = if completion == 0.0 {
                            StoredF32::NAN
                        } else {
                            StoredF32::from(*input_count as f64 / (completion * ONE_DAY_IN_SEC_F64))
                        };
                        (i, per_sec)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_outputs_per_sec.compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    &outputs_count.indexes_to_count.dateindex.sum_cum.sum.0,
                    &indexes.time.dateindex_to_date,
                    |(i, output_count, date, ..)| {
                        let completion = date.completion();
                        let per_sec = if completion == 0.0 {
                            StoredF32::NAN
                        } else {
                            StoredF32::from(*output_count as f64 / (completion * ONE_DAY_IN_SEC_F64))
                        };
                        (i, per_sec)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
