//! ComputedFromTxDistribution - eager txindex source + derived distribution.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{TxIndex, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, TxDerivedDistribution, NumericValue},
};

const VERSION: Version = Version::ZERO;

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromTxDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub txindex: M::Stored<EagerVec<PcoVec<TxIndex, T>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub distribution: TxDerivedDistribution<T, M>,
}

impl<T> ComputedFromTxDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        let txindex = EagerVec::forced_import(db, name, v)?;
        let distribution = TxDerivedDistribution::forced_import(db, name, v, indexes)?;
        Ok(Self { txindex, distribution })
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
        self.distribution.derive_from_with_skip(
            indexer,
            indexes,
            starting_indexes,
            &self.txindex,
            exit,
            skip_count,
        )
    }
}
