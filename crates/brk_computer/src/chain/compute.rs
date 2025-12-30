use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{indexes, price, txins, ComputeIndexes};

use super::Vecs;

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
        // Independent computations first
        self.block.compute(indexer, indexes, starting_indexes, exit)?;
        self.epoch.compute(indexes, starting_indexes, exit)?;
        self.transaction.compute(indexer, indexes, txins, starting_indexes, price, exit)?;

        // Coinbase depends on block and transaction
        self.coinbase.compute(
            indexer,
            indexes,
            &self.block,
            &self.transaction,
            starting_indexes,
            price,
            exit,
        )?;

        // Output type depends on transaction
        self.output_type.compute(indexer, indexes, &self.transaction, starting_indexes, exit)?;

        // Volume depends on transaction and coinbase
        self.volume.compute(
            indexer,
            indexes,
            &self.transaction,
            &self.coinbase,
            starting_indexes,
            price,
            exit,
        )?;

        // Mining depends on block and coinbase
        self.mining.compute(
            indexer,
            indexes,
            &self.block,
            &self.coinbase,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
