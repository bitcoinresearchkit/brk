use std::path::Path;

use brk_error::Result;
use brk_grouper::{AddressGroups, AmountFilter, Filter, Filtered};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Version};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{Database, Exit, IterableVec};

use crate::{
    Indexes, indexes, price,
    stateful::{
        address_cohort,
        r#trait::{CohortVecs, DynCohortVecs},
    },
};

const VERSION: Version = Version::new(0);

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct Vecs(AddressGroups<address_cohort::Vecs>);

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
    ) -> Result<Self> {
        Ok(Self(AddressGroups::new(|filter| {
            let states_path = match &filter {
                Filter::Amount(AmountFilter::Range(_)) => Some(states_path),
                _ => None,
            };

            address_cohort::Vecs::forced_import(
                db,
                filter,
                version + VERSION + Version::ZERO,
                indexes,
                price,
                states_path,
            )
            .unwrap()
        })))
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
                .par_iter_mut()
                .map(|vecs| {
                    let filter = vecs.filter().clone();
                    (
                        vecs,
                        by_size_range
                            .iter()
                            .filter(|other| filter.includes(other.filter()))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.0
                .lt_amount
                .par_iter_mut()
                .map(|vecs| {
                    let filter = vecs.filter().clone();
                    (
                        vecs,
                        by_size_range
                            .iter()
                            .filter(|other| filter.includes(other.filter()))
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
        self.par_iter_mut()
            .try_for_each(|v| v.compute_rest_part1(indexes, price, starting_indexes, exit))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        height_to_realized_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.0.par_iter_mut().try_for_each(|v| {
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
        self.par_iter_separate_mut()
            .try_for_each(|v| v.safe_flush_stateful_vecs(height, exit))
    }
}
