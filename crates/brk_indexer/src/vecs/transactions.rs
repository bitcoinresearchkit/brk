use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Height, RawLockTime, StoredBool, StoredU32, TxInIndex, TxIndex, TxOutIndex, TxVersion, Txid,
    Version,
};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, BytesVec, Database, ImportableVec, PcoVec, Stamp, WritableVec};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct TransactionsVecs {
    pub first_txindex: PcoVec<Height, TxIndex>,
    pub height: PcoVec<TxIndex, Height>,
    pub txid: BytesVec<TxIndex, Txid>,
    pub txversion: PcoVec<TxIndex, TxVersion>,
    pub rawlocktime: PcoVec<TxIndex, RawLockTime>,
    pub base_size: PcoVec<TxIndex, StoredU32>,
    pub total_size: PcoVec<TxIndex, StoredU32>,
    pub is_explicitly_rbf: PcoVec<TxIndex, StoredBool>,
    pub first_txinindex: PcoVec<TxIndex, TxInIndex>,
    pub first_txoutindex: BytesVec<TxIndex, TxOutIndex>,
}

pub struct TxMetadataVecs<'a> {
    pub height: &'a mut PcoVec<TxIndex, Height>,
    pub txversion: &'a mut PcoVec<TxIndex, TxVersion>,
    pub txid: &'a mut BytesVec<TxIndex, Txid>,
    pub rawlocktime: &'a mut PcoVec<TxIndex, RawLockTime>,
    pub base_size: &'a mut PcoVec<TxIndex, StoredU32>,
    pub total_size: &'a mut PcoVec<TxIndex, StoredU32>,
    pub is_explicitly_rbf: &'a mut PcoVec<TxIndex, StoredBool>,
}

impl TransactionsVecs {
    pub fn split_for_finalize(
        &mut self,
    ) -> (
        &mut BytesVec<TxIndex, TxOutIndex>,
        &mut PcoVec<TxIndex, TxInIndex>,
        TxMetadataVecs<'_>,
    ) {
        (
            &mut self.first_txoutindex,
            &mut self.first_txinindex,
            TxMetadataVecs {
                height: &mut self.height,
                txversion: &mut self.txversion,
                txid: &mut self.txid,
                rawlocktime: &mut self.rawlocktime,
                base_size: &mut self.base_size,
                total_size: &mut self.total_size,
                is_explicitly_rbf: &mut self.is_explicitly_rbf,
            },
        )
    }

    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            first_txindex,
            height,
            txid,
            txversion,
            rawlocktime,
            base_size,
            total_size,
            is_explicitly_rbf,
            first_txinindex,
            first_txoutindex,
        ) = parallel_import! {
            first_txindex = PcoVec::forced_import(db, "first_txindex", version),
            height = PcoVec::forced_import(db, "height", version),
            txid = BytesVec::forced_import(db, "txid", version),
            txversion = PcoVec::forced_import(db, "txversion", version),
            rawlocktime = PcoVec::forced_import(db, "rawlocktime", version),
            base_size = PcoVec::forced_import(db, "base_size", version),
            total_size = PcoVec::forced_import(db, "total_size", version),
            is_explicitly_rbf = PcoVec::forced_import(db, "is_explicitly_rbf", version),
            first_txinindex = PcoVec::forced_import(db, "first_txinindex", version),
            first_txoutindex = BytesVec::forced_import(db, "first_txoutindex", version),
        };
        Ok(Self {
            first_txindex,
            height,
            txid,
            txversion,
            rawlocktime,
            base_size,
            total_size,
            is_explicitly_rbf,
            first_txinindex,
            first_txoutindex,
        })
    }

    pub fn truncate(&mut self, height: Height, txindex: TxIndex, stamp: Stamp) -> Result<()> {
        self.first_txindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height.truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txid.truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txversion
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.rawlocktime
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.base_size
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.total_size
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.is_explicitly_rbf
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.first_txinindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.first_txoutindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_txindex as &mut dyn AnyStoredVec,
            &mut self.height,
            &mut self.txid,
            &mut self.txversion,
            &mut self.rawlocktime,
            &mut self.base_size,
            &mut self.total_size,
            &mut self.is_explicitly_rbf,
            &mut self.first_txinindex,
            &mut self.first_txoutindex,
        ]
        .into_par_iter()
    }
}
