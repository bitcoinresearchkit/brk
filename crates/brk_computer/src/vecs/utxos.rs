use std::{cmp::Ordering, collections::BTreeMap, fs, mem, path::Path, thread};

use brk_core::{
    Bitcoin, CheckedSub, Dollars, Height, InputIndex, OutputIndex, OutputType, Sats, StoredUsize,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, BaseVecIterator, CollectableVec, Compressed, Computation, EagerVec,
    GenericStoredVec, IndexedVec, Result, StoredIndex, StoredVec, VecIterator, Version,
};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;

use crate::states::{
    BlockState, CohortState, Outputs, OutputsByEpoch, OutputsByFrom, OutputsByRange, OutputsBySize,
    OutputsByTerm, OutputsByType, OutputsByUpTo, OutputsByValue, RealizedState, ReceivedState,
    SentState,
};

use super::{
    Indexes, fetched,
    grouped::{ComputedValueVecsFromHeight, ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes, transactions,
};

const VERSION: Version = Version::new(3);

#[derive(Clone)]
pub struct Vecs {
    chain_state: Vec<BlockState>,
    saved_chain_state: StoredVec<Height, BlockState>,

    // unspendable
    // cointime ?
    pub height_to_unspendable_supply: EagerVec<Height, Sats>,
    pub indexes_to_unspendable_supply: ComputedValueVecsFromHeight,
    utxos_vecs: Outputs<Vecs_>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        _computation: Computation,
        compressed: Compressed,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        let mut states_path = path.to_owned();
        states_path.pop();
        states_path = states_path.join("states");

        Ok(Self {
            chain_state: vec![],
            saved_chain_state: StoredVec::forced_import(
                &states_path,
                "chain",
                Version::ZERO,
                Compressed::NO,
            )?,

            height_to_unspendable_supply: EagerVec::forced_import(
                path,
                "unspendable_supply",
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
            utxos_vecs: {
                Outputs {
                    all: Vecs_::forced_import(path, None, _computation, compressed, fetched)?,
                    by_term: OutputsByTerm {
                        short: Vecs_::forced_import(
                            path,
                            Some("sth"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        long: Vecs_::forced_import(
                            path,
                            Some("lth"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_up_to: OutputsByUpTo {
                        _1d: Vecs_::forced_import(
                            path,
                            Some("up_to_1d"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _1w: Vecs_::forced_import(
                            path,
                            Some("up_to_1w"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _1m: Vecs_::forced_import(
                            path,
                            Some("up_to_1m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _2m: Vecs_::forced_import(
                            path,
                            Some("up_to_2m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _3m: Vecs_::forced_import(
                            path,
                            Some("up_to_3m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _4m: Vecs_::forced_import(
                            path,
                            Some("up_to_4m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _5m: Vecs_::forced_import(
                            path,
                            Some("up_to_5m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _6m: Vecs_::forced_import(
                            path,
                            Some("up_to_6m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _1y: Vecs_::forced_import(
                            path,
                            Some("up_to_1y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _2y: Vecs_::forced_import(
                            path,
                            Some("up_to_2y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _3y: Vecs_::forced_import(
                            path,
                            Some("up_to_3y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _5y: Vecs_::forced_import(
                            path,
                            Some("up_to_5y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _7y: Vecs_::forced_import(
                            path,
                            Some("up_to_7y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _10y: Vecs_::forced_import(
                            path,
                            Some("up_to_10y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _15y: Vecs_::forced_import(
                            path,
                            Some("up_to_15y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_from: OutputsByFrom {
                        _1y: Vecs_::forced_import(
                            path,
                            Some("from_1y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _2y: Vecs_::forced_import(
                            path,
                            Some("from_2y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _4y: Vecs_::forced_import(
                            path,
                            Some("from_4y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _10y: Vecs_::forced_import(
                            path,
                            Some("from_10y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _15y: Vecs_::forced_import(
                            path,
                            Some("from_15y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_range: OutputsByRange {
                        _1d_to_1w: Vecs_::forced_import(
                            path,
                            Some("from_1d_to_1w"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _1w_to_1m: Vecs_::forced_import(
                            path,
                            Some("from_1w_to_1m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _1m_to_3m: Vecs_::forced_import(
                            path,
                            Some("from_1m_to_3m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _3m_to_6m: Vecs_::forced_import(
                            path,
                            Some("from_3m_to_6m"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _6m_to_1y: Vecs_::forced_import(
                            path,
                            Some("from_6m_to_1y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _1y_to_2y: Vecs_::forced_import(
                            path,
                            Some("from_1y_to_2y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _2y_to_3y: Vecs_::forced_import(
                            path,
                            Some("from_2y_to_3y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _3y_to_5y: Vecs_::forced_import(
                            path,
                            Some("from_3y_to_5y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _5y_to_7y: Vecs_::forced_import(
                            path,
                            Some("from_5y_to_7y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _7y_to_10y: Vecs_::forced_import(
                            path,
                            Some("from_7y_to_10y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _10y_to_15y: Vecs_::forced_import(
                            path,
                            Some("from_10y_to_15y"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_epoch: OutputsByEpoch {
                        _1: Vecs_::forced_import(
                            path,
                            Some("epoch_1"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _2: Vecs_::forced_import(
                            path,
                            Some("epoch_2"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _3: Vecs_::forced_import(
                            path,
                            Some("epoch_3"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _4: Vecs_::forced_import(
                            path,
                            Some("epoch_4"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        _5: Vecs_::forced_import(
                            path,
                            Some("epoch_5"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_size: OutputsBySize {
                        from_1sat_to_10sats: Vecs_::forced_import(
                            path,
                            Some("from_1sat_to_10sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10sats_to_100sats: Vecs_::forced_import(
                            path,
                            Some("from_10sats_to_100sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100sats_to_1_000sats: Vecs_::forced_import(
                            path,
                            Some("from_100sats_to_1_000sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1_000sats_to_10_000sats: Vecs_::forced_import(
                            path,
                            Some("from_1_000sats_to_10_000sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10_000sats_to_100_000sats: Vecs_::forced_import(
                            path,
                            Some("from_10_000sats_to_100_000sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100_000sats_to_1_000_000sats: Vecs_::forced_import(
                            path,
                            Some("from_100_000sats_to_1_000_000sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1_000_000sats_to_10_000_000sats: Vecs_::forced_import(
                            path,
                            Some("from_1_000_000sats_to_10_000_000sats"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10_000_000sats_to_1btc: Vecs_::forced_import(
                            path,
                            Some("from_10_000_000sats_to_1btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1btc_to_10btc: Vecs_::forced_import(
                            path,
                            Some("from_1btc_to_10btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10btc_to_100btc: Vecs_::forced_import(
                            path,
                            Some("from_10btc_to_100btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100btc_to_1_000btc: Vecs_::forced_import(
                            path,
                            Some("from_100btc_to_1_000btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1_000btc_to_10_000btc: Vecs_::forced_import(
                            path,
                            Some("from_1_000btc_to_10_000btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10_000btc_to_100_000btc: Vecs_::forced_import(
                            path,
                            Some("from_10_000btc_to_100_000btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100_000btc: Vecs_::forced_import(
                            path,
                            Some("from_100_000btc"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_value: OutputsByValue {
                        up_to_1cent: Vecs_::forced_import(
                            path,
                            Some("up_to_1cent"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1c_to_10c: Vecs_::forced_import(
                            path,
                            Some("from_1c_to_10c"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10c_to_1d: Vecs_::forced_import(
                            path,
                            Some("from_10c_to_1d"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1d_to_10d: Vecs_::forced_import(
                            path,
                            Some("from_1d_to_10d"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10usd_to_100usd: Vecs_::forced_import(
                            path,
                            Some("from_10usd_to_100usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100usd_to_1_000usd: Vecs_::forced_import(
                            path,
                            Some("from_100usd_to_1_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1_000usd_to_10_000usd: Vecs_::forced_import(
                            path,
                            Some("from_1_000usd_to_10_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10_000usd_to_100_000usd: Vecs_::forced_import(
                            path,
                            Some("from_10_000usd_to_100_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100_000usd_to_1_000_000usd: Vecs_::forced_import(
                            path,
                            Some("from_100_000usd_to_1_000_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1_000_000usd_to_10_000_000usd: Vecs_::forced_import(
                            path,
                            Some("from_1_000_000usd_to_10_000_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_10_000_000usd_to_100_000_000usd: Vecs_::forced_import(
                            path,
                            Some("from_10_000_000usd_to_100_000_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_100_000_000usd_to_1_000_000_000usd: Vecs_::forced_import(
                            path,
                            Some("from_100_000_000usd_to_1_000_000_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        from_1_000_000_000usd: Vecs_::forced_import(
                            path,
                            Some("from_1_000_000_000usd"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    by_type: OutputsByType {
                        p2pk65: Vecs_::forced_import(
                            path,
                            Some("p2pk65"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2pk33: Vecs_::forced_import(
                            path,
                            Some("p2pk33"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2pkh: Vecs_::forced_import(
                            path,
                            Some("p2pkh"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2ms: Vecs_::forced_import(
                            path,
                            Some("p2ms"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2sh: Vecs_::forced_import(
                            path,
                            Some("p2sh"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        op_return: Vecs_::forced_import(
                            path,
                            Some("op_return"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2wpkh: Vecs_::forced_import(
                            path,
                            Some("p2wpkh"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2wsh: Vecs_::forced_import(
                            path,
                            Some("p2wsh"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2tr: Vecs_::forced_import(
                            path,
                            Some("p2tr"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        p2a: Vecs_::forced_import(
                            path,
                            Some("p2a"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        empty: Vecs_::forced_import(
                            path,
                            Some("empty"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        unknown: Vecs_::forced_import(
                            path,
                            Some("unknown"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                }
            },
        })
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
        let mut outputindex_to_outputtype_iter = outputindex_to_outputtype.into_iter();
        let mut outputindex_to_outputtype_iter_2 = outputindex_to_outputtype.into_iter();
        let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
        let mut height_to_timestamp_fixed_iter = height_to_timestamp_fixed.into_iter();

        let mut flat_vecs_ = self.utxos_vecs.mut_flatten();

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
            + outputindex_to_outputtype.version()
            + height_to_unclaimed_rewards.version()
            + height_to_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version());

        flat_vecs_
            .iter_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;
        self.height_to_unspendable_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_unspendable_supply.inner_version(),
            )?;

        let chain_state_starting_height = Height::from(self.saved_chain_state.len());

        let starting_height = flat_vecs_
            .iter_mut()
            .map(|v| v.init(starting_indexes))
            .min()
            .unwrap_or_default()
            .min(chain_state_starting_height);

        // self.state.unspendable_supply = self
        //     .height_to_unspendable_supply
        //     .into_iter()
        //     .unwrap_get_inner(prev_height);
        //
        // / self.state.unspendable_supply = self
        //     .height_to_unspendable_supply
        //     .
        // starting_height

        match starting_height.cmp(&chain_state_starting_height) {
            Ordering::Greater => unreachable!(),
            Ordering::Equal => {
                self.chain_state = self.saved_chain_state.collect_range(None, None)?;
            }
            Ordering::Less => {
                todo!("rollback instead");
                // self.chain_state = vec![];
                // starting_height = Height::ZERO;
            }
        }

        let mut height = Height::ZERO;
        (starting_height.unwrap_to_usize()..height_to_first_outputindex_iter.len())
            .map(Height::from)
            .try_for_each(|_height| -> color_eyre::Result<()> {
                height = _height;

                // let sent_state = SentState::default();
                // let received_state = ReceivedState::default();
                // let realized_state = RealizedState::default();

                let first_outputindex = height_to_first_outputindex_iter
                    .unwrap_get_inner(height)
                    .unwrap_to_usize();
                let first_inputindex = height_to_first_inputindex_iter
                    .unwrap_get_inner(height)
                    .unwrap_to_usize();
                let output_count = height_to_output_count_iter.unwrap_get_inner(height);
                let input_count = height_to_input_count_iter.unwrap_get_inner(height);
                // let opreturn_count = height_to_opreturn_count_iter.unwrap_get_inner(height);

                let ((height_to_sent_data, block_state_to_subtract), mut received) =
                    thread::scope(|s| {
                        let sent = s.spawn(|| {
                            let mut txindex_to_height = BTreeMap::new();
                            let mut height_to_sent_data = BTreeMap::new();
                            let mut block_state_to_subtract = BlockState::default();

                            // Skip coinbase
                            (first_inputindex + 1..first_inputindex + *input_count)
                                .map(InputIndex::from)
                                .map(|inputindex| {
                                    inputindex_to_outputindex_iter.unwrap_get_inner(inputindex)
                                })
                                .for_each(|outputindex| {
                                    let value =
                                        outputindex_to_value_iter.unwrap_get_inner(outputindex);

                                    let _type = outputindex_to_outputtype_iter
                                        .unwrap_get_inner(outputindex);

                                    let txindex =
                                        outputindex_to_txindex_iter.unwrap_get_inner(outputindex);

                                    let input_height =
                                        *txindex_to_height.entry(txindex).or_insert_with(|| {
                                            txindex_to_height_iter.unwrap_get_inner(txindex)
                                        });

                                    match input_height.cmp(&height) {
                                        Ordering::Equal => {
                                            block_state_to_subtract.utxos += 1;
                                            block_state_to_subtract.value += value;
                                        }
                                        Ordering::Greater => unreachable!(),
                                        Ordering::Less => {
                                            let block_state = self
                                                .chain_state
                                                .get_mut(input_height.unwrap_to_usize())
                                                .unwrap();
                                            block_state.utxos -= 1;
                                            block_state.value -= value;
                                        }
                                    }

                                    let input_data = height_to_sent_data
                                        .entry(input_height)
                                        .or_insert_with(|| {
                                            let timestamp = height_to_timestamp_fixed_iter
                                                .unwrap_get_inner(input_height);

                                            if let Some(height_to_close_iter) =
                                                height_to_close_iter.as_mut()
                                            {
                                                let dollars = *height_to_close_iter
                                                    .unwrap_get_inner(input_height);

                                                (timestamp, dollars, Sats::ZERO, _type)
                                            } else {
                                                (timestamp, Dollars::ZERO, Sats::ZERO, _type)
                                            }
                                        });

                                    input_data.2 += value;
                                });

                            (height_to_sent_data, block_state_to_subtract)
                        });

                        let received = s.spawn(|| {
                            let mut by_type: OutputsByType<BlockState> = OutputsByType::default();

                            (first_outputindex..first_outputindex + *output_count)
                                .map(OutputIndex::from)
                                .for_each(|outputindex| {
                                    let value =
                                        outputindex_to_value_iter_2.unwrap_get_inner(outputindex);

                                    let outputtype = outputindex_to_outputtype_iter_2
                                        .unwrap_get_inner(outputindex);

                                    by_type.get_mut(outputtype).value += value;
                                    by_type.get_mut(outputtype).utxos += 1;
                                });

                            by_type
                        });

                        (sent.join().unwrap(), received.join().unwrap())
                    });

                // Compute then push current block state
                let mut block_state = BlockState::default();
                received
                    .to_spendable_vec()
                    .into_iter()
                    .for_each(|spendable_block_state| {
                        block_state += spendable_block_state;
                    });
                block_state -= block_state_to_subtract;
                self.chain_state.push(block_state);

                let (sent, realized_cap_destroyed) = height_to_sent_data
                    .par_iter()
                    .map(|(_, (_, dollars, sats, _))| {
                        let dollars = *dollars;
                        let sats = *sats;
                        (sats, dollars * Bitcoin::from(sats))
                    })
                    .reduce(
                        || (Sats::ZERO, Dollars::ZERO),
                        |acc, (sats, dollars)| (acc.0 + sats, acc.1 + dollars),
                    );

                let utxos_created = *output_count - received.op_return.utxos;

                let mut received_unspendable = OutputType::as_vec()
                    .into_iter()
                    .filter(|t| t.is_unspendable())
                    .map(|t| received.get_mut(t).value)
                    .sum::<Sats>()
                    + height_to_unclaimed_rewards_iter.unwrap_get_inner(height);

                // Three invalid coinbases which all have 1 output
                let utxos_destroyed = if height == Height::new(0)
                    || height == Height::new(91_842)
                    || height == Height::new(91_880)
                {
                    // They're all p2pk65
                    received.p2pk65.utxos -= 1;
                    received.p2pk65.value -= Sats::FIFTY_BTC;
                    received_unspendable += Sats::FIFTY_BTC;
                    *input_count
                } else {
                    *input_count - 1
                };

                // state.supply -= sent;

                // state.supply += received_spendable;
                // state.unspendable_supply += received_unspendable;

                // *state.utxo_count += utxos_created;
                // *state.utxo_count -= utxos_destroyed;

                // if let Some(height_to_close_iter) = height_to_close_iter.as_mut() {
                //     let received = received_spendable + received_unspendable;
                //     let price = *height_to_close_iter.unwrap_get_inner(height);
                //     let realized_cap_created = price * Bitcoin::from(received);
                //     state.realized_cap = (state.realized_cap + realized_cap_created)
                //         .checked_sub(realized_cap_destroyed)
                //         .unwrap();
                // }

                self.utxos_vecs
                    .mut_flatten()
                    .iter_mut()
                    .try_for_each(|v| v.forced_pushed_at(height, exit))?;

                // // self.height_to_unspendable_supply.forced_push_at(
                //     height,
                //     self.state.unspendable_supply,
                //     exit,
                // )?;

                Ok(())
            })?;

        exit.block();

        let mut flat_vecs_ = self.utxos_vecs.mut_flatten();

        // Flush rest of values
        flat_vecs_
            .iter_mut()
            .try_for_each(|v| v.safe_flush_height_vecs(exit))?;
        self.height_to_unspendable_supply.safe_flush(exit)?;

        // Compute other vecs from height vecs
        flat_vecs_
            .iter_mut()
            .try_for_each(|v| v.compute_rest(indexer, indexes, fetched, starting_indexes, exit))?;
        self.indexes_to_unspendable_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_unspendable_supply),
        )?;

        // Save chain state
        self.saved_chain_state.truncate_if_needed(Height::ZERO)?;
        mem::take(&mut self.chain_state)
            .into_iter()
            .for_each(|block_state| {
                self.saved_chain_state.push(block_state);
            });
        self.saved_chain_state.flush()?;

        exit.release();

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.utxos_vecs.all.vecs(),
            self.indexes_to_unspendable_supply.vecs(),
            vec![&self.height_to_unspendable_supply],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}

pub struct Vecs_ {
    starting_height: Height,
    state: CohortState,

    pub height_to_realized_cap: Option<EagerVec<Height, Dollars>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_supply: EagerVec<Height, Sats>,
    pub indexes_to_supply: ComputedValueVecsFromHeight,
    pub height_to_utxo_count: EagerVec<Height, StoredUsize>,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredUsize>,
}

impl Vecs_ {
    pub fn forced_import(
        path: &Path,
        cohort_name: Option<&str>,
        _computation: Computation,
        compressed: Compressed,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        fs::create_dir_all(path)?;

        // let prefix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{s}_{name}"));

        let suffix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{name}_{s}"));

        Ok(Self {
            starting_height: Height::ZERO,
            state: CohortState::default(),

            height_to_realized_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_cap"),
                    VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_cap"),
                    false,
                    VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_supply: EagerVec::forced_import(
                path,
                &suffix("supply"),
                VERSION + Version::ZERO,
                compressed,
            )?,
            indexes_to_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                &suffix("supply"),
                false,
                VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            height_to_utxo_count: EagerVec::forced_import(
                path,
                &suffix("utxo_count"),
                VERSION + Version::ZERO,
                compressed,
            )?,
            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                path,
                &suffix("utxo_count"),
                false,
                VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
        })
    }

    pub fn init(&mut self, starting_indexes: &Indexes) -> Height {
        self.starting_height = [
            self.height_to_supply.len(),
            // self.height_to_unspendable_supply.len(),
            self.height_to_utxo_count.len(),
            self.height_to_realized_cap
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
        ]
        .into_iter()
        .map(Height::from)
        .min()
        .unwrap()
        .min(starting_indexes.height);

        if let Some(prev_height) = self.starting_height.checked_sub(Height::new(1)) {
            self.state.supply = self
                .height_to_supply
                .into_iter()
                .unwrap_get_inner(prev_height);
            self.state.utxo_count = self
                .height_to_utxo_count
                .into_iter()
                .unwrap_get_inner(prev_height);
            if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
                self.state.realized_cap = height_to_realized_cap
                    .into_iter()
                    .unwrap_get_inner(prev_height);
            }
        }

        self.starting_height
    }

    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.height_to_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_supply.inner_version(),
            )?;

        self.height_to_utxo_count
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_utxo_count.inner_version(),
            )?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut().as_mut() {
            height_to_realized_cap.validate_computed_version_or_reset_file(
                base_version + height_to_realized_cap.inner_version(),
            )?;
        }
        Ok(())
    }

    pub fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.height_to_supply
            .forced_push_at(height, self.state.supply, exit)?;

        self.height_to_utxo_count
            .forced_push_at(height, self.state.utxo_count, exit)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            height_to_realized_cap.forced_push_at(height, self.state.realized_cap, exit)?;
        }
        Ok(())
    }

    pub fn safe_flush_height_vecs(&mut self, exit: &Exit) -> Result<()> {
        self.height_to_supply.safe_flush(exit)?;

        self.height_to_utxo_count.safe_flush(exit)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            height_to_realized_cap.safe_flush(exit)?;
        }

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_supply),
        )?;

        self.indexes_to_utxo_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_utxo_count),
        )?;

        if let Some(indexes_to_realized_cap) = self.indexes_to_realized_cap.as_mut() {
            indexes_to_realized_cap.compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_cap.as_ref().unwrap()),
            )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![
                &self.height_to_supply as &dyn AnyCollectableVec,
                &self.height_to_utxo_count,
            ],
            self.height_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_supply.vecs(),
            self.indexes_to_utxo_count.vecs(),
            self.indexes_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}

impl Clone for Vecs_ {
    fn clone(&self) -> Self {
        Self {
            starting_height: self.starting_height,
            state: CohortState::default(),

            height_to_realized_cap: self.height_to_realized_cap.clone(),
            indexes_to_realized_cap: self.indexes_to_realized_cap.clone(),
            height_to_supply: self.height_to_supply.clone(),
            indexes_to_supply: self.indexes_to_supply.clone(),
            height_to_utxo_count: self.height_to_utxo_count.clone(),
            indexes_to_utxo_count: self.indexes_to_utxo_count.clone(),
        }
    }
}
