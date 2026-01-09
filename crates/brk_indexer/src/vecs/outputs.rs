use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutputType, Sats, TxIndex, TxOutIndex, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct OutputsVecs {
    pub first_txoutindex: PcoVec<Height, TxOutIndex>,
    pub value: BytesVec<TxOutIndex, Sats>,
    pub outputtype: BytesVec<TxOutIndex, OutputType>,
    pub typeindex: BytesVec<TxOutIndex, TypeIndex>,
    pub txindex: PcoVec<TxOutIndex, TxIndex>,
}

impl OutputsVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            first_txoutindex,
            value,
            outputtype,
            typeindex,
            txindex,
        ) = parallel_import! {
            first_txoutindex = PcoVec::forced_import(db, "first_txoutindex", version),
            value = BytesVec::forced_import(db, "value", version),
            outputtype = BytesVec::forced_import(db, "outputtype", version),
            typeindex = BytesVec::forced_import(db, "typeindex", version),
            txindex = PcoVec::forced_import(db, "txindex", version),
        };
        Ok(Self {
            first_txoutindex,
            value,
            outputtype,
            typeindex,
            txindex,
        })
    }

    pub fn truncate(&mut self, height: Height, txoutindex: TxOutIndex, stamp: Stamp) -> Result<()> {
        self.first_txoutindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.value
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.outputtype
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.typeindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_txoutindex as &mut dyn AnyStoredVec,
            &mut self.value,
            &mut self.outputtype,
            &mut self.typeindex,
            &mut self.txindex,
        ]
        .into_par_iter()
    }
}
