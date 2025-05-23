use std::{fs, path::Path};

use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, Computation};

pub mod blocks;
pub mod constants;
pub mod fetched;
pub mod grouped;
pub mod indexes;
pub mod market;
pub mod mining;
pub mod transactions;
pub mod utxos;

pub use indexes::Indexes;

#[derive(Clone)]
pub struct Vecs {
    pub indexes: indexes::Vecs,
    pub constants: constants::Vecs,
    pub blocks: blocks::Vecs,
    pub mining: mining::Vecs,
    pub market: market::Vecs,
    pub transactions: transactions::Vecs,
    pub utxos: utxos::Vecs,
    pub fetched: Option<fetched::Vecs>,
}

impl Vecs {
    pub fn import(
        path: &Path,
        indexer: &Indexer,
        fetch: bool,
        computation: Computation,
        compressed: Compressed,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let indexes = indexes::Vecs::forced_import(path, indexer, computation, compressed)?;

        let fetched =
            fetch.then(|| fetched::Vecs::forced_import(path, computation, compressed).unwrap());

        Ok(Self {
            blocks: blocks::Vecs::forced_import(path, computation, compressed)?,
            mining: mining::Vecs::forced_import(path, computation, compressed)?,
            constants: constants::Vecs::forced_import(path, computation, compressed)?,
            market: market::Vecs::forced_import(path, computation, compressed)?,
            utxos: utxos::Vecs::forced_import(path, computation, compressed, fetched.as_ref())?,
            transactions: transactions::Vecs::forced_import(
                path,
                indexer,
                &indexes,
                computation,
                compressed,
                fetched.as_ref(),
            )?,
            indexes,
            fetched,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        fetcher: Option<&mut Fetcher>,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let starting_indexes = self.indexes.compute(indexer, starting_indexes, exit)?;

        self.constants
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        self.blocks
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        self.mining
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        if let Some(fetched) = self.fetched.as_mut() {
            fetched.compute(
                indexer,
                &self.indexes,
                &starting_indexes,
                fetcher.unwrap(),
                exit,
            )?;
        }

        self.transactions.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            self.fetched.as_ref(),
            exit,
        )?;

        if let Some(fetched) = self.fetched.as_ref() {
            self.market.compute(
                indexer,
                &self.indexes,
                fetched,
                &mut self.transactions,
                &starting_indexes,
                exit,
            )?;
        }

        self.utxos.compute(
            indexer,
            &self.indexes,
            &self.transactions,
            self.fetched.as_ref(),
            &starting_indexes,
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.constants.vecs(),
            self.indexes.vecs(),
            self.blocks.vecs(),
            self.mining.vecs(),
            self.market.vecs(),
            self.transactions.vecs(),
            self.utxos.vecs(),
            self.fetched.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
