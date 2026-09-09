mod compute;
mod import;

use bitview_plugin::{Plugin, PluginStorage};
use bitview_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

use super::{AgeRangeVecs, AggregateSources, AggregateVecs};
use crate::STORAGE;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    db: Database,

    /// Coinflow estimates how likely UTXOs of each age are to be spent, using
    /// observed age-specific spending rates and a fitted declining tail for
    /// ages beyond the measured ranges.
    pub age_range: AgeRangeVecs<M>,
    #[traversable(flatten)]
    /// All-chain Coinflow aggregates weight every UTXO age range by its
    /// estimated future spending probability.
    pub all: AggregateVecs,
    /// Short-term-holder Coinflow aggregates use UTXO age ranges younger than
    /// 150 days and weight them by estimated future spending probability.
    pub sth: AggregateVecs,
    /// Long-term-holder Coinflow aggregates use UTXO age ranges at least 150
    /// days old and weight them by estimated future spending probability.
    pub lth: AggregateVecs,
    /// Height-indexed source stored for all, short-term-holder, and
    /// long-term-holder Coinflow aggregates.
    #[traversable(hidden)]
    pub aggregate_sources: AggregateSources<M>,
}

impl<M: StorageMode> Plugin for Vecs<M>
where
    Self: Traversable + Send + Sync,
{
    fn storage(&self) -> PluginStorage {
        STORAGE
    }
}
