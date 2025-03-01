#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("main.rs")]
#![doc = "```"]

use std::path::{Path, PathBuf};

use brk_exit::Exit;
use brk_indexer::{Indexer, Indexes};
pub use brk_parser::rpc;

mod storage;

use brk_core::Date;
use storage::{Stores, Vecs};

#[derive(Clone)]
pub struct Computer {
    path: PathBuf,
    pub vecs: Vecs,
    pub stores: Stores,
}

impl Computer {
    pub fn import(computed_dir: &Path) -> color_eyre::Result<Self> {
        let vecs = Vecs::import(&computed_dir.join("vecs"))?;

        let stores = Stores::import(&computed_dir.join("stores"))?;

        Ok(Self {
            path: computed_dir.to_owned(),
            vecs,
            stores,
        })
    }
}

impl Computer {
    pub fn compute(&mut self, indexer: &mut Indexer, starting_indexes: Indexes, exit: &Exit) -> color_eyre::Result<()> {
        let height_count = indexer.vecs.height_to_size.len();
        let txindexes_count = indexer.vecs.txindex_to_txid.len();
        let txinindexes_count = indexer.vecs.txinindex_to_txoutindex.len();
        let txoutindexes_count = indexer.vecs.txoutindex_to_addressindex.len();

        // TODO: Remove all outdated

        // self.vecs.txindex_to_last_txinindex.compute_last_index_from_first(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs.txindex_to_first_txinindex,
        //     txinindexes_count,
        //     exit,
        // )?;

        // self.vecs.txindex_to_inputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs.txindex_to_first_txinindex,
        //     &mut self.vecs.txindex_to_last_txinindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_last_txoutindex.compute_last_index_from_first(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs.txindex_to_first_txoutindex,
        //     txoutindexes_count,
        //     exit,
        // )?;

        // self.vecs.txindex_to_outputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs.txindex_to_first_txoutindex,
        //     &mut self.vecs.txindex_to_last_txoutindex,
        //     exit,
        // )?;

        self.vecs.height_to_date.compute_transform(
            starting_indexes.height,
            &mut indexer.vecs.height_to_timestamp,
            |timestamp| Date::from(*timestamp),
            exit,
        )?;

        // self.vecs.height_to_last_txindex.compute_last_index_from_first(
        //     starting_indexes.height,
        //     &mut indexer.vecs.height_to_first_txindex,
        //     height_count,
        //     exit,
        // )?;

        // self.vecs.txindex_to_height.compute_inverse_less_to_more(
        //     starting_indexes.height,
        //     &mut indexer.vecs.height_to_first_txindex,
        //     &mut self.vecs.height_to_last_txindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_is_coinbase.compute_is_first_ordered(
        //     starting_indexes.txindex,
        //     &mut self.vecs.txindex_to_height,
        //     &mut indexer.vecs.height_to_first_txindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_fee.compute_transform(
        //     &mut self.vecs.txindex_to_height,
        //     &mut indexer.vecs.height_to_first_txindex,
        // )?;

        let date_count = self.vecs.height_to_date.len();

        // self.vecs.height_to_dateindex.compute(...)

        // self.vecs
        //     .dateindex_to_first_height
        //     .compute_inverse_more_to_less(&mut self.vecs.height_to_dateindex, exit)?;

        // ---
        // Date to X
        // ---
        // ...

        // ---
        // Month to X
        // ---
        // ...

        // ---
        // Year to X
        // ---
        // ...

        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
