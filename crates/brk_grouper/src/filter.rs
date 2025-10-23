use std::ops::Range;

use brk_traversable::{Traversable, TreeNode};
use brk_types::{HalvingEpoch, OutputType};
use vecdb::AnyCollectableVec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    All,
    LowerThan(usize),
    Range(Range<usize>),
    GreaterOrEqual(usize),
    Epoch(HalvingEpoch),
    Type(OutputType),
}

impl Filter {
    pub fn contains(&self, value: usize) -> bool {
        match self {
            Filter::Range(r) => r.contains(&value),
            Filter::LowerThan(max) => *max > value,
            Filter::GreaterOrEqual(min) => *min <= value,
            Filter::All => true,
            Filter::Epoch(_) | Filter::Type(_) => false,
        }
    }

    pub fn includes(&self, other: &Filter) -> bool {
        match self {
            Filter::All => true,
            Filter::LowerThan(max) => match other {
                Filter::LowerThan(max2) => max >= max2,
                Filter::Range(range) => range.end <= *max,
                Filter::All | Filter::GreaterOrEqual(_) | Filter::Epoch(_) | Filter::Type(_) => {
                    false
                }
            },
            Filter::GreaterOrEqual(min) => match other {
                Filter::Range(range) => range.start >= *min,
                Filter::GreaterOrEqual(min2) => min <= min2,
                Filter::All | Filter::LowerThan(_) | Filter::Epoch(_) | Filter::Type(_) => false,
            },
            Filter::Range(_) | Filter::Epoch(_) | Filter::Type(_) => false,
        }
    }
}

#[derive(Clone)]
pub struct Filtered<T>(pub Filter, pub T);

impl<T> Filtered<T> {
    pub fn includes(&self, other: &Filter) -> bool {
        self.0.includes(other)
    }

    pub fn filter(&self) -> &Filter {
        &self.0
    }

    pub fn unwrap(self) -> T {
        self.1
    }

    pub fn t(&self) -> &T {
        &self.1
    }

    pub fn mut_t(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<T> From<(Filter, T)> for Filtered<T> {
    fn from(value: (Filter, T)) -> Self {
        Self(value.0, value.1)
    }
}

impl<T: Traversable> Traversable for Filtered<T> {
    fn to_tree_node(&self) -> TreeNode {
        self.1.to_tree_node()
    }

    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        self.1.iter_any_collectable()
    }
}
