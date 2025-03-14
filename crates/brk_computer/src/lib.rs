#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::path::{Path, PathBuf};

use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::{Indexer, Indexes};
pub use brk_parser::rpc;

mod storage;

use brk_vec::Compressed;
use log::info;
use storage::{Stores, Vecs};

#[derive(Clone)]
pub struct Computer {
    path: PathBuf,
    fetcher: Option<Fetcher>,
    vecs: Option<Vecs>,
    stores: Option<Stores>,
    compressed: Compressed,
}

impl Computer {
    pub fn new(computed_dir: PathBuf, fetcher: Option<Fetcher>, compressed: bool) -> Self {
        Self {
            path: computed_dir,
            fetcher,
            vecs: None,
            stores: None,
            compressed: Compressed::from(compressed),
        }
    }

    pub fn import_vecs(&mut self) -> color_eyre::Result<()> {
        self.vecs = Some(Vecs::import(
            &self.path.join("vecs"),
            self.fetcher.is_some(),
            self.compressed,
        )?);
        Ok(())
    }

    /// Do NOT import multiple times or things will break !!!
    /// Clone struct instead
    pub fn import_stores(&mut self) -> color_eyre::Result<()> {
        self.stores = Some(Stores::import(&self.path.join("stores"))?);
        Ok(())
    }
}

impl Computer {
    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        starting_indexes: Indexes,
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

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn vecs(&self) -> &Vecs {
        self.vecs.as_ref().unwrap()
    }

    pub fn mut_vecs(&mut self) -> &mut Vecs {
        self.vecs.as_mut().unwrap()
    }

    pub fn stores(&self) -> &Stores {
        self.stores.as_ref().unwrap()
    }

    pub fn mut_stores(&mut self) -> &mut Stores {
        self.stores.as_mut().unwrap()
    }
}
