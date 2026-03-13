use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutPoint, OutputType, TxInIndex, TxIndex, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, ImportableVec, PcoVec, Rw, Stamp, StorageMode, WritableVec};

use crate::parallel_import;

#[derive(Traversable)]
pub struct InputsVecs<M: StorageMode = Rw> {
    pub first_txin_index: M::Stored<PcoVec<Height, TxInIndex>>,
    pub outpoint: M::Stored<PcoVec<TxInIndex, OutPoint>>,
    pub tx_index: M::Stored<PcoVec<TxInIndex, TxIndex>>,
    pub output_type: M::Stored<PcoVec<TxInIndex, OutputType>>,
    pub type_index: M::Stored<PcoVec<TxInIndex, TypeIndex>>,
}

impl InputsVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (first_txin_index, outpoint, tx_index, output_type, type_index) = parallel_import! {
            first_txin_index = PcoVec::forced_import(db, "first_txin_index", version),
            outpoint = PcoVec::forced_import(db, "outpoint", version),
            tx_index = PcoVec::forced_import(db, "tx_index", version),
            output_type = PcoVec::forced_import(db, "output_type", version),
            type_index = PcoVec::forced_import(db, "type_index", version),
        };
        Ok(Self {
            first_txin_index,
            outpoint,
            tx_index,
            output_type,
            type_index,
        })
    }

    pub fn truncate(&mut self, height: Height, txin_index: TxInIndex, stamp: Stamp) -> Result<()> {
        self.first_txin_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.outpoint
            .truncate_if_needed_with_stamp(txin_index, stamp)?;
        self.tx_index
            .truncate_if_needed_with_stamp(txin_index, stamp)?;
        self.output_type
            .truncate_if_needed_with_stamp(txin_index, stamp)?;
        self.type_index
            .truncate_if_needed_with_stamp(txin_index, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_txin_index as &mut dyn AnyStoredVec,
            &mut self.outpoint,
            &mut self.tx_index,
            &mut self.output_type,
            &mut self.type_index,
        ]
        .into_par_iter()
    }
}
