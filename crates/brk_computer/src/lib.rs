#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{path::Path, sync::Arc};

use brk_core::Version;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vecs::{Computation, File, Format};
use log::info;

mod all;
mod blocks;
mod cointime;
mod constants;
mod fetched;
mod grouped;
mod indexes;
mod market;
mod mining;
mod stateful;
mod states;
mod transactions;
mod utils;

use indexes::Indexes;

use states::*;

#[derive(Clone)]
pub struct Computer {
    file: Arc<File>,
    fetcher: Option<Fetcher>,
    pub vecs: all::Vecs,
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
        let computed_path = outputs_dir.join("computed");
        let states_path = computed_path.join("states");

        let file = Arc::new(File::open(&computed_path.join("vecs"))?);
        let file_fetched = Arc::new(File::open(&outputs_dir.join("fetched/vecs"))?);

        Ok(Self {
            vecs: all::Vecs::import(
                &file,
                VERSION + Version::ZERO,
                indexer,
                fetcher.is_some(),
                computation,
                format,
                &file_fetched,
                &states_path,
            )?,
            fetcher,
            file,
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
        self.vecs
            .compute(indexer, starting_indexes, self.fetcher.as_mut(), exit)?;
        self.file.flush()?;
        self.file.punch_holes()?;
        Ok(())
    }
}
