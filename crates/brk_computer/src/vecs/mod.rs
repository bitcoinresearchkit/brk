use std::{fs, path::Path, thread};

use brk_core::Version;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Computation, Format};

pub mod blocks;
pub mod cointime;
pub mod constants;
pub mod fetched;
pub mod grouped;
pub mod indexes;
pub mod market;
pub mod mining;
pub mod stateful;
pub mod transactions;

pub use indexes::Indexes;
use log::info;

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
    pub cointime: cointime::Vecs,
}

impl Vecs {
    pub fn import(
        path: &Path,
        version: Version,
        indexer: &Indexer,
        fetch: bool,
        computation: Computation,
        format: Format,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let (indexes, fetched) = thread::scope(|s| {
            let indexes_handle = s.spawn(|| {
                indexes::Vecs::forced_import(
                    path,
                    version + VERSION + Version::ZERO,
                    indexer,
                    computation,
                    format,
                )
                .unwrap()
            });

            let fetch_handle = s.spawn(|| {
                fetch.then(|| {
                    fetched::Vecs::forced_import(
                        path,
                        version + VERSION + Version::ZERO,
                        computation,
                        format,
                    )
                    .unwrap()
                })
            });

            (indexes_handle.join().unwrap(), fetch_handle.join().unwrap())
        });

        Ok(Self {
            blocks: blocks::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                format,
            )?,
            mining: mining::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                format,
            )?,
            constants: constants::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                format,
            )?,
            market: market::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                format,
            )?,
            stateful: stateful::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                format,
                fetched.as_ref(),
            )?,
            transactions: transactions::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                indexer,
                &indexes,
                computation,
                format,
                fetched.as_ref(),
            )?,
            cointime: cointime::Vecs::forced_import(
                path,
                version + VERSION + Version::ZERO,
                computation,
                format,
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
        info!("Computing indexes...");
        let mut starting_indexes = self.indexes.compute(indexer, starting_indexes, exit)?;

        info!("Computing constants...");
        self.constants
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        info!("Computing blocks...");
        self.blocks
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        info!("Computing mining...");
        self.mining
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;

        if let Some(fetched) = self.fetched.as_mut() {
            info!("Computing fetched...");
            fetched.compute(
                indexer,
                &self.indexes,
                &starting_indexes,
                fetcher.unwrap(),
                exit,
            )?;
        }

        info!("Computing transactions...");
        self.transactions.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            self.fetched.as_ref(),
            exit,
        )?;

        if let Some(fetched) = self.fetched.as_ref() {
            info!("Computing market...");
            self.market.compute(
                indexer,
                &self.indexes,
                fetched,
                &mut self.transactions,
                &starting_indexes,
                exit,
            )?;
        }

        info!("Computing stateful...");
        self.stateful.compute(
            indexer,
            &self.indexes,
            &self.transactions,
            self.fetched.as_ref(),
            &self.market,
            &mut starting_indexes,
            exit,
        )?;

        self.cointime.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            self.fetched.as_ref(),
            &self.transactions,
            &self.stateful,
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
            self.cointime.vecs(),
            self.fetched.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
