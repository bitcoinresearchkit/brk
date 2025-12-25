use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, TxInIndex, TxIndex, TxOutData, TxOutIndex, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, GenericStoredVec, ImportableVec, IterableCloneableVec,
    LazyVecFrom1, PcoVec, Stamp,
};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct TxoutVecs {
    pub height_to_first_txoutindex: PcoVec<Height, TxOutIndex>,
    pub txoutindex_to_txoutdata: BytesVec<TxOutIndex, TxOutData>,
    pub txoutindex_to_txindex: PcoVec<TxOutIndex, TxIndex>,
    pub txoutindex_to_txinindex: BytesVec<TxOutIndex, TxInIndex>,
    pub txoutindex_to_value: LazyVecFrom1<TxOutIndex, Sats, TxOutIndex, TxOutData>,
}

impl TxoutVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            height_to_first_txoutindex,
            txoutindex_to_txoutdata,
            txoutindex_to_txindex,
            txoutindex_to_txinindex,
        ) = parallel_import! {
            height_to_first_txoutindex = PcoVec::forced_import(db, "first_txoutindex", version),
            txoutindex_to_txoutdata = BytesVec::forced_import(db, "txoutdata", version),
            txoutindex_to_txindex = PcoVec::forced_import(db, "txindex", version),
            txoutindex_to_txinindex = BytesVec::forced_import(db, "txinindex", version),
        };
        let txoutindex_to_value = LazyVecFrom1::init(
            "value",
            txoutindex_to_txoutdata.version(),
            txoutindex_to_txoutdata.boxed_clone(),
            |index, iter| iter.get(index).map(|txoutdata: TxOutData| txoutdata.value),
        );
        Ok(Self {
            height_to_first_txoutindex,
            txoutindex_to_txoutdata,
            txoutindex_to_txindex,
            txoutindex_to_txinindex,
            txoutindex_to_value,
        })
    }

    pub fn truncate(&mut self, height: Height, txoutindex: TxOutIndex, stamp: Stamp) -> Result<()> {
        self.height_to_first_txoutindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.txoutindex_to_txoutdata
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_txindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_txinindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_first_txoutindex as &mut dyn AnyStoredVec,
            &mut self.txoutindex_to_txoutdata,
            &mut self.txoutindex_to_txindex,
            &mut self.txoutindex_to_txinindex,
        ]
        .into_par_iter()
    }
}
