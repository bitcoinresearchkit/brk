use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{TxIndex, VSize, Version, Weight};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom2, VecIndex};

use super::Vecs;
use crate::{indexes, internal::LazyTxDistribution};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let txindex_to_weight = LazyVecFrom2::init(
            "weight",
            version,
            indexer.vecs.transactions.base_size.boxed_clone(),
            indexer.vecs.transactions.total_size.boxed_clone(),
            |index: TxIndex, base_size_iter, total_size_iter| {
                let index = index.to_usize();
                base_size_iter.get_at(index).map(|base_size| {
                    let total_size = total_size_iter.get_at_unwrap(index);
                    Weight::from_sizes(*base_size, *total_size)
                })
            },
        );

        let txindex_to_vsize = LazyVecFrom2::init(
            "vsize",
            version,
            indexer.vecs.transactions.base_size.boxed_clone(),
            indexer.vecs.transactions.total_size.boxed_clone(),
            |index: TxIndex, base_size_iter, total_size_iter| {
                let index = index.to_usize();
                base_size_iter.get_at(index).map(|base_size| {
                    let total_size = total_size_iter.get_at_unwrap(index);
                    VSize::from(Weight::from_sizes(*base_size, *total_size))
                })
            },
        );

        Ok(Self {
            vsize: LazyTxDistribution::forced_import(
                db,
                "tx_vsize",
                version,
                txindex_to_vsize,
                indexes,
            )?,
            weight: LazyTxDistribution::forced_import(
                db,
                "tx_weight",
                version,
                txindex_to_weight,
                indexes,
            )?,
        })
    }
}
