use std::{fs, path::Path};

use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyStoredVec, Compressed};

pub mod base;
pub mod blocks;
pub mod grouped;
pub mod indexes;
pub mod marketprice;
pub mod transactions;

use base::*;
use indexes::*;

#[derive(Clone)]
pub struct Vecs {
    pub blocks: blocks::Vecs,
    pub indexes: indexes::Vecs,
    pub transactions: transactions::Vecs,
    pub marketprice: Option<marketprice::Vecs>,
}

impl Vecs {
    pub fn import(path: &Path, fetch: bool, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            blocks: blocks::Vecs::forced_import(path, compressed)?,
            indexes: indexes::Vecs::forced_import(path, compressed)?,
            transactions: transactions::Vecs::forced_import(path, compressed, fetch)?,
            marketprice: fetch.then(|| marketprice::Vecs::forced_import(path, compressed).unwrap()),
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        starting_indexes: brk_indexer::Indexes,
        fetcher: Option<&mut Fetcher>,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let starting_indexes = self.indexes.compute(indexer, starting_indexes, exit)?;

        self.blocks
            .compute(indexer, &mut self.indexes, &starting_indexes, exit)?;

        if let Some(marketprice) = self.marketprice.as_mut() {
            marketprice.compute(
                indexer,
                &mut self.indexes,
                &starting_indexes,
                fetcher.unwrap(),
                exit,
            )?;
        }

        self.transactions.compute(
            indexer,
            &mut self.indexes,
            &starting_indexes,
            &mut self.marketprice.as_mut(),
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            self.indexes.as_any_vecs(),
            self.blocks.as_any_vecs(),
            self.transactions.as_any_vecs(),
            self.marketprice
                .as_ref()
                .map_or(vec![], |v| v.as_any_vecs()),
        ]
        .concat()
    }
}
