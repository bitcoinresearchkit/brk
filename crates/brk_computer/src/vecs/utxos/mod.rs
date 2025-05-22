use std::{cmp::Ordering, collections::BTreeMap, mem, path::Path, thread};

use brk_core::{Height, InputIndex, OutputIndex, Sats};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, BaseVecIterator, CollectableVec, Compressed, Computation, EagerVec,
    GenericStoredVec, StoredIndex, StoredVec, VecIterator, Version,
};
use log::info;

use crate::states::{
    BlockState, OutputFilter, Outputs, OutputsByEpoch, OutputsByFrom, OutputsByRange,
    OutputsBySize, OutputsBySpendableType, OutputsByTerm, OutputsByType, OutputsByUpTo,
    ReceivedBlockStateData, SupplyState,
};

use super::{
    Indexes, fetched,
    grouped::{ComputedValueVecsFromHeight, StorableVecGeneatorOptions},
    indexes, transactions,
};

pub mod cohort;

const VERSION: Version = Version::new(50);

#[derive(Clone)]
pub struct Vecs {
    chain_state: StoredVec<Height, SupplyState>,

    // cointime,...
    pub height_to_unspendable_supply: EagerVec<Height, Sats>,
    pub indexes_to_unspendable_supply: ComputedValueVecsFromHeight,
    utxos_vecs: Outputs<(OutputFilter, cohort::Vecs)>,
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
            chain_state: StoredVec::forced_import(
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
                Outputs::<(OutputFilter, cohort::Vecs)>::from(Outputs {
                    all: cohort::Vecs::forced_import(
                        path,
                        None,
                        _computation,
                        compressed,
                        fetched,
                    )?,
                    by_term: OutputsByTerm {
                        short: cohort::Vecs::forced_import(
                            path,
                            Some("sth"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                        long: cohort::Vecs::forced_import(
                            path,
                            Some("lth"),
                            _computation,
                            compressed,
                            fetched,
                        )?,
                    },
                    // by_up_to: OutputsByUpTo {
                    //     _1d: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_1d"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1w: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_1w"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_1m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _2m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_2m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_3m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _4m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_4m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _5m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_5m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _6m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_6m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_1y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _2y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_2y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_3y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _4y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_4y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _5y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_5y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _6y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_6y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _7y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_7y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _8y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_8y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _10y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_10y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _15y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_15y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    // },
                    // by_from: OutputsByFrom {
                    //     _1d: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1d"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1w: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1w"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _2m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_2m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_3m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _4m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_4m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _5m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_5m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _6m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_6m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _2y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_2y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_3y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _4y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_4y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _5y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_5y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _6y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_6y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _7y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_7y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _8y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_8y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _10y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _15y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_15y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    // },
                    // by_range: OutputsByRange {
                    //     _1d_to_1w: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1d_to_1w"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1w_to_1m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1w_to_1m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1m_to_3m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1m_to_3m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3m_to_6m: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_3m_to_6m"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _6m_to_1y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_6m_to_1y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1y_to_2y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1y_to_2y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _2y_to_3y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_2y_to_3y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3y_to_4y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_3y_to_4y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _4y_to_5y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_4y_to_5y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _5y_to_7y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_5y_to_7y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _7y_to_10y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_7y_to_10y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _10y_to_15y: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10y_to_15y"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    // },
                    // by_epoch: OutputsByEpoch {
                    //     _0: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("epoch_0"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _1: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("epoch_1"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _2: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("epoch_2"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _3: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("epoch_3"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     _4: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("epoch_4"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    // },
                    // by_size: OutputsBySize {
                    //     from_1sat_to_10sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1sat_to_10sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10sats_to_100sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10sats_to_100sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100sats_to_1_000sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100sats_to_1_000sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1_000sats_to_10_000sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000sats_to_10_000sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10_000sats_to_100_000sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000sats_to_100_000sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100_000sats_to_1_000_000sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100_000sats_to_1_000_000sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1_000_000sats_to_10_000_000sats: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000_000sats_to_10_000_000sats"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10_000_000sats_to_1btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000_000sats_to_1btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1btc_to_10btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1btc_to_10btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10btc_to_100btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10btc_to_100btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100btc_to_1_000btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100btc_to_1_000btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1_000btc_to_10_000btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000btc_to_10_000btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10_000btc_to_100_000btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000btc_to_100_000btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100_000btc: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100_000btc"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    // },
                    // by_value: OutputsByValue {
                    //     up_to_1cent: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_1cent"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1c_to_10c: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1c_to_10c"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10c_to_1d: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10c_to_1d"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1d_to_10d: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1d_to_10d"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10usd_to_100usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10usd_to_100usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100usd_to_1_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100usd_to_1_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1_000usd_to_10_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000usd_to_10_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10_000usd_to_100_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000usd_to_100_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100_000usd_to_1_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100_000usd_to_1_000_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1_000_000usd_to_10_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000_000usd_to_10_000_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_10_000_000usd_to_100_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000_000usd_to_100_000_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_100_000_000usd_to_1_000_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100_000_000usd_to_1_000_000_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     from_1_000_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000_000_000usd"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    // },
                    // by_spendable_type: OutputsBySpendableType {
                    //     p2pk65: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2pk65"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2pk33: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2pk33"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2pkh: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2pkh"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2ms: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2ms"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2sh: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2sh"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     // op_return: cohort::Vecs::forced_import(
                    //     //     path,
                    //     //     Some("op_return"),
                    //     //     _computation,
                    //     //     compressed,
                    //     //     fetched,
                    //     // )?,
                    //     p2wpkh: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2wpkh"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2wsh: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2wsh"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2tr: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2tr"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     p2a: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("p2a"),
                    //         _computation,
                    //         compressed,
                    //         fetched,
                    //     )?,
                    //     // empty: cohort::Vecs::forced_import(
                    //     //     path,
                    //     //     Some("empty"),
                    //     //     _computation,
                    //     //     compressed,
                    //     //     fetched,
                    //     // )?,
                    //     // unknown: cohort::Vecs::forced_import(
                    //     //     path,
                    //     //     Some("unknown"),
                    //     //     _computation,
                    //     //     compressed,
                    //     //     fetched,
                    //     // )?,
                    // },
                })
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

        let mut flat_vecs_ = self.utxos_vecs.as_mut_vec();

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
            .try_for_each(|(_, v)| v.validate_computed_versions(base_version))?;
        self.height_to_unspendable_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_unspendable_supply.inner_version(),
            )?;

        let mut chain_state: Vec<BlockState>;
        let mut chain_state_starting_height = Height::from(self.chain_state.len());

        let stateful_starting_height = match flat_vecs_
            .iter_mut()
            .map(|(_, v)| v.init(starting_indexes))
            .min()
            .unwrap_or_default()
            .min(chain_state_starting_height)
            .cmp(&chain_state_starting_height)
        {
            Ordering::Greater => unreachable!(),
            Ordering::Equal => {
                chain_state = self
                    .chain_state
                    .collect_range(None, None)?
                    .into_iter()
                    .enumerate()
                    .map(|(height, supply)| {
                        let height = Height::from(height);
                        let timestamp = height_to_timestamp_fixed_iter.unwrap_get_inner(height);
                        let price = height_to_close_iter
                            .as_mut()
                            .map(|i| *i.unwrap_get_inner(height));
                        BlockState {
                            timestamp,
                            price,
                            supply,
                        }
                    })
                    .collect::<Vec<_>>();
                chain_state_starting_height
            }
            Ordering::Less => {
                // todo!("rollback instead");
                chain_state = vec![];
                chain_state_starting_height = Height::ZERO;
                Height::ZERO
            }
        };
        if stateful_starting_height.is_zero() {
            info!("Starting processing utxos from the start");
        }
        let starting_height =
            stateful_starting_height.min(Height::from(self.height_to_unspendable_supply.len()));

        let mut unspendable_supply = if let Some(prev_height) = starting_height.decremented() {
            self.height_to_unspendable_supply
                .into_iter()
                .unwrap_get_inner(prev_height)
        } else {
            Sats::ZERO
        };

        let mut height = Height::ZERO;
        (starting_height.unwrap_to_usize()..height_to_first_outputindex_iter.len())
            .map(Height::from)
            .try_for_each(|_height| -> color_eyre::Result<()> {
                height = _height;

                info!("Processing utxo set at {height}...");

                let timestamp = height_to_timestamp_fixed_iter.unwrap_get_inner(height);
                let price = height_to_close_iter
                    .as_mut()
                    .map(|i| *i.unwrap_get_inner(height));
                let first_outputindex = height_to_first_outputindex_iter
                    .unwrap_get_inner(height)
                    .unwrap_to_usize();
                let first_inputindex = height_to_first_inputindex_iter
                    .unwrap_get_inner(height)
                    .unwrap_to_usize();
                let output_count = height_to_output_count_iter.unwrap_get_inner(height);
                let input_count = height_to_input_count_iter.unwrap_get_inner(height);

                // let sent_state = SentState::default();
                // let received_state = ReceivedState::default();
                // let realized_state = RealizedState::default();

                let (mut height_to_sent, mut received) = thread::scope(|s| {
                    if chain_state_starting_height <= height {
                        s.spawn(|| {
                            self.utxos_vecs
                                .tick_tock_next_block(&chain_state, timestamp);
                        });
                    }

                    let sent_handle = s.spawn(|| {
                        let mut txindex_to_height = BTreeMap::new();
                        let mut height_to_sent: BTreeMap<
                            Height,
                            OutputsByType<(SupplyState, Vec<Sats>)>,
                        > = BTreeMap::new();

                        // Skip coinbase
                        (first_inputindex + 1..first_inputindex + *input_count)
                            .map(InputIndex::from)
                            .for_each(|inputindex| {
                                let outputindex =
                                    inputindex_to_outputindex_iter.unwrap_get_inner(inputindex);

                                let value = outputindex_to_value_iter.unwrap_get_inner(outputindex);

                                let input_type =
                                    outputindex_to_outputtype_iter.unwrap_get_inner(outputindex);

                                let input_txindex =
                                    outputindex_to_txindex_iter.unwrap_get_inner(outputindex);

                                let input_height =
                                    *txindex_to_height.entry(input_txindex).or_insert_with(|| {
                                        txindex_to_height_iter.unwrap_get_inner(input_txindex)
                                    });

                                let input_data = height_to_sent.entry(input_height).or_default();

                                let (sent_supply, sats_vec) = input_data.get_mut(input_type);

                                sent_supply.utxos += 1;
                                sent_supply.value += value;
                                sats_vec.push(value);
                            });

                        height_to_sent
                    });

                    let received_handle = s.spawn(|| {
                        let mut by_type: OutputsByType<(SupplyState, Vec<Sats>)> =
                            OutputsByType::default();

                        (first_outputindex..first_outputindex + *output_count)
                            .map(OutputIndex::from)
                            .for_each(|outputindex| {
                                let value =
                                    outputindex_to_value_iter_2.unwrap_get_inner(outputindex);

                                let output_type =
                                    outputindex_to_outputtype_iter_2.unwrap_get_inner(outputindex);

                                let (sent_supply, sats_vec) = by_type.get_mut(output_type);

                                sent_supply.value += value;
                                sent_supply.utxos += 1;
                                sats_vec.push(value);
                            });

                        by_type
                    });

                    (sent_handle.join().unwrap(), received_handle.join().unwrap())
                });

                unspendable_supply += received
                    .unspendable
                    .as_vec()
                    .into_iter()
                    .map(|state| state.0.value)
                    .sum::<Sats>()
                    + height_to_unclaimed_rewards_iter.unwrap_get_inner(height);

                if height == Height::new(0) {
                    received.spendable.p2pk65.1.remove(0);
                    received.spendable.p2pk65.0.utxos -= 1;
                    received.spendable.p2pk65.0.value -= Sats::FIFTY_BTC;
                    unspendable_supply += Sats::FIFTY_BTC;
                } else {
                    // Need to destroy invalid coinbases due to duplicate txids
                    if height == Height::new(91_842) || height == Height::new(91_880) {
                        let entry = if height == Height::new(91_842) {
                            height_to_sent.entry(Height::new(91_812)).or_default()
                        } else {
                            height_to_sent.entry(Height::new(91_722)).or_default()
                        };
                        entry.spendable.p2pk65.0.value += Sats::FIFTY_BTC;
                        entry.spendable.p2pk65.0.utxos += 1;
                        entry.spendable.p2pk65.1.push(Sats::FIFTY_BTC);
                    }
                };

                if chain_state_starting_height <= height {
                    // RECEIVE

                    // Push current block state before processing sends and receives
                    chain_state.push(BlockState::from(ReceivedBlockStateData {
                        received: &received,
                        price,
                        timestamp,
                    }));

                    self.utxos_vecs.receive(received, height, price);

                    // ---

                    // SEND

                    // Apply sent to
                    height_to_sent
                        .iter()
                        .for_each(|(height, sent_data_by_type)| {
                            let block_state =
                                chain_state.get_mut(height.unwrap_to_usize()).unwrap();
                            sent_data_by_type
                                .as_vec()
                                .into_iter()
                                .for_each(|(supply, _)| {
                                    block_state.supply -= supply.clone();
                                });
                        });

                    self.utxos_vecs.send(height_to_sent, chain_state.as_slice());
                } else {
                    panic!("temp, just making sure")
                }

                // received.
                // 2. push received
                // 3. subtract sent

                // 4. check what's the point with wth is after this message

                // let (sent, realized_cap_destroyed) = height_to_sent
                //     .par_iter()
                //     .map(|(_, (_, dollars, sats, _))| {
                //         let dollars = *dollars;
                //         let sats = *sats;
                //         (sats, dollars * Bitcoin::from(sats))
                //     })
                //     .reduce(
                //         || (Sats::ZERO, Dollars::ZERO),
                //         |acc, (sats, dollars)| (acc.0 + sats, acc.1 + dollars),
                //     );

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
                    .as_mut_vec()
                    .iter_mut()
                    .try_for_each(|(_, v)| v.forced_pushed_at(height, exit))?;

                self.height_to_unspendable_supply.forced_push_at(
                    height,
                    unspendable_supply,
                    exit,
                )?;

                Ok(())
            })?;

        exit.block();

        let mut flat_vecs_ = self.utxos_vecs.as_mut_vec();

        // Flush rest of values
        flat_vecs_
            .iter_mut()
            .try_for_each(|(_, v)| v.safe_flush_height_vecs(exit))?;
        self.height_to_unspendable_supply.safe_flush(exit)?;

        // Compute other vecs from height vecs
        flat_vecs_.iter_mut().try_for_each(|(_, v)| {
            v.compute_rest(indexer, indexes, fetched, starting_indexes, exit)
        })?;
        self.indexes_to_unspendable_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_unspendable_supply),
        )?;

        // Save chain state
        self.chain_state.truncate_if_needed(Height::ZERO)?;
        mem::take(&mut chain_state)
            .into_iter()
            .for_each(|block_state| {
                self.chain_state.push(block_state.supply);
            });
        self.chain_state.flush()?;

        exit.release();

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.utxos_vecs
                .vecs()
                .into_iter()
                .flat_map(|v| v.vecs())
                .collect::<Vec<_>>(),
            self.indexes_to_unspendable_supply.vecs(),
            vec![&self.height_to_unspendable_supply],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
