use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    EmptyOutputIndex, Height, OpReturnIndex, P2MSOutputIndex, TxIndex, UnknownOutputIndex, Version,
};
use vecdb::{AnyStoredVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

#[derive(Clone, Traversable)]
pub struct OutputVecs {
    // Height to first output index (per output type)
    pub height_to_first_emptyoutputindex: PcoVec<Height, EmptyOutputIndex>,
    pub height_to_first_opreturnindex: PcoVec<Height, OpReturnIndex>,
    pub height_to_first_p2msoutputindex: PcoVec<Height, P2MSOutputIndex>,
    pub height_to_first_unknownoutputindex: PcoVec<Height, UnknownOutputIndex>,
    // Output index to txindex (per output type)
    pub emptyoutputindex_to_txindex: PcoVec<EmptyOutputIndex, TxIndex>,
    pub opreturnindex_to_txindex: PcoVec<OpReturnIndex, TxIndex>,
    pub p2msoutputindex_to_txindex: PcoVec<P2MSOutputIndex, TxIndex>,
    pub unknownoutputindex_to_txindex: PcoVec<UnknownOutputIndex, TxIndex>,
}

impl OutputVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_to_first_emptyoutputindex: PcoVec::forced_import(
                db,
                "first_emptyoutputindex",
                version,
            )?,
            height_to_first_opreturnindex: PcoVec::forced_import(
                db,
                "first_opreturnindex",
                version,
            )?,
            height_to_first_p2msoutputindex: PcoVec::forced_import(
                db,
                "first_p2msoutputindex",
                version,
            )?,
            height_to_first_unknownoutputindex: PcoVec::forced_import(
                db,
                "first_unknownoutputindex",
                version,
            )?,
            emptyoutputindex_to_txindex: PcoVec::forced_import(db, "txindex", version)?,
            opreturnindex_to_txindex: PcoVec::forced_import(db, "txindex", version)?,
            p2msoutputindex_to_txindex: PcoVec::forced_import(db, "txindex", version)?,
            unknownoutputindex_to_txindex: PcoVec::forced_import(db, "txindex", version)?,
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
        self.height_to_first_emptyoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_opreturnindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2msoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_unknownoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.emptyoutputindex_to_txindex
            .truncate_if_needed_with_stamp(emptyoutputindex, stamp)?;
        self.opreturnindex_to_txindex
            .truncate_if_needed_with_stamp(opreturnindex, stamp)?;
        self.p2msoutputindex_to_txindex
            .truncate_if_needed_with_stamp(p2msoutputindex, stamp)?;
        self.unknownoutputindex_to_txindex
            .truncate_if_needed_with_stamp(unknownoutputindex, stamp)?;
        Ok(())
    }

    pub fn iter_mut_any(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_first_emptyoutputindex as &mut dyn AnyStoredVec,
            &mut self.height_to_first_opreturnindex,
            &mut self.height_to_first_p2msoutputindex,
            &mut self.height_to_first_unknownoutputindex,
            &mut self.emptyoutputindex_to_txindex,
            &mut self.opreturnindex_to_txindex,
            &mut self.p2msoutputindex_to_txindex,
            &mut self.unknownoutputindex_to_txindex,
        ]
        .into_iter()
    }
}
