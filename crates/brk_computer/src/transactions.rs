use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{
    CheckedSub, Feerate, HalvingEpoch, Height, InputIndex, OutputIndex, Sats, StoredBool,
    StoredU32, StoredU64, TxIndex, TxVersion, Version, Weight,
};
use vecdb::{
    AnyCloneableIterableVec, AnyCollectableVec, AnyIterableVec, Database, EagerVec, Exit,
    LazyVecFrom1, LazyVecFrom2, LazyVecFrom3, PAGE_SIZE, StoredIndex, VecIterator,
};

use crate::grouped::{
    ComputedValueVecsFromHeight, ComputedValueVecsFromTxindex, ComputedVecsFromHeight,
    ComputedVecsFromTxindex, Source, VecBuilderOptions,
};

use super::{Indexes, indexes, price};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    db: Database,

    // pub txindex_to_is_v1: LazyVec<Txindex, bool>,
    // pub txindex_to_is_v2: LazyVec<Txindex, bool>,
    // pub txindex_to_is_v3: LazyVec<Txindex, bool>,
    pub indexes_to_coinbase: ComputedValueVecsFromHeight,
    pub indexes_to_emptyoutput_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_fee: ComputedValueVecsFromTxindex,
    pub indexes_to_feerate: ComputedVecsFromTxindex<Feerate>,
    /// Value == 0 when Coinbase
    pub txindex_to_input_value:
        LazyVecFrom3<TxIndex, Sats, TxIndex, InputIndex, TxIndex, StoredU64, InputIndex, Sats>,
    // pub indexes_to_input_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredU64>,
    pub txindex_to_output_value:
        LazyVecFrom3<TxIndex, Sats, TxIndex, OutputIndex, TxIndex, StoredU64, OutputIndex, Sats>,
    // pub indexes_to_output_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_p2a_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2ms_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pk33_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pk65_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pkh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2sh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2tr_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2wpkh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2wsh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight,
    pub indexes_to_tx_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_vsize: ComputedVecsFromTxindex<StoredU64>,
    pub indexes_to_tx_weight: ComputedVecsFromTxindex<Weight>,
    pub indexes_to_unknownoutput_count: ComputedVecsFromHeight<StoredU64>,
    pub inputindex_to_value:
        LazyVecFrom2<InputIndex, Sats, InputIndex, OutputIndex, OutputIndex, Sats>,
    pub indexes_to_input_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
    pub indexes_to_output_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_vsize: LazyVecFrom1<TxIndex, StoredU64, TxIndex, Weight>,
    pub txindex_to_weight: LazyVecFrom2<TxIndex, Weight, TxIndex, StoredU32, TxIndex, StoredU32>,
    pub txindex_to_fee: EagerVec<TxIndex, Sats>,
    pub txindex_to_feerate: EagerVec<TxIndex, Feerate>,
    pub indexes_to_exact_utxo_count: ComputedVecsFromHeight<StoredU64>,
}

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent.join("transactions"))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let compute_dollars = price.is_some();

        let inputindex_to_value = LazyVecFrom2::init(
            "value",
            version + VERSION + Version::ZERO,
            indexer.vecs.inputindex_to_outputindex.boxed_clone(),
            indexer.vecs.outputindex_to_value.boxed_clone(),
            |index: InputIndex, inputindex_to_outputindex_iter, outputindex_to_value_iter| {
                inputindex_to_outputindex_iter
                    .next_at(index.unwrap_to_usize())
                    .map(|(inputindex, outputindex)| {
                        let outputindex = outputindex.into_owned();
                        if outputindex == OutputIndex::COINBASE {
                            Sats::ZERO
                        } else if let Some((_, value)) =
                            outputindex_to_value_iter.next_at(outputindex.unwrap_to_usize())
                        {
                            value.into_owned()
                        } else {
                            dbg!(inputindex, outputindex);
                            panic!()
                        }
                    })
            },
        );

        let txindex_to_weight = LazyVecFrom2::init(
            "weight",
            version + VERSION + Version::ZERO,
            indexer.vecs.txindex_to_base_size.boxed_clone(),
            indexer.vecs.txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.unwrap_to_usize();
                txindex_to_base_size_iter
                    .next_at(index)
                    .map(|(_, base_size)| {
                        let base_size = base_size.into_owned();
                        let total_size = txindex_to_total_size_iter
                            .next_at(index)
                            .unwrap()
                            .1
                            .into_owned();

                        // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                        let wu = usize::from(base_size) * 3 + usize::from(total_size);

                        Weight::from(bitcoin::Weight::from_wu_usize(wu))
                    })
            },
        );

        let txindex_to_vsize = LazyVecFrom1::init(
            "vsize",
            version + VERSION + Version::ZERO,
            txindex_to_weight.boxed_clone(),
            |index: TxIndex, iter| {
                let index = index.unwrap_to_usize();
                iter.next_at(index).map(|(_, weight)| {
                    StoredU64::from(
                        bitcoin::Weight::from(weight.into_owned()).to_vbytes_ceil() as usize
                    )
                })
            },
        );

        let txindex_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version + VERSION + Version::ZERO,
            indexes.txindex_to_height.boxed_clone(),
            indexer.vecs.height_to_first_txindex.boxed_clone(),
            |index: TxIndex, txindex_to_height_iter, height_to_first_txindex_iter| {
                txindex_to_height_iter
                    .next_at(index.unwrap_to_usize())
                    .map(|(_, height)| {
                        let height = height.into_owned();
                        let txindex = height_to_first_txindex_iter
                            .next_at(height.unwrap_to_usize())
                            .unwrap()
                            .1
                            .into_owned();
                        StoredBool::from(index == txindex)
                    })
            },
        );

        let txindex_to_input_value = LazyVecFrom3::init(
            "input_value",
            version + VERSION + Version::ZERO,
            indexer.vecs.txindex_to_first_inputindex.boxed_clone(),
            indexes.txindex_to_input_count.boxed_clone(),
            inputindex_to_value.boxed_clone(),
            |index: TxIndex,
             txindex_to_first_inputindex_iter,
             txindex_to_input_count_iter,
             inputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_inputindex_iter
                    .next_at(txindex)
                    .map(|(_, first_index)| {
                        let first_index = usize::from(first_index.into_owned());
                        let count = *txindex_to_input_count_iter
                            .next_at(txindex)
                            .unwrap()
                            .1
                            .into_owned();
                        let range = first_index..first_index + count as usize;
                        range.into_iter().fold(Sats::ZERO, |total, inputindex| {
                            total
                                + inputindex_to_value_iter
                                    .next_at(inputindex)
                                    .unwrap()
                                    .1
                                    .into_owned()
                        })
                    })
            },
        );

        // let indexes_to_input_value: ComputedVecsFromTxindex<Sats> =
        //     ComputedVecsFromTxindex::forced_import(
        //         db,
        //         "input_value",
        //         true,
        //         version + VERSION + Version::ZERO,
        //         format,
        // computation,
        // StorableVecGeneatorOptions::default()
        //             .add_average()
        //             .add_sum()
        //             .add_cumulative(),
        //     )?;

        let txindex_to_output_value = LazyVecFrom3::init(
            "output_value",
            version + VERSION + Version::ZERO,
            indexer.vecs.txindex_to_first_outputindex.boxed_clone(),
            indexes.txindex_to_output_count.boxed_clone(),
            indexer.vecs.outputindex_to_value.boxed_clone(),
            |index: TxIndex,
             txindex_to_first_outputindex_iter,
             txindex_to_output_count_iter,
             outputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_outputindex_iter
                    .next_at(txindex)
                    .map(|(_, first_index)| {
                        let first_index = usize::from(first_index.into_owned());
                        let count = *txindex_to_output_count_iter
                            .next_at(txindex)
                            .unwrap()
                            .1
                            .into_owned();
                        let range = first_index..first_index + count as usize;
                        range.into_iter().fold(Sats::ZERO, |total, outputindex| {
                            let v = outputindex_to_value_iter
                                .next_at(outputindex)
                                .unwrap()
                                .1
                                .into_owned();
                            total + v
                        })
                    })
            },
        );

        // let indexes_to_output_value: ComputedVecsFromTxindex<Sats> =
        //     ComputedVecsFromTxindex::forced_import(
        //         db,
        //         "output_value",
        //         true,
        //         version + VERSION + Version::ZERO,
        //         format,
        // computation,
        // StorableVecGeneatorOptions::default()
        //             .add_average()
        //             .add_sum()
        //             .add_cumulative(),
        //     )?;

        let txindex_to_fee =
            EagerVec::forced_import_compressed(&db, "fee", version + VERSION + Version::ZERO)?;

        let txindex_to_feerate =
            EagerVec::forced_import_compressed(&db, "feerate", version + VERSION + Version::ZERO)?;

        Ok(Self {
            indexes_to_tx_count: ComputedVecsFromHeight::forced_import(
                &db,
                "tx_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_input_count: ComputedVecsFromTxindex::forced_import(
                &db,
                "input_count",
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_output_count: ComputedVecsFromTxindex::forced_import(
                &db,
                "output_count",
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_tx_v1: ComputedVecsFromHeight::forced_import(
                &db,
                "tx_v1",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_tx_v2: ComputedVecsFromHeight::forced_import(
                &db,
                "tx_v2",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_tx_v3: ComputedVecsFromHeight::forced_import(
                &db,
                "tx_v3",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                &db,
                "fee",
                indexes,
                Source::Vec(txindex_to_fee.boxed_clone()),
                version + VERSION + Version::ZERO,
                price,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_feerate: ComputedVecsFromTxindex::forced_import(
                &db,
                "feerate",
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_tx_vsize: ComputedVecsFromTxindex::forced_import(
                &db,
                "tx_vsize",
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_tx_weight: ComputedVecsFromTxindex::forced_import(
                &db,
                "tx_weight",
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                &db,
                "subsidy",
                Source::Compute,
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                &db,
                "coinbase",
                Source::Compute,
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight::forced_import(
                &db,
                "unclaimed_rewards",
                Source::Compute,
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_p2a_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2a_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2ms_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2ms_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2pk33_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2pk33_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2pk65_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2pk65_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2pkh_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2pkh_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2sh_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2sh_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2tr_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2tr_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2wpkh_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2wpkh_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_p2wsh_count: ComputedVecsFromHeight::forced_import(
                &db,
                "p2wsh_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_opreturn_count: ComputedVecsFromHeight::forced_import(
                &db,
                "opreturn_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_unknownoutput_count: ComputedVecsFromHeight::forced_import(
                &db,
                "unknownoutput_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_emptyoutput_count: ComputedVecsFromHeight::forced_import(
                &db,
                "emptyoutput_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative(),
            )?,
            indexes_to_exact_utxo_count: ComputedVecsFromHeight::forced_import(
                &db,
                "exact_utxo_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            txindex_to_is_coinbase,
            inputindex_to_value,
            // indexes_to_input_value,
            // indexes_to_output_value,
            txindex_to_input_value,
            txindex_to_output_value,
            txindex_to_fee,
            txindex_to_feerate,
            txindex_to_vsize,
            txindex_to_weight,

            db,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, price, exit)?;
        self.db.flush_then_punch()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_tx_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_txindex,
                    &indexer.vecs.txindex_to_txid,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_input_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.txindex_to_input_count),
        )?;

        self.indexes_to_output_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.txindex_to_output_count),
        )?;

        let compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedVecsFromHeight<StoredU64>, txversion| {
                let mut txindex_to_txversion_iter = indexer.vecs.txindex_to_txversion.iter();
                indexes_to_tx_vany.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, indexer, _, starting_indexes, exit| {
                        vec.compute_filtered_count_from_indexes(
                            starting_indexes.height,
                            &indexer.vecs.height_to_first_txindex,
                            &indexer.vecs.txindex_to_txid,
                            |txindex| {
                                let v = txindex_to_txversion_iter.unwrap_get_inner(txindex);
                                v == txversion
                            },
                            exit,
                        )?;
                        Ok(())
                    },
                )
            };
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v1, TxVersion::ONE)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v2, TxVersion::TWO)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v3, TxVersion::THREE)?;

        // self.indexes_to_output_value.compute_all(
        //     indexer,
        //     indexes,
        //     starting_indexes,
        //     exit,
        //     |vec, indexer, _, starting_indexes, exit| {
        //         vec.compute_sum_from_indexes(
        //             starting_indexes.txindex,
        //             &indexer.vecs.txindex_to_first_outputindex,
        //             self.indexes_to_output_count.txindex.as_ref().unwrap(),
        //             &indexer.vecs.outputindex_to_value,
        //             exit,
        //         )
        //     },
        // )?;

        // self.indexes_to_input_value.compute_all(
        //     indexer,
        //     indexes,
        //     starting_indexes,
        //     exit,
        //     |vec, indexer, _, starting_indexes, exit| {
        //         vec.compute_sum_from_indexes(
        //             starting_indexes.txindex,
        //             &indexer.vecs.txindex_to_first_inputindex,
        //             self.indexes_to_input_count.txindex.as_ref().unwrap(),
        //             &self.inputindex_to_value,
        //             exit,
        //         )
        //     },
        // )?;

        self.txindex_to_fee.compute_transform3(
            starting_indexes.txindex,
            &self.txindex_to_input_value,
            &self.txindex_to_output_value,
            &self.txindex_to_is_coinbase,
            |(i, input, output, coinbase, ..)| {
                (
                    i,
                    if coinbase.is_true() {
                        Sats::ZERO
                    } else {
                        input.checked_sub(output).unwrap()
                    },
                )
            },
            exit,
        )?;

        self.txindex_to_feerate.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_fee,
            &self.txindex_to_vsize,
            |(txindex, fee, vsize, ..)| (txindex, Feerate::from((fee, vsize))),
            exit,
        )?;

        self.indexes_to_fee.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee),
            price,
        )?;

        self.indexes_to_feerate.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_feerate),
        )?;

        self.indexes_to_tx_weight.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_weight),
        )?;

        self.indexes_to_tx_vsize.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_vsize),
        )?;

        self.indexes_to_coinbase.compute_all(
            indexer,
            indexes,
            price,
            starting_indexes,
            exit,
            |vec, indexer, _, starting_indexes, exit| {
                let mut txindex_to_first_outputindex_iter =
                    indexer.vecs.txindex_to_first_outputindex.iter();
                let mut txindex_to_output_count_iter = indexes.txindex_to_output_count.iter();
                let mut outputindex_to_value_iter = indexer.vecs.outputindex_to_value.iter();
                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_txindex,
                    |(height, txindex, ..)| {
                        let first_outputindex = txindex_to_first_outputindex_iter
                            .unwrap_get_inner(txindex)
                            .unwrap_to_usize();
                        let output_count = txindex_to_output_count_iter.unwrap_get_inner(txindex);
                        let mut sats = Sats::ZERO;
                        (first_outputindex..first_outputindex + usize::from(output_count))
                            .for_each(|outputindex| {
                                sats += outputindex_to_value_iter
                                    .unwrap_get_inner(OutputIndex::from(outputindex));
                            });
                        (height, sats)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_subsidy.compute_all(
            indexer,
            indexes,
            price,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut indexes_to_fee_sum_iter =
                    self.indexes_to_fee.sats.height.unwrap_sum().iter();
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_coinbase.sats.height.as_ref().unwrap(),
                    |(height, coinbase, ..)| {
                        let fees = indexes_to_fee_sum_iter.unwrap_get_inner(height);
                        (height, coinbase.checked_sub(fees).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_unclaimed_rewards.compute_all(
            indexer,
            indexes,
            price,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_subsidy.sats.height.as_ref().unwrap(),
                    |(height, subsidy, ..)| {
                        let halving = HalvingEpoch::from(height);
                        let expected =
                            Sats::FIFTY_BTC / 2_usize.pow(halving.unwrap_to_usize() as u32);
                        (height, expected.checked_sub(subsidy).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2a_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2aaddressindex,
                    &indexer.vecs.p2aaddressindex_to_p2abytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2ms_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2msoutputindex,
                    &indexer.vecs.p2msoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2pk33_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2pk33addressindex,
                    &indexer.vecs.p2pk33addressindex_to_p2pk33bytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2pk65_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2pk65addressindex,
                    &indexer.vecs.p2pk65addressindex_to_p2pk65bytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2pkh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2pkhaddressindex,
                    &indexer.vecs.p2pkhaddressindex_to_p2pkhbytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2sh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2shaddressindex,
                    &indexer.vecs.p2shaddressindex_to_p2shbytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2tr_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2traddressindex,
                    &indexer.vecs.p2traddressindex_to_p2trbytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2wpkh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2wpkhaddressindex,
                    &indexer.vecs.p2wpkhaddressindex_to_p2wpkhbytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_p2wsh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2wshaddressindex,
                    &indexer.vecs.p2wshaddressindex_to_p2wshbytes,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_opreturn_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_opreturnindex,
                    &indexer.vecs.opreturnindex_to_txindex,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_unknownoutput_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_unknownoutputindex,
                    &indexer.vecs.unknownoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_emptyoutput_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_emptyoutputindex,
                    &indexer.vecs.emptyoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_exact_utxo_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut input_count_iter = self
                    .indexes_to_input_count
                    .height
                    .unwrap_cumulative()
                    .into_iter();
                let mut opreturn_count_iter = self
                    .indexes_to_opreturn_count
                    .height_extra
                    .unwrap_cumulative()
                    .into_iter();
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_output_count.height.unwrap_cumulative(),
                    |(h, output_count, ..)| {
                        let input_count = input_count_iter.unwrap_get_inner(h);
                        let opreturn_count = opreturn_count_iter.unwrap_get_inner(h);
                        let block_count = u64::from(h + 1_usize);
                        // -1 > genesis output is unspendable
                        let mut utxo_count =
                            *output_count - (*input_count - block_count) - *opreturn_count - 1;

                        // txid dup: e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468
                        // Block 91_722 https://mempool.space/block/00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e
                        // Block 91_880 https://mempool.space/block/00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721
                        //
                        // txid dup: d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599
                        // Block 91_812 https://mempool.space/block/00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f
                        // Block 91_842 https://mempool.space/block/00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec
                        //
                        // Warning: Dups invalidate the previous coinbase according to
                        // https://chainquery.com/bitcoin-cli/gettxoutsetinfo

                        if h >= Height::new(91_842) {
                            utxo_count -= 1;
                        }
                        if h >= Height::new(91_880) {
                            utxo_count -= 1;
                        }

                        (h, StoredU64::from(utxo_count))
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![
                &self.inputindex_to_value as &dyn AnyCollectableVec,
                &self.txindex_to_fee,
                &self.txindex_to_feerate,
                &self.txindex_to_input_value,
                &self.txindex_to_is_coinbase,
                &self.txindex_to_output_value,
                &self.txindex_to_vsize,
                &self.txindex_to_weight,
            ],
            self.indexes_to_coinbase.vecs(),
            self.indexes_to_emptyoutput_count.vecs(),
            self.indexes_to_fee.vecs(),
            self.indexes_to_feerate.vecs(),
            self.indexes_to_input_count.vecs(),
            self.indexes_to_opreturn_count.vecs(),
            self.indexes_to_output_count.vecs(),
            self.indexes_to_p2a_count.vecs(),
            self.indexes_to_p2ms_count.vecs(),
            self.indexes_to_p2pk33_count.vecs(),
            self.indexes_to_p2pk65_count.vecs(),
            self.indexes_to_p2pkh_count.vecs(),
            self.indexes_to_p2sh_count.vecs(),
            self.indexes_to_p2tr_count.vecs(),
            self.indexes_to_p2wpkh_count.vecs(),
            self.indexes_to_p2wsh_count.vecs(),
            self.indexes_to_subsidy.vecs(),
            self.indexes_to_tx_count.vecs(),
            self.indexes_to_tx_v1.vecs(),
            self.indexes_to_tx_v2.vecs(),
            self.indexes_to_tx_v3.vecs(),
            self.indexes_to_tx_vsize.vecs(),
            self.indexes_to_tx_weight.vecs(),
            self.indexes_to_unknownoutput_count.vecs(),
            self.indexes_to_exact_utxo_count.vecs(),
            self.indexes_to_unclaimed_rewards.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
