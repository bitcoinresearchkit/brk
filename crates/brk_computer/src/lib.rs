#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::path::Path;

use brk_core::Version;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{Computation, Format};
use log::info;

mod groups;
mod states;
mod stores;
mod utils;
mod vecs;

use groups::*;
use states::*;
use stores::Stores;
use vecs::Vecs;

#[derive(Clone)]
pub struct Computer {
    fetcher: Option<Fetcher>,
    pub vecs: Vecs,
    pub stores: Stores,
}

const VERSION: Version = Version::ONE;

impl Computer {
    /// Do NOT import multiple times or things will break !!!
    pub fn forced_import(
        outputs_dir: &Path,
        indexer: &Indexer,
        computation: Computation,
        fetcher: Option<Fetcher>,
        format: Format,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            vecs: Vecs::import(
                // TODO: Give self.path, join inside import
                &outputs_dir.join("vecs/computed"),
                VERSION + Version::ZERO,
                indexer,
                fetcher.is_some(),
                computation,
                format,
            )?,
            stores: Stores::import(
                // TODO: Give self.path, join inside import
                &outputs_dir.join("stores"),
                VERSION + Version::ZERO,
                &indexer.stores.keyspace,
            )?,
            fetcher,
        })
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
        self.vecs.compute(
            indexer,
            starting_indexes,
            self.fetcher.as_mut(),
            exit,
            &mut self.stores,
        )
    }
}
