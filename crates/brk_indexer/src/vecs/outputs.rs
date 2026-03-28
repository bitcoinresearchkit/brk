use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutputType, Sats, TxIndex, TxOutIndex, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, BytesVec, Database, ImportableVec, PcoVec, Rw, Stamp, StorageMode, WritableVec,
};

use crate::parallel_import;

#[derive(Traversable)]
pub struct OutputsVecs<M: StorageMode = Rw> {
    pub first_txout_index: M::Stored<PcoVec<Height, TxOutIndex>>,
    pub value: M::Stored<BytesVec<TxOutIndex, Sats>>,
    pub output_type: M::Stored<BytesVec<TxOutIndex, OutputType>>,
    pub type_index: M::Stored<BytesVec<TxOutIndex, TypeIndex>>,
    pub tx_index: M::Stored<PcoVec<TxOutIndex, TxIndex>>,
}

impl OutputsVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (first_txout_index, value, output_type, type_index, tx_index) = parallel_import! {
            first_txout_index = PcoVec::forced_import(db, "first_txout_index", version),
            value = BytesVec::forced_import(db, "value", version),
            output_type = BytesVec::forced_import(db, "output_type", version),
            type_index = BytesVec::forced_import(db, "type_index", version),
            tx_index = PcoVec::forced_import(db, "tx_index", version),
        };
        Ok(Self {
            first_txout_index,
            value,
            output_type,
            type_index,
            tx_index,
        })
    }

    pub fn truncate(
        &mut self,
        height: Height,
        txout_index: TxOutIndex,
        stamp: Stamp,
    ) -> Result<()> {
        self.first_txout_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.value
            .truncate_if_needed_with_stamp(txout_index, stamp)?;
        self.output_type
            .truncate_if_needed_with_stamp(txout_index, stamp)?;
        self.type_index
            .truncate_if_needed_with_stamp(txout_index, stamp)?;
        self.tx_index
            .truncate_if_needed_with_stamp(txout_index, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_txout_index as &mut dyn AnyStoredVec,
            &mut self.value,
            &mut self.output_type,
            &mut self.type_index,
            &mut self.tx_index,
        ]
        .into_par_iter()
    }
}
