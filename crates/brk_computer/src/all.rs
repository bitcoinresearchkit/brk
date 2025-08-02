use std::{path::Path, sync::Arc};

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_structs::Version;
use brk_vecs::{AnyCollectableVec, Computation, Exit, File, Format};
use log::info;

use crate::{blocks, cointime, constants, fetched, indexes, market, mining, price, transactions};

use super::stateful;

const VERSION: Version = Version::ONE;

#[derive(Clone)]
pub struct Vecs {
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

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn import(
        file: &Arc<File>,
        version: Version,
        indexer: &Indexer,
        fetcher: Option<Fetcher>,
        computation: Computation,
        format: Format,
        fetched_file: &Arc<File>,
        states_path: &Path,
    ) -> Result<Self> {
        let indexes = indexes::Vecs::forced_import(
            file,
            version + VERSION + Version::ZERO,
            indexer,
            computation,
            format,
        )?;

        let fetched = fetcher.map(|fetcher| {
            fetched::Vecs::forced_import(
                file,
                fetched_file,
                fetcher,
                version + VERSION + Version::ZERO,
            )
            .unwrap()
        });

        let price = fetched.is_some().then(|| {
            price::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
            )
            .unwrap()
        });

        Ok(Self {
            blocks: blocks::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
            )?,
            mining: mining::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
            )?,
            constants: constants::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
            )?,
            market: market::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
            )?,
            stateful: stateful::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
                price.as_ref(),
                states_path,
            )?,
            transactions: transactions::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                indexer,
                &indexes,
                computation,
                format,
                price.as_ref(),
            )?,
            cointime: cointime::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
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
}
