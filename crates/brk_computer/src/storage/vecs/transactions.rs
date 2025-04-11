use std::{fs, path::Path};

use brk_core::{
    CheckedSub, Feerate, Sats, StoredU32, StoredU64, StoredUsize, TxVersion, Txindex, Txinindex,
    Txoutindex, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{Compressed, DynamicVec, Version};

use super::{
    ComputedVec, Indexes,
    grouped::{ComputedVecsFromHeight, ComputedVecsFromTxindex, StorableVecGeneatorOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    // pub height_to_fee: ComputedVec<Txindex, Sats>,
    // pub height_to_inputcount: ComputedVec<Height, u32>,
    // pub height_to_maxfeerate: ComputedVec<Height, Feerate>,
    // pub height_to_medianfeerate: ComputedVec<Height, Feerate>,
    // pub height_to_minfeerate: ComputedVec<Height, Feerate>,
    // pub height_to_outputcount: ComputedVec<Height, u32>,
    // pub height_to_subsidy: ComputedVec<Height, u32>,
    // pub height_to_totalfees: ComputedVec<Height, Sats>,
    pub height_to_tx_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_fee: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_feerate: ComputedVecsFromTxindex<Feerate>,
    pub indexes_to_input_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_output_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredU32>,
    pub txindex_to_input_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_is_coinbase: ComputedVec<Txindex, bool>,
    pub txindex_to_output_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_vsize: ComputedVec<Txindex, StoredUsize>,
    pub txindex_to_weight: ComputedVec<Txindex, Weight>,
    /// Value == 0 when Coinbase
    pub txinindex_to_value: ComputedVec<Txinindex, Sats>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            height_to_tx_count: ComputedVecsFromHeight::forced_import(
                path,
                "tx_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            // height_to_fee: StorableVec::forced_import(&path.join("height_to_fee"), Version::ZERO)?,
            // height_to_input_count: StorableVec::forced_import(
            //     &path.join("height_to_input_count"),
            //     Version::ZERO,
            // )?,
            // height_to_maxfeerate: StorableVec::forced_import(&path.join("height_to_maxfeerate"), Version::ZERO)?,
            // height_to_medianfeerate: StorableVec::forced_import(&path.join("height_to_medianfeerate"), Version::ZERO)?,
            // height_to_minfeerate: StorableVec::forced_import(&path.join("height_to_minfeerate"), Version::ZERO)?,
            // height_to_output_count: StorableVec::forced_import(
            //     &path.join("height_to_output_count"),
            //     Version::ZERO,
            // )?,
            // height_to_subsidy: StorableVec::forced_import(&path.join("height_to_subsidy"), Version::ZERO)?,
            // height_to_totalfees: StorableVec::forced_import(&path.join("height_to_totalfees"), Version::ZERO)?,
            // height_to_txcount: StorableVec::forced_import(&path.join("height_to_txcount"), Version::ZERO)?,
            txindex_to_is_coinbase: ComputedVec::forced_import(
                &path.join("txindex_to_is_coinbase"),
                Version::ZERO,
                compressed,
            )?,
            // txindex_to_feerate: StorableVec::forced_import(&path.join("txindex_to_feerate"), Version::ZERO)?,
            txindex_to_input_count: ComputedVecsFromTxindex::forced_import(
                path,
                "input_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            txindex_to_output_count: ComputedVecsFromTxindex::forced_import(
                path,
                "output_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            // txindex_to_output_value: ComputedVecsFromTxindex::forced_import(
            //     path,
            //     "output_value",
            //     Version::ZERO,
            //     compressed,
            //     StorableVecGeneatorOptions::default().add_sum().add_total(),
            // )?,
            txinindex_to_value: ComputedVec::forced_import(
                &path.join("txinindex_to_value"),
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
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_output_value: ComputedVecsFromTxindex::forced_import(
                path,
                "output_value",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_fee: ComputedVecsFromTxindex::forced_import(
                path,
                "fee",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
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
            txindex_to_weight: ComputedVec::forced_import(
                &path.join("txindex_to_weight"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_vsize: ComputedVec::forced_import(
                &path.join("txindex_to_vsize"),
                Version::ZERO,
                compressed,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.height_to_tx_count.compute_all(
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
                    indexer.mut_vecs().txindex_to_first_txinindex.mut_vec(),
                    indexes.txindex_to_last_txinindex.mut_vec(),
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
                    indexer.mut_vecs().txindex_to_first_txoutindex.mut_vec(),
                    indexes.txindex_to_last_txoutindex.mut_vec(),
                    exit,
                )
            },
        )?;

        // self.txindex_to_output_value.compute_rest(
        //     indexer,
        //     indexes,
        //     starting_indexes,
        //     exit,
        //     Some(indexer.mut_vecs().txoutindex_to_value.mut_vec()),
        // )?;

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
                                    .cached_get(txindex)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();
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
            indexer_vecs.txindex_to_height.mut_vec(),
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
                    .cached_get(txindex)
                    .unwrap()
                    .unwrap()
                    .into_inner();

                // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                let wu = base_size * 3 + total_size;
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

        self.txinindex_to_value.compute_transform(
            starting_indexes.txinindex,
            indexer_vecs.txinindex_to_txoutindex.mut_vec(),
            |(txinindex, txoutindex, slf, other)| {
                let value = if txoutindex == Txoutindex::COINBASE {
                    Sats::ZERO
                } else if let Ok(Some(value)) = indexer_vecs
                    .txoutindex_to_value
                    .mut_vec()
                    .cached_get(txoutindex)
                {
                    *value
                } else {
                    dbg!(txinindex, txoutindex, slf.len(), other.len());
                    panic!()
                };
                (txinindex, value)
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
                    indexer_vecs.txindex_to_first_txoutindex.mut_vec(),
                    indexes.txindex_to_last_txoutindex.mut_vec(),
                    indexer_vecs.txoutindex_to_value.mut_vec(),
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
                    indexer_vecs.txindex_to_first_txinindex.mut_vec(),
                    indexes.txindex_to_last_txinindex.mut_vec(),
                    self.txinindex_to_value.mut_vec(),
                    exit,
                )
            },
        )?;

        self.indexes_to_fee.compute_all(
            indexer,
            indexes,
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
                            let output_value = txindex_to_output_value
                                .cached_get(txindex)
                                .unwrap()
                                .unwrap()
                                .into_inner();
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
                    self.indexes_to_fee.txindex.as_mut().unwrap().mut_vec(),
                    |(txindex, fee, ..)| {
                        let vsize = self
                            .txindex_to_vsize
                            .mut_vec()
                            .cached_get(txindex)
                            .unwrap()
                            .unwrap()
                            .into_inner();

                        (txindex, Feerate::from((fee, vsize)))
                    },
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
                self.txinindex_to_value.any_vec(),
                self.txindex_to_weight.any_vec(),
                self.txindex_to_vsize.any_vec(),
            ],
            self.height_to_tx_count.any_vecs(),
            self.indexes_to_input_value.any_vecs(),
            self.indexes_to_output_value.any_vecs(),
            self.indexes_to_tx_v1.any_vecs(),
            self.indexes_to_tx_v2.any_vecs(),
            self.indexes_to_tx_v3.any_vecs(),
            self.indexes_to_fee.any_vecs(),
            self.indexes_to_feerate.any_vecs(),
            self.txindex_to_input_count.any_vecs(),
            self.txindex_to_output_count.any_vecs(),
        ]
        .concat()
    }
}
