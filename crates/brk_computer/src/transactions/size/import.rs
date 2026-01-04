use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{TxIndex, VSize, Version, Weight};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom2, VecIndex};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromTxindex, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
        };

        let txindex_to_weight = LazyVecFrom2::init(
            "weight",
            version,
            indexer.vecs.tx.txindex_to_base_size.boxed_clone(),
            indexer.vecs.tx.txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.to_usize();
                txindex_to_base_size_iter.get_at(index).map(|base_size| {
                    let total_size = txindex_to_total_size_iter.get_at_unwrap(index);
                    Weight::from_sizes(*base_size, *total_size)
                })
            },
        );

        // Derive directly from eager sources to avoid Lazy <- Lazy
        let txindex_to_vsize = LazyVecFrom2::init(
            "vsize",
            version,
            indexer.vecs.tx.txindex_to_base_size.boxed_clone(),
            indexer.vecs.tx.txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.to_usize();
                txindex_to_base_size_iter.get_at(index).map(|base_size| {
                    let total_size = txindex_to_total_size_iter.get_at_unwrap(index);
                    VSize::from(Weight::from_sizes(*base_size, *total_size))
                })
            },
        );

        Ok(Self {
            indexes_to_tx_vsize: ComputedVecsFromTxindex::forced_import(
                db,
                "tx_vsize",
                Source::Vec(txindex_to_vsize.boxed_clone()),
                version,
                indexes,
                stats(),
            )?,
            indexes_to_tx_weight: ComputedVecsFromTxindex::forced_import(
                db,
                "tx_weight",
                Source::Vec(txindex_to_weight.boxed_clone()),
                version,
                indexes,
                stats(),
            )?,
            txindex_to_vsize,
            txindex_to_weight,
        })
    }
}
