use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU64, TxVersion};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::internal::PerBlockCumulativeWithSums;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let tx_vany = |tx_vany: &mut PerBlockCumulativeWithSums<StoredU64, StoredU64>,
                       tx_version: TxVersion| {
            let tx_version_vec = &indexer.vecs.transactions.tx_version;
            // Cursor avoids per-transaction PcoVec page decompression.
            // Txindex values are sequential, so the cursor only advances forward.
            let mut cursor = tx_version_vec.cursor();
            tx_vany.compute(starting_indexes.height, exit, |vec| {
                vec.compute_filtered_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_tx_index,
                    &indexer.vecs.transactions.txid,
                    |tx_index| {
                        let ti = tx_index.to_usize();
                        cursor.advance(ti - cursor.position());
                        cursor.next().unwrap() == tx_version
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
