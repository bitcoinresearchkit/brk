use std::{fs, path::Path};

use brk_core::Version;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, Computation};
use fjall::TransactionalKeyspace;

pub mod blocks;
pub mod constants;
pub mod fetched;
pub mod grouped;
pub mod indexes;
pub mod market;
pub mod mining;
pub mod stateful;
pub mod transactions;

pub use indexes::Indexes;

const VERSION: Version = Version::ONE;

#[derive(Clone)]
pub struct Vecs {
    pub indexes: indexes::Vecs,
    pub constants: constants::Vecs,
    pub blocks: blocks::Vecs,
    pub mining: mining::Vecs,
    pub market: market::Vecs,
    pub transactions: transactions::Vecs,
    pub stateful: stateful::Vecs,
    pub fetched: Option<fetched::Vecs>,
}

impl Vecs {
    pub fn import(
        path: &Path,
        version: Version,
        indexer: &Indexer,
        fetch: bool,
        computation: Computation,
        compressed: Compressed,
        keyspace: &TransactionalKeyspace,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let indexes = indexes::Vecs::forced_import(
            path,
            version + VERSION + Version::ZERO,
            indexer,
            computation,
            compressed,
        )?;

        let fetched = fetch.then(|| {
            fetched::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                compressed,
            )
            .unwrap()
        });

        Ok(Self {
            blocks: blocks::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                compressed,
            )?,
            mining: mining::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                compressed,
            )?,
            constants: constants::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                compressed,
            )?,
            market: market::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                compressed,
            )?,
            stateful: stateful::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                compressed,
                fetched.as_ref(),
                keyspace,
            )?,
            transactions: transactions::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
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

        self.stateful.compute(
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
            self.stateful.vecs(),
            self.fetched.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
