use std::{ops::Deref, path::Path};

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Version};
use vecdb::{Database, Exit, IterableVec};

use crate::{
    Indexes, UTXOCohortState, indexes, price,
    stateful::{
        common,
        r#trait::{CohortVecs, DynCohortVecs},
    },
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    state_starting_height: Option<Height>,

    #[traversable(skip)]
    pub state: Option<UTXOCohortState>,

    #[traversable(flatten)]
    pub inner: common::Vecs,
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        cohort_name: Option<&str>,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: Option<&Path>,
        extended: bool,
        compute_rel_to_all: bool,
        compute_adjusted: bool,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();

        Ok(Self {
            state_starting_height: None,

            state: states_path.map(|states_path| {
                UTXOCohortState::new(
                    states_path,
                    cohort_name.unwrap_or_default(),
                    compute_dollars,
                )
            }),

            inner: common::Vecs::forced_import(
                db,
                cohort_name,
                version,
                indexes,
                price,
                extended,
                compute_rel_to_all,
                compute_adjusted,
            )?,
        })
    }
}

impl DynCohortVecs for Vecs {
    fn min_height_vecs_len(&self) -> usize {
        self.inner.min_height_vecs_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        let starting_height = self
            .inner
            .import_state(starting_height, self.state.as_mut().unwrap())?;

        self.state_starting_height = Some(starting_height);

        Ok(starting_height)
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.inner.validate_computed_versions(base_version)
    }

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.state_starting_height.unwrap() > height {
            return Ok(());
        }

        self.inner
            .truncate_push(height, self.state.as_ref().unwrap())
    }

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
    ) -> Result<()> {
        self.inner.compute_then_truncate_push_unrealized_states(
            height,
            height_price,
            dateindex,
            date_price,
            self.state.as_mut().unwrap(),
        )
    }

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.inner
            .safe_flush_stateful_vecs(height, exit, self.state.as_mut().unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.inner
            .compute_rest_part1(indexes, price, starting_indexes, exit)
    }
}

impl CohortVecs for Vecs {
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
    fn compute_rest_part2(
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
        self.inner.compute_rest_part2(
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
    }
}

impl Deref for Vecs {
    type Target = common::Vecs;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
