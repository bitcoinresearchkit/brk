//! ValueFromTxFull - eager txindex Sats source + ValueTxDerivedFull (sats/bitcoin/dollars).

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Sats, TxIndex, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{ComputeIndexes, indexes, internal::ValueTxDerivedFull, prices};

const VERSION: Version = Version::ZERO;

#[derive(Deref, DerefMut, Traversable)]
pub struct ValueFromTxFull<M: StorageMode = Rw> {
    #[traversable(rename = "txindex")]
    pub base: M::Stored<EagerVec<PcoVec<TxIndex, Sats>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub indexes: ValueTxDerivedFull<M>,
}

impl ValueFromTxFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        indexer: &Indexer,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        let txindex = EagerVec::forced_import(db, name, v)?;
        let derived =
            ValueTxDerivedFull::forced_import(db, name, v, indexes, indexer, prices, &txindex)?;
        Ok(Self {
            base: txindex,
            indexes: derived,
        })
    }

    /// Derive from source, skipping first N transactions per block from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub(crate) fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()> {
        self.indexes.derive_from_with_skip(
            indexer,
            indexes,
            starting_indexes,
            &self.base,
            exit,
            skip_count,
        )
    }
}
