//! ComputedFromTxDistribution - stored per-tx EagerVec + computed distribution.
//!
//! Like LazyFromTxDistribution, but the per-tx source is eagerly computed
//! and stored rather than lazily derived.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::TxIndex;
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode, Version};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, NumericValue, TxDerivedDistribution},
};

#[derive(Traversable)]
pub struct ComputedFromTxDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub txindex: M::Stored<EagerVec<PcoVec<TxIndex, T>>>,
    #[traversable(flatten)]
    pub distribution: TxDerivedDistribution<T, M>,
}

impl<T> ComputedFromTxDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let txindex = EagerVec::forced_import(db, name, version)?;
        let distribution = TxDerivedDistribution::forced_import(db, name, version)?;
        Ok(Self {
            txindex,
            distribution,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
    {
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
