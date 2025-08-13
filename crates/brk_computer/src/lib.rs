#![doc = include_str!("../README.md")]

use std::path::Path;

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_structs::Version;
use log::info;
use vecdb::{AnyCollectableVec, Exit, Format};

mod blocks;
mod cointime;
mod constants;
mod fetched;
mod grouped;
mod indexes;
mod market;
mod mining;
mod price;
mod stateful;
mod states;
mod traits;
mod transactions;
mod utils;

use indexes::Indexes;

use states::*;

#[derive(Clone)]
pub struct Computer {
    pub indexes: indexes::Vecs,
    pub constants: constants::Vecs,
    pub blocks: blocks::Vecs,
    pub mining: mining::Vecs,
    pub market: market::Vecs,
    pub price: Option<price::Vecs>,
    pub transactions: transactions::Vecs,
    pub stateful: stateful::Vecs,
    pub fetched: Option<fetched::Vecs>,
    pub cointime: cointime::Vecs,
}

const VERSION: Version = Version::TWO;

impl Computer {
    /// Do NOT import multiple times or things will break !!!
    pub fn forced_import(
        outputs_path: &Path,
        indexer: &Indexer,
        fetcher: Option<Fetcher>,
    ) -> Result<Self> {
        let computed_path = outputs_path.join("computed");

        let indexes =
            indexes::Vecs::forced_import(&computed_path, VERSION + Version::ZERO, indexer)?;

        let fetched = fetcher.map(|fetcher| {
            fetched::Vecs::forced_import(outputs_path, fetcher, VERSION + Version::ZERO).unwrap()
        });

        let format = Format::Compressed;

        let price = fetched.is_some().then(|| {
            price::Vecs::forced_import(&computed_path, VERSION + Version::ZERO, format, &indexes)
                .unwrap()
        });

        Ok(Self {
            blocks: blocks::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                format,
                &indexes,
            )?,
            mining: mining::Vecs::forced_import(&computed_path, VERSION + Version::ZERO, &indexes)?,
            constants: constants::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                &indexes,
            )?,
            market: market::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                format,
                &indexes,
            )?,
            stateful: stateful::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                format,
                &indexes,
                price.as_ref(),
                &computed_path.join("states"),
            )?,
            transactions: transactions::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                indexer,
                &indexes,
                format,
                price.as_ref(),
            )?,
            cointime: cointime::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                &indexes,
                price.as_ref(),
            )?,
            indexes,
            fetched,
            price,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<()> {
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
            fetched.compute(indexer, &self.indexes, &starting_indexes, exit)?;

            info!("Computing prices...");
            self.price.as_mut().unwrap().compute(
                indexer,
                &self.indexes,
                &starting_indexes,
                fetched,
                exit,
            )?;
        }

        info!("Computing transactions...");
        self.transactions.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            self.price.as_ref(),
            exit,
        )?;

        if let Some(price) = self.price.as_ref() {
            info!("Computing market...");
            self.market.compute(
                indexer,
                &self.indexes,
                price,
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
            self.price.as_ref(),
            &self.market,
            &mut starting_indexes,
            exit,
        )?;

        self.cointime.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            self.price.as_ref(),
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
            self.price.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }

    pub fn static_clone(&self) -> &'static Self {
        Box::leak(Box::new(self.clone()))
    }
}
