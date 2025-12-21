use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Height, RawLockTime, StoredBool, StoredU32, TxInIndex, TxIndex, TxOutIndex, TxVersion, Txid,
    Version,
};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

#[derive(Clone, Traversable)]
pub struct TxVecs {
    pub height_to_first_txindex: PcoVec<Height, TxIndex>,
    pub txindex_to_height: PcoVec<TxIndex, Height>,
    pub txindex_to_txid: BytesVec<TxIndex, Txid>,
    pub txindex_to_txversion: PcoVec<TxIndex, TxVersion>,
    pub txindex_to_rawlocktime: PcoVec<TxIndex, RawLockTime>,
    pub txindex_to_base_size: PcoVec<TxIndex, StoredU32>,
    pub txindex_to_total_size: PcoVec<TxIndex, StoredU32>,
    pub txindex_to_is_explicitly_rbf: PcoVec<TxIndex, StoredBool>,
    pub txindex_to_first_txinindex: PcoVec<TxIndex, TxInIndex>,
    pub txindex_to_first_txoutindex: BytesVec<TxIndex, TxOutIndex>,
}

impl TxVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_to_first_txindex: PcoVec::forced_import(db, "first_txindex", version)?,
            txindex_to_height: PcoVec::forced_import(db, "height", version)?,
            txindex_to_txid: BytesVec::forced_import(db, "txid", version)?,
            txindex_to_txversion: PcoVec::forced_import(db, "txversion", version)?,
            txindex_to_rawlocktime: PcoVec::forced_import(db, "rawlocktime", version)?,
            txindex_to_base_size: PcoVec::forced_import(db, "base_size", version)?,
            txindex_to_total_size: PcoVec::forced_import(db, "total_size", version)?,
            txindex_to_is_explicitly_rbf: PcoVec::forced_import(db, "is_explicitly_rbf", version)?,
            txindex_to_first_txinindex: PcoVec::forced_import(db, "first_txinindex", version)?,
            txindex_to_first_txoutindex: BytesVec::forced_import(db, "first_txoutindex", version)?,
        })
    }

    pub fn truncate(&mut self, height: Height, txindex: TxIndex, stamp: Stamp) -> Result<()> {
        self.height_to_first_txindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.txindex_to_height
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_txid
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_txversion
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_rawlocktime
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_base_size
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_total_size
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_is_explicitly_rbf
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_first_txinindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_first_txoutindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_first_txindex as &mut dyn AnyStoredVec,
            &mut self.txindex_to_height,
            &mut self.txindex_to_txid,
            &mut self.txindex_to_txversion,
            &mut self.txindex_to_rawlocktime,
            &mut self.txindex_to_base_size,
            &mut self.txindex_to_total_size,
            &mut self.txindex_to_is_explicitly_rbf,
            &mut self.txindex_to_first_txinindex,
            &mut self.txindex_to_first_txoutindex,
        ]
        .into_par_iter()
    }
}
