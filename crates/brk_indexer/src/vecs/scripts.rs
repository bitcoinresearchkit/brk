use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    EmptyOutputIndex, Height, OpReturnIndex, P2MSOutputIndex, TxIndex, UnknownOutputIndex, Version,
};
use rayon::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    AnyStoredVec, Database, Formattable, ImportableVec, PcoVec, PcoVecValue, Rw, Stamp,
    StorageMode, VecIndex, WritableVec,
};

use crate::parallel_import;

#[derive(Traversable)]
pub struct ScriptTypeVecs<I: VecIndex + PcoVecValue + Formattable + Serialize + JsonSchema, M: StorageMode = Rw> {
    pub first_index: M::Stored<PcoVec<Height, I>>,
    pub to_txindex: M::Stored<PcoVec<I, TxIndex>>,
}

#[derive(Traversable)]
pub struct ScriptsVecs<M: StorageMode = Rw> {
    pub empty: ScriptTypeVecs<EmptyOutputIndex, M>,
    pub opreturn: ScriptTypeVecs<OpReturnIndex, M>,
    pub p2ms: ScriptTypeVecs<P2MSOutputIndex, M>,
    pub unknown: ScriptTypeVecs<UnknownOutputIndex, M>,
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
            empty: ScriptTypeVecs { first_index: first_emptyoutputindex, to_txindex: emptyoutputindex_to_txindex },
            opreturn: ScriptTypeVecs { first_index: first_opreturnindex, to_txindex: opreturnindex_to_txindex },
            p2ms: ScriptTypeVecs { first_index: first_p2msoutputindex, to_txindex: p2msoutputindex_to_txindex },
            unknown: ScriptTypeVecs { first_index: first_unknownoutputindex, to_txindex: unknownoutputindex_to_txindex },
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
        self.empty.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.opreturn.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2ms.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.unknown.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.empty.to_txindex
            .truncate_if_needed_with_stamp(emptyoutputindex, stamp)?;
        self.opreturn.to_txindex
            .truncate_if_needed_with_stamp(opreturnindex, stamp)?;
        self.p2ms.to_txindex
            .truncate_if_needed_with_stamp(p2msoutputindex, stamp)?;
        self.unknown.to_txindex
            .truncate_if_needed_with_stamp(unknownoutputindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.empty.first_index as &mut dyn AnyStoredVec,
            &mut self.opreturn.first_index,
            &mut self.p2ms.first_index,
            &mut self.unknown.first_index,
            &mut self.empty.to_txindex,
            &mut self.opreturn.to_txindex,
            &mut self.p2ms.to_txindex,
            &mut self.unknown.to_txindex,
        ]
        .into_par_iter()
    }
}
