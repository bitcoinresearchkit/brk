#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::path::{Path, PathBuf};

use brk_exit::Exit;
use brk_indexer::{Indexer, Indexes};
pub use brk_parser::rpc;

mod storage;

use log::info;
use storage::{Stores, Vecs};

#[derive(Clone)]
pub struct Computer {
    path: PathBuf,
    vecs: Option<Vecs>,
    stores: Option<Stores>,
}

impl Computer {
    pub fn new(computed_dir: PathBuf) -> Self {
        Self {
            path: computed_dir,
            vecs: None,
            stores: None,
        }
    }

    pub fn import_vecs(&mut self) -> color_eyre::Result<()> {
        self.vecs = Some(Vecs::import(&self.path.join("vecs"))?);
        Ok(())
    }

    /// Do NOT import multiple times are things will break !!!
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

        self.mut_vecs().compute(indexer, starting_indexes, exit)?;

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
