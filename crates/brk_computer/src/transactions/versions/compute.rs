use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredU64, TxVersion};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{indexes, internal::ComputedBlockSumCum, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedBlockSumCum<StoredU64>, txversion: TxVersion| {
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

        Ok(())
    }
}
