use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{TxIndex, VSize, Version, Weight};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom2, VecIndex};

use super::Vecs;
use crate::{indexes, internal::LazyFromTxDistribution};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let txindex_to_weight = LazyVecFrom2::init(
            "tx_weight",
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
            "tx_vsize",
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
            vsize: LazyFromTxDistribution::forced_import(
                db,
                "tx_vsize",
                version,
                txindex_to_vsize,
                indexes,
            )?,
            weight: LazyFromTxDistribution::forced_import(
                db,
                "tx_weight",
                version,
                txindex_to_weight,
                indexes,
            )?,
        })
    }
}
