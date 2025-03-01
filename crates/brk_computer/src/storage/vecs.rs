use std::{fs, path::Path};

use brk_core::{
    Addressindex, Cents, Close, Date, Dateindex, Dollars, Feerate, Height, High, Low, Open, Sats, Timestamp, Txindex,
    Txinindex, Txoutindex,
};
use brk_vec::{AnyStorableVec, StorableVec, Version};

// mod base;

// use base::*;

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_first_height: StorableVec<Dateindex, Height>,
    // pub dateindex_to_last_height: StorableVec<Dateindex, Height>,
    // pub height_to_block_interval: StorableVec<Height, Timestamp>,
    pub dateindex_to_close_in_cents: StorableVec<Dateindex, Close<Cents>>,
    pub dateindex_to_close_in_dollars: StorableVec<Dateindex, Close<Dollars>>,
    pub dateindex_to_high_in_cents: StorableVec<Dateindex, High<Cents>>,
    pub dateindex_to_high_in_dollars: StorableVec<Dateindex, High<Dollars>>,
    pub dateindex_to_low_in_cents: StorableVec<Dateindex, Low<Cents>>,
    pub dateindex_to_low_in_dollars: StorableVec<Dateindex, Low<Dollars>>,
    pub dateindex_to_open_in_cents: StorableVec<Dateindex, Open<Cents>>,
    pub dateindex_to_open_in_dollars: StorableVec<Dateindex, Open<Dollars>>,
    pub height_to_close_in_cents: StorableVec<Height, Close<Cents>>,
    pub height_to_close_in_dollars: StorableVec<Height, Close<Dollars>>,
    pub height_to_high_in_cents: StorableVec<Height, High<Cents>>,
    pub height_to_high_in_dollars: StorableVec<Height, High<Dollars>>,
    pub height_to_low_in_cents: StorableVec<Height, Low<Cents>>,
    pub height_to_low_in_dollars: StorableVec<Height, Low<Dollars>>,
    pub height_to_open_in_cents: StorableVec<Height, Open<Cents>>,
    pub height_to_open_in_dollars: StorableVec<Height, Open<Dollars>>,
    pub height_to_date: StorableVec<Height, Date>,
    pub height_to_dateindex: StorableVec<Height, Dateindex>,
    // pub height_to_fee: StorableVec<Txindex, Amount>,
    // pub height_to_inputcount: StorableVec<Height, u32>,
    // pub height_to_last_addressindex: StorableVec<Height, Addressindex>,
    pub height_to_last_txindex: StorableVec<Height, Txindex>,
    // pub height_to_last_txoutindex: StorableVec<Height, Txoutindex>,
    // pub height_to_maxfeerate: StorableVec<Height, Feerate>,
    // pub height_to_medianfeerate: StorableVec<Height, Feerate>,
    // pub height_to_minfeerate: StorableVec<Height, Feerate>,
    // pub height_to_outputcount: StorableVec<Height, u32>,
    // pub height_to_subsidy: StorableVec<Height, u32>,
    // pub height_to_totalfees: StorableVec<Height, Amount>,
    // pub height_to_txcount: StorableVec<Height, u32>,
    pub txindex_to_fee: StorableVec<Txindex, Sats>,
    pub txindex_to_height: StorableVec<Txindex, Height>,
    pub txindex_to_is_coinbase: StorableVec<Txindex, bool>,
    // pub txindex_to_feerate: StorableVec<Txindex, Feerate>,
    pub txindex_to_inputs_count: StorableVec<Txindex, u32>,
    pub txindex_to_inputs_sum: StorableVec<Txindex, Sats>,
    pub txindex_to_last_txinindex: StorableVec<Txindex, Txinindex>,
    pub txindex_to_last_txoutindex: StorableVec<Txindex, Txoutindex>,
    pub txindex_to_outputs_count: StorableVec<Txindex, u32>,
    pub txindex_to_outputs_sum: StorableVec<Txindex, Sats>,
}

impl Vecs {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_first_height: StorableVec::forced_import(
                &path.join("dateindex_to_first_height"),
                Version::from(1),
            )?,
            // height_to_block_interval: StorableVec::forced_import(&path.join("height_to_block_interval"), Version::from(1))?,
            dateindex_to_close_in_cents: StorableVec::import(
                &path.join("dateindex_to_close_in_cents"),
                Version::from(1),
            )?,
            dateindex_to_close_in_dollars: StorableVec::import(
                &path.join("dateindex_to_close_in_dollars"),
                Version::from(1),
            )?,
            dateindex_to_high_in_cents: StorableVec::import(
                &path.join("dateindex_to_high_in_cents"),
                Version::from(1),
            )?,
            dateindex_to_high_in_dollars: StorableVec::import(
                &path.join("dateindex_to_high_in_dollars"),
                Version::from(1),
            )?,
            dateindex_to_low_in_cents: StorableVec::import(&path.join("dateindex_to_low_in_cents"), Version::from(1))?,
            dateindex_to_low_in_dollars: StorableVec::import(
                &path.join("dateindex_to_low_in_dollars"),
                Version::from(1),
            )?,
            dateindex_to_open_in_cents: StorableVec::import(
                &path.join("dateindex_to_open_in_cents"),
                Version::from(1),
            )?,
            dateindex_to_open_in_dollars: StorableVec::import(
                &path.join("dateindex_to_open_in_dollars"),
                Version::from(1),
            )?,
            height_to_close_in_cents: StorableVec::import(&path.join("height_to_close_in_cents"), Version::from(1))?,
            height_to_close_in_dollars: StorableVec::import(
                &path.join("height_to_close_in_dollars"),
                Version::from(1),
            )?,
            height_to_high_in_cents: StorableVec::import(&path.join("height_to_high_in_cents"), Version::from(1))?,
            height_to_high_in_dollars: StorableVec::import(&path.join("height_to_high_in_dollars"), Version::from(1))?,
            height_to_low_in_cents: StorableVec::import(&path.join("height_to_low_in_cents"), Version::from(1))?,
            height_to_low_in_dollars: StorableVec::import(&path.join("height_to_low_in_dollars"), Version::from(1))?,
            height_to_open_in_cents: StorableVec::import(&path.join("height_to_open_in_cents"), Version::from(1))?,
            height_to_open_in_dollars: StorableVec::import(&path.join("height_to_open_in_dollars"), Version::from(1))?,
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

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        vec![
            &self.height_to_date as &dyn AnyStorableVec,
            // &self.dateindex_to_close_in_dollars,
            // &self.dateindex_to_high_in_cents,
            // &self.dateindex_to_high_in_dollars,
            // &self.dateindex_to_low_in_cents,
            // &self.dateindex_to_low_in_dollars,
            // &self.dateindex_to_open_in_cents,
            // &self.dateindex_to_open_in_dollars,
            // &self.height_to_close_in_cents,
            // &self.height_to_close_in_dollars,
            // &self.height_to_high_in_cents,
            // &self.height_to_high_in_dollars,
            // &self.height_to_low_in_cents,
            // &self.height_to_low_in_dollars,
            // &self.height_to_open_in_cents,
            // &self.height_to_open_in_dollars,
        ]
    }
}
