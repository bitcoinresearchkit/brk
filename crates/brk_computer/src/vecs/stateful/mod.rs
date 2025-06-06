use std::{cmp::Ordering, collections::BTreeMap, path::Path, thread};

use brk_core::{DateIndex, Height, InputIndex, OutputIndex, OutputType, Result, Sats, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, BaseVecIterator, CollectableVec, Computation, EagerVec, Format,
    GenericStoredVec, StoredIndex, StoredVec, UnsafeSlice, VecIterator,
};
use fjall::TransactionalKeyspace;
use log::info;
use outputs::OutputCohorts;
use rayon::prelude::*;

use brk_state::{
    BlockState, OutputFilter, Outputs, OutputsByDateRange, OutputsByEpoch, OutputsByFromDate,
    OutputsByFromSize, OutputsBySizeRange, OutputsBySpendableType, OutputsByTerm,
    OutputsByUpToDate, OutputsByUpToSize, SupplyState, Transacted,
};

use crate::vecs::market;

use super::{
    Indexes, fetched,
    grouped::{ComputedValueVecsFromHeight, StorableVecGeneatorOptions},
    indexes, transactions,
};

pub mod cohort;
mod outputs;

const VERSION: Version = Version::new(4);

#[derive(Clone)]
pub struct Vecs {
    chain_state: StoredVec<Height, SupplyState>,

    // cointime,...
    pub height_to_unspendable_supply: EagerVec<Height, Sats>,
    pub indexes_to_unspendable_supply: ComputedValueVecsFromHeight,
    pub height_to_opreturn_supply: EagerVec<Height, Sats>,
    pub indexes_to_opreturn_supply: ComputedValueVecsFromHeight,
    utxos_vecs: Outputs<(OutputFilter, cohort::Vecs)>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        _computation: Computation,
        format: Format,
        fetched: Option<&fetched::Vecs>,
        keyspace: &TransactionalKeyspace,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        let mut root_path = path.to_owned();
        root_path.pop();
        let states_path = root_path.join("states");
        root_path.pop();
        let stores_path = root_path.join("stores");

        Ok(Self {
            chain_state: StoredVec::forced_import(
                &states_path,
                "chain",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,

            height_to_unspendable_supply: EagerVec::forced_import(
                path,
                "unspendable_supply",
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_unspendable_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "unspendable_supply",
                false,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            height_to_opreturn_supply: EagerVec::forced_import(
                path,
                "opreturn_supply",
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_opreturn_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "opreturn_supply",
                false,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            utxos_vecs: {
                Outputs::<(OutputFilter, cohort::Vecs)>::from(Outputs {
                    all: cohort::Vecs::forced_import(
                        path,
                        None,
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        keyspace,
                        &stores_path,
                        false,
                    )?,
                    by_term: OutputsByTerm {
                        short: cohort::Vecs::forced_import(
                            path,
                            Some("sth"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        long: cohort::Vecs::forced_import(
                            path,
                            Some("lth"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_up_to_date: OutputsByUpToDate {
                        _1d: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_1d"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1w: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_1w"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1m: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_1m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2m: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_2m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3m: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_3m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4m: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_4m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _5m: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_5m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _6m: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_6m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_1y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_2y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_3y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_4y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _5y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_5y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _6y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_6y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _7y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_7y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _8y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_8y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _10y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_10y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _15y: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_15y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_from_date: OutputsByFromDate {
                        _1d: cohort::Vecs::forced_import(
                            path,
                            Some("from_1d"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1w: cohort::Vecs::forced_import(
                            path,
                            Some("from_1w"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1m: cohort::Vecs::forced_import(
                            path,
                            Some("from_1m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2m: cohort::Vecs::forced_import(
                            path,
                            Some("from_2m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3m: cohort::Vecs::forced_import(
                            path,
                            Some("from_3m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4m: cohort::Vecs::forced_import(
                            path,
                            Some("from_4m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _5m: cohort::Vecs::forced_import(
                            path,
                            Some("from_5m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _6m: cohort::Vecs::forced_import(
                            path,
                            Some("from_6m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1y: cohort::Vecs::forced_import(
                            path,
                            Some("from_1y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2y: cohort::Vecs::forced_import(
                            path,
                            Some("from_2y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3y: cohort::Vecs::forced_import(
                            path,
                            Some("from_3y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4y: cohort::Vecs::forced_import(
                            path,
                            Some("from_4y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _5y: cohort::Vecs::forced_import(
                            path,
                            Some("from_5y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _6y: cohort::Vecs::forced_import(
                            path,
                            Some("from_6y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _7y: cohort::Vecs::forced_import(
                            path,
                            Some("from_7y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _8y: cohort::Vecs::forced_import(
                            path,
                            Some("from_8y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _10y: cohort::Vecs::forced_import(
                            path,
                            Some("from_10y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _15y: cohort::Vecs::forced_import(
                            path,
                            Some("from_15y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_date_range: OutputsByDateRange {
                        start_to_1d: cohort::Vecs::forced_import(
                            path,
                            Some("start_to_1d"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1d_to_1w: cohort::Vecs::forced_import(
                            path,
                            Some("from_1d_to_1w"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1w_to_1m: cohort::Vecs::forced_import(
                            path,
                            Some("from_1w_to_1m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1m_to_2m: cohort::Vecs::forced_import(
                            path,
                            Some("from_1m_to_2m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2m_to_3m: cohort::Vecs::forced_import(
                            path,
                            Some("from_2m_to_3m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3m_to_4m: cohort::Vecs::forced_import(
                            path,
                            Some("from_3m_to_4m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4m_to_5m: cohort::Vecs::forced_import(
                            path,
                            Some("from_4m_to_5m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _5m_to_6m: cohort::Vecs::forced_import(
                            path,
                            Some("from_5m_to_6m"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _6m_to_1y: cohort::Vecs::forced_import(
                            path,
                            Some("from_6m_to_1y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1y_to_2y: cohort::Vecs::forced_import(
                            path,
                            Some("from_1y_to_2y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2y_to_3y: cohort::Vecs::forced_import(
                            path,
                            Some("from_2y_to_3y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3y_to_4y: cohort::Vecs::forced_import(
                            path,
                            Some("from_3y_to_4y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4y_to_5y: cohort::Vecs::forced_import(
                            path,
                            Some("from_4y_to_5y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _5y_to_6y: cohort::Vecs::forced_import(
                            path,
                            Some("from_5y_to_6y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _6y_to_7y: cohort::Vecs::forced_import(
                            path,
                            Some("from_6y_to_7y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _7y_to_8y: cohort::Vecs::forced_import(
                            path,
                            Some("from_7y_to_8y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _8y_to_10y: cohort::Vecs::forced_import(
                            path,
                            Some("from_8y_to_10y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _10y_to_15y: cohort::Vecs::forced_import(
                            path,
                            Some("from_10y_to_15y"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _15y_to_end: cohort::Vecs::forced_import(
                            path,
                            Some("from_15y_to_end"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_epoch: OutputsByEpoch {
                        _0: cohort::Vecs::forced_import(
                            path,
                            Some("epoch_0"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1: cohort::Vecs::forced_import(
                            path,
                            Some("epoch_1"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _2: cohort::Vecs::forced_import(
                            path,
                            Some("epoch_2"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _3: cohort::Vecs::forced_import(
                            path,
                            Some("epoch_3"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _4: cohort::Vecs::forced_import(
                            path,
                            Some("epoch_4"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_size_range: OutputsBySizeRange {
                        _0sats: cohort::Vecs::forced_import(
                            path,
                            Some("0sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_1sat_to_10sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_1sat_to_10sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_10sats_to_100sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_10sats_to_100sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_100sats_to_1_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_100sats_to_1_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_1_000sats_to_10_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_1_000sats_to_10_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_10_000sats_to_100_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_10_000sats_to_100_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_100_000sats_to_1_000_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_100_000sats_to_1_000_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_1_000_000sats_to_10_000_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_1_000_000sats_to_10_000_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_10_000_000sats_to_1btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_10_000_000sats_to_1btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_1btc_to_10btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_1btc_to_10btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_10btc_to_100btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_10btc_to_100btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_100btc_to_1_000btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_100btc_to_1_000btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_1_000btc_to_10_000btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_1_000btc_to_10_000btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_10_000btc_to_100_000btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_10_000btc_to_100_000btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        from_100_000btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_100_000btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_up_to_size: OutputsByUpToSize {
                        _1_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_1_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _10_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_10_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1btc: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_1btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _10btc: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_10btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _100btc: cohort::Vecs::forced_import(
                            path,
                            Some("up_to_100btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    by_from_size: OutputsByFromSize {
                        _1_000sats: cohort::Vecs::forced_import(
                            path,
                            Some("from_1_000sats"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _1btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_1btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _10btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_10btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        _100btc: cohort::Vecs::forced_import(
                            path,
                            Some("from_100btc"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                    // by_value: OutputsByValue {
                    //     up_to_1cent: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("up_to_1cent"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_1c_to_10c: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1c_to_10c"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_10c_to_1d: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10c_to_1d"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_1d_to_10d: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1d_to_10d"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_10usd_to_100usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10usd_to_100usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_100usd_to_1_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100usd_to_1_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_1_000usd_to_10_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000usd_to_10_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_10_000usd_to_100_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000usd_to_100_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_100_000usd_to_1_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100_000usd_to_1_000_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_1_000_000usd_to_10_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000_000usd_to_10_000_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_10_000_000usd_to_100_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_10_000_000usd_to_100_000_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_100_000_000usd_to_1_000_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_100_000_000usd_to_1_000_000_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    //     from_1_000_000_000usd: cohort::Vecs::forced_import(
                    //         path,
                    //         Some("from_1_000_000_000usd"),
                    //         _computation,
                    //         format,
                    //         fetched,
                    //     )?,
                    // },
                    by_type: OutputsBySpendableType {
                        p2pk65: cohort::Vecs::forced_import(
                            path,
                            Some("p2pk65"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2pk33: cohort::Vecs::forced_import(
                            path,
                            Some("p2pk33"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2pkh: cohort::Vecs::forced_import(
                            path,
                            Some("p2pkh"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2ms: cohort::Vecs::forced_import(
                            path,
                            Some("p2ms"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2sh: cohort::Vecs::forced_import(
                            path,
                            Some("p2sh"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2wpkh: cohort::Vecs::forced_import(
                            path,
                            Some("p2wpkh"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2wsh: cohort::Vecs::forced_import(
                            path,
                            Some("p2wsh"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2tr: cohort::Vecs::forced_import(
                            path,
                            Some("p2tr"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        p2a: cohort::Vecs::forced_import(
                            path,
                            Some("p2a"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        empty: cohort::Vecs::forced_import(
                            path,
                            Some("empty"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                        unknown: cohort::Vecs::forced_import(
                            path,
                            Some("unknown"),
                            _computation,
                            format,
                            version + VERSION + Version::ZERO,
                            fetched,
                            keyspace,
                            &stores_path,
                            true,
                        )?,
                    },
                })
            },
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transactions: &transactions::Vecs,
        fetched: Option<&fetched::Vecs>,
        market: &market::Vecs,
        // Must take ownership as its indexes will be updated for this specific function
        mut starting_indexes: Indexes,
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
            .unwrap();
        let height_to_close = fetched
            .as_ref()
            .map(|fetched| &fetched.chainindexes_to_close.height);
        let dateindex_to_close = fetched
            .as_ref()
            .map(|fetched| fetched.timeindexes_to_close.dateindex.as_ref().unwrap());
        let height_to_date_fixed = &indexes.height_to_date_fixed;
        let dateindex_to_first_height = &indexes.dateindex_to_first_height;
        let dateindex_to_height_count = &indexes.dateindex_to_height_count;

        let inputindex_to_outputindex_mmap = inputindex_to_outputindex.mmap().load();
        let outputindex_to_value_mmap = outputindex_to_value.mmap().load();
        let outputindex_to_outputtype_mmap = outputindex_to_outputtype.mmap().load();
        let outputindex_to_txindex_mmap = outputindex_to_txindex.mmap().load();
        let txindex_to_height_mmap = txindex_to_height.mmap().load();

        let mut height_to_first_outputindex_iter = height_to_first_outputindex.into_iter();
        let mut height_to_first_inputindex_iter = height_to_first_inputindex.into_iter();
        let mut height_to_output_count_iter = height_to_output_count.into_iter();
        let mut height_to_input_count_iter = height_to_input_count.into_iter();
        // let mut outputindex_to_value_iter_2 = outputindex_to_value.into_iter();
        let mut height_to_close_iter = height_to_close.as_ref().map(|v| v.into_iter());
        // let mut outputindex_to_outputtype_iter_2 = outputindex_to_outputtype.into_iter();
        let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
        let mut height_to_timestamp_fixed_iter = height_to_timestamp_fixed.into_iter();
        let mut dateindex_to_close_iter = dateindex_to_close.as_ref().map(|v| v.into_iter());
        let mut height_to_date_fixed_iter = height_to_date_fixed.into_iter();
        let mut dateindex_to_first_height_iter = dateindex_to_first_height.into_iter();
        let mut dateindex_to_height_count_iter = dateindex_to_height_count.into_iter();

        let mut separate_utxo_vecs = self.utxos_vecs.as_mut_separate_vecs();

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
                .map_or(Version::ZERO, |v| v.version())
            + dateindex_to_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + height_to_date_fixed.version()
            + dateindex_to_first_height.version()
            + dateindex_to_height_count.version();

        separate_utxo_vecs
            .par_iter_mut()
            .try_for_each(|(_, v)| v.validate_computed_versions(base_version))?;
        self.height_to_unspendable_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_unspendable_supply.inner_version(),
            )?;
        self.height_to_opreturn_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_opreturn_supply.inner_version(),
            )?;

        let mut chain_state: Vec<BlockState>;
        let mut chain_state_starting_height = Height::from(self.chain_state.len());

        let stateful_starting_height = match separate_utxo_vecs
            .par_iter_mut()
            .map(|(_, v)| v.starting_height())
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
            separate_utxo_vecs
                .par_iter_mut()
                .try_for_each(|(_, v)| v.state.price_to_amount.reset_partition())?;
        }
        let starting_height = starting_indexes
            .height
            .min(stateful_starting_height)
            .min(Height::from(self.height_to_unspendable_supply.len()))
            .min(Height::from(self.height_to_opreturn_supply.len()));

        if starting_height == Height::from(height_to_date_fixed.len()) {
            return Ok(());
        }

        // ---
        // INIT
        // ---

        separate_utxo_vecs
            .par_iter_mut()
            .for_each(|(_, v)| v.init(starting_height));

        let mut unspendable_supply = if let Some(prev_height) = starting_height.decremented() {
            self.height_to_unspendable_supply
                .into_iter()
                .unwrap_get_inner(prev_height)
        } else {
            Sats::ZERO
        };
        let mut opreturn_supply = if let Some(prev_height) = starting_height.decremented() {
            self.height_to_opreturn_supply
                .into_iter()
                .unwrap_get_inner(prev_height)
        } else {
            Sats::ZERO
        };

        let mut height = starting_height;
        starting_indexes.update_from_height(height, indexes);

        (height.unwrap_to_usize()..height_to_first_outputindex_iter.len())
            .map(Height::from)
            .try_for_each(|_height| -> color_eyre::Result<()> {
                height = _height;

                self.utxos_vecs
                    .as_mut_separate_vecs()
                    .iter_mut()
                    .for_each(|(_, v)| v.state.reset_single_iteration_values());

                info!("Processing chain at {height}...");

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

                let (mut height_to_sent, mut received) = thread::scope(|s| {
                    if chain_state_starting_height <= height {
                        s.spawn(|| {
                            self.utxos_vecs
                                .tick_tock_next_block(&chain_state, timestamp);
                        });
                    }

                    let sent_handle = s.spawn(|| {
                        // Skip coinbase
                        (first_inputindex + 1..first_inputindex + *input_count)
                            .into_par_iter()
                            .map(InputIndex::from)
                            .map(|inputindex| {
                                let outputindex = inputindex_to_outputindex
                                    .get_or_read(inputindex, &inputindex_to_outputindex_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                let value = outputindex_to_value
                                    .get_or_read(outputindex, &outputindex_to_value_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                let input_type = outputindex_to_outputtype
                                    .get_or_read(outputindex, &outputindex_to_outputtype_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                // dbg!(input_type);

                                if input_type.is_unspendable() {
                                    unreachable!()
                                }

                                let input_txindex = outputindex_to_txindex
                                    .get_or_read(outputindex, &outputindex_to_txindex_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                let height = txindex_to_height
                                    .get_or_read(input_txindex, &txindex_to_height_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                (height, value, input_type)
                            })
                            .fold(
                                BTreeMap::<Height, Transacted>::default,
                                |mut tree, (height, value, input_type)| {
                                    tree.entry(height).or_default().iterate(value, input_type);
                                    tree
                                },
                            )
                            .reduce(BTreeMap::<Height, Transacted>::default, |first, second| {
                                let (mut source, to_consume) = if first.len() > second.len() {
                                    (first, second)
                                } else {
                                    (second, first)
                                };
                                to_consume.into_iter().for_each(|(k, v)| {
                                    *source.entry(k).or_default() += v;
                                });
                                source
                            })
                    });

                    let received_handle = s.spawn(|| {
                        (first_outputindex..first_outputindex + *output_count)
                            .into_par_iter()
                            .map(OutputIndex::from)
                            .map(|outputindex| {
                                let value = outputindex_to_value
                                    .get_or_read(outputindex, &outputindex_to_value_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                let output_type = outputindex_to_outputtype
                                    .get_or_read(outputindex, &outputindex_to_outputtype_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_inner();

                                (value, output_type)
                            })
                            .fold(
                                Transacted::default,
                                |mut transacted, (value, output_type)| {
                                    transacted.iterate(value, output_type);
                                    transacted
                                },
                            )
                            .reduce(Transacted::default, |acc, transacted| acc + transacted)
                    });

                    (sent_handle.join().unwrap(), received_handle.join().unwrap())
                });

                unspendable_supply += received
                    .by_type
                    .unspendable
                    .as_vec()
                    .into_iter()
                    .map(|state| state.value)
                    .sum::<Sats>()
                    + height_to_unclaimed_rewards_iter.unwrap_get_inner(height);

                opreturn_supply += received.by_type.unspendable.opreturn.value;

                if height == Height::new(0) {
                    received = Transacted::default();
                    unspendable_supply += Sats::FIFTY_BTC;
                } else if height == Height::new(91_842) || height == Height::new(91_880) {
                    // Need to destroy invalid coinbases due to duplicate txids
                    if height == Height::new(91_842) {
                        height_to_sent.entry(Height::new(91_812)).or_default()
                    } else {
                        height_to_sent.entry(Height::new(91_722)).or_default()
                    }
                    .iterate(Sats::FIFTY_BTC, OutputType::P2PK65);
                };

                if chain_state_starting_height <= height {
                    // Push current block state before processing sends and receives
                    chain_state.push(BlockState {
                        supply: received.spendable_supply.clone(),
                        price,
                        timestamp,
                    });

                    self.utxos_vecs.receive(received, height, price);

                    let unsafe_chain_state = UnsafeSlice::new(&mut chain_state);

                    height_to_sent.par_iter().for_each(|(height, sent)| unsafe {
                        (*unsafe_chain_state.get(height.unwrap_to_usize())).supply -=
                            &sent.spendable_supply;
                    });

                    self.utxos_vecs.send(height_to_sent, chain_state.as_slice());
                } else {
                    dbg!(chain_state_starting_height, height);
                    panic!("temp, just making sure")
                }

                let mut separate_utxo_vecs = self.utxos_vecs.as_mut_separate_vecs();

                separate_utxo_vecs
                    .iter_mut()
                    .try_for_each(|(_, v)| v.forced_pushed_at(height, exit))?;

                self.height_to_unspendable_supply.forced_push_at(
                    height,
                    unspendable_supply,
                    exit,
                )?;

                self.height_to_opreturn_supply
                    .forced_push_at(height, opreturn_supply, exit)?;

                let date = height_to_date_fixed_iter.unwrap_get_inner(height);
                let dateindex = DateIndex::try_from(date).unwrap();
                let date_first_height = dateindex_to_first_height_iter.unwrap_get_inner(dateindex);
                let date_height_count = dateindex_to_height_count_iter.unwrap_get_inner(dateindex);
                let is_date_last_height = date_first_height
                    + Height::from(date_height_count).decremented().unwrap()
                    == height;
                let date_price = dateindex_to_close_iter
                    .as_mut()
                    .map(|v| is_date_last_height.then(|| *v.unwrap_get_inner(dateindex)));

                separate_utxo_vecs.par_iter_mut().try_for_each(|(_, v)| {
                    v.compute_then_force_push_unrealized_states(
                        height,
                        price,
                        is_date_last_height.then_some(dateindex),
                        date_price,
                        exit,
                    )
                })?;

                if height != Height::ZERO && height.unwrap_to_usize() % 20_000 == 0 {
                    info!("Flushing...");
                    exit.block();
                    self.flush_states(height, &chain_state, exit)?;
                    exit.release();
                }

                Ok(())
            })?;

        exit.block();

        info!("Flushing...");

        self.flush_states(height, &chain_state, exit)?;

        info!("Computing overlaping...");

        self.utxos_vecs
            .compute_overlaping_vecs(&starting_indexes, exit)?;

        info!("Computing rest part 1...");

        self.utxos_vecs
            .as_mut_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| {
                v.compute_rest_part1(indexer, indexes, fetched, &starting_indexes, exit)
            })?;

        info!("Computing rest part 2...");

        let height_to_supply = self.utxos_vecs.all.1.height_to_supply_value.bitcoin.clone();
        let dateindex_to_supply = self
            .utxos_vecs
            .all
            .1
            .indexes_to_supply
            .bitcoin
            .dateindex
            .clone();
        let height_to_realized_cap = self.utxos_vecs.all.1.height_to_realized_cap.clone();

        self.utxos_vecs
            .as_mut_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| {
                v.compute_rest_part2(
                    indexer,
                    indexes,
                    fetched,
                    &starting_indexes,
                    market,
                    &height_to_supply,
                    dateindex_to_supply.as_ref().unwrap(),
                    height_to_realized_cap.as_ref(),
                    exit,
                )
            })?;
        self.indexes_to_unspendable_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            &starting_indexes,
            exit,
            Some(&self.height_to_unspendable_supply),
        )?;
        self.indexes_to_opreturn_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            &starting_indexes,
            exit,
            Some(&self.height_to_opreturn_supply),
        )?;

        exit.release();

        Ok(())
    }

    fn flush_states(
        &mut self,
        height: Height,
        chain_state: &[BlockState],
        exit: &Exit,
    ) -> Result<()> {
        self.utxos_vecs
            .as_mut_separate_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| v.safe_flush_stateful_vecs(height, exit))?;
        self.height_to_unspendable_supply.safe_flush(exit)?;
        self.height_to_opreturn_supply.safe_flush(exit)?;

        self.chain_state.truncate_if_needed(Height::ZERO)?;
        chain_state.iter().for_each(|block_state| {
            self.chain_state.push(block_state.supply.clone());
        });
        self.chain_state.flush()?;

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
            self.indexes_to_opreturn_supply.vecs(),
            vec![
                &self.height_to_unspendable_supply,
                &self.height_to_opreturn_supply,
            ],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
