use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredBool, TxIndex, VSize, Version, Weight};
use vecdb::{
    Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1, LazyVecFrom2, VecIndex,
};

use super::Vecs;
use crate::{
    grouped::{
        ComputedValueVecsFromTxindex, ComputedVecsFromHeight, ComputedVecsFromTxindex, Source,
        VecBuilderOptions,
    },
    indexes, price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v0 = Version::ZERO;

        let stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
        };
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();

        let txindex_to_weight = LazyVecFrom2::init(
            "weight",
            version + v0,
            indexer.vecs.tx.txindex_to_base_size.boxed_clone(),
            indexer.vecs.tx.txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.to_usize();
                txindex_to_base_size_iter.get_at(index).map(|base_size| {
                    let total_size = txindex_to_total_size_iter.get_at_unwrap(index);
                    let wu = usize::from(base_size) * 3 + usize::from(total_size);
                    Weight::from(bitcoin::Weight::from_wu_usize(wu))
                })
            },
        );

        let txindex_to_vsize = LazyVecFrom1::init(
            "vsize",
            version + v0,
            txindex_to_weight.boxed_clone(),
            |index: TxIndex, iter| iter.get(index).map(VSize::from),
        );

        let txindex_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version + v0,
            indexer.vecs.tx.txindex_to_height.boxed_clone(),
            indexer.vecs.tx.height_to_first_txindex.boxed_clone(),
            |index: TxIndex, txindex_to_height_iter, height_to_first_txindex_iter| {
                txindex_to_height_iter.get(index).map(|height| {
                    let txindex = height_to_first_txindex_iter.get_unwrap(height);
                    StoredBool::from(index == txindex)
                })
            },
        );

        let txindex_to_input_value = EagerVec::forced_import(db, "input_value", version + v0)?;
        let txindex_to_output_value = EagerVec::forced_import(db, "output_value", version + v0)?;
        let txindex_to_fee = EagerVec::forced_import(db, "fee", version + v0)?;
        let txindex_to_fee_rate = EagerVec::forced_import(db, "fee_rate", version + v0)?;

        Ok(Self {
            indexes_to_tx_count: ComputedVecsFromHeight::forced_import(
                db,
                "tx_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_tx_v1: ComputedVecsFromHeight::forced_import(
                db,
                "tx_v1",
                Source::Compute,
                version + v0,
                indexes,
                sum_cum(),
            )?,
            indexes_to_tx_v2: ComputedVecsFromHeight::forced_import(
                db,
                "tx_v2",
                Source::Compute,
                version + v0,
                indexes,
                sum_cum(),
            )?,
            indexes_to_tx_v3: ComputedVecsFromHeight::forced_import(
                db,
                "tx_v3",
                Source::Compute,
                version + v0,
                indexes,
                sum_cum(),
            )?,
            indexes_to_tx_vsize: ComputedVecsFromTxindex::forced_import(
                db,
                "tx_vsize",
                Source::Vec(txindex_to_vsize.boxed_clone()),
                version + v0,
                indexes,
                stats(),
            )?,
            indexes_to_tx_weight: ComputedVecsFromTxindex::forced_import(
                db,
                "tx_weight",
                Source::Vec(txindex_to_weight.boxed_clone()),
                version + v0,
                indexes,
                stats(),
            )?,
            indexes_to_input_count: ComputedVecsFromTxindex::forced_import(
                db,
                "input_count",
                Source::Vec(indexes.transaction.txindex_to_input_count.boxed_clone()),
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_output_count: ComputedVecsFromTxindex::forced_import(
                db,
                "output_count",
                Source::Vec(indexes.transaction.txindex_to_output_count.boxed_clone()),
                version + v0,
                indexes,
                full_stats(),
            )?,
            txindex_to_is_coinbase,
            txindex_to_vsize,
            txindex_to_weight,
            txindex_to_input_value,
            txindex_to_output_value,
            txindex_to_fee: txindex_to_fee.clone(),
            txindex_to_fee_rate: txindex_to_fee_rate.clone(),
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                db,
                "fee",
                indexer,
                indexes,
                Source::Vec(txindex_to_fee.boxed_clone()),
                version + v0,
                price,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_fee_rate: ComputedVecsFromTxindex::forced_import(
                db,
                "fee_rate",
                Source::Vec(txindex_to_fee_rate.boxed_clone()),
                version + v0,
                indexes,
                stats(),
            )?,
        })
    }
}
