#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::path::{Path, PathBuf};

use brk_exit::Exit;
use brk_indexer::{Indexer, Indexes};
pub use brk_parser::rpc;

mod storage;

use brk_core::Date;
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

        let height_count = indexer.vecs().height_to_size.len();
        let txindexes_count = indexer.vecs().txindex_to_txid.len();
        let txinindexes_count = indexer.vecs().txinindex_to_txoutindex.len();
        let txoutindexes_count = indexer.vecs().txoutindex_to_addressindex.len();

        // self.vecs.txindex_to_last_txinindex.compute_last_index_from_first(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs().txindex_to_first_txinindex,
        //     txinindexes_count,
        //     exit,
        // )?;

        // self.vecs.txindex_to_inputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs().txindex_to_first_txinindex,
        //     &mut self.vecs.txindex_to_last_txinindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_last_txoutindex.compute_last_index_from_first(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs().txindex_to_first_txoutindex,
        //     txoutindexes_count,
        //     exit,
        // )?;

        // self.vecs.txindex_to_outputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs().txindex_to_first_txoutindex,
        //     &mut self.vecs.txindex_to_last_txoutindex,
        //     exit,
        // )?;

        self.mut_vecs().height_to_height.compute_transform(
            starting_indexes.height,
            &mut indexer.mut_vecs().height_to_timestamp,
            |_, height| height,
            exit,
        )?;

        self.mut_vecs().height_to_date.compute_transform(
            starting_indexes.height,
            &mut indexer.mut_vecs().height_to_timestamp,
            |timestamp, _| Date::from(*timestamp),
            exit,
        )?;

        // self.vecs.height_to_last_txindex.compute_last_index_from_first(
        //     starting_indexes.height,
        //     &mut indexer.vecs().height_to_first_txindex,
        //     height_count,
        //     exit,
        // )?;

        // self.vecs.txindex_to_height.compute_inverse_less_to_more(
        //     starting_indexes.height,
        //     &mut indexer.vecs().height_to_first_txindex,
        //     &mut self.vecs.height_to_last_txindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_is_coinbase.compute_is_first_ordered(
        //     starting_indexes.txindex,
        //     &mut self.vecs.txindex_to_height,
        //     &mut indexer.vecs().height_to_first_txindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_fee.compute_transform(
        //     &mut self.vecs.txindex_to_height,
        //     &mut indexer.vecs().height_to_first_txindex,
        // )?;

        let date_count = self.vecs().height_to_date.len();

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
