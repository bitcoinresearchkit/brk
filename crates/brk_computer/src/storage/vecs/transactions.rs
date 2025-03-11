use std::{fs, path::Path};

use brk_core::Txindex;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, StorableVec, Version};

use super::indexes::{self, Indexes};

#[derive(Clone)]
pub struct Vecs {
    // pub height_to_block_interval: StorableVec<Height, Timestamp>,
    // pub height_to_fee: StorableVec<Txindex, Sats>,
    // pub height_to_inputcount: StorableVec<Height, u32>,
    // pub height_to_maxfeerate: StorableVec<Height, Feerate>,
    // pub height_to_medianfeerate: StorableVec<Height, Feerate>,
    // pub height_to_minfeerate: StorableVec<Height, Feerate>,
    // pub height_to_outputcount: StorableVec<Height, u32>,
    // pub height_to_subsidy: StorableVec<Height, u32>,
    // pub height_to_totalfees: StorableVec<Height, Sats>,
    // pub height_to_txcount: StorableVec<Height, u32>,
    // pub txindex_to_fee: StorableVec<Txindex, Sats>,
    pub txindex_to_is_coinbase: StorableVec<Txindex, bool>,
    // pub txindex_to_feerate: StorableVec<Txindex, Feerate>,
    // pub txindex_to_inputs_count: StorableVec<Txindex, u32>,
    // pub txindex_to_inputs_sum: StorableVec<Txindex, Sats>,
    // pub txindex_to_outputs_count: StorableVec<Txindex, u32>,
    // pub txindex_to_outputs_sum: StorableVec<Txindex, Sats>,
}

impl Vecs {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            // height_to_block_interval: StorableVec::forced_import(&path.join("height_to_block_interval"), Version::from(1))?,
            // height_to_fee: StorableVec::forced_import(&path.join("height_to_fee"), Version::from(1))?,
            // height_to_inputcount: StorableVec::forced_import(&path.join("height_to_inputcount"), Version::from(1))?,
            // height_to_maxfeerate: StorableVec::forced_import(&path.join("height_to_maxfeerate"), Version::from(1))?,
            // height_to_medianfeerate: StorableVec::forced_import(&path.join("height_to_medianfeerate"), Version::from(1))?,
            // height_to_minfeerate: StorableVec::forced_import(&path.join("height_to_minfeerate"), Version::from(1))?,
            // height_to_outputcount: StorableVec::forced_import(&path.join("height_to_outputcount"), Version::from(1))?,
            // height_to_subsidy: StorableVec::forced_import(&path.join("height_to_subsidy"), Version::from(1))?,
            // height_to_totalfees: StorableVec::forced_import(&path.join("height_to_totalfees"), Version::from(1))?,
            // height_to_txcount: StorableVec::forced_import(&path.join("height_to_txcount"), Version::from(1))?,
            // txindex_to_fee: StorableVec::forced_import(
            //     &path.join("txindex_to_fee"),
            //     Version::from(1),
            // )?,
            txindex_to_is_coinbase: StorableVec::forced_import(
                &path.join("txindex_to_is_coinbase"),
                Version::from(1),
            )?,
            // txindex_to_feerate: StorableVec::forced_import(&path.join("txindex_to_feerate"), Version::from(1))?,
            // txindex_to_inputs_count: StorableVec::forced_import(
            //     &path.join("txindex_to_inputs_count"),
            //     Version::from(1),
            // )?,
            // txindex_to_inputs_sum: StorableVec::forced_import(
            //     &path.join("txindex_to_inputs_sum"),
            //     Version::from(1),
            // )?,
            // txindex_to_outputs_count: StorableVec::forced_import(
            //     &path.join("txindex_to_outputs_count"),
            //     Version::from(1),
            // )?,
            // txindex_to_outputs_sum: StorableVec::forced_import(
            //     &path.join("txindex_to_outputs_sum"),
            //     Version::from(1),
            // )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let indexer_vecs = indexer.mut_vecs();

        // self.vecs.txindex_to_inputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs().txindex_to_first_txinindex,
        //     &mut self.vecs.txindex_to_last_txinindex,
        //     exit,
        // )?;

        // self.vecs.txindex_to_outputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     &mut indexer.vecs().txindex_to_first_txoutindex,
        //     &mut self.vecs.txindex_to_last_txoutindex,
        //     exit,
        // )?;

        self.txindex_to_is_coinbase.compute_is_first_ordered(
            starting_indexes.txindex,
            &mut indexer_vecs.txindex_to_height,
            &mut indexer_vecs.height_to_first_txindex,
            exit,
        )?;

        // self.vecs.txindex_to_fee.compute_transform(
        //     &mut self.vecs.txindex_to_height,
        //     &mut indexer.vecs().height_to_first_txindex,
        // )?;

        // self.vecs.height_to_dateindex.compute(...)

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

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        vec![&self.txindex_to_is_coinbase]
    }
}
