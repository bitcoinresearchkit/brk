use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutputType, Sats, TxIndex, TxOutIndex, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

#[derive(Clone, Traversable)]
pub struct TxoutVecs {
    pub height_to_first_txoutindex: PcoVec<Height, TxOutIndex>,
    pub txoutindex_to_value: BytesVec<TxOutIndex, Sats>,
    pub txoutindex_to_outputtype: BytesVec<TxOutIndex, OutputType>,
    pub txoutindex_to_typeindex: BytesVec<TxOutIndex, TypeIndex>,
    pub txoutindex_to_txindex: PcoVec<TxOutIndex, TxIndex>,
}

impl TxoutVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_to_first_txoutindex: PcoVec::forced_import(db, "first_txoutindex", version)?,
            txoutindex_to_value: BytesVec::forced_import(db, "value", version)?,
            txoutindex_to_outputtype: BytesVec::forced_import(db, "outputtype", version)?,
            txoutindex_to_typeindex: BytesVec::forced_import(db, "typeindex", version)?,
            txoutindex_to_txindex: PcoVec::forced_import(db, "txindex", version)?,
        })
    }

    pub fn truncate(&mut self, height: Height, txoutindex: TxOutIndex, stamp: Stamp) -> Result<()> {
        self.height_to_first_txoutindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.txoutindex_to_value
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_outputtype
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_typeindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_txindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_first_txoutindex as &mut dyn AnyStoredVec,
            &mut self.txoutindex_to_value,
            &mut self.txoutindex_to_outputtype,
            &mut self.txoutindex_to_typeindex,
            &mut self.txoutindex_to_txindex,
        ]
        .into_par_iter()
    }
}
