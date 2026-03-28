//! PerTxDistribution - stored per-tx EagerVec + computed distribution.
//!
//! Like LazyFromTxDistribution, but the per-tx source is eagerly computed
//! and stored rather than lazily derived.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Indexes, TxIndex};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode, Version};

use crate::{
    indexes,
    internal::{ComputedVecValue, NumericValue, TxDerivedDistribution},
};

#[derive(Traversable)]
pub struct PerTxDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub tx_index: M::Stored<EagerVec<PcoVec<TxIndex, T>>>,
    #[traversable(flatten)]
    pub distribution: TxDerivedDistribution<T, M>,
}

impl<T> PerTxDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let tx_index = EagerVec::forced_import(db, name, version)?;
        let distribution = TxDerivedDistribution::forced_import(db, name, version, indexes)?;
        Ok(Self {
            tx_index,
            distribution,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
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
            &self.tx_index,
            exit,
            skip_count,
        )
    }
}
