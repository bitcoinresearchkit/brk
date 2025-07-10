use std::{ops::Deref, path::Path};

use brk_core::{Bitcoin, DateIndex, Dollars, Height, Result, StoredUsize, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, AnyVec, Computation, EagerVec, Format, VecIterator,
};

use crate::{
    states::AddressCohortState,
    vecs::{
        Indexes, fetched,
        grouped::{ComputedVecsFromHeight, Source, VecBuilderOptions},
        indexes, market,
        stateful::{
            common,
            r#trait::{CohortVecs, DynCohortVecs},
        },
    },
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    starting_height: Height,

    pub state: AddressCohortState,

    pub inner: common::Vecs,

    pub height_to_address_count: EagerVec<Height, StoredUsize>,
    pub indexes_to_address_count: ComputedVecsFromHeight<StoredUsize>,
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        path: &Path,
        cohort_name: Option<&str>,
        computation: Computation,
        format: Format,
        version: Version,
        fetched: Option<&fetched::Vecs>,
        states_path: &Path,
        compute_relative_to_all: bool,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        let suffix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{name}_{s}"));

        Ok(Self {
            starting_height: Height::ZERO,
            state: AddressCohortState::default_and_import(
                states_path,
                cohort_name.unwrap_or_default(),
                compute_dollars,
            )?,
            height_to_address_count: EagerVec::forced_import(
                path,
                &suffix("address_count"),
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_address_count: ComputedVecsFromHeight::forced_import(
                path,
                &suffix("address_count"),
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
                VecBuilderOptions::default().add_last(),
            )?,
            inner: common::Vecs::forced_import(
                path,
                cohort_name,
                computation,
                format,
                version,
                fetched,
                compute_relative_to_all,
            )?,
        })
    }
}

impl DynCohortVecs for Vecs {
    fn starting_height(&self) -> Height {
        [
            self.state.height().map_or(Height::MAX, |h| h.incremented()),
            self.height_to_address_count.len().into(),
            self.inner.starting_height(),
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    fn init(&mut self, starting_height: Height) {
        if starting_height > self.starting_height() {
            unreachable!()
        }

        self.starting_height = starting_height;

        if let Some(prev_height) = starting_height.decremented() {
            self.state.address_count = *self
                .height_to_address_count
                .into_iter()
                .unwrap_get_inner(prev_height);
        }

        self.inner
            .init(&mut self.starting_height, &mut self.state.inner);
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.height_to_address_count
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_address_count.inner_version(),
            )?;

        self.inner.validate_computed_versions(base_version)
    }

    fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()> {
        if self.starting_height > height {
            return Ok(());
        }

        self.height_to_address_count.forced_push_at(
            height,
            self.state.address_count.into(),
            exit,
        )?;

        self.inner.forced_pushed_at(height, exit, &self.state.inner)
    }

    fn compute_then_force_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_then_force_push_unrealized_states(
            height,
            height_price,
            dateindex,
            date_price,
            exit,
            &self.state.inner,
        )
    }

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.height_to_address_count.safe_flush(exit)?;

        self.inner
            .safe_flush_stateful_vecs(height, exit, &mut self.state.inner)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part1(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_address_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_address_count),
        )?;

        self.inner
            .compute_rest_part1(indexer, indexes, fetched, starting_indexes, exit)
    }

    fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.inner.vecs(),
            self.indexes_to_address_count.vecs(),
            vec![&self.height_to_address_count],
        ]
        .concat()
    }
}

impl CohortVecs for Vecs {
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_address_count.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_address_count)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.inner.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.inner).collect::<Vec<_>>(),
            exit,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part2(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        market: &market::Vecs,
        height_to_supply: &impl AnyIterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl AnyIterableVec<DateIndex, Bitcoin>,
        height_to_realized_cap: Option<&impl AnyIterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.inner.compute_rest_part2(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            market,
            height_to_supply,
            dateindex_to_supply,
            height_to_realized_cap,
            dateindex_to_realized_cap,
            exit,
        )
    }
}

impl Deref for Vecs {
    type Target = common::Vecs;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
