use std::{ops::Deref, path::Path};

use brk_core::{Bitcoin, DateIndex, Dollars, Height, Result, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, AnyIterableVec, Computation, Format};

use crate::{
    UTXOCohortState,
    vecs::{
        Indexes, fetched, indexes, market,
        stateful::{common, r#trait::CohortVecs},
    },
};

#[derive(Clone)]
pub struct Vecs {
    starting_height: Height,

    pub state: UTXOCohortState,

    inner: common::Vecs,
}

impl CohortVecs for Vecs {
    #[allow(clippy::too_many_arguments)]
    fn forced_import(
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

        Ok(Self {
            starting_height: Height::ZERO,

            state: UTXOCohortState::default_and_import(
                states_path,
                cohort_name.unwrap_or_default(),
                compute_dollars,
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

    fn starting_height(&self) -> Height {
        [
            self.state.height().map_or(Height::MAX, |h| h.incremented()),
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

        self.inner.init(&mut self.starting_height, &mut self.state);
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.inner.validate_computed_versions(base_version)
    }

    fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()> {
        if self.starting_height > height {
            return Ok(());
        }

        self.inner.forced_pushed_at(height, exit, &self.state)
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
            &self.state,
        )
    }

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.inner
            .safe_flush_stateful_vecs(height, exit, &mut self.state)
    }

    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.inner).collect::<Vec<_>>(),
            exit,
        )
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
        self.inner
            .compute_rest_part1(indexer, indexes, fetched, starting_indexes, exit)
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

    fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        self.inner.vecs()
    }
}

impl Deref for Vecs {
    type Target = common::Vecs;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
