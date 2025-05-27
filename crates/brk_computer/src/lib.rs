#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::path::{Path, PathBuf};

use brk_core::Version;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, Computation};

mod states;
mod stores;
mod utils;
mod vecs;

use log::info;
use stores::Stores;
use vecs::Vecs;

#[derive(Clone)]
pub struct Computer {
    path: PathBuf,
    fetcher: Option<Fetcher>,
    vecs: Option<Vecs>,
    stores: Option<Stores>,
    compressed: Compressed,
}

const VERSION: Version = Version::ONE;

impl Computer {
    pub fn new(outputs_dir: &Path, fetcher: Option<Fetcher>, compressed: bool) -> Self {
        Self {
            path: outputs_dir.to_owned(),
            fetcher,
            vecs: None,
            stores: None,
            compressed: Compressed::from(compressed),
        }
    }

    pub fn import_vecs(
        &mut self,
        indexer: &Indexer,
        computation: Computation,
    ) -> color_eyre::Result<()> {
        self.vecs = Some(Vecs::import(
            &self.path.join("vecs/computed"),
            VERSION + Version::ZERO,
            indexer,
            self.fetcher.is_some(),
            computation,
            self.compressed,
        )?);
        Ok(())
    }

    /// Do NOT import multiple times or things will break !!!
    /// Clone struct instead
    pub fn import_stores(&mut self, indexer: &Indexer) -> color_eyre::Result<()> {
        self.stores = Some(Stores::import(
            &self.path.join("stores"),
            VERSION + Version::ZERO,
            indexer.keyspace(),
        )?);
        Ok(())
    }
}

impl Computer {
    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        info!("Computing...");

        self.vecs.as_mut().unwrap().compute(
            indexer,
            starting_indexes,
            self.fetcher.as_mut(),
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        // pub fn vecs(&self) -> &Vecs {
        self.vecs.as_ref().unwrap().vecs()
    }

    // pub fn mut_vecs(&mut self) -> &mut Vecs {
    //     self.vecs.as_mut().unwrap()
    // }

    pub fn stores(&self) -> &Stores {
        self.stores.as_ref().unwrap()
    }

    pub fn mut_stores(&mut self) -> &mut Stores {
        self.stores.as_mut().unwrap()
    }
}
