use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{FeeRate, Sats, StoredU64, TxVersion};
use vecdb::{Exit, TypedVecIterator, unlikely};

use super::Vecs;
use crate::{ComputeIndexes, grouped::ComputedVecsFromHeight, indexes, price, txins};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        txins: &txins::Vecs,
        starting_indexes: &ComputeIndexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_tx_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.tx.height_to_first_txindex,
                    &indexer.vecs.tx.txindex_to_txid,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_input_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.transaction.txindex_to_input_count),
        )?;

        self.indexes_to_output_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.transaction.txindex_to_output_count),
        )?;

        let compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedVecsFromHeight<StoredU64>, txversion: TxVersion| {
                let mut txindex_to_txversion_iter = indexer.vecs.tx.txindex_to_txversion.iter()?;
                indexes_to_tx_vany.compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_filtered_count_from_indexes(
                        starting_indexes.height,
                        &indexer.vecs.tx.height_to_first_txindex,
                        &indexer.vecs.tx.txindex_to_txid,
                        |txindex| {
                            let v = txindex_to_txversion_iter.get_unwrap(txindex);
                            v == txversion
                        },
                        exit,
                    )?;
                    Ok(())
                })
            };
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v1, TxVersion::ONE)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v2, TxVersion::TWO)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v3, TxVersion::THREE)?;

        self.txindex_to_input_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txinindex,
            &indexes.transaction.txindex_to_input_count,
            &txins.txinindex_to_value,
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
            &self.txindex_to_vsize,
            |(txindex, fee, vsize, ..)| (txindex, FeeRate::from((fee, vsize))),
            exit,
        )?;

        self.indexes_to_fee.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee),
            price,
        )?;

        self.indexes_to_fee_rate.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee_rate),
        )?;

        self.indexes_to_tx_weight.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_weight),
        )?;

        self.indexes_to_tx_vsize.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_vsize),
        )?;

        Ok(())
    }
}
