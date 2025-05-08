use std::{fs, path::Path};

use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, Computation};

pub mod blocks;
pub mod grouped;
pub mod indexes;
pub mod marketprice;
pub mod mining;
pub mod transactions;

pub use indexes::Indexes;

#[derive(Clone)]
pub struct Vecs {
    pub indexes: indexes::Vecs,
    pub blocks: blocks::Vecs,
    pub mining: mining::Vecs,
    pub transactions: transactions::Vecs,
    pub marketprice: Option<marketprice::Vecs>,
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

        let marketprice =
            fetch.then(|| marketprice::Vecs::forced_import(path, computation, compressed).unwrap());

        Ok(Self {
            blocks: blocks::Vecs::forced_import(path, computation, compressed)?,
            mining: mining::Vecs::forced_import(path, computation, compressed)?,
            transactions: transactions::Vecs::forced_import(
                path,
                indexer,
                &indexes,
                computation,
                compressed,
                marketprice.as_ref(),
            )?,
            indexes,
            marketprice,
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

        self.blocks
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        self.mining
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        if let Some(marketprice) = self.marketprice.as_mut() {
            marketprice.compute(
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
            self.marketprice.as_ref(),
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.indexes.vecs(),
            self.blocks.vecs(),
            self.mining.vecs(),
            self.transactions.vecs(),
            self.marketprice.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .concat()
    }
}
