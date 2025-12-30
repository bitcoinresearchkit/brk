use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::ONE_DAY_IN_SEC_F64;
use vecdb::Exit;

use super::Vecs;
use crate::{
    chain::{coinbase, transaction},
    indexes, price, ComputeIndexes,
};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transaction_vecs: &transaction::Vecs,
        coinbase_vecs: &coinbase::Vecs,
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
                    &transaction_vecs.txindex_to_input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_sent_sum.sats.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume_btc
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_sent_sum.bitcoin.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_tx_btc_velocity
            .compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    self.indexes_to_annualized_volume_btc
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    coinbase_vecs
                        .indexes_to_subsidy
                        .bitcoin
                        .dateindex
                        .unwrap_cumulative(),
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(indexes_to_sent_sum) = self.indexes_to_sent_sum.dollars.as_ref() {
            self.indexes_to_annualized_volume_usd
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_sum(
                        starting_indexes.dateindex,
                        indexes_to_sent_sum.dateindex.unwrap_sum(),
                        365,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_tx_usd_velocity
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_divide(
                        starting_indexes.dateindex,
                        self.indexes_to_annualized_volume_usd
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        coinbase_vecs
                            .indexes_to_subsidy
                            .dollars
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_cumulative(),
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        self.indexes_to_tx_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    transaction_vecs.indexes_to_tx_count.dateindex.unwrap_sum(),
                    &indexes.time.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_inputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    transaction_vecs
                        .indexes_to_input_count
                        .dateindex
                        .unwrap_sum(),
                    &indexes.time.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_outputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    transaction_vecs
                        .indexes_to_output_count
                        .dateindex
                        .unwrap_sum(),
                    &indexes.time.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
