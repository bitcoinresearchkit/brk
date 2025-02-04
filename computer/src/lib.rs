use std::path::{Path, PathBuf};

use bindex::Indexer;
use biter::rpc;
use exit::Exit;

mod storage;
mod structs;

use storable_vec::{CACHED_GETS, SINGLE_THREAD};
use storage::{Fjalls, StorableVecs};
use structs::*;

pub struct Computer<const MODE: u8> {
    outputs_dir: PathBuf,
    vecs: StorableVecs<MODE>,
    trees: Fjalls,
}

impl<const MODE: u8> Computer<MODE> {
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

    fn open_indexer<const MODE_IDX: u8>(&self) -> color_eyre::Result<Indexer<MODE_IDX>> {
        Indexer::import(&self.outputs_dir.join("indexes"))
    }
}

impl Computer<SINGLE_THREAD> {
    pub fn compute(&mut self, bitcoin_dir: &Path, rpc: rpc::Client, exit: &Exit) -> color_eyre::Result<()> {
        if false {
            let mut indexer: Indexer<CACHED_GETS> = self.open_indexer()?;
            indexer.index(bitcoin_dir, rpc, exit)?;
        }

        let mut indexer: Indexer<SINGLE_THREAD> = self.open_indexer()?;

        let height_count = indexer.vecs.height_to_size.len();
        let txindexes_count = indexer.vecs.txindex_to_txid.len();
        let txinindexes_count = indexer.vecs.txinindex_to_txoutindex.len();
        let txoutindexes_count = indexer.vecs.txoutindex_to_addressindex.len();

        // TODO: Remove all outdated

        self.vecs
            .txindex_to_last_txinindex
            .compute_last_index_from_first(&indexer.vecs.txindex_to_first_txinindex, txinindexes_count)?;

        self.vecs.txindex_to_inputcount.compute_count_from_indexes(
            &indexer.vecs.txindex_to_first_txinindex,
            &self.vecs.txindex_to_last_txinindex,
        )?;

        self.vecs
            .txindex_to_last_txoutindex
            .compute_last_index_from_first(&indexer.vecs.txindex_to_first_txoutindex, txoutindexes_count)?;

        self.vecs.txindex_to_outputcount.compute_count_from_indexes(
            &indexer.vecs.txindex_to_first_txoutindex,
            &self.vecs.txindex_to_last_txoutindex,
        )?;

        self.vecs
            .height_to_date
            .compute_transform(&mut indexer.vecs.height_to_timestamp, |timestamp| Date::from(timestamp))?;

        self.vecs
            .height_to_last_txindex
            .compute_last_index_from_first(&indexer.vecs.height_to_first_txindex, height_count)?;

        self.vecs.txindex_to_height.compute_inverse_less_to_more(
            &mut indexer.vecs.height_to_first_txindex,
            &mut self.vecs.height_to_last_txindex,
        )?;

        let date_count = self.vecs.height_to_date.len();

        self.vecs
            .date_to_first_height
            .compute_inverse_more_to_less(&mut self.vecs.height_to_date)?;

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
}
