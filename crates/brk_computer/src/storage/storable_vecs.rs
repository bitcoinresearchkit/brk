use std::{fs, path::Path};

use brk_fetcher::{Date, Dateindex};
use brk_indexer::{Addressindex, Height, Sats, Timestamp, Txindex, Txinindex, Txoutindex};
use storable_vec::{StorableVec, Version};

use crate::structs::Feerate;

// mod base;

// use base::*;

pub struct StorableVecs<const MODE: u8> {
    pub dateindex_to_first_height: StorableVec<Dateindex, Height, MODE>,
    // pub dateindex_to_last_height: StorableVec<Dateindex, Height, MODE>,
    // pub height_to_block_interval: StorableVec<Height, Timestamp, MODE>,
    pub height_to_date: StorableVec<Height, Date, MODE>,
    pub height_to_dateindex: StorableVec<Height, Dateindex, MODE>,
    // pub height_to_fee: StorableVec<Txindex, Amount, MODE>,
    // pub height_to_inputcount: StorableVec<Height, u32, MODE>,
    // pub height_to_last_addressindex: StorableVec<Height, Addressindex, MODE>,
    pub height_to_last_txindex: StorableVec<Height, Txindex, MODE>,
    // pub height_to_last_txoutindex: StorableVec<Height, Txoutindex, MODE>,
    // pub height_to_maxfeerate: StorableVec<Height, Feerate, MODE>,
    // pub height_to_medianfeerate: StorableVec<Height, Feerate, MODE>,
    // pub height_to_minfeerate: StorableVec<Height, Feerate, MODE>,
    // pub height_to_outputcount: StorableVec<Height, u32, MODE>,
    // pub height_to_subsidy: StorableVec<Height, u32, MODE>,
    // pub height_to_totalfees: StorableVec<Height, Amount, MODE>,
    // pub height_to_txcount: StorableVec<Height, u32, MODE>,
    pub txindex_to_fee: StorableVec<Txindex, Sats, MODE>,
    pub txindex_to_height: StorableVec<Txindex, Height, MODE>,
    pub txindex_to_is_coinbase: StorableVec<Txindex, bool, MODE>,
    // pub txindex_to_feerate: StorableVec<Txindex, Feerate, MODE>,
    pub txindex_to_inputs_count: StorableVec<Txindex, u32, MODE>,
    pub txindex_to_inputs_sum: StorableVec<Txindex, Sats, MODE>,
    pub txindex_to_last_txinindex: StorableVec<Txindex, Txinindex, MODE>,
    pub txindex_to_last_txoutindex: StorableVec<Txindex, Txoutindex, MODE>,
    pub txindex_to_outputs_count: StorableVec<Txindex, u32, MODE>,
    pub txindex_to_outputs_sum: StorableVec<Txindex, Sats, MODE>,
}

impl<const MODE: u8> StorableVecs<MODE> {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_first_height: StorableVec::forced_import(
                &path.join("dateindex_to_first_height"),
                Version::from(1),
            )?,
            // height_to_block_interval: StorableVec::forced_import(&path.join("height_to_block_interval"), Version::from(1))?,
            height_to_date: StorableVec::forced_import(&path.join("height_to_date"), Version::from(1))?,
            height_to_dateindex: StorableVec::forced_import(&path.join("height_to_dateindex"), Version::from(1))?,
            // height_to_fee: StorableVec::forced_import(&path.join("height_to_fee"), Version::from(1))?,
            // height_to_inputcount: StorableVec::forced_import(&path.join("height_to_inputcount"), Version::from(1))?,
            // height_to_last_addressindex: StorableVec::forced_import(
            //     &path.join("height_to_last_addressindex"),
            //     Version::from(1),
            // )?,
            height_to_last_txindex: StorableVec::forced_import(&path.join("height_to_last_txindex"), Version::from(1))?,
            // height_to_last_txoutindex: StorableVec::forced_import(&path.join("height_to_last_txoutindex"), Version::from(1))?,
            // height_to_maxfeerate: StorableVec::forced_import(&path.join("height_to_maxfeerate"), Version::from(1))?,
            // height_to_medianfeerate: StorableVec::forced_import(&path.join("height_to_medianfeerate"), Version::from(1))?,
            // height_to_minfeerate: StorableVec::forced_import(&path.join("height_to_minfeerate"), Version::from(1))?,
            // height_to_outputcount: StorableVec::forced_import(&path.join("height_to_outputcount"), Version::from(1))?,
            // height_to_subsidy: StorableVec::forced_import(&path.join("height_to_subsidy"), Version::from(1))?,
            // height_to_totalfees: StorableVec::forced_import(&path.join("height_to_totalfees"), Version::from(1))?,
            // height_to_txcount: StorableVec::forced_import(&path.join("height_to_txcount"), Version::from(1))?,
            txindex_to_fee: StorableVec::forced_import(&path.join("txindex_to_fee"), Version::from(1))?,
            txindex_to_height: StorableVec::forced_import(&path.join("txindex_to_height"), Version::from(1))?,
            txindex_to_is_coinbase: StorableVec::forced_import(&path.join("txindex_to_is_coinbase"), Version::from(1))?,
            // txindex_to_feerate: StorableVec::forced_import(&path.join("txindex_to_feerate"), Version::from(1))?,
            txindex_to_inputs_count: StorableVec::forced_import(
                &path.join("txindex_to_inputs_count"),
                Version::from(1),
            )?,
            txindex_to_inputs_sum: StorableVec::forced_import(&path.join("txindex_to_inputs_sum"), Version::from(1))?,
            txindex_to_last_txinindex: StorableVec::forced_import(
                &path.join("txindex_to_last_txinindex"),
                Version::from(1),
            )?,
            txindex_to_last_txoutindex: StorableVec::forced_import(
                &path.join("txindex_to_last_txoutindex"),
                Version::from(1),
            )?,
            txindex_to_outputs_count: StorableVec::forced_import(
                &path.join("txindex_to_outputs_count"),
                Version::from(1),
            )?,
            txindex_to_outputs_sum: StorableVec::forced_import(&path.join("txindex_to_outputs_sum"), Version::from(1))?,
        })
    }

    // pub fn as_slice(&self) -> [&dyn AnyComputedStorableVec; 1] {
    //     [&self.dateindex_to_first_height]
    // }

    // pub fn as_mut_slice(&mut self) -> [&mut dyn AnyComputedStorableVec; 1] {
    //     [&mut self.dateindex_to_first_height]
    // }
}
