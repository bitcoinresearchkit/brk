#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use brk_types::{Halving, Height};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{CohortName, Filter};

/// Epoch values
pub const EPOCH_VALUES: ByEpoch<Halving> = ByEpoch {
    _0: Halving::new(0),
    _1: Halving::new(1),
    _2: Halving::new(2),
    _3: Halving::new(3),
    _4: Halving::new(4),
};

/// Epoch filters
pub const EPOCH_FILTERS: ByEpoch<Filter> = ByEpoch {
    _0: Filter::Epoch(EPOCH_VALUES._0),
    _1: Filter::Epoch(EPOCH_VALUES._1),
    _2: Filter::Epoch(EPOCH_VALUES._2),
    _3: Filter::Epoch(EPOCH_VALUES._3),
    _4: Filter::Epoch(EPOCH_VALUES._4),
};

/// Epoch names
pub const EPOCH_NAMES: ByEpoch<CohortName> = ByEpoch {
    _0: CohortName::new("epoch_0", "0", "Epoch 0"),
    _1: CohortName::new("epoch_1", "1", "Epoch 1"),
    _2: CohortName::new("epoch_2", "2", "Epoch 2"),
    _3: CohortName::new("epoch_3", "3", "Epoch 3"),
    _4: CohortName::new("epoch_4", "4", "Epoch 4"),
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct ByEpoch<T> {
    /// Uses UTXOs created during subsidy-halving epoch 0.
    pub _0: T,
    /// Uses UTXOs created during subsidy-halving epoch 1.
    pub _1: T,
    /// Uses UTXOs created during subsidy-halving epoch 2.
    pub _2: T,
    /// Uses UTXOs created during subsidy-halving epoch 3.
    pub _3: T,
    /// Uses UTXOs created during subsidy-halving epoch 4.
    pub _4: T,
}

define_column_id!(
    EpochId for ByEpoch, version = 1 {
        _0 => _0,
        _1 => _1,
        _2 => _2,
        _3 => _3,
        _4 => _4,
    }
);

impl ByEpoch<CohortName> {
    pub const fn names() -> &'static Self {
        &EPOCH_NAMES
    }
}

impl<T> ByEpoch<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        let f = EPOCH_FILTERS;
        let n = EPOCH_NAMES;
        Self {
            _0: create(f._0, n._0.id),
            _1: create(f._1, n._1.id),
            _2: create(f._2, n._2.id),
            _3: create(f._3, n._3.id),
            _4: create(f._4, n._4.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(Filter, &'static str) -> Result<T, E>,
    {
        let f = EPOCH_FILTERS;
        let n = EPOCH_NAMES;
        Ok(Self {
            _0: create(f._0, n._0.id)?,
            _1: create(f._1, n._1.id)?,
            _2: create(f._2, n._2.id)?,
            _3: create(f._3, n._3.id)?,
            _4: create(f._4, n._4.id)?,
        })
    }

    pub fn mut_vec_from_height(&mut self, height: Height) -> Option<&mut T> {
        let epoch = Halving::from(height);
        if epoch == Halving::new(0) {
            Some(&mut self._0)
        } else if epoch == Halving::new(1) {
            Some(&mut self._1)
        } else if epoch == Halving::new(2) {
            Some(&mut self._2)
        } else if epoch == Halving::new(3) {
            Some(&mut self._3)
        } else if epoch == Halving::new(4) {
            Some(&mut self._4)
        } else {
            None
        }
    }
}
impl EpochId {
    pub fn matching(filter: &Filter) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|id| id.select(&EPOCH_FILTERS) == filter)
    }
}
