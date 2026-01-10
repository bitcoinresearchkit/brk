//! LazyFromTxDistribution - lazy txindex source + computed distribution.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{TxIndex, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{CollectableVec, Database, Exit, LazyVecFrom2};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, TxDerivedDistribution, NumericValue},
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyFromTxDistribution<T, S1, S2>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1: ComputedVecValue,
    S2: ComputedVecValue,
{
    pub txindex: LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub distribution: TxDerivedDistribution<T>,
}

impl<T, S1, S2> LazyFromTxDistribution<T, S1, S2>
where
    T: NumericValue + JsonSchema,
    S1: ComputedVecValue + JsonSchema,
    S2: ComputedVecValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        txindex: LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        let distribution = TxDerivedDistribution::forced_import(db, name, v, indexes)?;
        Ok(Self {
            txindex,
            distribution,
        })
    }

    pub fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()>
    where
        LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>: CollectableVec<TxIndex, T>,
    {
        self.distribution
            .derive_from(indexer, indexes, starting_indexes, &self.txindex, exit)
    }
}
