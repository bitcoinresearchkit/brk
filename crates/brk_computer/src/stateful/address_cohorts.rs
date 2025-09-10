use std::path::Path;

use brk_error::Result;
use brk_structs::{
    AddressGroups, Bitcoin, ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount, DateIndex,
    Dollars, GroupFilter, Height, Version,
};
use derive_deref::{Deref, DerefMut};
use vecdb::{AnyIterableVec, Database, Exit, Format};

use crate::{
    Indexes, indexes, price,
    stateful::{
        address_cohort,
        r#trait::{CohortVecs, DynCohortVecs},
    },
};

const VERSION: Version = Version::new(0);

#[derive(Clone, Deref, DerefMut)]
pub struct Vecs(AddressGroups<(GroupFilter, address_cohort::Vecs)>);

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
            AddressGroups {
                amount_range: ByAmountRange {
                    _0sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_with_0sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _1sat_to_10sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1sat_under_10sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _10sats_to_100sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10sats_under_100sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _100sats_to_1k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100sats_under_1k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _1k_sats_to_10k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1k_sats_under_10k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _10k_sats_to_100k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10k_sats_under_100k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _100k_sats_to_1m_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100k_sats_under_1m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _1m_sats_to_10m_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1m_sats_under_10m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _10m_sats_to_1btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10m_sats_under_1btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _1btc_to_10btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1btc_under_10btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _10btc_to_100btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10btc_under_100btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _100btc_to_1k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100btc_under_1k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _1k_btc_to_10k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1k_btc_under_10k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _10k_btc_to_100k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10k_btc_under_100k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                    _100k_btc_or_more: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        Some(states_path),
                        true,
                    )?,
                },
                lt_amount: ByLowerThanAmount {
                    _10sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_10sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_100sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_1k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_10k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_100k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1m_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_1m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10m_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_10m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_1btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_10btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_100btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_1k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_10k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_under_100k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                },
                ge_amount: ByGreatEqualAmount {
                    _1sat: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1sat"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100k_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100k_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1m_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10m_sats: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10m_sats"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _100btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_100btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _1k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_1k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                    _10k_btc: address_cohort::Vecs::forced_import(
                        db,
                        Some("addrs_above_10k_btc"),
                        format,
                        version + VERSION + Version::ZERO,
                        indexes,
                        price,
                        None,
                        true,
                    )?,
                },
            }
            .into(),
        ))
    }

    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_size_range = &self.0.amount_range;

        [
            self.0
                .ge_amount
                .iter_mut()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_size_range
                            .iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.0
                .lt_amount
                .iter_mut()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_size_range
                            .iter()
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
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.iter_mut()
            .into_iter()
            .try_for_each(|(_, v)| v.compute_rest_part1(indexes, price, starting_indexes, exit))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
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
        self.0.iter_mut().try_for_each(|(_, v)| {
            v.compute_rest_part2(
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
    }

    pub fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.iter_separate_mut()
            .into_iter()
            .try_for_each(|(_, v)| v.safe_flush_stateful_vecs(height, exit))
    }
}
