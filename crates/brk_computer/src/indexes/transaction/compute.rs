use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;

impl Vecs {
    pub fn compute(&mut self, indexer: &Indexer, starting_indexes: &brk_indexer::Indexes, exit: &Exit) -> Result<()> {
        self.txindex_to_input_count.compute_count_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txinindex,
            &indexer.vecs.txin.txinindex_to_outpoint,
            exit,
        )?;

        self.txindex_to_output_count.compute_count_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txoutindex,
            &indexer.vecs.txout.txoutindex_to_value,
            exit,
        )?;

        Ok(())
    }
}
