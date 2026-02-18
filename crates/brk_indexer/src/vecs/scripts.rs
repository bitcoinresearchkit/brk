use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    EmptyOutputIndex, Height, OpReturnIndex, P2MSOutputIndex, TxIndex, UnknownOutputIndex, Version,
};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, WritableVec, ImportableVec, PcoVec, Stamp};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct ScriptsVecs {
    // Height to first output index (per output type)
    pub first_emptyoutputindex: PcoVec<Height, EmptyOutputIndex>,
    pub first_opreturnindex: PcoVec<Height, OpReturnIndex>,
    pub first_p2msoutputindex: PcoVec<Height, P2MSOutputIndex>,
    pub first_unknownoutputindex: PcoVec<Height, UnknownOutputIndex>,
    // Output index to txindex (per output type)
    pub empty_to_txindex: PcoVec<EmptyOutputIndex, TxIndex>,
    pub opreturn_to_txindex: PcoVec<OpReturnIndex, TxIndex>,
    pub p2ms_to_txindex: PcoVec<P2MSOutputIndex, TxIndex>,
    pub unknown_to_txindex: PcoVec<UnknownOutputIndex, TxIndex>,
}

impl ScriptsVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            first_emptyoutputindex,
            first_opreturnindex,
            first_p2msoutputindex,
            first_unknownoutputindex,
            emptyoutputindex_to_txindex,
            opreturnindex_to_txindex,
            p2msoutputindex_to_txindex,
            unknownoutputindex_to_txindex,
        ) = parallel_import! {
            first_emptyoutputindex = PcoVec::forced_import(db, "first_emptyoutputindex", version),
            first_opreturnindex = PcoVec::forced_import(db, "first_opreturnindex", version),
            first_p2msoutputindex = PcoVec::forced_import(db, "first_p2msoutputindex", version),
            first_unknownoutputindex = PcoVec::forced_import(db, "first_unknownoutputindex", version),
            emptyoutputindex_to_txindex = PcoVec::forced_import(db, "txindex", version),
            opreturnindex_to_txindex = PcoVec::forced_import(db, "txindex", version),
            p2msoutputindex_to_txindex = PcoVec::forced_import(db, "txindex", version),
            unknownoutputindex_to_txindex = PcoVec::forced_import(db, "txindex", version),
        };
        Ok(Self {
            first_emptyoutputindex,
            first_opreturnindex,
            first_p2msoutputindex,
            first_unknownoutputindex,
            empty_to_txindex: emptyoutputindex_to_txindex,
            opreturn_to_txindex: opreturnindex_to_txindex,
            p2ms_to_txindex: p2msoutputindex_to_txindex,
            unknown_to_txindex: unknownoutputindex_to_txindex,
        })
    }

    pub fn truncate(
        &mut self,
        height: Height,
        emptyoutputindex: EmptyOutputIndex,
        opreturnindex: OpReturnIndex,
        p2msoutputindex: P2MSOutputIndex,
        unknownoutputindex: UnknownOutputIndex,
        stamp: Stamp,
    ) -> Result<()> {
        self.first_emptyoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_opreturnindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2msoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_unknownoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.empty_to_txindex
            .truncate_if_needed_with_stamp(emptyoutputindex, stamp)?;
        self.opreturn_to_txindex
            .truncate_if_needed_with_stamp(opreturnindex, stamp)?;
        self.p2ms_to_txindex
            .truncate_if_needed_with_stamp(p2msoutputindex, stamp)?;
        self.unknown_to_txindex
            .truncate_if_needed_with_stamp(unknownoutputindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_emptyoutputindex as &mut dyn AnyStoredVec,
            &mut self.first_opreturnindex,
            &mut self.first_p2msoutputindex,
            &mut self.first_unknownoutputindex,
            &mut self.empty_to_txindex,
            &mut self.opreturn_to_txindex,
            &mut self.p2ms_to_txindex,
            &mut self.unknown_to_txindex,
        ]
        .into_par_iter()
    }
}
