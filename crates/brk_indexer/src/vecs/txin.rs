use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, OutPoint, TxInIndex, Version};
use vecdb::{AnyStoredVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

#[derive(Clone, Traversable)]
pub struct TxinVecs {
    pub height_to_first_txinindex: PcoVec<Height, TxInIndex>,
    pub txinindex_to_outpoint: PcoVec<TxInIndex, OutPoint>,
}

impl TxinVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_to_first_txinindex: PcoVec::forced_import(db, "first_txinindex", version)?,
            txinindex_to_outpoint: PcoVec::forced_import(db, "outpoint", version)?,
        })
    }

    pub fn truncate(&mut self, height: Height, txinindex: TxInIndex, stamp: Stamp) -> Result<()> {
        self.height_to_first_txinindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.txinindex_to_outpoint
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        Ok(())
    }

    pub fn iter_mut_any(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_first_txinindex as &mut dyn AnyStoredVec,
            &mut self.txinindex_to_outpoint,
        ]
        .into_iter()
    }
}
