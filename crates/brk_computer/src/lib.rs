#![doc = include_str!("../README.md")]

use std::path::Path;

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
mod pools;
mod price;
mod stateful;
mod states;
mod traits;
mod utils;

use indexes::Indexes;

pub use pools::*;
pub use states::PriceToAmount;
use states::*;

#[derive(Clone)]
pub struct Computer {
    pub indexes: indexes::Vecs,
    pub constants: constants::Vecs,
    pub market: market::Vecs,
    pub pools: pools::Vecs,
    pub price: Option<price::Vecs>,
    pub chain: chain::Vecs,
    pub stateful: stateful::Vecs,
    pub fetched: Option<fetched::Vecs>,
    pub cointime: cointime::Vecs,
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
            pools: pools::Vecs::forced_import(
                &computed_path,
                VERSION + Version::ZERO,
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
            .compute(&self.indexes, &starting_indexes, exit)?;

        if let Some(fetched) = self.fetched.as_mut() {
            info!("Computing fetched...");
            fetched.compute(indexer, &self.indexes, &starting_indexes, exit)?;

            info!("Computing prices...");
            self.price.as_mut().unwrap().compute(
                &self.indexes,
                &starting_indexes,
                fetched,
                exit,
            )?;
        }

        std::thread::scope(|scope| -> Result<()> {
            let chain = scope.spawn(|| -> Result<()> {
                info!("Computing chain...");
                self.chain.compute(
                    indexer,
                    &self.indexes,
                    &starting_indexes,
                    self.price.as_ref(),
                    exit,
                )?;
                Ok(())
            });

            if let Some(price) = self.price.as_ref() {
                info!("Computing market...");
                self.market.compute(price, &starting_indexes, exit)?;
            }

            chain.join().unwrap()?;
            Ok(())
        })?;

        self.pools.compute(
            indexer,
            &self.indexes,
            &starting_indexes,
            &self.chain,
            self.price.as_ref(),
            exit,
        )?;

        // return Ok(());

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
            &self.indexes,
            &starting_indexes,
            self.price.as_ref(),
            &self.chain,
            &self.stateful,
            exit,
        )?;

        Ok(())
    }

    pub fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self.fetched.iter().flat_map(|v| v.iter_any_collectable()));

        iter = Box::new(iter.chain(self.price.iter().flat_map(|v| v.iter_any_collectable())));
        iter = Box::new(iter.chain(self.pools.iter_any_collectable()));
        iter = Box::new(iter.chain(self.constants.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes.iter_any_collectable()));
        iter = Box::new(iter.chain(self.market.iter_any_collectable()));
        iter = Box::new(iter.chain(self.chain.iter_any_collectable()));
        iter = Box::new(iter.chain(self.stateful.iter_any_collectable()));
        iter = Box::new(iter.chain(self.cointime.iter_any_collectable()));

        iter
    }

    pub fn static_clone(&self) -> &'static Self {
        Box::leak(Box::new(self.clone()))
    }
}

// pub fn generate_allocation_files(monitored: &pools::Vecs) -> Result<()> {
//     info!("Generating Allocative files...");

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

//     info!("Successfully generated Allocative files");

//     Ok(())
// }
