use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredU64, TxVersion};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, blocks, internal::ComputedFromHeightCumulativeSum};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        count_vecs: &blocks::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = count_vecs.window_starts();

        let tx_vany = |tx_vany: &mut ComputedFromHeightCumulativeSum<StoredU64>, txversion: TxVersion| {
            let txversion_vec = &indexer.vecs.transactions.txversion;
            // Cursor avoids per-transaction PcoVec page decompression.
            // Txindex values are sequential, so the cursor only advances forward.
            let mut cursor = txversion_vec.cursor();
            tx_vany.compute(starting_indexes.height, &window_starts, exit, |vec| {
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
