#![doc = include_str!("../README.md")]

use std::{fs, path::Path, thread, time::Instant};

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::Traversable;
use brk_types::Version;
use tracing::info;
use vecdb::Exit;

mod blocks;
mod cointime;
mod constants;
mod distribution;
pub mod indexes;
mod inputs;
mod internal;
mod market;
mod outputs;
mod pools;
mod positions;
pub mod price;
mod scripts;
mod supply;
mod traits;
mod transactions;
mod utils;

use indexes::ComputeIndexes;

#[derive(Clone, Traversable)]
pub struct Computer {
    pub blocks: blocks::Vecs,
    pub transactions: transactions::Vecs,
    pub scripts: scripts::Vecs,
    pub positions: positions::Vecs,
    pub cointime: cointime::Vecs,
    pub constants: constants::Vecs,
    pub indexes: indexes::Vecs,
    pub market: market::Vecs,
    pub pools: pools::Vecs,
    pub price: Option<price::Vecs>,
    pub distribution: distribution::Vecs,
    pub supply: supply::Vecs,
    pub inputs: inputs::Vecs,
    pub outputs: outputs::Vecs,
}

const VERSION: Version = Version::new(4);

impl Computer {
    /// Do NOT import multiple times or things will break !!!
    pub fn forced_import(
        outputs_path: &Path,
        indexer: &Indexer,
        fetcher: Option<Fetcher>,
    ) -> Result<Self> {
        info!("Importing computer...");
        let import_start = Instant::now();

        let computed_path = outputs_path.join("computed");

        const STACK_SIZE: usize = 512 * 1024 * 1024;
        let big_thread = || thread::Builder::new().stack_size(STACK_SIZE);

        let i = Instant::now();
        let (indexes, positions) = thread::scope(|s| -> Result<_> {
            let positions_handle = big_thread().spawn_scoped(s, || {
                positions::Vecs::forced_import(&computed_path, VERSION)
            })?;

            let indexes = indexes::Vecs::forced_import(&computed_path, VERSION, indexer)?;
            let positions = positions_handle.join().unwrap()?;

            Ok((indexes, positions))
        })?;
        info!("Imported indexes/positions in {:?}", i.elapsed());

        // inputs/outputs need indexes for count imports
        let i = Instant::now();
        let (inputs, outputs) = thread::scope(|s| -> Result<_> {
            let inputs_handle = big_thread().spawn_scoped(s, || {
                inputs::Vecs::forced_import(&computed_path, VERSION, &indexes)
            })?;

            let outputs_handle = big_thread().spawn_scoped(s, || {
                outputs::Vecs::forced_import(&computed_path, VERSION, &indexes)
            })?;

            let inputs = inputs_handle.join().unwrap()?;
            let outputs = outputs_handle.join().unwrap()?;

            Ok((inputs, outputs))
        })?;
        info!("Imported inputs/outputs in {:?}", i.elapsed());

        let i = Instant::now();
        let constants = constants::Vecs::new(VERSION, &indexes);
        // Price must be created before market since market's lazy vecs reference price
        let price = price::Vecs::forced_import(&computed_path, VERSION, &indexes, fetcher)?;
        let price = price.has_fetcher().then_some(price);
        info!("Imported price/constants in {:?}", i.elapsed());

        let i = Instant::now();
        let (blocks, transactions, scripts, pools, cointime) = thread::scope(|s| -> Result<_> {
            // Import blocks module
            let blocks_handle = big_thread().spawn_scoped(s, || {
                blocks::Vecs::forced_import(
                    &computed_path,
                    VERSION,
                    indexer,
                    &indexes,
                    price.as_ref(),
                )
            })?;

            // Import transactions module
            let transactions_handle = big_thread().spawn_scoped(s, || {
                transactions::Vecs::forced_import(
                    &computed_path,
                    VERSION,
                    indexer,
                    &indexes,
                    price.as_ref(),
                )
            })?;

            // Import scripts module (depends on outputs for adoption ratio denominators)
            let scripts_handle = big_thread().spawn_scoped(s, || {
                scripts::Vecs::forced_import(
                    &computed_path,
                    VERSION,
                    &indexes,
                    price.as_ref(),
                    &outputs,
                )
            })?;

            let cointime =
                cointime::Vecs::forced_import(&computed_path, VERSION, &indexes, price.as_ref())?;

            let blocks = blocks_handle.join().unwrap()?;
            let transactions = transactions_handle.join().unwrap()?;
            let scripts = scripts_handle.join().unwrap()?;

            // pools depends on blocks and transactions for lazy dominance vecs
            let pools = pools::Vecs::forced_import(
                &computed_path,
                VERSION,
                &indexes,
                price.as_ref(),
                &blocks,
                &transactions,
            )?;

            Ok((blocks, transactions, scripts, pools, cointime))
        })?;
        info!(
            "Imported blocks/transactions/scripts/pools/cointime in {:?}",
            i.elapsed()
        );

        // Threads inside
        let i = Instant::now();
        let distribution =
            distribution::Vecs::forced_import(&computed_path, VERSION, &indexes, price.as_ref())?;
        info!("Imported distribution in {:?}", i.elapsed());

        // Supply must be imported after distribution (references distribution's supply)
        let i = Instant::now();
        let supply = supply::Vecs::forced_import(
            &computed_path,
            VERSION,
            &indexes,
            price.as_ref(),
            &distribution,
        )?;
        info!("Imported supply in {:?}", i.elapsed());

        // Market must be imported after distribution and transactions (for NVT indicator)
        let i = Instant::now();
        let market = market::Vecs::forced_import(
            &computed_path,
            VERSION,
            &indexes,
            price.as_ref(),
            &distribution,
            &transactions,
        )?;
        info!("Imported market in {:?}", i.elapsed());

        info!("Total import time: {:?}", import_start.elapsed());

        let this = Self {
            blocks,
            transactions,
            scripts,
            constants,
            market,
            distribution,
            supply,
            positions,
            pools,
            cointime,
            indexes,
            inputs,
            price,
            outputs,
        };

        Self::retain_databases(&computed_path)?;

        Ok(this)
    }

    /// Removes database folders that are no longer in use.
    fn retain_databases(computed_path: &Path) -> Result<()> {
        const EXPECTED_DBS: &[&str] = &[
            blocks::DB_NAME,
            transactions::DB_NAME,
            scripts::DB_NAME,
            positions::DB_NAME,
            cointime::DB_NAME,
            indexes::DB_NAME,
            market::DB_NAME,
            pools::DB_NAME,
            price::DB_NAME,
            distribution::DB_NAME,
            supply::DB_NAME,
            inputs::DB_NAME,
            outputs::DB_NAME,
        ];

        if !computed_path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(computed_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if !file_type.is_dir() {
                continue;
            }

            if let Some(name) = entry.file_name().to_str()
                && !EXPECTED_DBS.contains(&name)
            {
                info!("Removing obsolete database folder: {}", name);
                fs::remove_dir_all(entry.path())?;
            }
        }

        Ok(())
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        reader: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        let compute_start = Instant::now();

        // Compute blocks.time early (height_to_date, height_to_timestamp_monotonic, height_to_date_monotonic)
        // These are needed by indexes::block to compute height_to_dateindex
        info!("Computing blocks.time (early)...");
        let i = Instant::now();
        self.blocks
            .time
            .compute_early(indexer, starting_indexes.height, exit)?;
        info!("Computed blocks.time (early) in {:?}", i.elapsed());

        info!("Computing indexes...");
        let i = Instant::now();
        let mut starting_indexes =
            self.indexes
                .compute(indexer, &self.blocks.time, starting_indexes, exit)?;
        info!("Computed indexes in {:?}", i.elapsed());

        if let Some(price) = self.price.as_mut() {
            info!("Fetching prices...");
            let i = Instant::now();
            price.fetch(indexer, &self.indexes, &starting_indexes, exit)?;
            info!("Fetched prices in {:?}", i.elapsed());

            info!("Computing prices...");
            let i = Instant::now();
            price.compute(indexer, &self.indexes, &starting_indexes, exit)?;
            info!("Computed prices in {:?}", i.elapsed());
        }

        thread::scope(|scope| -> Result<()> {
            let positions = scope.spawn(|| -> Result<()> {
                info!("Computing positions metadata...");
                let i = Instant::now();
                self.positions
                    .compute(indexer, &starting_indexes, reader, exit)?;
                info!("Computed positions in {:?}", i.elapsed());
                Ok(())
            });

            // Inputs must complete first
            info!("Computing inputs...");
            let i = Instant::now();
            self.inputs
                .compute(indexer, &self.indexes, &starting_indexes, exit)?;
            info!("Computed inputs in {:?}", i.elapsed());

            // Scripts (needed for outputs.count.utxo_count)
            info!("Computing scripts...");
            let i = Instant::now();
            self.scripts
                .compute(indexer, &self.indexes, &starting_indexes, exit)?;
            info!("Computed scripts in {:?}", i.elapsed());

            // Outputs depends on inputs and scripts (for utxo_count)
            info!("Computing outputs...");
            let i = Instant::now();
            self.outputs.compute(
                indexer,
                &self.indexes,
                &self.inputs,
                &self.scripts,
                &starting_indexes,
                exit,
            )?;
            info!("Computed outputs in {:?}", i.elapsed());

            // Transactions: count, versions, size, fees, volume
            info!("Computing transactions...");
            let i = Instant::now();
            self.transactions.compute(
                indexer,
                &self.indexes,
                &self.inputs,
                &self.outputs,
                &starting_indexes,
                exit,
            )?;
            info!("Computed transactions in {:?}", i.elapsed());

            // Blocks depends on transactions.fees for rewards computation
            info!("Computing blocks...");
            let i = Instant::now();
            self.blocks.compute(
                indexer,
                &self.indexes,
                &self.transactions,
                &starting_indexes,
                exit,
            )?;
            info!("Computed blocks in {:?}", i.elapsed());

            positions.join().unwrap()?;
            Ok(())
        })?;

        let starting_indexes_clone = starting_indexes.clone();
        thread::scope(|scope| -> Result<()> {
            let pools = scope.spawn(|| -> Result<()> {
                info!("Computing pools...");
                let i = Instant::now();
                self.pools.compute(
                    indexer,
                    &self.indexes,
                    &self.blocks,
                    &starting_indexes_clone,
                    exit,
                )?;
                info!("Computed pools in {:?}", i.elapsed());
                Ok(())
            });

            info!("Computing distribution...");
            let i = Instant::now();
            self.distribution.compute(
                indexer,
                &self.indexes,
                &self.inputs,
                &self.outputs,
                &self.transactions,
                &self.blocks,
                self.price.as_ref(),
                &mut starting_indexes,
                exit,
            )?;
            info!("Computed distribution in {:?}", i.elapsed());

            pools.join().unwrap()?;
            Ok(())
        })?;

        // Market must be computed after distribution (uses distribution data for gini)
        if let Some(price) = self.price.as_ref() {
            info!("Computing market...");
            let i = Instant::now();
            self.market.compute(
                price,
                &self.blocks,
                &self.distribution,
                &starting_indexes,
                exit,
            )?;
            info!("Computed market in {:?}", i.elapsed());
        }

        // Supply must be computed after distribution (uses actual circulating supply)
        info!("Computing supply...");
        let i = Instant::now();
        self.supply.compute(
            &self.indexes,
            &self.scripts,
            &self.blocks,
            &self.transactions,
            &self.distribution,
            &starting_indexes,
            exit,
        )?;
        info!("Computed supply in {:?}", i.elapsed());

        info!("Computing cointime...");
        let i = Instant::now();
        self.cointime.compute(
            &self.indexes,
            &starting_indexes,
            self.price.as_ref(),
            &self.blocks,
            &self.supply,
            &self.distribution,
            exit,
        )?;
        info!("Computed cointime in {:?}", i.elapsed());

        info!("Total compute time: {:?}", compute_start.elapsed());
        Ok(())
    }

    /// Iterate over all exportable vecs with their database name.
    pub fn iter_named_exportable(
        &self,
    ) -> impl Iterator<Item = (&'static str, &dyn vecdb::AnyExportableVec)> {
        use brk_traversable::Traversable;

        std::iter::empty()
            .chain(
                self.blocks
                    .iter_any_exportable()
                    .map(|v| (blocks::DB_NAME, v)),
            )
            .chain(
                self.transactions
                    .iter_any_exportable()
                    .map(|v| (transactions::DB_NAME, v)),
            )
            .chain(
                self.scripts
                    .iter_any_exportable()
                    .map(|v| (scripts::DB_NAME, v)),
            )
            .chain(
                self.positions
                    .iter_any_exportable()
                    .map(|v| (positions::DB_NAME, v)),
            )
            .chain(
                self.cointime
                    .iter_any_exportable()
                    .map(|v| (cointime::DB_NAME, v)),
            )
            .chain(
                self.constants
                    .iter_any_exportable()
                    .map(|v| (constants::DB_NAME, v)),
            )
            .chain(
                self.indexes
                    .iter_any_exportable()
                    .map(|v| (indexes::DB_NAME, v)),
            )
            .chain(
                self.market
                    .iter_any_exportable()
                    .map(|v| (market::DB_NAME, v)),
            )
            .chain(
                self.pools
                    .iter_any_exportable()
                    .map(|v| (pools::DB_NAME, v)),
            )
            .chain(
                self.price
                    .iter_any_exportable()
                    .map(|v| (price::DB_NAME, v)),
            )
            .chain(
                self.distribution
                    .iter_any_exportable()
                    .map(|v| (distribution::DB_NAME, v)),
            )
            .chain(
                self.supply
                    .iter_any_exportable()
                    .map(|v| (supply::DB_NAME, v)),
            )
            .chain(
                self.inputs
                    .iter_any_exportable()
                    .map(|v| (inputs::DB_NAME, v)),
            )
            .chain(
                self.outputs
                    .iter_any_exportable()
                    .map(|v| (outputs::DB_NAME, v)),
            )
    }
}
