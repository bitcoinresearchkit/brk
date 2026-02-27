#![doc = include_str!("../README.md")]

use std::{fs, path::Path, thread, time::Instant};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::Traversable;
use brk_types::Version;
use tracing::info;
use vecdb::{Exit, Ro, Rw, StorageMode};

mod blocks;
mod cointime;
mod constants;
mod distribution;
pub mod indexes;
mod inputs;
mod internal;
mod market;
mod mining;
mod outputs;
mod pools;
mod positions;
pub mod prices;
mod scripts;
mod supply;
mod traits;
mod transactions;

use indexes::ComputeIndexes;

#[derive(Traversable)]
pub struct Computer<M: StorageMode = Rw> {
    pub blocks: Box<blocks::Vecs<M>>,
    pub mining: Box<mining::Vecs<M>>,
    pub transactions: Box<transactions::Vecs<M>>,
    pub scripts: Box<scripts::Vecs<M>>,
    pub positions: Box<positions::Vecs<M>>,
    pub cointime: Box<cointime::Vecs<M>>,
    pub constants: Box<constants::Vecs>,
    pub indexes: Box<indexes::Vecs<M>>,
    pub market: Box<market::Vecs<M>>,
    pub pools: Box<pools::Vecs<M>>,
    pub prices: Box<prices::Vecs<M>>,
    pub distribution: Box<distribution::Vecs<M>>,
    pub supply: Box<supply::Vecs<M>>,
    pub inputs: Box<inputs::Vecs<M>>,
    pub outputs: Box<outputs::Vecs<M>>,
}

const VERSION: Version = Version::new(5);

impl Computer {
    /// Do NOT import multiple times or things will break !!!
    pub fn forced_import(outputs_path: &Path, indexer: &Indexer) -> Result<Self> {
        info!("Importing computer...");
        let import_start = Instant::now();

        let computed_path = outputs_path.join("computed");

        const STACK_SIZE: usize = 8 * 1024 * 1024;
        let big_thread = || thread::Builder::new().stack_size(STACK_SIZE);

        let i = Instant::now();
        let (indexes, positions) = thread::scope(|s| -> Result<_> {
            let positions_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                Ok(Box::new(positions::Vecs::forced_import(&computed_path, VERSION)?))
            })?;

            let indexes = Box::new(indexes::Vecs::forced_import(&computed_path, VERSION, indexer)?);
            let positions = positions_handle.join().unwrap()?;

            Ok((indexes, positions))
        })?;
        info!("Imported indexes/positions in {:?}", i.elapsed());

        // inputs/outputs need indexes for count imports
        let i = Instant::now();
        let (inputs, outputs) = thread::scope(|s| -> Result<_> {
            let inputs_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                Ok(Box::new(inputs::Vecs::forced_import(&computed_path, VERSION, &indexes)?))
            })?;

            let outputs_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                Ok(Box::new(outputs::Vecs::forced_import(&computed_path, VERSION, &indexes)?))
            })?;

            let inputs = inputs_handle.join().unwrap()?;
            let outputs = outputs_handle.join().unwrap()?;

            Ok((inputs, outputs))
        })?;
        info!("Imported inputs/outputs in {:?}", i.elapsed());

        let i = Instant::now();
        let constants = Box::new(constants::Vecs::new(VERSION, &indexes));
        // Price must be created before market since market's lazy vecs reference price
        let prices = Box::new(prices::Vecs::forced_import(&computed_path, VERSION, &indexes)?);
        info!("Imported price/constants in {:?}", i.elapsed());

        let i = Instant::now();
        let (blocks, mining, transactions, scripts, pools, cointime) =
            thread::scope(|s| -> Result<_> {
                // Import blocks module (no longer needs prices)
                let blocks_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                    Ok(Box::new(blocks::Vecs::forced_import(&computed_path, VERSION, indexer, &indexes)?))
                })?;

                // Import mining module (separate database)
                let mining_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                    Ok(Box::new(mining::Vecs::forced_import(&computed_path, VERSION, &indexes)?))
                })?;

                // Import transactions module
                let transactions_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                    Ok(Box::new(transactions::Vecs::forced_import(
                        &computed_path,
                        VERSION,
                        indexer,
                        &indexes,
                    )?))
                })?;

                let scripts_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                    Ok(Box::new(scripts::Vecs::forced_import(
                        &computed_path,
                        VERSION,
                        &indexes,
                    )?))
                })?;

                let cointime = Box::new(
                    cointime::Vecs::forced_import(&computed_path, VERSION, &indexes)?
                );

                let blocks = blocks_handle.join().unwrap()?;
                let mining = mining_handle.join().unwrap()?;
                let transactions = transactions_handle.join().unwrap()?;
                let scripts = scripts_handle.join().unwrap()?;

                let pools = Box::new(pools::Vecs::forced_import(
                    &computed_path,
                    VERSION,
                    &indexes,
                )?);

                Ok((blocks, mining, transactions, scripts, pools, cointime))
            })?;
        info!(
            "Imported blocks/mining/transactions/scripts/pools/cointime in {:?}",
            i.elapsed()
        );

        // Threads inside
        let i = Instant::now();
        let distribution = Box::new(
            distribution::Vecs::forced_import(&computed_path, VERSION, &indexes)?
        );
        info!("Imported distribution in {:?}", i.elapsed());

        // Supply must be imported after distribution (references distribution's supply)
        let i = Instant::now();
        let supply = Box::new(
            supply::Vecs::forced_import(&computed_path, VERSION, &indexes, &distribution)?
        );
        info!("Imported supply in {:?}", i.elapsed());

        let i = Instant::now();
        let market = Box::new(market::Vecs::forced_import(
            &computed_path,
            VERSION,
            &indexes,
        )?);
        info!("Imported market in {:?}", i.elapsed());

        info!("Total import time: {:?}", import_start.elapsed());

        let this = Self {
            blocks,
            mining,
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
            prices,
            outputs,
        };

        Self::retain_databases(&computed_path)?;

        Ok(this)
    }

    /// Removes database folders that are no longer in use.
    fn retain_databases(computed_path: &Path) -> Result<()> {
        const EXPECTED_DBS: &[&str] = &[
            blocks::DB_NAME,
            mining::DB_NAME,
            transactions::DB_NAME,
            scripts::DB_NAME,
            positions::DB_NAME,
            cointime::DB_NAME,
            indexes::DB_NAME,
            market::DB_NAME,
            pools::DB_NAME,
            prices::DB_NAME,
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

        // 1. Indexes (absorbs blocks.time.compute — timestamp_monotonic)
        info!("Computing indexes...");
        let i = Instant::now();
        let mut starting_indexes =
            self.indexes
                .compute(indexer, &mut self.blocks, starting_indexes, exit)?;
        info!("Computed indexes in {:?}", i.elapsed());

        // 2. Prices
        info!("Computing prices...");
        let i = Instant::now();
        self.prices
            .compute(indexer, &self.indexes, &starting_indexes, exit)?;
        info!("Computed prices in {:?}", i.elapsed());

        // 3. Main scope
        thread::scope(|scope| -> Result<()> {
            let positions = scope.spawn(|| -> Result<()> {
                info!("Computing positions metadata...");
                let i = Instant::now();
                self.positions
                    .compute(indexer, &starting_indexes, reader, exit)?;
                info!("Computed positions in {:?}", i.elapsed());
                Ok(())
            });

            // Blocks first (needed for window starts used by scripts, transactions, etc.)
            info!("Computing blocks...");
            let i = Instant::now();
            self.blocks
                .compute(indexer, &self.indexes, &starting_indexes, exit)?;
            info!("Computed blocks in {:?}", i.elapsed());

            // Inputs → scripts → outputs (sequential)
            info!("Computing inputs...");
            let i = Instant::now();
            self.inputs
                .compute(indexer, &self.indexes, &self.blocks, &starting_indexes, exit)?;
            info!("Computed inputs in {:?}", i.elapsed());

            info!("Computing scripts...");
            let i = Instant::now();
            self.scripts
                .compute(indexer, &self.blocks, &self.outputs, &self.prices, &starting_indexes, exit)?;
            info!("Computed scripts in {:?}", i.elapsed());

            info!("Computing outputs...");
            let i = Instant::now();
            self.outputs.compute(
                indexer,
                &self.indexes,
                &self.inputs,
                &self.scripts,
                &self.blocks,
                &starting_indexes,
                exit,
            )?;
            info!("Computed outputs in {:?}", i.elapsed());

            // Transactions (needs blocks for count/interval, prices for USD conversion)
            info!("Computing transactions...");
            let i = Instant::now();
            self.transactions.compute(
                indexer,
                &self.indexes,
                &self.blocks,
                &self.inputs,
                &self.outputs,
                &self.prices,
                &starting_indexes,
                exit,
            )?;
            info!("Computed transactions in {:?}", i.elapsed());

            // Mining (needs blocks + transactions)
            info!("Computing mining...");
            let i = Instant::now();
            self.mining.compute(
                indexer,
                &self.indexes,
                &self.blocks,
                &self.transactions,
                &self.prices,
                &starting_indexes,
                exit,
            )?;
            info!("Computed mining in {:?}", i.elapsed());

            positions.join().unwrap()?;
            Ok(())
        })?;

        if true {
            return Ok(());
        }

        // 4. Pools || distribution
        let starting_indexes_clone = starting_indexes.clone();
        thread::scope(|scope| -> Result<()> {
            let pools = scope.spawn(|| -> Result<()> {
                info!("Computing pools...");
                let i = Instant::now();
                self.pools.compute(
                    indexer,
                    &self.indexes,
                    &self.blocks,
                    &self.prices,
                    &self.mining,
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
                &self.prices,
                &mut starting_indexes,
                exit,
            )?;
            info!("Computed distribution in {:?}", i.elapsed());

            pools.join().unwrap()?;
            Ok(())
        })?;

        // 5. Market and supply are independent — both depend on distribution but not each other
        thread::scope(|scope| -> Result<()> {
            let market = scope.spawn(|| -> Result<()> {
                info!("Computing market...");
                let i = Instant::now();
                self.market.compute(
                    &self.indexes,
                    &self.prices,
                    &self.blocks,
                    &self.mining,
                    &self.distribution,
                    &self.transactions,
                    &starting_indexes,
                    exit,
                )?;
                info!("Computed market in {:?}", i.elapsed());
                Ok(())
            });

            info!("Computing supply...");
            let i = Instant::now();
            self.supply.compute(
                &self.scripts,
                &self.blocks,
                &self.mining,
                &self.transactions,
                &self.prices,
                &self.distribution,
                &starting_indexes,
                exit,
            )?;
            info!("Computed supply in {:?}", i.elapsed());

            market.join().unwrap()?;
            Ok(())
        })?;

        // 6. Cointime (depends on supply, distribution, mining)
        info!("Computing cointime...");
        let i = Instant::now();
        self.cointime.compute(
            &starting_indexes,
            &self.prices,
            &self.blocks,
            &self.mining,
            &self.supply,
            &self.distribution,
            exit,
        )?;
        info!("Computed cointime in {:?}", i.elapsed());

        info!("Total compute time: {:?}", compute_start.elapsed());
        Ok(())
    }
}

impl Computer<Ro> {
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
                self.mining
                    .iter_any_exportable()
                    .map(|v| (mining::DB_NAME, v)),
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
                self.prices
                    .iter_any_exportable()
                    .map(|v| (prices::DB_NAME, v)),
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
