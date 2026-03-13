use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU64, TxVersion};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::internal::ComputedPerBlockCumulativeWithSums;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let tx_vany = |tx_vany: &mut ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64>,
                       txversion: TxVersion| {
            let txversion_vec = &indexer.vecs.transactions.txversion;
            // Cursor avoids per-transaction PcoVec page decompression.
            // Txindex values are sequential, so the cursor only advances forward.
            let mut cursor = txversion_vec.cursor();
            tx_vany.compute(starting_indexes.height, exit, |vec| {
                vec.compute_filtered_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexer.vecs.transactions.txid,
                    |txindex| {
                        let ti = txindex.to_usize();
                        cursor.advance(ti - cursor.position());
                        cursor.next().unwrap() == txversion
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
