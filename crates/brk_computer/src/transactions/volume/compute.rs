use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{ONE_DAY_IN_SEC_F64, StoredF32};
use vecdb::Exit;

use super::super::{count, fees};
use super::Vecs;
use crate::{ComputeIndexes, indexes, inputs, outputs};

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
        exit: &Exit,
    ) -> Result<()> {
        self.sent_sum
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_filtered_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexes.height.txindex_count,
                    &fees_vecs.input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?;
                Ok(())
            })?;

        self.annualized_volume.compute_sats(|v| {
            v.compute_sum(
                starting_indexes.dateindex,
                &self.sent_sum.sats.dateindex.0,
                365,
                exit,
            )?;
            Ok(())
        })?;

        if let Some(sent_sum_dollars) = self.sent_sum.dollars.as_ref() {
            self.annualized_volume.compute_dollars(|dollars| {
                dollars.compute_all(starting_indexes, exit, |v| {
                    v.compute_sum(
                        starting_indexes.dateindex,
                        &sent_sum_dollars.dateindex.0,
                        365,
                        exit,
                    )?;
                    Ok(())
                })
            })?;
        }

        self.tx_per_sec.compute_all(starting_indexes, exit, |v| {
            v.compute_transform2(
                starting_indexes.dateindex,
                &count_vecs.tx_count.dateindex.sum_cum.sum.0,
                &indexes.dateindex.date,
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

        self.inputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    &inputs_count.dateindex.sum_cum.sum.0,
                    &indexes.dateindex.date,
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

        self.outputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    &outputs_count.total_count.dateindex.sum_cum.sum.0,
                    &indexes.dateindex.date,
                    |(i, output_count, date, ..)| {
                        let completion = date.completion();
                        let per_sec = if completion == 0.0 {
                            StoredF32::NAN
                        } else {
                            StoredF32::from(
                                *output_count as f64 / (completion * ONE_DAY_IN_SEC_F64),
                            )
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
