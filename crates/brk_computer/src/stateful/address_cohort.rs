use std::path::Path;

use brk_error::Result;
use brk_structs::{Bitcoin, DateIndex, Dollars, Height, StoredU64, Version};
use brk_vecs::IVecs;
use vecdb::{
    AnyIterableVec, AnyStoredVec, AnyVec, Database, EagerVec, Exit, Format, GenericStoredVec,
    VecIterator,
};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes, price,
    stateful::{
        common,
        r#trait::{CohortVecs, DynCohortVecs},
    },
    states::AddressCohortState,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, IVecs)]
pub struct Vecs {
    starting_height: Option<Height>,

    #[vecs(skip)]
    pub state: Option<AddressCohortState>,

    pub inner: common::Vecs,

    pub height_to_addr_count: EagerVec<Height, StoredU64>,
    pub indexes_to_addr_count: ComputedVecsFromHeight<StoredU64>,
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        cohort_name: Option<&str>,
        format: Format,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: Option<&Path>,
        compute_rel_to_all: bool,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();

        let suffix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{name}_{s}"));

        Ok(Self {
            starting_height: None,
            state: states_path.map(|states_path| {
                AddressCohortState::new(
                    states_path,
                    cohort_name.unwrap_or_default(),
                    compute_dollars,
                )
            }),
            height_to_addr_count: EagerVec::forced_import(
                db,
                &suffix("addr_count"),
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_addr_count: ComputedVecsFromHeight::forced_import(
                db,
                &suffix("addr_count"),
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            inner: common::Vecs::forced_import(
                db,
                cohort_name,
                format,
                version,
                indexes,
                price,
                false,
                compute_rel_to_all,
                false,
            )?,
        })
    }
}

impl DynCohortVecs for Vecs {
    fn min_height_vecs_len(&self) -> usize {
        std::cmp::min(
            self.height_to_addr_count.len(),
            self.inner.min_height_vecs_len(),
        )
    }

    fn reset_state_starting_height(&mut self) {
        self.starting_height = Some(Height::ZERO);
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        let starting_height = self
            .inner
            .import_state(starting_height, &mut self.state.as_mut().unwrap().inner)?;

        self.starting_height = Some(starting_height);

        if let Some(prev_height) = starting_height.decremented() {
            self.state.as_mut().unwrap().addr_count = *self
                .height_to_addr_count
                .into_iter()
                .unwrap_get_inner(prev_height);
        }

        Ok(starting_height)
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.height_to_addr_count
            .validate_computed_version_or_reset(
                base_version + self.height_to_addr_count.inner_version(),
            )?;

        self.inner.validate_computed_versions(base_version)
    }

    fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()> {
        if self.starting_height.unwrap() > height {
            return Ok(());
        }

        self.height_to_addr_count.forced_push_at(
            height,
            self.state.as_ref().unwrap().addr_count.into(),
            exit,
        )?;

        self.inner
            .forced_pushed_at(height, exit, &self.state.as_ref().unwrap().inner)
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
            &self.state.as_ref().unwrap().inner,
        )
    }

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.height_to_addr_count.safe_flush(exit)?;

        self.inner
            .safe_flush_stateful_vecs(height, exit, &mut self.state.as_mut().unwrap().inner)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_addr_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_addr_count),
        )?;

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
        self.height_to_addr_count.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_addr_count)
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
