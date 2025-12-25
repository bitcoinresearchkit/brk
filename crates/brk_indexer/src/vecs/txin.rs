use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutPoint, OutputType, Sats, TxInIndex, TxIndex, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct TxinVecs {
    pub height_to_first_txinindex: PcoVec<Height, TxInIndex>,
    pub txinindex_to_outpoint: PcoVec<TxInIndex, OutPoint>,
    pub txinindex_to_txindex: PcoVec<TxInIndex, TxIndex>,
    pub txinindex_to_value: PcoVec<TxInIndex, Sats>,
    pub txinindex_to_prev_height: PcoVec<TxInIndex, Height>,
    pub txinindex_to_outputtype: PcoVec<TxInIndex, OutputType>,
    pub txinindex_to_typeindex: PcoVec<TxInIndex, TypeIndex>,
}

impl TxinVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            height_to_first_txinindex,
            txinindex_to_outpoint,
            txinindex_to_txindex,
            txinindex_to_value,
            txinindex_to_prev_height,
            txinindex_to_outputtype,
            txinindex_to_typeindex,
        ) = parallel_import! {
            height_to_first_txinindex = PcoVec::forced_import(db, "first_txinindex", version),
            txinindex_to_outpoint = PcoVec::forced_import(db, "outpoint", version),
            txinindex_to_txindex = PcoVec::forced_import(db, "txindex", version),
            txinindex_to_value = PcoVec::forced_import(db, "value", version),
            txinindex_to_prev_height = PcoVec::forced_import(db, "prev_height", version),
            txinindex_to_outputtype = PcoVec::forced_import(db, "outputtype", version),
            txinindex_to_typeindex = PcoVec::forced_import(db, "typeindex", version),
        };
        Ok(Self {
            height_to_first_txinindex,
            txinindex_to_outpoint,
            txinindex_to_txindex,
            txinindex_to_value,
            txinindex_to_prev_height,
            txinindex_to_outputtype,
            txinindex_to_typeindex,
        })
    }

    pub fn truncate(&mut self, height: Height, txinindex: TxInIndex, stamp: Stamp) -> Result<()> {
        self.height_to_first_txinindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.txinindex_to_outpoint
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.txinindex_to_txindex
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.txinindex_to_value
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.txinindex_to_prev_height
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.txinindex_to_outputtype
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.txinindex_to_typeindex
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_first_txinindex as &mut dyn AnyStoredVec,
            &mut self.txinindex_to_outpoint,
            &mut self.txinindex_to_txindex,
            &mut self.txinindex_to_value,
            &mut self.txinindex_to_prev_height,
            &mut self.txinindex_to_outputtype,
            &mut self.txinindex_to_typeindex,
        ]
        .into_par_iter()
    }
}
