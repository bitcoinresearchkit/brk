//! ValueTxFull - eager txindex Sats source + ValueDerivedTxFull (sats/bitcoin/dollars).

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Sats, TxIndex, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec};

use crate::{ComputeIndexes, indexes, price};

use super::ValueDerivedTxFull;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ValueTxFull {
    #[traversable(wrap = "sats")]
    pub base: EagerVec<PcoVec<TxIndex, Sats>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub indexes: ValueDerivedTxFull,
}

impl ValueTxFull {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        indexer: &Indexer,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;
        let txindex = EagerVec::forced_import(db, name, v)?;
        let derived =
            ValueDerivedTxFull::forced_import(db, name, v, indexes, indexer, price, &txindex)?;
        Ok(Self {
            base: txindex,
            indexes: derived,
        })
    }

    pub fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes
            .derive_from(indexer, indexes, starting_indexes, &self.base, exit)
    }
}
