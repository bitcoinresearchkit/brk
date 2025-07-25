use std::{path::Path, sync::Arc};

use brk_core::Version;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vecs::{AnyCollectableVec, Computation, File, Format};
use log::info;

use crate::{blocks, cointime, constants, fetched, indexes, market, mining, transactions};

use super::stateful;

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
    #[allow(clippy::too_many_arguments)]
    pub fn import(
        file: &Arc<File>,
        version: Version,
        indexer: &Indexer,
        fetch: bool,
        computation: Computation,
        format: Format,
        fetched_file: &Arc<File>,
        states_path: &Path,
    ) -> color_eyre::Result<Self> {
        let indexes = indexes::Vecs::forced_import(
            file,
            version + VERSION + Version::ZERO,
            indexer,
            computation,
            format,
        )?;

        let fetched = fetch.then(|| {
            fetched::Vecs::forced_import(
                file,
                fetched_file,
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
                fetched.as_ref(),
                states_path,
            )?,
            transactions: transactions::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                indexer,
                &indexes,
                computation,
                format,
                fetched.as_ref(),
            )?,
            cointime: cointime::Vecs::forced_import(
                file,
                version + VERSION + Version::ZERO,
                computation,
                format,
                &indexes,
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
