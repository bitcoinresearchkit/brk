use std::{fs, path::Path};

use brk_core::{
    CheckedSub, Feerate, InputIndex, OutputIndex, Sats, StoredU32, StoredU64, StoredUsize, TxIndex,
    TxVersion, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{Compressed, DynamicVec, StoredIndex, Version};

use super::{
    EagerVec, Indexes,
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
    pub indexes_to_emptyoutput_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_fee: ComputedValueVecsFromTxindex,
    pub indexes_to_feerate: ComputedVecsFromTxindex<Feerate>,
    /// Value == 0 when Coinbase
    pub indexes_to_input_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_output_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_p2a_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2ms_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2pk33_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2pk65_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2pkh_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2sh_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2tr_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2wpkh_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_p2wsh_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_tx_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_tx_vsize: ComputedVecsFromTxindex<StoredUsize>,
    pub indexes_to_tx_weight: ComputedVecsFromTxindex<Weight>,
    pub indexes_to_unknownoutput_count: ComputedVecsFromHeight<StoredU32>,
    pub inputindex_to_value: EagerVec<InputIndex, Sats>,
    pub txindex_to_input_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_is_coinbase: EagerVec<TxIndex, bool>,
    pub txindex_to_output_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_vsize: EagerVec<TxIndex, StoredUsize>,
    pub txindex_to_weight: EagerVec<TxIndex, Weight>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
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
            txindex_to_input_count: ComputedVecsFromTxindex::forced_import(
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
            txindex_to_output_count: ComputedVecsFromTxindex::forced_import(
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
            inputindex_to_value: EagerVec::forced_import(
                &path.join("inputindex_to_value"),
                Version::ZERO,
                compressed,
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
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        marketprices: &mut Option<&mut marketprice::Vecs>,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_tx_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_txindex.mut_vec(),
                    indexes.height_to_last_txindex.mut_vec(),
                    exit,
                )
            },
        )?;

        self.txindex_to_input_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.txindex,
                    indexer.mut_vecs().txindex_to_first_inputindex.mut_vec(),
                    indexes.txindex_to_last_inputindex.mut_vec(),
                    exit,
                )
            },
        )?;

        self.txindex_to_output_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.txindex,
                    indexer.mut_vecs().txindex_to_first_outputindex.mut_vec(),
                    indexes.txindex_to_last_outputindex.mut_vec(),
                    exit,
                )
            },
        )?;

        let mut compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedVecsFromHeight<StoredU32>, txversion| {
                indexes_to_tx_vany.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, indexer, indexes, starting_indexes, exit| {
                        let indexer_vecs = indexer.mut_vecs();
                        vec.compute_filtered_count_from_indexes(
                            starting_indexes.height,
                            indexer_vecs.height_to_first_txindex.mut_vec(),
                            indexes.height_to_last_txindex.mut_vec(),
                            |txindex| {
                                let v = indexer_vecs
                                    .txindex_to_txversion
                                    .double_unwrap_cached_get(txindex);
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

        let indexer_vecs = indexer.mut_vecs();

        self.txindex_to_is_coinbase.compute_is_first_ordered(
            starting_indexes.txindex,
            indexes.txindex_to_height.mut_vec(),
            indexer_vecs.height_to_first_txindex.mut_vec(),
            exit,
        )?;

        self.txindex_to_weight.compute_transform(
            starting_indexes.txindex,
            indexer_vecs.txindex_to_base_size.mut_vec(),
            |(txindex, base_size, ..)| {
                let total_size = indexer_vecs
                    .txindex_to_total_size
                    .mut_vec()
                    .double_unwrap_cached_get(txindex);

                // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                let wu = usize::from(base_size) * 3 + usize::from(total_size);
                let weight = Weight::from(bitcoin::Weight::from_wu_usize(wu));

                (txindex, weight)
            },
            exit,
        )?;

        self.txindex_to_vsize.compute_transform(
            starting_indexes.txindex,
            self.txindex_to_weight.mut_vec(),
            |(txindex, weight, ..)| {
                let vbytes =
                    StoredUsize::from(bitcoin::Weight::from(weight).to_vbytes_ceil() as usize);
                (txindex, vbytes)
            },
            exit,
        )?;

        let inputs_len = indexer_vecs.inputindex_to_outputindex.vec().len();
        self.inputindex_to_value.compute_transform(
            starting_indexes.inputindex,
            indexer_vecs.inputindex_to_outputindex.mut_vec(),
            |(inputindex, outputindex, slf, ..)| {
                let value = if outputindex == OutputIndex::COINBASE {
                    Sats::ZERO
                } else if let Some(value) = indexer_vecs
                    .outputindex_to_value
                    .mut_vec()
                    .unwrap_cached_get(outputindex)
                {
                    value
                } else {
                    dbg!(inputindex, outputindex, slf.len(), inputs_len);
                    panic!()
                };
                (inputindex, value)
            },
            exit,
        )?;

        self.indexes_to_output_value.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, indexer, indexes, starting_indexes, exit| {
                let indexer_vecs = indexer.mut_vecs();
                vec.compute_sum_from_indexes(
                    starting_indexes.txindex,
                    indexer_vecs.txindex_to_first_outputindex.mut_vec(),
                    indexes.txindex_to_last_outputindex.mut_vec(),
                    indexer_vecs.outputindex_to_value.mut_vec(),
                    exit,
                )
            },
        )?;

        self.indexes_to_input_value.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, indexer, indexes, starting_indexes, exit| {
                let indexer_vecs = indexer.mut_vecs();
                vec.compute_sum_from_indexes(
                    starting_indexes.txindex,
                    indexer_vecs.txindex_to_first_inputindex.mut_vec(),
                    indexes.txindex_to_last_inputindex.mut_vec(),
                    self.inputindex_to_value.mut_vec(),
                    exit,
                )
            },
        )?;

        self.indexes_to_fee.compute_all(
            indexer,
            indexes,
            marketprices,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let txindex_to_output_value = self
                    .indexes_to_output_value
                    .txindex
                    .as_mut()
                    .unwrap()
                    .mut_vec();
                vec.compute_transform(
                    starting_indexes.txindex,
                    self.indexes_to_input_value
                        .txindex
                        .as_mut()
                        .unwrap()
                        .mut_vec(),
                    |(txindex, input_value, ..)| {
                        if input_value.is_zero() {
                            (txindex, input_value)
                        } else {
                            let output_value =
                                txindex_to_output_value.double_unwrap_cached_get(txindex);
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
                vec.compute_transform(
                    starting_indexes.txindex,
                    self.indexes_to_fee.sats.txindex.as_mut().unwrap().mut_vec(),
                    |(txindex, fee, ..)| {
                        let vsize = self
                            .txindex_to_vsize
                            .mut_vec()
                            .double_unwrap_cached_get(txindex);

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
            Some(self.txindex_to_weight.mut_vec()),
        )?;

        self.indexes_to_tx_vsize.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(self.txindex_to_vsize.mut_vec()),
        )?;

        self.indexes_to_subsidy.compute_all(
            indexer,
            indexes,
            marketprices,
            starting_indexes,
            exit,
            |vec, indexer, indexes, starting_indexes, exit| {
                let indexer_vecs = indexer.mut_vecs();
                vec.compute_transform(
                    starting_indexes.height,
                    indexer_vecs.height_to_first_txindex.mut_vec(),
                    |(height, txindex, ..)| {
                        let first_outputindex = indexer_vecs
                            .txindex_to_first_outputindex
                            .double_unwrap_cached_get(txindex)
                            .unwrap_to_usize();
                        let last_outputindex = indexes
                            .txindex_to_last_outputindex
                            .mut_vec()
                            .double_unwrap_cached_get(txindex)
                            .unwrap_to_usize();
                        let mut sats = Sats::ZERO;
                        (first_outputindex..=last_outputindex).for_each(|outputindex| {
                            sats += indexer_vecs
                                .outputindex_to_value
                                .double_unwrap_cached_get(OutputIndex::from(outputindex));
                        });
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
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_subsidy
                        .sats
                        .height
                        .as_mut()
                        .unwrap()
                        .mut_vec(),
                    |(height, subsidy, ..)| {
                        let fees = self
                            .indexes_to_fee
                            .sats
                            .height
                            .unwrap_sum()
                            .mut_vec()
                            .double_unwrap_cached_get(height);
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
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2aindex.mut_vec(),
                    indexes.height_to_last_p2aindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2ms_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2msindex.mut_vec(),
                    indexes.height_to_last_p2msindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2pk33_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2pk33index.mut_vec(),
                    indexes.height_to_last_p2pk33index.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2pk65_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2pk65index.mut_vec(),
                    indexes.height_to_last_p2pk65index.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2pkh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2pkhindex.mut_vec(),
                    indexes.height_to_last_p2pkhindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2sh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2shindex.mut_vec(),
                    indexes.height_to_last_p2shindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2tr_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2trindex.mut_vec(),
                    indexes.height_to_last_p2trindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2wpkh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2wpkhindex.mut_vec(),
                    indexes.height_to_last_p2wpkhindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_p2wsh_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_p2wshindex.mut_vec(),
                    indexes.height_to_last_p2wshindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_opreturn_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_first_opreturnindex.mut_vec(),
                    indexes.height_to_last_opreturnindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_unknownoutput_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer
                        .mut_vecs()
                        .height_to_first_unknownoutputindex
                        .mut_vec(),
                    indexes.height_to_last_unknownoutputindex.mut_vec(),
                    exit,
                )
            },
        )?;
        self.indexes_to_emptyoutput_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, indexes, starting_indexes, exit| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    indexer
                        .mut_vecs()
                        .height_to_first_emptyoutputindex
                        .mut_vec(),
                    indexes.height_to_last_emptyoutputindex.mut_vec(),
                    exit,
                )
            },
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        [
            vec![
                self.txindex_to_is_coinbase.any_vec(),
                self.inputindex_to_value.any_vec(),
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
            self.txindex_to_input_count.any_vecs(),
            self.txindex_to_output_count.any_vecs(),
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
