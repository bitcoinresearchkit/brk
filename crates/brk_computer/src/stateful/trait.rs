use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{Bitcoin, DateIndex, Dollars, Height, Version};
use vecdb::{AnyCollectableVec, AnyIterableVec, Exit};

use crate::{Indexes, indexes, market, price};

pub trait DynCohortVecs: Send + Sync {
    fn starting_height(&self) -> Height;

    fn init(&mut self, starting_height: Height);

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;

    fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()>;

    fn compute_then_force_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        exit: &Exit,
    ) -> Result<()>;

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()>;

    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part1(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()>;

    fn vecs(&self) -> Vec<&dyn AnyCollectableVec>;
}

pub trait CohortVecs: DynCohortVecs {
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()>;

    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part2(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        market: &market::Vecs,
        height_to_supply: &impl AnyIterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl AnyIterableVec<DateIndex, Bitcoin>,
        height_to_realized_cap: Option<&impl AnyIterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()>;
}
