use std::path::Path;

use brk_core::{
    AddressGroups, GroupFilter, GroupedByFromSize, GroupedBySizeRange, GroupedByUpToSize, Height,
    Result, Version,
};
use brk_exit::Exit;
use brk_vec::{Computation, Format};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;

use crate::vecs::{
    Indexes, fetched,
    stateful::{address_cohort, r#trait::CohortVecs},
};

const VERSION: Version = Version::new(0);

#[derive(Clone, Deref, DerefMut)]
pub struct Vecs(AddressGroups<(GroupFilter, address_cohort::Vecs)>);

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
            AddressGroups {
                by_size_range: GroupedBySizeRange {
                    _0sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_0sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1sat_to_10sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1sat_to_10sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10sats_to_100sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_10sats_to_100sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100sats_to_1_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_100sats_to_1_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1_000sats_to_10_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1_000sats_to_10_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10_000sats_to_100_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_10_000sats_to_100_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100_000sats_to_1_000_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_100_000sats_to_1_000_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1_000_000sats_to_10_000_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1_000_000sats_to_10_000_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10_000_000sats_to_1btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_10_000_000sats_to_1btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1btc_to_10btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1btc_to_10btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10btc_to_100btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_10btc_to_100btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100btc_to_1_000btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_100btc_to_1_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_1_000btc_to_10_000btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1_000btc_to_10_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_10_000btc_to_100_000btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_10_000btc_to_100_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    from_100_000btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_100_000btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_up_to_size: GroupedByUpToSize {
                    _1_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_up_to_1_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_up_to_10_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_up_to_1btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_up_to_10btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_up_to_100btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                },
                by_from_size: GroupedByFromSize {
                    _1_000sats: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1_000sats"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _1btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_1btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _10btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_10btc"),
                        _computation,
                        format,
                        version + VERSION + Version::ZERO,
                        fetched,
                        states_path,
                        true,
                    )?,
                    _100btc: address_cohort::Vecs::forced_import(
                        path,
                        Some("addresses_from_100btc"),
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

    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_size_range = self.0.by_size_range.as_vec();

        [
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
