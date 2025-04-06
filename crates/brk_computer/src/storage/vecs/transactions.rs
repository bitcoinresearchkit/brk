use std::{fs, path::Path};

use brk_core::{CounterU64, Txindex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Version};

use super::{
    ComputedVec, Indexes,
    grouped::{ComputedVecsFromTxindex, StorableVecGeneatorOptions},
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
    // pub height_to_txcount: ComputedVec<Height, u32>,
    // pub txindex_to_fee: ComputedVec<Txindex, Sats>,
    pub txindex_to_is_coinbase: ComputedVec<Txindex, bool>,
    // pub txindex_to_feerate: ComputedVec<Txindex, Feerate>,
    pub txindex_to_inputs_count: ComputedVecsFromTxindex<CounterU64>,
    // pub txindex_to_inputs_sum: ComputedVec<Txindex, Sats>,
    pub txindex_to_outputs_count: ComputedVecsFromTxindex<CounterU64>,
    // pub txindex_to_outputs_sum: ComputedVec<Txindex, Sats>,
    // pub txinindex_to_value: ComputedVec<Txinindex, Sats>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            // height_to_fee: StorableVec::forced_import(&path.join("height_to_fee"), Version::ONE)?,
            // height_to_input_count: StorableVec::forced_import(
            //     &path.join("height_to_input_count"),
            //     Version::ONE,
            // )?,
            // height_to_maxfeerate: StorableVec::forced_import(&path.join("height_to_maxfeerate"), Version::ONE)?,
            // height_to_medianfeerate: StorableVec::forced_import(&path.join("height_to_medianfeerate"), Version::ONE)?,
            // height_to_minfeerate: StorableVec::forced_import(&path.join("height_to_minfeerate"), Version::ONE)?,
            // height_to_output_count: StorableVec::forced_import(
            //     &path.join("height_to_output_count"),
            //     Version::ONE,
            // )?,
            // height_to_subsidy: StorableVec::forced_import(&path.join("height_to_subsidy"), Version::ONE)?,
            // height_to_totalfees: StorableVec::forced_import(&path.join("height_to_totalfees"), Version::ONE)?,
            // height_to_txcount: StorableVec::forced_import(&path.join("height_to_txcount"), Version::ONE)?,
            // txindex_to_fee: StorableVec::forced_import(
            //     &path.join("txindex_to_fee"),
            //     Version::ONE,
            // )?,
            txindex_to_is_coinbase: ComputedVec::forced_import(
                &path.join("txindex_to_is_coinbase"),
                Version::ONE,
                compressed,
            )?,
            // txindex_to_feerate: StorableVec::forced_import(&path.join("txindex_to_feerate"), Version::ONE)?,
            txindex_to_inputs_count: ComputedVecsFromTxindex::forced_import(
                path,
                "inputs_count",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            txindex_to_outputs_count: ComputedVecsFromTxindex::forced_import(
                path,
                "outputs_count",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            // txinindex_to_value: StorableVec::forced_import(
            //     &path.join("txinindex_to_value"),
            //     Version::ONE,
            //     compressed,
            // )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.txindex_to_inputs_count.compute(
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

        self.txindex_to_outputs_count.compute(
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

        // self.txindex_to_inputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     indexer_vecs.txindex_to_first_txinindex.mut_vec(),
        //     indexes.txindex_to_last_txinindex.mut_vec(),
        //     exit,
        // )?;

        // self.txindex_to_outputs_count.compute_count_from_indexes(
        //     starting_indexes.txindex,
        //     indexer_vecs.txindex_to_first_txoutindex.mut_vec(),
        //     indexes.txindex_to_last_txoutindex.mut_vec(),
        //     exit,
        // )?;

        let indexer_vecs = indexer.mut_vecs();

        self.txindex_to_is_coinbase.compute_is_first_ordered(
            starting_indexes.txindex,
            indexer_vecs.txindex_to_height.mut_vec(),
            indexer_vecs.height_to_first_txindex.mut_vec(),
            exit,
        )?;

        // self.txinindex_to_value.compute_transform(
        //     starting_indexes.txinindex,
        //     indexer_vecs.txinindex_to_txoutindex.mut_vec(),
        //     |(txinindex, txoutindex, slf, other)| {
        //         let value =
        //             if let Ok(Some(value)) = indexer_vecs.txoutindex_to_value.read(txoutindex) {
        //                 *value
        //             } else {
        //                 dbg!(txinindex, txoutindex, slf.len(), other.len());
        //                 panic!()
        //             };
        //         (txinindex, value)
        //     },
        //     exit,
        // )?;

        // self.vecs.txindex_to_fee.compute_transform(
        //     &mut self.vecs.txindex_to_height,
        //     &mut indexer.vecs().height_to_first_txindex,
        // )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            vec![self.txindex_to_is_coinbase.any_vec()],
            self.txindex_to_outputs_count.any_vecs(),
            self.txindex_to_inputs_count.any_vecs(),
        ]
        .concat()
    }
}
