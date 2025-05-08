use std::{fs, path::Path};

use brk_core::{
    CheckedSub, Feerate, Height, InputIndex, OutputIndex, Sats, StoredU32, StoredUsize, TxIndex,
    TxVersion, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, CloneableAnyIterableVec, Compressed, Computation,
    ComputedVec, ComputedVecFrom1, ComputedVecFrom2, ComputedVecFrom3, StoredIndex, Version,
};

use super::{
    Indexes,
    grouped::{
        ComputedValueVecsFromHeight, ComputedValueVecsFromTxindex, ComputedVecsFromHeight,
        ComputedVecsFromTxindex, StorableVecGeneatorOptions,
    },
    indexes, marketprice,
};

#[derive(Clone)]
pub struct Vecs {
    // pub txindex_to_is_v1: LazyVec<Txindex, bool>,
    // pub txindex_to_is_v2: LazyVec<Txindex, bool>,
    // pub txindex_to_is_v3: LazyVec<Txindex, bool>,
    pub indexes_to_coinbase: ComputedValueVecsFromHeight,
    pub indexes_to_emptyoutput_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_fee: ComputedValueVecsFromTxindex,
    pub indexes_to_feerate: ComputedVecsFromTxindex<Feerate>,
    /// Value == 0 when Coinbase
    pub txindex_to_input_value: ComputedVecFrom3<
        TxIndex,
        Sats,
        TxIndex,
        InputIndex,
        TxIndex,
        StoredUsize,
        InputIndex,
        Sats,
    >,
    // pub indexes_to_input_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredUsize>,
    pub txindex_to_output_value: ComputedVecFrom3<
        TxIndex,
        Sats,
        TxIndex,
        OutputIndex,
        TxIndex,
        StoredUsize,
        OutputIndex,
        Sats,
    >,
    // pub indexes_to_output_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_p2a_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2ms_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2pk33_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2pk65_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2pkh_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2sh_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2tr_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2wpkh_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_p2wsh_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_tx_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_tx_vsize: ComputedVecsFromTxindex<StoredUsize>,
    pub indexes_to_tx_weight: ComputedVecsFromTxindex<Weight>,
    pub indexes_to_unknownoutput_count: ComputedVecsFromHeight<StoredUsize>,
    pub inputindex_to_value:
        ComputedVecFrom2<InputIndex, Sats, InputIndex, OutputIndex, OutputIndex, Sats>,
    pub txindex_to_input_count:
        ComputedVecFrom2<TxIndex, StoredUsize, TxIndex, InputIndex, InputIndex, OutputIndex>,
    pub indexes_to_input_count: ComputedVecsFromTxindex<StoredUsize>,
    pub txindex_to_is_coinbase: ComputedVecFrom2<TxIndex, bool, TxIndex, Height, Height, TxIndex>,
    pub txindex_to_output_count:
        ComputedVecFrom2<TxIndex, StoredUsize, TxIndex, OutputIndex, OutputIndex, Sats>,
    pub indexes_to_output_count: ComputedVecsFromTxindex<StoredUsize>,
    pub txindex_to_vsize: ComputedVecFrom1<TxIndex, StoredUsize, TxIndex, Weight>,
    pub txindex_to_weight:
        ComputedVecFrom2<TxIndex, Weight, TxIndex, StoredU32, TxIndex, StoredU32>,
    pub txindex_to_fee: ComputedVecFrom2<TxIndex, Sats, TxIndex, Sats, TxIndex, Sats>,
    pub txindex_to_feerate: ComputedVecFrom2<TxIndex, Feerate, TxIndex, Sats, TxIndex, StoredUsize>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        computation: Computation,
        compressed: Compressed,
        marketprices: Option<&marketprice::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = marketprices.is_some();

        fs::create_dir_all(path)?;

        let inputindex_to_value = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "inputindex_to_value",
            Version::ZERO,
            compressed,
            indexer.vecs().inputindex_to_outputindex.boxed_clone(),
            indexer.vecs().outputindex_to_value.boxed_clone(),
            |index: InputIndex, inputindex_to_outputindex_iter, outputindex_to_value_iter| {
                inputindex_to_outputindex_iter
                    .next_at(index.unwrap_to_usize())
                    .map(|(inputindex, outputindex)| {
                        let outputindex = outputindex.into_inner();
                        if outputindex == OutputIndex::COINBASE {
                            Sats::ZERO
                        } else if let Some((_, value)) =
                            outputindex_to_value_iter.next_at(outputindex.unwrap_to_usize())
                        {
                            value.into_inner()
                        } else {
                            dbg!(inputindex, outputindex);
                            panic!()
                        }
                    })
            },
        )?;

        let txindex_to_weight = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "txindex_to_weight",
            Version::ZERO,
            compressed,
            indexer.vecs().txindex_to_base_size.boxed_clone(),
            indexer.vecs().txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.unwrap_to_usize();
                txindex_to_base_size_iter
                    .next_at(index)
                    .map(|(_, base_size)| {
                        let base_size = base_size.into_inner();
                        let total_size = txindex_to_total_size_iter
                            .next_at(index)
                            .unwrap()
                            .1
                            .into_inner();

                        // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                        let wu = usize::from(base_size) * 3 + usize::from(total_size);

                        Weight::from(bitcoin::Weight::from_wu_usize(wu))
                    })
            },
        )?;

        let txindex_to_vsize = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "txindex_to_vsize",
            Version::ZERO,
            compressed,
            txindex_to_weight.boxed_clone(),
            |index: TxIndex, iter| {
                let index = index.unwrap_to_usize();
                iter.next_at(index).map(|(_, weight)| {
                    StoredUsize::from(
                        bitcoin::Weight::from(weight.into_inner()).to_vbytes_ceil() as usize
                    )
                })
            },
        )?;

        let txindex_to_is_coinbase = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "txindex_to_is_coinbase",
            Version::ZERO,
            compressed,
            indexes.txindex_to_height.boxed_clone(),
            indexer.vecs().height_to_first_txindex.boxed_clone(),
            |index: TxIndex, txindex_to_height_iter, height_to_first_txindex_iter| {
                txindex_to_height_iter
                    .next_at(index.unwrap_to_usize())
                    .map(|(_, height)| {
                        let height = height.into_inner();
                        let txindex = height_to_first_txindex_iter
                            .next_at(height.unwrap_to_usize())
                            .unwrap()
                            .1
                            .into_inner();

                        index == txindex
                    })
            },
        )?;

        let txindex_to_input_count = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "txindex_to_input_count",
            Version::ZERO,
            compressed,
            indexer.vecs().txindex_to_first_inputindex.boxed_clone(),
            indexer.vecs().inputindex_to_outputindex.boxed_clone(),
            |index: TxIndex, txindex_to_first_inputindex_iter, inputindex_to_outputindex_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_inputindex_iter
                    .next_at(txindex)
                    .map(|(_, start)| {
                        let start = usize::from(start.into_inner());
                        let end = txindex_to_first_inputindex_iter
                            .next_at(txindex + 1)
                            .map(|(_, v)| usize::from(v.into_inner()))
                            .unwrap_or_else(|| inputindex_to_outputindex_iter.len());
                        StoredUsize::from((start..end).count())
                    })
            },
        )?;

        let txindex_to_input_value = ComputedVec::forced_import_or_init_from_3(
            computation,
            path,
            "txindex_to_input_value",
            Version::ZERO,
            compressed,
            indexer.vecs().txindex_to_first_inputindex.boxed_clone(),
            txindex_to_input_count.boxed_clone(),
            inputindex_to_value.boxed_clone(),
            |index: TxIndex,
             txindex_to_first_inputindex_iter,
             txindex_to_input_count_iter,
             inputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_inputindex_iter
                    .next_at(txindex)
                    .map(|(_, first_index)| {
                        let first_index = usize::from(first_index.into_inner());
                        let count = *txindex_to_input_count_iter
                            .next_at(txindex)
                            .unwrap()
                            .1
                            .into_inner();
                        let range = first_index..first_index + count;
                        range.into_iter().fold(Sats::ZERO, |total, inputindex| {
                            total
                                + inputindex_to_value_iter
                                    .next_at(inputindex)
                                    .unwrap()
                                    .1
                                    .into_inner()
                        })
                    })
            },
        )?;

        // let indexes_to_input_value: ComputedVecsFromTxindex<Sats> =
        //     ComputedVecsFromTxindex::forced_import(
        //         path,
        //         "input_value",
        //         true,
        //         Version::ZERO,
        //         compressed,
        //         StorableVecGeneatorOptions::default()
        //             .add_average()
        //             .add_sum()
        //             .add_total(),
        //     )?;

        let txindex_to_output_count = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "txindex_to_output_count",
            Version::ZERO,
            compressed,
            indexer.vecs().txindex_to_first_outputindex.boxed_clone(),
            indexer.vecs().outputindex_to_value.boxed_clone(),
            |index: TxIndex, txindex_to_first_outputindex_iter, outputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_outputindex_iter
                    .next_at(txindex)
                    .map(|(_, start)| {
                        let start = usize::from(start.into_inner());
                        let end = txindex_to_first_outputindex_iter
                            .next_at(txindex + 1)
                            .map(|(_, v)| usize::from(v.into_inner()))
                            .unwrap_or_else(|| outputindex_to_value_iter.len());
                        StoredUsize::from((start..end).count())
                    })
            },
        )?;

        let txindex_to_output_value = ComputedVec::forced_import_or_init_from_3(
            computation,
            path,
            "txindex_to_output_value",
            Version::ZERO,
            compressed,
            indexer.vecs().txindex_to_first_outputindex.boxed_clone(),
            txindex_to_output_count.boxed_clone(),
            indexer.vecs().outputindex_to_value.boxed_clone(),
            |index: TxIndex,
             txindex_to_first_outputindex_iter,
             txindex_to_output_count_iter,
             outputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_outputindex_iter
                    .next_at(txindex)
                    .map(|(_, first_index)| {
                        let first_index = usize::from(first_index.into_inner());
                        let count = *txindex_to_output_count_iter
                            .next_at(txindex)
                            .unwrap()
                            .1
                            .into_inner();
                        let range = first_index..first_index + count;
                        range.into_iter().fold(Sats::ZERO, |total, outputindex| {
                            total
                                + outputindex_to_value_iter
                                    .next_at(outputindex)
                                    .unwrap()
                                    .1
                                    .into_inner()
                        })
                    })
            },
        )?;

        // let indexes_to_output_value: ComputedVecsFromTxindex<Sats> =
        //     ComputedVecsFromTxindex::forced_import(
        //         path,
        //         "output_value",
        //         true,
        //         Version::ZERO,
        //         compressed,
        //         StorableVecGeneatorOptions::default()
        //             .add_average()
        //             .add_sum()
        //             .add_total(),
        //     )?;

        let txindex_to_fee = ComputedVecFrom2::forced_import_or_init_from_2(
            computation,
            path,
            "txindex_to_fee",
            Version::ZERO,
            compressed,
            txindex_to_input_value.boxed_clone(),
            txindex_to_output_value.boxed_clone(),
            |txindex: TxIndex, input_iter, output_iter| {
                let txindex = txindex.unwrap_to_usize();
                input_iter.next_at(txindex).and_then(|(_, value)| {
                    let input = value.into_inner();
                    if input.is_zero() {
                        return Some(Sats::ZERO);
                    }
                    output_iter.next_at(txindex).map(|(_, value)| {
                        let output = value.into_inner();
                        input.checked_sub(output).unwrap()
                    })
                })
            },
        )?;

        let txindex_to_feerate = ComputedVecFrom2::forced_import_or_init_from_2(
            computation,
            path,
            "txindex_to_feerate",
            Version::ZERO,
            compressed,
            txindex_to_fee.boxed_clone(),
            txindex_to_vsize.boxed_clone(),
            |txindex: TxIndex, fee_iter, vsize_iter| {
                let txindex = txindex.unwrap_to_usize();
                fee_iter.next_at(txindex).and_then(|(_, value)| {
                    let fee = value.into_inner();
                    vsize_iter.next_at(txindex).map(|(_, value)| {
                        let vsize = value.into_inner();
                        Feerate::from((fee, vsize))
                    })
                })
            },
        )?;

        Ok(Self {
            indexes_to_tx_count: ComputedVecsFromHeight::forced_import(
                path,
                "tx_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_input_count: ComputedVecsFromTxindex::forced_import(
                path,
                "input_count",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_output_count: ComputedVecsFromTxindex::forced_import(
                path,
                "output_count",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_tx_v1: ComputedVecsFromHeight::forced_import(
                path,
                "tx_v1",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_tx_v2: ComputedVecsFromHeight::forced_import(
                path,
                "tx_v2",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_tx_v3: ComputedVecsFromHeight::forced_import(
                path,
                "tx_v3",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                path,
                "fee",
                indexes,
                Some(txindex_to_fee.boxed_clone()),
                Version::ZERO,
                computation,
                compressed,
                marketprices,
                StorableVecGeneatorOptions::default()
                    .add_sum()
                    .add_total()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_feerate: ComputedVecsFromTxindex::forced_import(
                path,
                "feerate",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_tx_vsize: ComputedVecsFromTxindex::forced_import(
                path,
                "tx_vsize",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_tx_weight: ComputedVecsFromTxindex::forced_import(
                path,
                "tx_weight",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                path,
                "subsidy",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_sum()
                    .add_total()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                path,
                "coinbase",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_sum()
                    .add_total()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
            )?,
            indexes_to_p2a_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2a_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2ms_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2ms_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2pk33_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2pk33_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2pk65_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2pk65_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2pkh_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2pkh_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2sh_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2sh_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2tr_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2tr_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2wpkh_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2wpkh_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_p2wsh_count: ComputedVecsFromHeight::forced_import(
                path,
                "p2wsh_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_opreturn_count: ComputedVecsFromHeight::forced_import(
                path,
                "opreturn_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_unknownoutput_count: ComputedVecsFromHeight::forced_import(
                path,
                "unknownoutput_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_emptyoutput_count: ComputedVecsFromHeight::forced_import(
                path,
                "emptyoutput_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_minmax()
                    .add_percentiles()
                    .add_sum()
                    .add_total(),
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
            txindex_to_input_count,
            txindex_to_output_count,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        marketprices: Option<&marketprice::Vecs>,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_tx_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs().height_to_first_txindex,
                    &indexer.vecs().txindex_to_txid,
                    exit,
                )
            },
        )?;

        self.indexes_to_input_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_input_count),
        )?;

        self.indexes_to_output_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_output_count),
        )?;

        let compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedVecsFromHeight<StoredUsize>, txversion| {
                let mut txindex_to_txversion_iter = indexer.vecs().txindex_to_txversion.iter();
                indexes_to_tx_vany.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, indexer, _, starting_indexes, exit| {
                        vec.compute_filtered_count_from_indexes(
                            starting_indexes.height,
                            &indexer.vecs().height_to_first_txindex,
                            &indexer.vecs().txindex_to_txid,
                            |txindex| {
                                let v = txindex_to_txversion_iter.unwrap_get_inner(txindex);
                                v == txversion
                            },
                            exit,
                        )
                    },
                )
            };
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v1, TxVersion::ONE)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v2, TxVersion::TWO)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v3, TxVersion::THREE)?;

        self.txindex_to_is_coinbase
            .compute_if_necessary(starting_indexes.txindex, exit)?;

        self.txindex_to_weight
            .compute_if_necessary(starting_indexes.txindex, exit)?;

        self.txindex_to_vsize
            .compute_if_necessary(starting_indexes.txindex, exit)?;

        self.inputindex_to_value
            .compute_if_necessary(starting_indexes.inputindex, exit)?;

        self.txindex_to_output_value
            .compute_if_necessary(starting_indexes.txindex, exit)?;

        // self.indexes_to_output_value.compute_all(
        //     indexer,
        //     indexes,
        //     starting_indexes,
        //     exit,
        //     |vec, indexer, _, starting_indexes, exit| {
        //         vec.compute_sum_from_indexes(
        //             starting_indexes.txindex,
        //             &indexer.vecs().txindex_to_first_outputindex,
        //             self.indexes_to_output_count.txindex.as_ref().unwrap(),
        //             &indexer.vecs().outputindex_to_value,
        //             exit,
        //         )
        //     },
        // )?;

        self.txindex_to_input_value
            .compute_if_necessary(starting_indexes.txindex, exit)?;

        // self.indexes_to_input_value.compute_all(
        //     indexer,
        //     indexes,
        //     starting_indexes,
        //     exit,
        //     |vec, indexer, _, starting_indexes, exit| {
        //         vec.compute_sum_from_indexes(
        //             starting_indexes.txindex,
        //             &indexer.vecs().txindex_to_first_inputindex,
        //             self.indexes_to_input_count.txindex.as_ref().unwrap(),
        //             &self.inputindex_to_value,
        //             exit,
        //         )
        //     },
        // )?;

        self.indexes_to_fee.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee),
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
            marketprices,
            starting_indexes,
            exit,
            |vec, indexer, _, starting_indexes, exit| {
                let mut txindex_to_first_outputindex_iter =
                    indexer.vecs().txindex_to_first_outputindex.iter();
                let mut txindex_to_output_count_iter = self.txindex_to_output_count.iter();
                let mut outputindex_to_value_iter = indexer.vecs().outputindex_to_value.iter();
                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs().height_to_first_txindex,
                    |(height, txindex, ..)| {
                        let first_outputindex = txindex_to_first_outputindex_iter
                            .unwrap_get_inner(txindex)
                            .unwrap_to_usize();
                        let output_count = txindex_to_output_count_iter.unwrap_get_inner(txindex);
                        let mut sats = Sats::ZERO;
                        (first_outputindex..first_outputindex + *output_count).for_each(
                            |outputindex| {
                                sats += outputindex_to_value_iter
                                    .unwrap_get_inner(OutputIndex::from(outputindex));
                            },
                        );
                        (height, sats)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_subsidy.compute_all(
            indexer,
            indexes,
            marketprices,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut indexes_to_fee_sum_iter =
                    self.indexes_to_fee.sats.height.unwrap_sum().iter();
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_coinbase.sats.height.as_ref().unwrap(),
                    |(height, subsidy, ..)| {
                        let fees = indexes_to_fee_sum_iter.unwrap_get_inner(height);
                        (height, subsidy.checked_sub(fees).unwrap())
                    },
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2aindex,
                    &indexer.vecs().p2aindex_to_p2abytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2msindex,
                    &indexer.vecs().p2msindex_to_txindex,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2pk33index,
                    &indexer.vecs().p2pk33index_to_p2pk33bytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2pk65index,
                    &indexer.vecs().p2pk65index_to_p2pk65bytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2pkhindex,
                    &indexer.vecs().p2pkhindex_to_p2pkhbytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2shindex,
                    &indexer.vecs().p2shindex_to_p2shbytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2trindex,
                    &indexer.vecs().p2trindex_to_p2trbytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2wpkhindex,
                    &indexer.vecs().p2wpkhindex_to_p2wpkhbytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_p2wshindex,
                    &indexer.vecs().p2wshindex_to_p2wshbytes,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_opreturnindex,
                    &indexer.vecs().opreturnindex_to_txindex,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_unknownoutputindex,
                    &indexer.vecs().unknownoutputindex_to_txindex,
                    exit,
                )
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
                    &indexer.vecs().height_to_first_emptyoutputindex,
                    &indexer.vecs().emptyoutputindex_to_txindex,
                    exit,
                )
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
                &self.txindex_to_input_count,
                &self.txindex_to_input_value,
                &self.txindex_to_is_coinbase,
                &self.txindex_to_output_count,
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
        ]
        .concat()
    }
}
