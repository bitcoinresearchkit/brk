use std::{collections::BTreeMap, ops::ControlFlow, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{
    Bitcoin, ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge,
    ByMinAge, BySpendableType, ByTerm, CheckedSub, DateIndex, Dollars, GroupFilter, HalvingEpoch,
    Height, Timestamp, UTXOGroups, Version,
};
use derive_deref::{Deref, DerefMut};
use vecdb::{AnyIterableVec, Database, Exit, Format, StoredIndex};

use crate::{
    Indexes, indexes, price,
    stateful::r#trait::DynCohortVecs,
    states::{BlockState, Transacted},
};

use super::{r#trait::CohortVecs, utxo_cohort};

const VERSION: Version = Version::new(0);

#[derive(Clone, Deref, DerefMut)]
pub struct Vecs(UTXOGroups<(GroupFilter, utxo_cohort::Vecs)>);

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        format: Format,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
    ) -> Result<Self> {
        Ok(Self(
            UTXOGroups {
                all: utxo_cohort::Vecs::forced_import(
                    db,
                    None,
                    format,
                    version + VERSION + Version::ONE,
                    indexes,
                    price,
                    None,
                    true,
                    false,
                    true,
                )?,
                term: ByTerm {
                    short: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("sth"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    long: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("lth"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                },
                epoch: ByEpoch {
                    _0: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("epoch_0"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _1: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("epoch_1"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _2: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("epoch_2"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _3: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("epoch_3"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _4: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("epoch_4"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                },
                _type: BySpendableType {
                    p2pk65: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2pk65"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2pk33: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2pk33"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2pkh: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2pkh"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2sh: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2sh"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2wpkh: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2wpkh"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2wsh: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2wsh"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2tr: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2tr"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2a: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2a"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    p2ms: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("p2ms_outputs"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    empty: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("empty_outputs"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    unknown: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("unknown_outputs"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                },
                max_age: ByMaxAge {
                    _1w: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_1w_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _1m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_1m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _2m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_2m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _3m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_3m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _4m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_4m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _5m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_5m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _6m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_6m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _1y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_1y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _2y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_2y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _3y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_3y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _4y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_4y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _5y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_5y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _6y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_6y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _7y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_7y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _8y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_8y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _10y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_10y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _12y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_12y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                    _15y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_15y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        true,
                    )?,
                },
                min_age: ByMinAge {
                    _1d: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1d_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _1w: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1w_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _1m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _2m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_2m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _3m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_3m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _4m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_4m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _5m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_5m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _6m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_6m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _1y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _2y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_2y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _3y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_3y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _4y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_4y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _5y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_5y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _6y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_6y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _7y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_7y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _8y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_8y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _10y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_10y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                    _12y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_12y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                        true,
                        false,
                    )?,
                },
                age_range: ByAgeRange {
                    up_to_1d: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_up_to_1d_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        true,
                    )?,
                    _1d_to_1w: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1d_up_to_1w_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _1w_to_1m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1w_up_to_1m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _1m_to_2m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1m_up_to_2m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _2m_to_3m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_2m_up_to_3m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _3m_to_4m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_3m_up_to_4m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _4m_to_5m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_4m_up_to_5m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _5m_to_6m: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_5m_up_to_6m_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _6m_to_1y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_6m_up_to_1y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _1y_to_2y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_1y_up_to_2y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _2y_to_3y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_2y_up_to_3y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _3y_to_4y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_3y_up_to_4y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _4y_to_5y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_4y_up_to_5y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _5y_to_6y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_5y_up_to_6y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _6y_to_7y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_6y_up_to_7y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _7y_to_8y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_7y_up_to_8y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _8y_to_10y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_8y_up_to_10y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _10y_to_12y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_10y_up_to_12y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    _12y_to_15y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_12y_up_to_15y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                    from_15y: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_at_least_15y_old"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                        true,
                        false,
                    )?,
                },
                amount_range: ByAmountRange {
                    _0sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_with_0sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _1sat_to_10sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1sat_under_10sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _10sats_to_100sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10sats_under_100sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _100sats_to_1k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100sats_under_1k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _1k_sats_to_10k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1k_sats_under_10k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _10k_sats_to_100k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10k_sats_under_100k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _100k_sats_to_1m_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100k_sats_under_1m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _1m_sats_to_10m_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1m_sats_under_10m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _10m_sats_to_1btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10m_sats_under_1btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _1btc_to_10btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1btc_under_10btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _10btc_to_100btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10btc_under_100btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _100btc_to_1k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100btc_under_1k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _1k_btc_to_10k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1k_btc_under_10k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _10k_btc_to_100k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10k_btc_under_100k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                    _100k_btc_or_more: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        false,
                        true,
                        false,
                    )?,
                },
                lt_amount: ByLowerThanAmount {
                    _10sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_10sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_100sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_1k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_10k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_100k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1m_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_1m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10m_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_10m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_1btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_10btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_100btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_1k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_10k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_under_100k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                },
                ge_amount: ByGreatEqualAmount {
                    _1sat: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1sat"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100k_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1m_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10m_sats: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _100btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_100btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _1k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_1k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
                    )?,
                    _10k_btc: utxo_cohort::Vecs::forced_import(
                        db,
                        Some("utxos_above_10k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        false,
                        true,
                        false,
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

        let mut vecs = self
            .age_range
            .as_mut_vec()
            .into_iter()
            .map(|(filter, v)| (filter, &mut v.state))
            .collect::<Vec<_>>();

        let _ = chain_state
            .iter()
            .try_for_each(|block_state| -> ControlFlow<()> {
                let prev_days_old =
                    prev_timestamp.difference_in_days_between(block_state.timestamp);
                let days_old = timestamp.difference_in_days_between(block_state.timestamp);

                if prev_days_old == days_old {
                    return ControlFlow::Continue(());
                }

                vecs.iter_mut().for_each(|(filter, state)| {
                    let is = filter.contains(days_old);
                    let was = filter.contains(prev_days_old);

                    if is && !was {
                        state
                            .as_mut()
                            .unwrap()
                            .increment(&block_state.supply, block_state.price);
                    } else if was && !is {
                        state
                            .as_mut()
                            .unwrap()
                            .decrement(&block_state.supply, block_state.price);
                    }
                });

                ControlFlow::Continue(())
            });
    }

    pub fn send(
        &mut self,
        height_to_sent: BTreeMap<Height, Transacted>,
        chain_state: &mut [BlockState],
    ) {
        let mut time_based_vecs = self
            .0
            .age_range
            .as_mut_vec()
            .into_iter()
            .chain(self.0.epoch.as_mut_vec())
            .collect::<Vec<_>>();

        let last_timestamp = chain_state.last().unwrap().timestamp;
        let current_price = chain_state.last().unwrap().price;

        let chain_state_len = chain_state.len();

        height_to_sent.into_iter().for_each(|(height, sent)| {
            chain_state[height.unwrap_to_usize()].supply -= &sent.spendable_supply;

            let block_state = chain_state.get(height.unwrap_to_usize()).unwrap();

            let prev_price = block_state.price;

            let blocks_old = chain_state_len - 1 - height.unwrap_to_usize();

            let days_old = last_timestamp.difference_in_days_between(block_state.timestamp);
            let days_old_float =
                last_timestamp.difference_in_days_between_float(block_state.timestamp);

            let older_than_hour = last_timestamp
                .checked_sub(block_state.timestamp)
                .unwrap()
                .is_more_than_hour();

            time_based_vecs
                .iter_mut()
                .filter(|(filter, _)| match filter {
                    GroupFilter::GreaterOrEqual(from) => *from <= days_old,
                    GroupFilter::LowerThan(to) => *to > days_old,
                    GroupFilter::Range(range) => range.contains(&days_old),
                    GroupFilter::Epoch(epoch) => *epoch == HalvingEpoch::from(height),
                    _ => unreachable!(),
                })
                .for_each(|(_, vecs)| {
                    vecs.state.as_mut().unwrap().send(
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
                    self.0
                        ._type
                        .get_mut(output_type)
                        .1
                        .state
                        .as_mut()
                        .unwrap()
                        .send(
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
                    self.0
                        .amount_range
                        .get_mut(group)
                        .1
                        .state
                        .as_mut()
                        .unwrap()
                        .send(
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
            &mut self.0.age_range.up_to_1d.1,
            &mut self.0.epoch.mut_vec_from_height(height).1,
        ]
        .into_iter()
        .for_each(|v| {
            v.state.as_mut().unwrap().receive(&supply_state, price);
        });

        self._type
            .as_mut_vec()
            .into_iter()
            .for_each(|(filter, vecs)| {
                let output_type = match filter {
                    GroupFilter::Type(output_type) => *output_type,
                    _ => unreachable!(),
                };
                vecs.state
                    .as_mut()
                    .unwrap()
                    .receive(received.by_type.get(output_type), price)
            });

        received
            .by_size_group
            .as_typed_vec()
            .into_iter()
            .for_each(|(group, supply_state)| {
                self.amount_range
                    .get_mut(group)
                    .1
                    .state
                    .as_mut()
                    .unwrap()
                    .receive(supply_state, price);
            });
    }

    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_date_range = self.0.age_range.as_vec();
        let by_size_range = self.0.amount_range.as_vec();

        [
            vec![(
                &mut self.0.all.1,
                by_date_range
                    .into_iter()
                    .map(|(_, v)| v)
                    .collect::<Vec<_>>(),
            )],
            self.0
                .min_age
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
                .max_age
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
                .term
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
                .ge_amount
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
                .lt_amount
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
        .into_iter()
        .flatten()
        .try_for_each(|(vecs, stateful)| {
            vecs.compute_from_stateful(starting_indexes, &stateful, exit)
        })
    }

    pub fn compute_rest_part1(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.as_mut_vecs().into_iter().try_for_each(|(_, v)| {
            v.compute_rest_part1(indexer, indexes, price, starting_indexes, exit)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl AnyIterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl AnyIterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl AnyIterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
        height_to_realized_cap: Option<&impl AnyIterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.0.as_boxed_mut_vecs().into_iter().try_for_each(|v| {
            v.into_iter().try_for_each(|(_, v)| {
                v.compute_rest_part2(
                    indexer,
                    indexes,
                    price,
                    starting_indexes,
                    height_to_supply,
                    dateindex_to_supply,
                    height_to_market_cap,
                    dateindex_to_market_cap,
                    height_to_realized_cap,
                    dateindex_to_realized_cap,
                    exit,
                )
            })
        })
    }

    pub fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.as_mut_separate_vecs()
            .into_iter()
            .try_for_each(|(_, v)| v.safe_flush_stateful_vecs(height, exit))
    }
}
