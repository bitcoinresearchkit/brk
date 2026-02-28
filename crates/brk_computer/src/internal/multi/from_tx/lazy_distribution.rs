//! LazyFromTxDistribution - lazy txindex source + computed distribution.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::TxIndex;
use schemars::JsonSchema;
use vecdb::{Database, Exit, LazyVecFrom2, ReadableVec, Rw, StorageMode, Version};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, NumericValue, TxDerivedDistribution},
};

#[derive(Traversable)]
pub struct LazyFromTxDistribution<T, S1, S2, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1: ComputedVecValue,
    S2: ComputedVecValue,
{
    pub txindex: LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>,
    #[traversable(flatten)]
    pub distribution: TxDerivedDistribution<T, M>,
}

impl<T, S1, S2> LazyFromTxDistribution<T, S1, S2>
where
    T: NumericValue + JsonSchema,
    S1: ComputedVecValue + JsonSchema,
    S2: ComputedVecValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        txindex: LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>,
    ) -> Result<Self> {
        let distribution = TxDerivedDistribution::forced_import(db, name, version)?;
        Ok(Self {
            txindex,
            distribution,
        })
    }

    pub(crate) fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
        LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>: ReadableVec<TxIndex, T>,
    {
        self.distribution.derive_from(
            indexer,
            indexes,
            starting_indexes,
            &self.txindex,
            exit,
        )
    }
}
