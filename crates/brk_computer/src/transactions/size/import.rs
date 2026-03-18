use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{TxIndex, VSize, Version, Weight};
use vecdb::{Database, LazyVecFrom2, ReadableCloneableVec};

use super::Vecs;
use crate::{indexes, internal::{LazyPerTxDistribution, LazyPerTxDistributionTransformed, VSizeToWeight}};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let tx_index_to_vsize = LazyVecFrom2::init(
            "tx_vsize",
            version,
            indexer.vecs.transactions.base_size.read_only_boxed_clone(),
            indexer.vecs.transactions.total_size.read_only_boxed_clone(),
            |_index: TxIndex, base_size, total_size| {
                VSize::from(Weight::from_sizes(*base_size, *total_size))
            },
        );

        let vsize =
            LazyPerTxDistribution::forced_import(db, "tx_vsize", version, indexes, tx_index_to_vsize)?;

        let tx_index_to_weight = LazyVecFrom2::init(
            "tx_weight",
            version,
            indexer.vecs.transactions.base_size.read_only_boxed_clone(),
            indexer.vecs.transactions.total_size.read_only_boxed_clone(),
            |_index: TxIndex, base_size, total_size| Weight::from_sizes(*base_size, *total_size),
        );

        let weight = LazyPerTxDistributionTransformed::new::<VSizeToWeight>(
            "tx_weight",
            version,
            tx_index_to_weight,
            &vsize.distribution,
        );

        Ok(Self { vsize, weight })
    }
}
