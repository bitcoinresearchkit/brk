use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutPoint, OutputType, TxInIndex, TxIndex, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct InputsVecs {
    pub first_txinindex: PcoVec<Height, TxInIndex>,
    pub outpoint: PcoVec<TxInIndex, OutPoint>,
    pub txindex: PcoVec<TxInIndex, TxIndex>,
    pub outputtype: PcoVec<TxInIndex, OutputType>,
    pub typeindex: PcoVec<TxInIndex, TypeIndex>,
}

impl InputsVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (first_txinindex, outpoint, txindex, outputtype, typeindex) = parallel_import! {
            first_txinindex = PcoVec::forced_import(db, "first_txinindex", version),
            outpoint = PcoVec::forced_import(db, "outpoint", version),
            txindex = PcoVec::forced_import(db, "txindex", version),
            outputtype = PcoVec::forced_import(db, "outputtype", version),
            typeindex = PcoVec::forced_import(db, "typeindex", version),
        };
        Ok(Self {
            first_txinindex,
            outpoint,
            txindex,
            outputtype,
            typeindex,
        })
    }

    pub fn truncate(&mut self, height: Height, txinindex: TxInIndex, stamp: Stamp) -> Result<()> {
        self.first_txinindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.outpoint
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.txindex
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.outputtype
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.typeindex
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_txinindex as &mut dyn AnyStoredVec,
            &mut self.outpoint,
            &mut self.txindex,
            &mut self.outputtype,
            &mut self.typeindex,
        ]
        .into_par_iter()
    }
}
