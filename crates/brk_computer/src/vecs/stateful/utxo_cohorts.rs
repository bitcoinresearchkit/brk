use std::{collections::BTreeMap, ops::ControlFlow, path::Path};

use brk_core::{
    CheckedSub, Dollars, GroupFilter, GroupedByDateRange, GroupedByEpoch, GroupedByFromDate,
    GroupedByFromSize, GroupedBySizeRange, GroupedBySpendableType, GroupedByTerm,
    GroupedByUpToDate, GroupedByUpToSize, HalvingEpoch, Height, Result, Timestamp, UTXOGroups,
    Version,
};
use brk_exit::Exit;
use brk_vec::{Computation, Format, StoredIndex};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;

use crate::{
    states::{BlockState, Transacted},
    vecs::{Indexes, fetched, stateful::r#trait::DynCohortVecs},
};

use super::{r#trait::CohortVecs, utxo_cohort};

const VERSION: Version = Version::new(0);

#[derive(Clone, Deref, DerefMut)]
pub struct Vecs(UTXOGroups<(GroupFilter, utxo_cohort::Vecs)>);

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        _computation: Computation,
        format: Format,
        fetched: Option<&fetched::Vecs>,
        states_path: &Path,
    ) -> color_eyre::Result<Self> {
        Ok(Self(
            UTXOGroups {
                all: utxo_cohort::Vecs::forced_import(
                    path,
                    None,
                    _computation,
                    format,
                    version + VERSION + Version::ZERO,
                    fetched,
                    states_path,
                    false,
                )?,
                by_term: GroupedByTerm {
                    short: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("sth"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    long: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("lth"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_up_to_date: GroupedByUpToDate {
                    _1d: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1d"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1w: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1w"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_2m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_3m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_4m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _5m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_5m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _6m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_6m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_2y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_3y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_4y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _5y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_5y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _6y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_6y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _7y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_7y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _8y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_8y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_10y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _15y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_15y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_from_date: GroupedByFromDate {
                    _1d: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1d"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1w: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1w"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_2m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_3m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_4m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _5m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_5m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _6m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_6m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_2y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_3y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_4y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _5y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_5y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _6y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_6y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _7y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_7y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _8y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_8y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _15y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_15y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_date_range: GroupedByDateRange {
                    start_to_1d: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("start_to_1d"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1d_to_1w: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1d_to_1w"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1w_to_1m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1w_to_1m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1m_to_2m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1m_to_2m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2m_to_3m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_2m_to_3m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3m_to_4m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_3m_to_4m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4m_to_5m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_4m_to_5m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _5m_to_6m: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_5m_to_6m"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _6m_to_1y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_6m_to_1y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1y_to_2y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1y_to_2y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2y_to_3y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_2y_to_3y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3y_to_4y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_3y_to_4y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4y_to_5y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_4y_to_5y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _5y_to_6y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_5y_to_6y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _6y_to_7y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_6y_to_7y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _7y_to_8y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_7y_to_8y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _8y_to_10y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_8y_to_10y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10y_to_15y: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10y_to_15y"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _15y_to_end: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_15y_to_end"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_epoch: GroupedByEpoch {
                    _0: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("epoch_0"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("epoch_1"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _2: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("epoch_2"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _3: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("epoch_3"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _4: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("epoch_4"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_size_range: GroupedBySizeRange {
                    _0sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("0sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1sat_to_10sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1sat_to_10sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10sats_to_100sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10sats_to_100sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100sats_to_1_000sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100sats_to_1_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1_000sats_to_10_000sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1_000sats_to_10_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10_000sats_to_100_000sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10_000sats_to_100_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100_000sats_to_1_000_000sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100_000sats_to_1_000_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1_000_000sats_to_10_000_000sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1_000_000sats_to_10_000_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10_000_000sats_to_1btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10_000_000sats_to_1btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1btc_to_10btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1btc_to_10btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10btc_to_100btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10btc_to_100btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100btc_to_1_000btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100btc_to_1_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1_000btc_to_10_000btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1_000btc_to_10_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10_000btc_to_100_000btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10_000btc_to_100_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100_000btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_up_to_size: GroupedByUpToSize {
                    _10sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_10sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_100sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1k_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1k_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10k_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_10k_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100k_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_100k_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1m_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1m_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10m_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_10m_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_10btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_100btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1k_btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_1k_btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10k_btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_10k_btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100k_btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("up_to_100k_btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_from_size: GroupedByFromSize {
                    _1sat: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1sat"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1k_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1k_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10k_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10k_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100k_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100k_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1m_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1m_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10m_sats: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10m_sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_100btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1k_btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_1k_btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10k_btc: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("from_10k_btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_type: GroupedBySpendableType {
                    p2pk65: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2pk65"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2pk33: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2pk33"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2pkh: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2pkh"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2ms: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2ms"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2sh: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2sh"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2wpkh: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2wpkh"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2wsh: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2wsh"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2tr: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2tr"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    p2a: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("p2a"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    empty: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("empty"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    unknown: utxo_cohort::Vecs::forced_import(
                        path,
                        Some("unknown"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
            }
            .into(),
        ))
    }

    pub fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;

        self.by_date_range
            .as_mut_vec()
            .into_par_iter()
            .for_each(|(filter, v)| {
                let state = &mut v.state;

                let _ = chain_state
                    .iter()
                    .try_for_each(|block_state| -> ControlFlow<()> {
                        let prev_days_old = block_state
                            .timestamp
                            .difference_in_days_between(prev_timestamp);
                        let days_old = block_state.timestamp.difference_in_days_between(timestamp);

                        if prev_days_old == days_old {
                            return ControlFlow::Continue(());
                        }

                        let is = filter.contains(days_old);
                        let was = filter.contains(prev_days_old);

                        if is && !was {
                            state.increment(&block_state.supply, block_state.price);
                        } else if was && !is {
                            state.decrement(&block_state.supply, block_state.price);
                        }

                        ControlFlow::Continue(())
                    });
            });
    }

    pub fn send(
        &mut self,
        height_to_sent: BTreeMap<Height, Transacted>,
        chain_state: &[BlockState],
    ) {
        let mut time_based_vecs = self
            .0
            .by_date_range
            .as_mut_vec()
            .into_iter()
            .chain(self.0.by_epoch.as_mut_vec())
            .collect::<Vec<_>>();

        let last_timestamp = chain_state.last().unwrap().timestamp;
        let current_price = chain_state.last().unwrap().price;

        // dbg!(&height_to_sent);

        height_to_sent.into_iter().for_each(|(height, sent)| {
            let block_state = chain_state.get(height.unwrap_to_usize()).unwrap();
            let prev_price = block_state.price;

            let blocks_old = chain_state.len() - 1 - height.unwrap_to_usize();

            let days_old = block_state
                .timestamp
                .difference_in_days_between(last_timestamp);

            let days_old_float = block_state
                .timestamp
                .difference_in_days_between_float(last_timestamp);

            let older_than_hour = last_timestamp
                .checked_sub(block_state.timestamp)
                .unwrap()
                .is_more_than_hour();

            time_based_vecs
                .iter_mut()
                .filter(|(filter, _)| match filter {
                    GroupFilter::From(from) => *from <= days_old,
                    GroupFilter::To(to) => *to > days_old,
                    GroupFilter::Range(range) => range.contains(&days_old),
                    GroupFilter::Epoch(epoch) => *epoch == HalvingEpoch::from(height),
                    _ => unreachable!(),
                })
                .for_each(|(_, vecs)| {
                    vecs.state.send(
                        &sent.spendable_supply,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    );
                });

            sent.by_type.spendable.as_typed_vec().into_iter().for_each(
                |(output_type, supply_state)| {
                    self.0.by_type.get_mut(output_type).1.state.send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    )
                },
            );

            sent.by_size_group
                .as_typed_vec()
                .into_iter()
                .for_each(|(group, supply_state)| {
                    self.0.by_size_range.get_mut(group).1.state.send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_float,
                        older_than_hour,
                    );
                });
        });
    }

    pub fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>) {
        let supply_state = received.spendable_supply;

        [
            &mut self.0.by_date_range.start_to_1d.1,
            &mut self.0.by_epoch.mut_vec_from_height(height).1,
        ]
        .into_iter()
        .for_each(|v| {
            v.state.receive(&supply_state, price);
        });

        self.by_type
            .as_mut_vec()
            .into_iter()
            .for_each(|(filter, vecs)| {
                let output_type = match filter {
                    GroupFilter::Type(output_type) => *output_type,
                    _ => unreachable!(),
                };
                vecs.state.receive(received.by_type.get(output_type), price)
            });

        received
            .by_size_group
            .as_typed_vec()
            .into_iter()
            .for_each(|(group, supply_state)| {
                self.by_size_range
                    .get_mut(group)
                    .1
                    .state
                    .receive(supply_state, price);
            });
    }

    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_date_range = self.0.by_date_range.as_vec();
        let by_size_range = self.0.by_size_range.as_vec();

        [
            vec![(&mut self.0.all.1, self.0.by_epoch.vecs().to_vec())],
            self.0
                .by_from_date
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_date_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.0
                .by_up_to_date
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_date_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.0
                .by_term
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_date_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.0
                .by_from_size
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_size_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.0
                .by_up_to_size
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_size_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        ]
        .into_par_iter()
        .flatten()
        .try_for_each(|(vecs, stateful)| {
            vecs.compute_from_stateful(starting_indexes, &stateful, exit)
        })
    }

    pub fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.as_mut_separate_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| v.safe_flush_stateful_vecs(height, exit))
    }
}
