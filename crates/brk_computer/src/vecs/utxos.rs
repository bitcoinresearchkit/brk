use std::{fs, path::Path};

use brk_core::{CheckedSub, Dollars, Height, Sats, StoredUsize};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, BaseVecIterator, Compressed, Computation, EagerVec, StoredIndex,
    VecIterator, Version,
};
use derive_deref::{Deref, DerefMut};

use crate::states::{CohortStates, Outputs};

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes, transactions,
};

#[derive(Clone, Deref, DerefMut)]
pub struct Vecs(Outputs<Vecs_>);

#[derive(Clone)]
pub struct Vecs_ {
    pub height_to_realized_cap: EagerVec<Height, Dollars>,
    pub indexes_to_realized_cap: ComputedVecsFromHeight<Dollars>,
    pub height_to_supply: EagerVec<Height, Sats>,
    pub indexes_to_supply: ComputedVecsFromHeight<Sats>,
    pub height_to_utxo_count: EagerVec<Height, StoredUsize>,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredUsize>,
}

const VERSION: Version = Version::ZERO;

impl Vecs {
    pub fn forced_import(
        path: &Path,
        _computation: Computation,
        compressed: Compressed,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self(Outputs {
            all: Vecs_ {
                height_to_realized_cap: EagerVec::forced_import(
                    &path.join("height_to_realized_cap"),
                    VERSION + Version::ZERO,
                    compressed,
                )?,
                indexes_to_realized_cap: ComputedVecsFromHeight::forced_import(
                    path,
                    "realized_cap",
                    false,
                    VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )?,
                height_to_supply: EagerVec::forced_import(
                    &path.join("height_to_supply"),
                    VERSION + Version::ZERO,
                    compressed,
                )?,
                indexes_to_supply: ComputedVecsFromHeight::forced_import(
                    path,
                    "supply",
                    false,
                    VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )?,
                height_to_utxo_count: EagerVec::forced_import(
                    &path.join("height_to_utxo_count"),
                    VERSION + Version::new(111),
                    compressed,
                )?,
                indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                    path,
                    "utxo_count",
                    false,
                    VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )?,
            },
        }))
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let indexer_vecs = indexer.vecs();

        let height_to_first_outputindex = &indexer_vecs.height_to_first_outputindex;
        let height_to_first_inputindex = &indexer_vecs.height_to_first_inputindex;
        let height_to_output_count = transactions.indexes_to_output_count.height.unwrap_last();
        let height_to_input_count = transactions.indexes_to_input_count.height.unwrap_last();
        let inputindex_to_outputindex = &indexer_vecs.inputindex_to_outputindex;
        let outputindex_to_value = &indexer_vecs.outputindex_to_value;
        let txindex_to_height = &indexes.txindex_to_height;
        let outputindex_to_txindex = &indexes.outputindex_to_txindex;

        let mut height_to_first_outputindex_iter = height_to_first_outputindex.into_iter();
        let mut height_to_first_inputindex_iter = height_to_first_inputindex.into_iter();
        let mut height_to_output_count_iter = height_to_output_count.into_iter();
        let mut height_to_input_count_iter = height_to_input_count.into_iter();
        let mut inputindex_to_outputindex_iter = inputindex_to_outputindex.into_iter();
        let mut outputindex_to_value_iter = outputindex_to_value.into_iter();
        let mut txindex_to_height_iter = txindex_to_height.into_iter();
        let mut outputindex_to_txindex_iter = outputindex_to_txindex.into_iter();

        let base_version = Version::ZERO
            + height_to_first_outputindex.version()
            + height_to_first_inputindex.version()
            + height_to_output_count.version()
            + height_to_input_count.version()
            + inputindex_to_outputindex.version()
            + outputindex_to_value.version()
            + txindex_to_height.version()
            + outputindex_to_txindex.version();

        let height_to_realized_cap = &mut self.0.all.height_to_realized_cap;
        let height_to_supply = &mut self.0.all.height_to_supply;
        let height_to_utxo_count = &mut self.0.all.height_to_utxo_count;

        height_to_realized_cap.validate_computed_version_or_reset_file(
            base_version + height_to_realized_cap.inner_version(),
        )?;
        height_to_supply.validate_computed_version_or_reset_file(
            base_version + height_to_supply.inner_version(),
        )?;
        height_to_utxo_count.validate_computed_version_or_reset_file(
            base_version + height_to_utxo_count.inner_version(),
        )?;

        let starting_height = [
            height_to_realized_cap.len(),
            height_to_supply.len(),
            height_to_utxo_count.len(),
        ]
        .into_iter()
        .map(Height::from)
        .min()
        .unwrap()
        .min(starting_indexes.height);

        let mut states = CohortStates::default();

        if let Some(prev_height) = starting_height.checked_sub(Height::new(1)) {
            states.realized_cap = height_to_realized_cap
                .into_iter()
                .unwrap_get_inner(prev_height);
            states.supply = height_to_supply.into_iter().unwrap_get_inner(prev_height);
            states.utxo_count = height_to_utxo_count
                .into_iter()
                .unwrap_get_inner(prev_height);
        }

        (starting_height.unwrap_to_usize()..height_to_first_outputindex_iter.len())
            .map(Height::from)
            .try_for_each(|height| -> color_eyre::Result<()> {
                let first_outputindex = height_to_first_outputindex_iter.unwrap_get_inner(height);
                let first_inputindex = height_to_first_inputindex_iter.unwrap_get_inner(height);
                let output_count = height_to_output_count_iter.unwrap_get_inner(height);
                let input_count = height_to_input_count_iter.unwrap_get_inner(height);

                Ok(())
            })?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        // [].concat()
        vec![]
    }
}
