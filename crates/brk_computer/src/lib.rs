#![doc = include_str!("../README.md")]

use std::{path::Path, thread};

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_structs::Version;
use log::info;
use vecdb::{AnyCollectableVec, Exit, Format};

mod chain;
mod cointime;
mod constants;
mod fetched;
mod grouped;
mod indexes;
mod market;
mod price;
mod stateful;
mod states;
mod traits;
mod utils;

use indexes::Indexes;

pub use states::PriceToAmount;
use states::*;

#[derive(Clone)]
pub struct Computer {
    pub indexes: indexes::Vecs,
    pub constants: constants::Vecs,
    pub market: market::Vecs,
    pub price: Option<price::Vecs>,
    pub chain: chain::Vecs,
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
        info!("Importing computer...");

        let computed_path = outputs_path.join("computed");

        let indexes =
            indexes::Vecs::forced_import(&computed_path, VERSION + Version::ZERO, indexer)?;

        let fetched = fetcher.map(|fetcher| {
            fetched::Vecs::forced_import(outputs_path, fetcher, VERSION + Version::ZERO).unwrap()
        });

        let price = fetched.is_some().then(|| {
            price::Vecs::forced_import(&computed_path, VERSION + Version::ZERO, &indexes).unwrap()
        });

        Ok(Self {
            constants: constants::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                &indexes,
            )?,
            market: market::Vecs::forced_import(&computed_path, VERSION + Version::ZERO, &indexes)?,
            stateful: stateful::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                Format::Compressed,
                &indexes,
                price.as_ref(),
            )?,
            chain: chain::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
                indexer,
                &indexes,
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

        thread::scope(|scope| -> Result<()> {
            let chain = scope.spawn(|| {
                info!("Computing chain...");
                self.chain.compute(
                    indexer,
                    &self.indexes,
                    &starting_indexes,
                    self.price.as_ref(),
                    exit,
                )
            });

            let market = scope.spawn(|| -> Result<()> {
                if let Some(price) = self.price.as_ref() {
                    info!("Computing market...");
                    self.market
                        .compute(indexer, &self.indexes, price, &starting_indexes, exit)?;
                }
                Ok(())
            });

            chain.join().unwrap()?;
            market.join().unwrap()?;
            Ok(())
        })?;

        info!("Computing stateful...");
        self.stateful.compute(
            indexer,
            &self.indexes,
            &self.chain,
            self.price.as_ref(),
            &mut starting_indexes,
            exit,
        )?;

        info!("Computing cointime...");
        self.cointime.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            self.price.as_ref(),
            &self.chain,
            &self.stateful,
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.constants.vecs(),
            self.indexes.vecs(),
            self.market.vecs(),
            self.chain.vecs(),
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
