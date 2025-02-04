use std::{fs, path::Path};

use bindex::{Addressindex, Amount, Height, Timestamp, Txindex, Txinindex, Txoutindex};
use storable_vec::Version;

use crate::structs::{Date, Feerate};

mod base;

use base::*;

pub struct StorableVecs<const MODE: u8> {
    pub date_to_first_height: StorableVec<Date, Height, MODE>,
    // pub height_to_block_interval: StorableVec<Height, Timestamp, MODE>,
    pub height_to_date: StorableVec<Height, Date, MODE>,
    // pub height_to_fee: StorableVec<Txindex, Amount, MODE>,
    // pub height_to_inputcount: StorableVec<Txindex, u32, MODE>,
    // pub height_to_last_addressindex: StorableVec<Height, Addressindex, MODE>,
    pub height_to_last_txindex: StorableVec<Height, Txindex, MODE>,
    // pub height_to_last_txoutindex: StorableVec<Height, Txoutindex, MODE>,
    // pub height_to_maxfeerate: StorableVec<Txindex, Feerate, MODE>,
    // pub height_to_medianfeerate: StorableVec<Txindex, Feerate, MODE>,
    // pub height_to_minfeerate: StorableVec<Txindex, Feerate, MODE>,
    // pub height_to_outputcount: StorableVec<Txindex, u32, MODE>,
    // pub height_to_subsidy: StorableVec<Txindex, u32, MODE>,
    // pub height_to_totalfees: StorableVec<Height, Amount, MODE>,
    // pub height_to_txcount: StorableVec<Txindex, u32, MODE>,
    pub txindex_to_fee: StorableVec<Txindex, Amount, MODE>,
    pub txindex_to_height: StorableVec<Txindex, Height, MODE>,
    pub txindex_to_is_coinbase: StorableVec<Txindex, bool, MODE>,
    // pub txindex_to_feerate: StorableVec<Txindex, Feerate, MODE>,
    pub txindex_to_inputcount: StorableVec<Txindex, u32, MODE>,
    pub txindex_to_last_txinindex: StorableVec<Txindex, Txinindex, MODE>,
    pub txindex_to_last_txoutindex: StorableVec<Txindex, Txoutindex, MODE>,
    pub txindex_to_outputcount: StorableVec<Txindex, u32, MODE>,
}

impl<const MODE: u8> StorableVecs<MODE> {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            date_to_first_height: StorableVec::import(&path.join("date_to_first_height"), Version::from(1))?,
            // height_to_block_interval: StorableVec::import(&path.join("height_to_block_interval"), Version::from(1))?,
            height_to_date: StorableVec::import(&path.join("height_to_date"), Version::from(1))?,
            // height_to_fee: StorableVec::import(&path.join("height_to_fee"), Version::from(1))?,
            // height_to_inputcount: StorableVec::import(&path.join("height_to_inputcount"), Version::from(1))?,
            // height_to_last_addressindex: StorableVec::import(
            //     &path.join("height_to_last_addressindex"),
            //     Version::from(1),
            // )?,
            height_to_last_txindex: StorableVec::import(&path.join("height_to_last_txindex"), Version::from(1))?,
            // height_to_last_txoutindex: StorableVec::import(&path.join("height_to_last_txoutindex"), Version::from(1))?,
            // height_to_maxfeerate: StorableVec::import(&path.join("height_to_maxfeerate"), Version::from(1))?,
            // height_to_medianfeerate: StorableVec::import(&path.join("height_to_medianfeerate"), Version::from(1))?,
            // height_to_minfeerate: StorableVec::import(&path.join("height_to_minfeerate"), Version::from(1))?,
            // height_to_outputcount: StorableVec::import(&path.join("height_to_outputcount"), Version::from(1))?,
            // height_to_subsidy: StorableVec::import(&path.join("height_to_subsidy"), Version::from(1))?,
            // height_to_totalfees: StorableVec::import(&path.join("height_to_totalfees"), Version::from(1))?,
            // height_to_txcount: StorableVec::import(&path.join("height_to_txcount"), Version::from(1))?,
            txindex_to_fee: StorableVec::import(&path.join("txindex_to_fee"), Version::from(1))?,
            txindex_to_height: StorableVec::import(&path.join("txindex_to_height"), Version::from(1))?,
            txindex_to_is_coinbase: StorableVec::import(&path.join("txindex_to_is_coinbase"), Version::from(1))?,
            // txindex_to_feerate: StorableVec::import(&path.join("txindex_to_feerate"), Version::from(1))?,
            txindex_to_inputcount: StorableVec::import(&path.join("txindex_to_inputcount"), Version::from(1))?,
            txindex_to_last_txinindex: StorableVec::import(&path.join("txindex_to_last_txinindex"), Version::from(1))?,
            txindex_to_last_txoutindex: StorableVec::import(
                &path.join("txindex_to_last_txoutindex"),
                Version::from(1),
            )?,
            txindex_to_outputcount: StorableVec::import(&path.join("txindex_to_outputcount"), Version::from(1))?,
        })
    }

    // pub fn as_slice(&self) -> [&dyn AnyComputedStorableVec; 1] {
    //     [&self.date_to_first_height]
    // }

    // pub fn as_mut_slice(&mut self) -> [&mut dyn AnyComputedStorableVec; 1] {
    //     [&mut self.date_to_first_height]
    // }
}
