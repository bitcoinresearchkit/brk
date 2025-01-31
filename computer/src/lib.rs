use std::path::{Path, PathBuf};

use bindex::{Height, Indexer};
use biter::rpc;
use exit::Exit;

mod storage;
mod structs;

use storage::{Fjalls, StorableVecs};
use structs::*;

pub struct Computer {
    outputs_dir: PathBuf,
    vecs: StorableVecs,
    trees: Fjalls,
}

impl Computer {
    pub fn import(outputs_dir: &Path) -> color_eyre::Result<Self> {
        let outputs_dir = outputs_dir.to_owned();
        let computed_dir = outputs_dir.join("computed");
        let vecs = StorableVecs::import(&computed_dir.join("vecs"))?;
        let trees = Fjalls::import(&computed_dir.join("fjall"))?;
        Ok(Self {
            outputs_dir,
            vecs,
            trees,
        })
    }

    pub fn compute(&mut self, bitcoin_dir: &Path, rpc: rpc::Client, exit: &Exit) -> color_eyre::Result<()> {
        let mut indexer = Indexer::import(&self.outputs_dir.join("indexes"))?;

        if false {
            indexer.index(bitcoin_dir, rpc, exit)?;
        }

        // TODO: Remove all outdated

        // Compute txindex to X

        // Compute height to X
        indexer
            .vecs()
            .height_to_timestamp
            .read_from_(self.vecs.height_to_date.len(), |(_height, timestamp)| {
                self.vecs
                    .height_to_date
                    .push_if_needed(Height::from(_height), Date::from(timestamp))
            })?;
        self.vecs
            .height_to_date
            .read_from_(self.vecs.date_to_first_height.len(), |(_height, date)| {
                self.vecs
                    .date_to_first_height
                    .push_if_needed(*date, Height::from(_height))
            })?;

        // Compute date to X
        // ...

        // Compute month to X
        // ...

        // Compute year to X
        // ...

        Ok(())
    }
}
