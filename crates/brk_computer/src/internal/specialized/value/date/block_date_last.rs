//! Value type for stateful Last pattern - height and dateindex both stored independently.
//!
//! Use this when dateindex values are NOT derivable from height (e.g., unrealized metrics
//! where end-of-day state differs from last-block-of-day).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes, price};

use super::super::block::LazyDerivedBlockValue;
use super::ValueDateLast;
use crate::internal::LazyLast;

/// Value type where both height and dateindex are stored independently.
/// Dateindex values cannot be derived from height (e.g., unrealized P&L).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ValueBlockDateLast {
    #[traversable(wrap = "sats")]
    pub height: EagerVec<PcoVec<Height, Sats>>,
    #[traversable(flatten)]
    pub height_value: LazyDerivedBlockValue,
    pub difficultyepoch: LazyLast<DifficultyEpoch, Sats, Height, DifficultyEpoch>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub indexes: ValueDateLast,
}

const VERSION: Version = Version::ZERO;

impl ValueBlockDateLast {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, Sats>> = EagerVec::forced_import(db, name, v)?;

        let price_source = price.map(|p| p.usd.split.close.height.boxed_clone());

        let height_value =
            LazyDerivedBlockValue::from_source(name, height.boxed_clone(), v, price_source);

        let difficultyepoch = LazyLast::from_source(
            name,
            v,
            height.boxed_clone(),
            indexes.difficultyepoch.identity.boxed_clone(),
        );

        let indexes = ValueDateLast::forced_import(db, name, v, compute_dollars, indexes)?;

        Ok(Self {
            height,
            height_value,
            difficultyepoch,
            indexes,
        })
    }

    /// Compute derived periods from dateindex.
    pub fn compute_dollars_from_price(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes
            .compute_dollars_from_price(price, starting_indexes, exit)
    }
}
