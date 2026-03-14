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
    pub to_tx_index: M::Stored<PcoVec<I, TxIndex>>,
}

#[derive(Traversable)]
pub struct ScriptsVecs<M: StorageMode = Rw> {
    pub empty: ScriptTypeVecs<EmptyOutputIndex, M>,
    pub op_return: ScriptTypeVecs<OpReturnIndex, M>,
    pub p2ms: ScriptTypeVecs<P2MSOutputIndex, M>,
    pub unknown: ScriptTypeVecs<UnknownOutputIndex, M>,
}

impl ScriptsVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            first_empty_output_index,
            first_op_return_index,
            first_p2ms_output_index,
            first_unknown_output_index,
            empty_output_index_to_tx_index,
            op_return_index_to_tx_index,
            p2ms_output_index_to_tx_index,
            unknown_output_index_to_tx_index,
        ) = parallel_import! {
            first_empty_output_index = PcoVec::forced_import(db, "first_empty_output_index", version),
            first_op_return_index = PcoVec::forced_import(db, "first_op_return_index", version),
            first_p2ms_output_index = PcoVec::forced_import(db, "first_p2ms_output_index", version),
            first_unknown_output_index = PcoVec::forced_import(db, "first_unknown_output_index", version),
            empty_output_index_to_tx_index = PcoVec::forced_import(db, "tx_index", version),
            op_return_index_to_tx_index = PcoVec::forced_import(db, "tx_index", version),
            p2ms_output_index_to_tx_index = PcoVec::forced_import(db, "tx_index", version),
            unknown_output_index_to_tx_index = PcoVec::forced_import(db, "tx_index", version),
        };
        Ok(Self {
            empty: ScriptTypeVecs { first_index: first_empty_output_index, to_tx_index: empty_output_index_to_tx_index },
            op_return: ScriptTypeVecs { first_index: first_op_return_index, to_tx_index: op_return_index_to_tx_index },
            p2ms: ScriptTypeVecs { first_index: first_p2ms_output_index, to_tx_index: p2ms_output_index_to_tx_index },
            unknown: ScriptTypeVecs { first_index: first_unknown_output_index, to_tx_index: unknown_output_index_to_tx_index },
        })
    }

    pub fn truncate(
        &mut self,
        height: Height,
        empty_output_index: EmptyOutputIndex,
        op_return_index: OpReturnIndex,
        p2ms_output_index: P2MSOutputIndex,
        unknown_output_index: UnknownOutputIndex,
        stamp: Stamp,
    ) -> Result<()> {
        self.empty.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.op_return.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2ms.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.unknown.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.empty.to_tx_index
            .truncate_if_needed_with_stamp(empty_output_index, stamp)?;
        self.op_return.to_tx_index
            .truncate_if_needed_with_stamp(op_return_index, stamp)?;
        self.p2ms.to_tx_index
            .truncate_if_needed_with_stamp(p2ms_output_index, stamp)?;
        self.unknown.to_tx_index
            .truncate_if_needed_with_stamp(unknown_output_index, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.empty.first_index as &mut dyn AnyStoredVec,
            &mut self.op_return.first_index,
            &mut self.p2ms.first_index,
            &mut self.unknown.first_index,
            &mut self.empty.to_tx_index,
            &mut self.op_return.to_tx_index,
            &mut self.p2ms.to_tx_index,
            &mut self.unknown.to_tx_index,
        ]
        .into_par_iter()
    }
}
