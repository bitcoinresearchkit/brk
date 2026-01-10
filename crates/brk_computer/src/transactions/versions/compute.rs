use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredU64, TxVersion};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{ComputeIndexes, indexes, internal::ComputedFromHeightSumCum};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let tx_vany = |tx_vany: &mut ComputedFromHeightSumCum<StoredU64>, txversion: TxVersion| {
            let mut txversion_iter = indexer.vecs.transactions.txversion.iter()?;
            tx_vany.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_filtered_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexer.vecs.transactions.txid,
                    |txindex| {
                        let v = txversion_iter.get_unwrap(txindex);
                        v == txversion
                    },
                    exit,
                )?;
                Ok(())
            })
        };
        tx_vany(&mut self.v1, TxVersion::ONE)?;
        tx_vany(&mut self.v2, TxVersion::TWO)?;
        tx_vany(&mut self.v3, TxVersion::THREE)?;

        Ok(())
    }
}
