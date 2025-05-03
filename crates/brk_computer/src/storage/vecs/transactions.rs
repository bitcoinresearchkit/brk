use std::{fs, path::Path};

use brk_core::{
    CheckedSub, Feerate, InputIndex, OutputIndex, Sats, StoredUsize, TxIndex, TxVersion, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{Compressed, StoredIndex, VecIterator, Version};

use super::{
    Computation, ComputedVec, ComputedVecFrom2, EagerVec, Indexes,
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
    pub indexes_to_input_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_output_value: ComputedVecsFromTxindex<Sats>,
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
        ComputedVecFrom2<InputIndex, Sats, OutputIndex, Sats, InputIndex, OutputIndex>,
    pub indexes_to_input_count: ComputedVecsFromTxindex<StoredUsize>,
    pub txindex_to_is_coinbase: EagerVec<TxIndex, bool>,
    pub indexes_to_output_count: ComputedVecsFromTxindex<StoredUsize>,
    pub txindex_to_vsize: EagerVec<TxIndex, StoredUsize>,
    pub txindex_to_weight: EagerVec<TxIndex, Weight>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        indexer: &Indexer,
        computation: Computation,
        compressed: Compressed,
        compute_dollars: bool,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

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
            // height_to_subsidy: StorableVec::forced_import(&path.join("height_to_subsidy"), Version::ZERO)?,
            txindex_to_is_coinbase: EagerVec::forced_import(
                &path.join("txindex_to_is_coinbase"),
                Version::ZERO,
                compressed,
            )?,
            indexes_to_input_count: ComputedVecsFromTxindex::forced_import(
                path,
                "input_count",
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
            indexes_to_output_count: ComputedVecsFromTxindex::forced_import(
                path,
                "output_count",
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
            inputindex_to_value: ComputedVec::forced_import_or_init_from_2(
                computation,
                &path.join("inputindex_to_value"),
                Version::ZERO,
                compressed,
                indexer.vecs().outputindex_to_value.vec().clone(),
                indexer.vecs().inputindex_to_outputindex.vec().clone(),
                |index, outputindex_to_value_iter, inputindex_to_outputindex_iter| {
                    inputindex_to_outputindex_iter.next_at(index).map(
                        |(inputindex, outputindex)| {
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
                        },
                    )
                },
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
            indexes_to_input_value: ComputedVecsFromTxindex::forced_import(
                path,
                "input_value",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_output_value: ComputedVecsFromTxindex::forced_import(
                path,
                "output_value",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_average()
                    .add_sum()
                    .add_total(),
            )?,
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                path,
                "fee",
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
            indexes_to_feerate: ComputedVecsFromTxindex::forced_import(
                path,
                "feerate",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            txindex_to_weight: EagerVec::forced_import(
                &path.join("txindex_to_weight"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_vsize: EagerVec::forced_import(
                &path.join("txindex_to_vsize"),
                Version::ZERO,
                compressed,
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
                    indexer.vecs().height_to_first_txindex.vec(),
                    indexer.vecs().txindex_to_txid.vec(),
                    exit,
                )
            },
        )?;

        self.indexes_to_input_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.txindex,
                    indexer.vecs().txindex_to_first_inputindex.vec(),
                    indexer.vecs().inputindex_to_outputindex.vec(),
                    exit,
                )
            },
        )?;

        self.indexes_to_output_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.txindex,
                    indexer.vecs().txindex_to_first_outputindex.vec(),
                    indexer.vecs().outputindex_to_value.vec(),
                    exit,
                )
            },
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
                            indexer.vecs().height_to_first_txindex.vec(),
                            indexer.vecs().txindex_to_txid.vec(),
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

        self.txindex_to_is_coinbase.compute_is_first_ordered(
            starting_indexes.txindex,
            indexes.txindex_to_height.vec(),
            indexer.vecs().height_to_first_txindex.vec(),
            exit,
        )?;

        let mut txindex_to_total_size_iter = indexer.vecs().txindex_to_total_size.iter();
        self.txindex_to_weight.compute_transform(
            starting_indexes.txindex,
            indexer.vecs().txindex_to_base_size.vec(),
            |(txindex, base_size, ..)| {
                let total_size = txindex_to_total_size_iter.unwrap_get_inner(txindex);

                // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                let wu = usize::from(base_size) * 3 + usize::from(total_size);
                let weight = Weight::from(bitcoin::Weight::from_wu_usize(wu));

                (txindex, weight)
            },
            exit,
        )?;

        self.txindex_to_vsize.compute_transform(
            starting_indexes.txindex,
            self.txindex_to_weight.vec(),
            |(txindex, weight, ..)| {
                let vbytes =
                    StoredUsize::from(bitcoin::Weight::from(weight).to_vbytes_ceil() as usize);
                (txindex, vbytes)
            },
            exit,
        )?;

        // let mut outputindex_to_value_iter = indexer.vecs().outputindex_to_value.iter();
        self.inputindex_to_value.compute_if_necessary(
            starting_indexes.inputindex,
            // indexer.vecs().inputindex_to_outputindex.vec(),
            // |(inputindex, outputindex, ..)| {
            //     let value = if outputindex == OutputIndex::COINBASE {
            //         Sats::ZERO
            //     } else if let Some(value) = outputindex_to_value_iter.get(outputindex) {
            //         value.into_inner()
            //     } else {
            //         dbg!(inputindex, outputindex);
            //         panic!()
            //     };
            //     (inputindex, value)
            // },
            exit,
        )?;

        self.indexes_to_output_value.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, indexer, _, starting_indexes, exit| {
                vec.compute_sum_from_indexes(
                    starting_indexes.txindex,
                    indexer.vecs().txindex_to_first_outputindex.vec(),
                    self.indexes_to_output_count.txindex.as_ref().unwrap().vec(),
                    indexer.vecs().outputindex_to_value.vec(),
                    exit,
                )
            },
        )?;

        // self.indexes_to_input_value.compute_all(
        //     indexer,
        //     indexes,
        //     starting_indexes,
        //     exit,
        //     |vec, indexer, _, starting_indexes, exit| {
        //         vec.compute_sum_from_indexes(
        //             starting_indexes.txindex,
        //             indexer.vecs().txindex_to_first_inputindex.vec(),
        //             self.indexes_to_input_count.txindex.as_ref().unwrap().vec(),
        //             self.inputindex_to_value.vec(),
        //             exit,
        //         )
        //     },
        // )?;

        self.indexes_to_fee.compute_all(
            indexer,
            indexes,
            marketprices,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut txindex_to_output_value_iter = self
                    .indexes_to_output_value
                    .txindex
                    .as_ref()
                    .unwrap()
                    .iter();
                vec.compute_transform(
                    starting_indexes.txindex,
                    self.indexes_to_input_value.txindex.as_ref().unwrap().vec(),
                    |(txindex, input_value, ..)| {
                        if input_value.is_zero() {
                            (txindex, input_value)
                        } else {
                            let output_value =
                                txindex_to_output_value_iter.unwrap_get_inner(txindex);
                            (txindex, input_value.checked_sub(output_value).unwrap())
                        }
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_feerate.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut txindex_to_vsize_iter = self.txindex_to_vsize.iter();
                vec.compute_transform(
                    starting_indexes.txindex,
                    self.indexes_to_fee.sats.txindex.as_ref().unwrap().vec(),
                    |(txindex, fee, ..)| {
                        let vsize = txindex_to_vsize_iter.unwrap_get_inner(txindex);
                        (txindex, Feerate::from((fee, vsize)))
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_tx_weight.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(self.txindex_to_weight.vec()),
        )?;

        self.indexes_to_tx_vsize.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(self.txindex_to_vsize.vec()),
        )?;

        self.indexes_to_subsidy.compute_all(
            indexer,
            indexes,
            marketprices,
            starting_indexes,
            exit,
            |vec, indexer, _, starting_indexes, exit| {
                let mut txindex_to_first_outputindex_iter =
                    indexer.vecs().txindex_to_first_outputindex.iter();
                let mut txindex_to_output_count_iter = self
                    .indexes_to_output_count
                    .txindex
                    .as_ref()
                    .unwrap()
                    .iter();
                let mut outputindex_to_value_iter = indexer.vecs().outputindex_to_value.iter();
                vec.compute_transform(
                    starting_indexes.height,
                    indexer.vecs().height_to_first_txindex.vec(),
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

        self.indexes_to_coinbase.compute_all(
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
                    self.indexes_to_subsidy.sats.height.as_ref().unwrap().vec(),
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
                    indexer.vecs().height_to_first_p2aindex.vec(),
                    indexer.vecs().p2aindex_to_p2abytes.vec(),
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
                    indexer.vecs().height_to_first_p2msindex.vec(),
                    indexer.vecs().p2msindex_to_txindex.vec(),
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
                    indexer.vecs().height_to_first_p2pk33index.vec(),
                    indexer.vecs().p2pk33index_to_p2pk33bytes.vec(),
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
                    indexer.vecs().height_to_first_p2pk65index.vec(),
                    indexer.vecs().p2pk65index_to_p2pk65bytes.vec(),
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
                    indexer.vecs().height_to_first_p2pkhindex.vec(),
                    indexer.vecs().p2pkhindex_to_p2pkhbytes.vec(),
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
                    indexer.vecs().height_to_first_p2shindex.vec(),
                    indexer.vecs().p2shindex_to_p2shbytes.vec(),
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
                    indexer.vecs().height_to_first_p2trindex.vec(),
                    indexer.vecs().p2trindex_to_p2trbytes.vec(),
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
                    indexer.vecs().height_to_first_p2wpkhindex.vec(),
                    indexer.vecs().p2wpkhindex_to_p2wpkhbytes.vec(),
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
                    indexer.vecs().height_to_first_p2wshindex.vec(),
                    indexer.vecs().p2wshindex_to_p2wshbytes.vec(),
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
                    indexer.vecs().height_to_first_opreturnindex.vec(),
                    indexer.vecs().opreturnindex_to_txindex.vec(),
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
                    indexer.vecs().height_to_first_unknownoutputindex.vec(),
                    indexer.vecs().unknownoutputindex_to_txindex.vec(),
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
                    indexer.vecs().height_to_first_emptyoutputindex.vec(),
                    indexer.vecs().emptyoutputindex_to_txindex.vec(),
                    exit,
                )
            },
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        [
            vec![
                self.txindex_to_is_coinbase.any_vec(),
                // self.inputindex_to_value.any_vec(),
                self.txindex_to_weight.any_vec(),
                self.txindex_to_vsize.any_vec(),
            ],
            self.indexes_to_tx_count.any_vecs(),
            self.indexes_to_coinbase.any_vecs(),
            self.indexes_to_fee.any_vecs(),
            self.indexes_to_feerate.any_vecs(),
            self.indexes_to_input_value.any_vecs(),
            self.indexes_to_output_value.any_vecs(),
            self.indexes_to_subsidy.any_vecs(),
            self.indexes_to_tx_v1.any_vecs(),
            self.indexes_to_tx_v2.any_vecs(),
            self.indexes_to_tx_v3.any_vecs(),
            self.indexes_to_tx_vsize.any_vecs(),
            self.indexes_to_tx_weight.any_vecs(),
            self.indexes_to_input_count.any_vecs(),
            self.indexes_to_output_count.any_vecs(),
            self.indexes_to_p2a_count.any_vecs(),
            self.indexes_to_p2ms_count.any_vecs(),
            self.indexes_to_p2pk33_count.any_vecs(),
            self.indexes_to_p2pk65_count.any_vecs(),
            self.indexes_to_p2pkh_count.any_vecs(),
            self.indexes_to_p2sh_count.any_vecs(),
            self.indexes_to_p2tr_count.any_vecs(),
            self.indexes_to_p2wpkh_count.any_vecs(),
            self.indexes_to_p2wsh_count.any_vecs(),
            self.indexes_to_opreturn_count.any_vecs(),
            self.indexes_to_unknownoutput_count.any_vecs(),
            self.indexes_to_emptyoutput_count.any_vecs(),
        ]
        .concat()
    }
}
