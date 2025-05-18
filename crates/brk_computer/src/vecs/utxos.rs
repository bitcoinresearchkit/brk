use std::{collections::BTreeMap, fs, path::Path, thread};

use brk_core::{
    Bitcoin, CheckedSub, Dollars, Height, InputIndex, OutputIndex, OutputType, Sats, StoredUsize,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, BaseVecIterator, Compressed, Computation, EagerVec, StoredIndex,
    VecIterator, Version,
};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;

use crate::states::{CohortState, Outputs, RealizedState, ReceivedState, SentState};

use super::{
    Indexes, fetched,
    grouped::{ComputedValueVecsFromHeight, ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes, transactions,
};

const VERSION: Version = Version::new(3);

#[derive(Clone, Deref, DerefMut)]
pub struct Vecs(Outputs<Vecs_>);

impl Vecs {
    pub fn forced_import(
        path: &Path,
        _computation: Computation,
        compressed: Compressed,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        Ok(Self(Outputs {
            all: Vecs_::forced_import(path, _computation, compressed, fetched)?,
        }))
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transactions: &transactions::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let indexer_vecs = indexer.vecs();

        let height_to_first_outputindex = &indexer_vecs.height_to_first_outputindex;
        let height_to_first_inputindex = &indexer_vecs.height_to_first_inputindex;
        let height_to_output_count = transactions.indexes_to_output_count.height.unwrap_sum();
        let height_to_input_count = transactions.indexes_to_input_count.height.unwrap_sum();
        let inputindex_to_outputindex = &indexer_vecs.inputindex_to_outputindex;
        let outputindex_to_value = &indexer_vecs.outputindex_to_value;
        let txindex_to_height = &indexes.txindex_to_height;
        let height_to_timestamp_fixed = &indexes.height_to_timestamp_fixed;
        let outputindex_to_txindex = &indexes.outputindex_to_txindex;
        let outputindex_to_outputtype = &indexer_vecs.outputindex_to_outputtype;
        let height_to_unclaimed_rewards = transactions
            .indexes_to_unclaimed_rewards
            .sats
            .height
            .as_ref()
            .unwrap()
            .as_ref();
        let height_to_close = &fetched
            .as_ref()
            .map(|fetched| &fetched.chainindexes_to_close.height);
        let height_to_opreturn_count = &transactions
            .indexes_to_opreturn_count
            .height
            .as_ref()
            .unwrap()
            .as_ref();

        let mut height_to_first_outputindex_iter = height_to_first_outputindex.into_iter();
        let mut height_to_first_inputindex_iter = height_to_first_inputindex.into_iter();
        let mut height_to_output_count_iter = height_to_output_count.into_iter();
        let mut height_to_input_count_iter = height_to_input_count.into_iter();
        let mut inputindex_to_outputindex_iter = inputindex_to_outputindex.into_iter();
        let mut outputindex_to_value_iter = outputindex_to_value.into_iter();
        let mut outputindex_to_value_iter_2 = outputindex_to_value.into_iter();
        let mut txindex_to_height_iter = txindex_to_height.into_iter();
        let mut outputindex_to_txindex_iter = outputindex_to_txindex.into_iter();
        let mut height_to_close_iter = height_to_close.as_ref().map(|v| v.into_iter());
        let mut height_to_opreturn_count_iter = height_to_opreturn_count.into_iter();
        let mut outputindex_to_outputtype_iter = outputindex_to_outputtype.into_iter();
        let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
        let mut height_to_timestamp_fixed_iter = height_to_timestamp_fixed.into_iter();

        let base_version = Version::ZERO
            + height_to_first_outputindex.version()
            + height_to_first_inputindex.version()
            + height_to_timestamp_fixed.version()
            + height_to_output_count.version()
            + height_to_input_count.version()
            + inputindex_to_outputindex.version()
            + outputindex_to_value.version()
            + txindex_to_height.version()
            + outputindex_to_txindex.version()
            + height_to_opreturn_count.version()
            + outputindex_to_outputtype.version()
            + height_to_unclaimed_rewards.version()
            + height_to_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version());

        let mut height_to_realized_cap = self.0.all.height_to_realized_cap.as_mut();
        let height_to_supply = &mut self.0.all.height_to_supply;
        let height_to_unspendable_supply = &mut self.0.all.height_to_unspendable_supply;
        let height_to_utxo_count = &mut self.0.all.height_to_utxo_count;

        height_to_supply.validate_computed_version_or_reset_file(
            base_version + height_to_supply.inner_version(),
        )?;
        height_to_unspendable_supply.validate_computed_version_or_reset_file(
            base_version + height_to_unspendable_supply.inner_version(),
        )?;
        height_to_utxo_count.validate_computed_version_or_reset_file(
            base_version + height_to_utxo_count.inner_version(),
        )?;
        if let Some(height_to_realized_cap) = height_to_realized_cap.as_mut() {
            height_to_realized_cap.validate_computed_version_or_reset_file(
                base_version + height_to_realized_cap.inner_version(),
            )?;
        }

        let starting_height = [
            height_to_supply.len(),
            height_to_unspendable_supply.len(),
            height_to_utxo_count.len(),
            height_to_realized_cap
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
        ]
        .into_iter()
        .map(Height::from)
        .min()
        .unwrap()
        .min(starting_indexes.height);

        let mut state = CohortState::default();

        if let Some(prev_height) = starting_height.checked_sub(Height::new(1)) {
            state.supply = height_to_supply.into_iter().unwrap_get_inner(prev_height);
            state.unspendable_supply = height_to_unspendable_supply
                .into_iter()
                .unwrap_get_inner(prev_height);
            state.utxo_count = height_to_utxo_count
                .into_iter()
                .unwrap_get_inner(prev_height);
            if let Some(height_to_realized_cap) = height_to_realized_cap.as_mut() {
                state.realized_cap = height_to_realized_cap
                    .into_iter()
                    .unwrap_get_inner(prev_height);
            }
        }

        (starting_height.unwrap_to_usize()..height_to_first_outputindex_iter.len())
            .map(Height::from)
            .try_for_each(|height| -> color_eyre::Result<()> {
                let sent_state = SentState::default();
                let received_state = ReceivedState::default();
                let realized_state = RealizedState::default();

                let first_outputindex = height_to_first_outputindex_iter
                    .unwrap_get_inner(height)
                    .unwrap_to_usize();
                let first_inputindex = height_to_first_inputindex_iter
                    .unwrap_get_inner(height)
                    .unwrap_to_usize();
                let output_count = height_to_output_count_iter.unwrap_get_inner(height);
                let input_count = height_to_input_count_iter.unwrap_get_inner(height);
                let opreturn_count = height_to_opreturn_count_iter.unwrap_get_inner(height);

                let (sent_sats_price_tuple, (mut received_spendable, mut received_unspendable)) =
                    thread::scope(|s| {
                        // Skip coinbase
                        let sent_sats_price_tuple = s.spawn(|| {
                            let mut txindex_to_height = BTreeMap::new();
                            let mut height_to_timestamp_price_sats = BTreeMap::new();

                            (first_inputindex + 1..first_inputindex + *input_count)
                                .map(InputIndex::from)
                                .map(|inputindex| {
                                    inputindex_to_outputindex_iter.unwrap_get_inner(inputindex)
                                })
                                .for_each(|outputindex| {
                                    let value =
                                        outputindex_to_value_iter.unwrap_get_inner(outputindex);

                                    let txindex =
                                        outputindex_to_txindex_iter.unwrap_get_inner(outputindex);

                                    let height =
                                        *txindex_to_height.entry(txindex).or_insert_with(|| {
                                            txindex_to_height_iter.unwrap_get_inner(txindex)
                                        });

                                    let entry = height_to_timestamp_price_sats
                                        .entry(height)
                                        .or_insert_with(|| {
                                            let timestamp = height_to_timestamp_fixed_iter
                                                .unwrap_get_inner(height);

                                            if let Some(height_to_close_iter) =
                                                height_to_close_iter.as_mut()
                                            {
                                                let dollars =
                                                    *height_to_close_iter.unwrap_get_inner(height);

                                                (timestamp, dollars, Sats::ZERO)
                                            } else {
                                                (timestamp, Dollars::ZERO, Sats::ZERO)
                                            }
                                        });

                                    entry.2 += value;
                                });

                            height_to_timestamp_price_sats
                        });

                        let received = s.spawn(|| {
                            let mut spendable = Sats::ZERO;
                            let mut unspendable = Sats::ZERO;
                            (first_outputindex..first_outputindex + *output_count)
                                .map(OutputIndex::from)
                                .for_each(|outputindex| {
                                    let value =
                                        outputindex_to_value_iter_2.unwrap_get_inner(outputindex);

                                    let outputtype = outputindex_to_outputtype_iter
                                        .unwrap_get_inner(outputindex);

                                    if outputtype == OutputType::OpReturn
                                        || outputtype == OutputType::Empty
                                        || outputtype == OutputType::Unknown
                                    {
                                        unspendable += value
                                    } else {
                                        spendable += value
                                    }
                                });
                            (spendable, unspendable)
                        });

                        (
                            sent_sats_price_tuple.join().unwrap(),
                            received.join().unwrap(),
                        )
                    });

                let (sent, realized_cap_destroyed) = sent_sats_price_tuple
                    .par_iter()
                    .map(|(_, (_, dollars, sats))| {
                        let dollars = *dollars;
                        let sats = *sats;
                        (sats, dollars * Bitcoin::from(sats))
                    })
                    .reduce(
                        || (Sats::ZERO, Dollars::ZERO),
                        |acc, (sats, dollars)| (acc.0 + sats, acc.1 + dollars),
                    );

                let utxos_created = *output_count - *opreturn_count;

                // Three invalid coinbases which all have 1 output
                let utxos_destroyed = if height == Height::new(0)
                    || height == Height::new(91_842)
                    || height == Height::new(91_880)
                {
                    received_spendable -= Sats::FIFTY_BTC;
                    received_unspendable += Sats::FIFTY_BTC;
                    *input_count
                } else {
                    *input_count - 1
                };

                received_unspendable += height_to_unclaimed_rewards_iter.unwrap_get_inner(height);

                state.supply -= sent;

                state.supply += received_spendable;
                state.unspendable_supply += received_unspendable;

                *state.utxo_count += utxos_created;
                *state.utxo_count -= utxos_destroyed;

                if let Some(height_to_close_iter) = height_to_close_iter.as_mut() {
                    let received = received_spendable + received_unspendable;
                    let price = *height_to_close_iter.unwrap_get_inner(height);
                    let realized_cap_created = price * Bitcoin::from(received);
                    state.realized_cap = (state.realized_cap + realized_cap_created)
                        .checked_sub(realized_cap_destroyed)
                        .unwrap();
                }

                height_to_supply.forced_push_at(height, state.supply, exit)?;
                height_to_unspendable_supply.forced_push_at(
                    height,
                    state.unspendable_supply,
                    exit,
                )?;
                height_to_utxo_count.forced_push_at(height, state.utxo_count, exit)?;
                if let Some(height_to_realized_cap) = height_to_realized_cap.as_mut() {
                    height_to_realized_cap.forced_push_at(height, state.realized_cap, exit)?;
                }

                Ok(())
            })?;

        height_to_supply.safe_flush(exit)?;
        height_to_unspendable_supply.safe_flush(exit)?;
        height_to_utxo_count.safe_flush(exit)?;
        if let Some(height_to_realized_cap) = height_to_realized_cap.as_mut() {
            height_to_realized_cap.safe_flush(exit)?;
        }

        self.0.all.indexes_to_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.0.all.height_to_supply),
        )?;
        self.0.all.indexes_to_unspendable_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.0.all.height_to_unspendable_supply),
        )?;
        self.0.all.indexes_to_utxo_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.0.all.height_to_utxo_count),
        )?;
        if let Some(indexes_to_realized_cap) = self.0.all.indexes_to_realized_cap.as_mut() {
            indexes_to_realized_cap.compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.0.all.height_to_realized_cap.as_ref().unwrap()),
            )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [self.all.vecs()].concat()
    }
}

#[derive(Clone)]
pub struct Vecs_ {
    pub height_to_realized_cap: Option<EagerVec<Height, Dollars>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_supply: EagerVec<Height, Sats>,
    pub indexes_to_supply: ComputedValueVecsFromHeight,
    pub height_to_unspendable_supply: EagerVec<Height, Sats>,
    pub indexes_to_unspendable_supply: ComputedValueVecsFromHeight,
    pub height_to_utxo_count: EagerVec<Height, StoredUsize>,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredUsize>,
}

impl Vecs_ {
    pub fn forced_import(
        path: &Path,
        _computation: Computation,
        compressed: Compressed,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        fs::create_dir_all(path)?;

        Ok(Self {
            height_to_realized_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    &path.join("height_to_realized_cap"),
                    VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    "realized_cap",
                    false,
                    VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_supply: EagerVec::forced_import(
                &path.join("height_to_supply"),
                VERSION + Version::ZERO,
                compressed,
            )?,
            indexes_to_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "supply",
                false,
                VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            height_to_unspendable_supply: EagerVec::forced_import(
                &path.join("height_to_unspendable_supply"),
                VERSION + Version::ZERO,
                compressed,
            )?,
            indexes_to_unspendable_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "unspendable_supply",
                false,
                VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
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
        })
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![
                &self.height_to_supply as &dyn AnyCollectableVec,
                &self.height_to_unspendable_supply,
                &self.height_to_utxo_count,
            ],
            self.height_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_supply.vecs(),
            self.indexes_to_unspendable_supply.vecs(),
            self.indexes_to_utxo_count.vecs(),
            self.indexes_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
        ]
        .concat()
    }
}
