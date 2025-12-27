#![doc = include_str!("../README.md")]

use std::{fs, path::Path, thread, time::Instant};

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::Traversable;
use brk_types::Version;
use log::info;
use vecdb::Exit;

mod blks;
mod chain;
mod cointime;
mod constants;
mod fetched;
mod grouped;
mod indexes;
mod market;
mod pools;
mod price;
mod stateful;
mod traits;
mod txins;
mod txouts;
mod utils;

use indexes::Indexes;
use utils::OptionExt;

#[derive(Clone, Traversable)]
pub struct Computer {
    pub blks: blks::Vecs,
    pub chain: chain::Vecs,
    pub cointime: cointime::Vecs,
    pub constants: constants::Vecs,
    pub fetched: Option<fetched::Vecs>,
    pub indexes: indexes::Vecs,
    pub market: market::Vecs,
    pub pools: pools::Vecs,
    pub price: Option<price::Vecs>,
    pub stateful: stateful::Vecs,
    pub txins: txins::Vecs,
    pub txouts: txouts::Vecs,
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
        let (indexes, fetched, blks, txins, txouts) = thread::scope(|s| -> Result<_> {
            let fetched_handle = fetcher
                .map(|fetcher| {
                    big_thread().spawn_scoped(s, move || {
                        fetched::Vecs::forced_import(outputs_path, fetcher, VERSION)
                    })
                })
                .transpose()?;

            let blks_handle = big_thread()
                .spawn_scoped(s, || blks::Vecs::forced_import(&computed_path, VERSION))?;

            let txins_handle = big_thread()
                .spawn_scoped(s, || txins::Vecs::forced_import(&computed_path, VERSION))?;

            let txouts_handle = big_thread()
                .spawn_scoped(s, || txouts::Vecs::forced_import(&computed_path, VERSION))?;

            let indexes = indexes::Vecs::forced_import(&computed_path, VERSION, indexer)?;
            let fetched = fetched_handle.map(|h| h.join().unwrap()).transpose()?;
            let blks = blks_handle.join().unwrap()?;
            let txins = txins_handle.join().unwrap()?;
            let txouts = txouts_handle.join().unwrap()?;

            Ok((indexes, fetched, blks, txins, txouts))
        })?;
        info!("Imported indexes/fetched/blks/txins/txouts in {:?}", i.elapsed());

        let i = Instant::now();
        let (price, constants, market) = thread::scope(|s| -> Result<_> {
            let constants_handle = big_thread().spawn_scoped(s, || {
                constants::Vecs::forced_import(&computed_path, VERSION, &indexes)
            })?;

            let market_handle = big_thread().spawn_scoped(s, || {
                market::Vecs::forced_import(&computed_path, VERSION, &indexes)
            })?;

            let price = fetched
                .is_some()
                .then(|| price::Vecs::forced_import(&computed_path, VERSION, &indexes).unwrap());

            let constants = constants_handle.join().unwrap()?;
            let market = market_handle.join().unwrap()?;

            Ok((price, constants, market))
        })?;
        info!("Imported price/constants/market in {:?}", i.elapsed());

        let i = Instant::now();
        let (chain, pools, cointime) = thread::scope(|s| -> Result<_> {
            let chain_handle = big_thread().spawn_scoped(s, || {
                chain::Vecs::forced_import(
                    &computed_path,
                    VERSION,
                    indexer,
                    &indexes,
                    price.as_ref(),
                )
            })?;

            let pools_handle = big_thread().spawn_scoped(s, || {
                pools::Vecs::forced_import(&computed_path, VERSION, &indexes, price.as_ref())
            })?;

            let cointime =
                cointime::Vecs::forced_import(&computed_path, VERSION, &indexes, price.as_ref())?;

            let chain = chain_handle.join().unwrap()?;
            let pools = pools_handle.join().unwrap()?;

            Ok((chain, pools, cointime))
        })?;
        info!("Imported chain/pools/cointime in {:?}", i.elapsed());

        // Threads inside
        let i = Instant::now();
        let stateful =
            stateful::Vecs::forced_import(&computed_path, VERSION, &indexes, price.as_ref())?;
        info!("Imported stateful in {:?}", i.elapsed());

        info!("Total import time: {:?}", import_start.elapsed());

        let this = Self {
            constants,
            market,
            stateful,
            chain,
            blks,
            pools,
            cointime,
            indexes,
            txins,
            fetched,
            price,
            txouts,
        };

        Self::retain_databases(&computed_path)?;

        Ok(this)
    }

    /// Removes database folders that are no longer in use.
    fn retain_databases(computed_path: &Path) -> Result<()> {
        const EXPECTED_DBS: &[&str] = &[
            blks::DB_NAME,
            chain::DB_NAME,
            cointime::DB_NAME,
            constants::DB_NAME,
            indexes::DB_NAME,
            market::DB_NAME,
            pools::DB_NAME,
            price::DB_NAME,
            stateful::DB_NAME,
            txins::DB_NAME,
            txouts::DB_NAME,
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

            if let Some(name) = entry.file_name().to_str() {
                if !EXPECTED_DBS.contains(&name) {
                    info!("Removing obsolete database folder: {}", name);
                    fs::remove_dir_all(entry.path())?;
                }
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
        info!("Computing indexes...");
        let i = Instant::now();
        let mut starting_indexes = self.indexes.compute(indexer, starting_indexes, exit)?;
        info!("Computed indexes in {:?}", i.elapsed());

        if let Some(fetched) = self.fetched.as_mut() {
            info!("Computing fetched...");
            let i = Instant::now();
            fetched.compute(indexer, &self.indexes, &starting_indexes, exit)?;
            info!("Computed fetched in {:?}", i.elapsed());

            info!("Computing prices...");
            let i = Instant::now();
            self.price
                .um()
                .compute(&self.indexes, &starting_indexes, fetched, exit)?;
            info!("Computed prices in {:?}", i.elapsed());
        }

        thread::scope(|scope| -> Result<()> {
            let blks = scope.spawn(|| -> Result<()> {
                info!("Computing BLKs metadata...");
                let i = Instant::now();
                self.blks
                    .compute(indexer, &starting_indexes, reader, exit)?;
                info!("Computed blk in {:?}", i.elapsed());
                Ok(())
            });

            let constants = scope.spawn(|| -> Result<()> {
                info!("Computing constants...");
                let i = Instant::now();
                self.constants
                    .compute(&self.indexes, &starting_indexes, exit)?;
                info!("Computed constants in {:?}", i.elapsed());
                Ok(())
            });

            // Txins must complete before txouts (txouts needs txinindex_to_txoutindex)
            // and before chain (chain needs txinindex_to_value)
            info!("Computing txins...");
            let i = Instant::now();
            self.txins.compute(indexer, &starting_indexes, exit)?;
            info!("Computed txins in {:?}", i.elapsed());

            let txouts = scope.spawn(|| -> Result<()> {
                info!("Computing txouts...");
                let i = Instant::now();
                self.txouts.compute(indexer, &self.txins, &starting_indexes, exit)?;
                info!("Computed txouts in {:?}", i.elapsed());
                Ok(())
            });

            info!("Computing chain...");
            let i = Instant::now();
            self.chain.compute(
                indexer,
                &self.indexes,
                &self.txins,
                &starting_indexes,
                self.price.as_ref(),
                exit,
            )?;
            info!("Computed chain in {:?}", i.elapsed());

            if let Some(price) = self.price.as_ref() {
                info!("Computing market...");
                let i = Instant::now();
                self.market.compute(price, &starting_indexes, exit)?;
                info!("Computed market in {:?}", i.elapsed());
            }

            blks.join().unwrap()?;
            constants.join().unwrap()?;
            txouts.join().unwrap()?;
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
                    &starting_indexes_clone,
                    &self.chain,
                    self.price.as_ref(),
                    exit,
                )?;
                info!("Computed pools in {:?}", i.elapsed());
                Ok(())
            });

            info!("Computing stateful...");
            let i = Instant::now();
            self.stateful.compute(
                indexer,
                &self.indexes,
                &self.txins,
                &self.chain,
                self.price.as_ref(),
                &mut starting_indexes,
                exit,
            )?;
            info!("Computed stateful in {:?}", i.elapsed());

            pools.join().unwrap()?;
            Ok(())
        })?;

        info!("Computing cointime...");
        let i = Instant::now();
        self.cointime.compute(
            &self.indexes,
            &starting_indexes,
            self.price.as_ref(),
            &self.chain,
            &self.stateful,
            exit,
        )?;
        info!("Computed cointime in {:?}", i.elapsed());

        info!("Total compute time: {:?}", compute_start.elapsed());
        Ok(())
    }
}

// pub fn generate_allocation_files(monitored: &pools::Vecs) -> Result<()> {
//     info!("Generating allocative files...");

//     let mut flamegraph = allocative::FlameGraphBuilder::default();
//     flamegraph.visit_root(monitored);
//     let output = flamegraph.finish();

//     let folder = format!(
//         "at-{}",
//         jiff::Timestamp::now().strftime("%Y-%m-%d_%Hh%Mm%Ss"),
//     );

//     let path = std::path::PathBuf::from(&format!("./target/flamegraph/{folder}"));
//     std::fs::create_dir_all(&path)?;

//     // fs::write(path.join("flamegraph.src"), &output.flamegraph())?;

//     let mut fg_svg = Vec::new();
//     inferno::flamegraph::from_reader(
//         &mut inferno::flamegraph::Options::default(),
//         output.flamegraph().write().as_bytes(),
//         &mut fg_svg,
//     )?;

//     std::fs::write(path.join("flamegraph.svg"), &fg_svg)?;

//     std::fs::write(path.join("warnings.txt"), output.warnings())?;

//     info!("Successfully generate allocative files");

//     Ok(())
// }
