use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BlkPosition, Height, RawLockTime, StoredBool, StoredU32, TxInIndex, TxIndex, TxOutIndex,
    TxVersion, Txid, Version,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, BytesVec, Database, ImportableVec, PcoVec, Rw, Stamp, StorageMode, WritableVec,
};

use crate::parallel_import;

#[derive(Traversable)]
pub struct TransactionsVecs<M: StorageMode = Rw> {
    pub first_tx_index: M::Stored<PcoVec<Height, TxIndex>>,
    pub txid: M::Stored<BytesVec<TxIndex, Txid>>,
    pub tx_version: M::Stored<PcoVec<TxIndex, TxVersion>>,
    pub raw_locktime: M::Stored<PcoVec<TxIndex, RawLockTime>>,
    pub base_size: M::Stored<PcoVec<TxIndex, StoredU32>>,
    pub total_size: M::Stored<PcoVec<TxIndex, StoredU32>>,
    pub is_explicitly_rbf: M::Stored<PcoVec<TxIndex, StoredBool>>,
    pub first_txin_index: M::Stored<PcoVec<TxIndex, TxInIndex>>,
    pub first_txout_index: M::Stored<BytesVec<TxIndex, TxOutIndex>>,
    #[traversable(hidden)]
    pub position: M::Stored<PcoVec<TxIndex, BlkPosition>>,
}

pub struct TxMetadataVecs<'a> {
    pub tx_version: &'a mut PcoVec<TxIndex, TxVersion>,
    pub txid: &'a mut BytesVec<TxIndex, Txid>,
    pub raw_locktime: &'a mut PcoVec<TxIndex, RawLockTime>,
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
            &mut self.first_txout_index,
            &mut self.first_txin_index,
            TxMetadataVecs {
                tx_version: &mut self.tx_version,
                txid: &mut self.txid,
                raw_locktime: &mut self.raw_locktime,
                base_size: &mut self.base_size,
                total_size: &mut self.total_size,
                is_explicitly_rbf: &mut self.is_explicitly_rbf,
            },
        )
    }

    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            first_tx_index,
            txid,
            tx_version,
            raw_locktime,
            base_size,
            total_size,
            is_explicitly_rbf,
            first_txin_index,
            first_txout_index,
            position,
        ) = parallel_import! {
            first_tx_index = PcoVec::forced_import(db, "first_tx_index", version),
            txid = BytesVec::forced_import(db, "txid", version),
            tx_version = PcoVec::forced_import(db, "tx_version", version),
            raw_locktime = PcoVec::forced_import(db, "raw_locktime", version),
            base_size = PcoVec::forced_import(db, "base_size", version),
            total_size = PcoVec::forced_import(db, "total_size", version),
            is_explicitly_rbf = PcoVec::forced_import(db, "is_explicitly_rbf", version),
            first_txin_index = PcoVec::forced_import(db, "first_txin_index", version),
            first_txout_index = BytesVec::forced_import(db, "first_txout_index", version),
            position = PcoVec::forced_import(db, "tx_position", version),
        };
        Ok(Self {
            first_tx_index,
            txid,
            tx_version,
            raw_locktime,
            base_size,
            total_size,
            is_explicitly_rbf,
            first_txin_index,
            first_txout_index,
            position,
        })
    }

    pub fn truncate(&mut self, height: Height, tx_index: TxIndex, stamp: Stamp) -> Result<()> {
        self.first_tx_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.txid.truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.tx_version
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.raw_locktime
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.base_size
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.total_size
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.is_explicitly_rbf
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.first_txin_index
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.first_txout_index
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        self.position
            .truncate_if_needed_with_stamp(tx_index, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_tx_index as &mut dyn AnyStoredVec,
            &mut self.txid,
            &mut self.tx_version,
            &mut self.raw_locktime,
            &mut self.base_size,
            &mut self.total_size,
            &mut self.is_explicitly_rbf,
            &mut self.first_txin_index,
            &mut self.first_txout_index,
            &mut self.position,
        ]
        .into_par_iter()
    }
}
