use bitview_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::ReadableCloneableVec;

use crate::{
    CentsUnsignedToDollars, Identity, IndexSources, LazyCumulativeValuePerBlock, LazyPerBlock,
    LazyRollingAvgsAmountFromHeight, LazyRollingSumsAmountFromHeight, LazyValueBlock,
    SatsToBitcoin, Windows,
};

#[derive(Clone, Traversable)]
pub struct LazyValuePerBlockCumulativeRolling {
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: LazyValueBlock,
    /// Cumulative value through the represented block. At time-period indexes,
    /// the value is taken at the period's final block.
    pub cumulative: LazyCumulativeValuePerBlock,
    pub sum: LazyRollingSumsAmountFromHeight,
    pub average: LazyRollingAvgsAmountFromHeight,
}

impl LazyValuePerBlockCumulativeRolling {
    pub fn from_cumulative_sources(
        name: &str,
        version: Version,
        cumulative_sats: &(impl ReadableCloneableVec<Height, Sats> + ?Sized),
        cumulative_cents: &(impl ReadableCloneableVec<Height, Cents> + ?Sized),
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let cumulative_name = format!("{name}_cumulative");
        let sats = LazyPerBlock::from_height_source::<Identity<Sats>>(
            &format!("{cumulative_name}_sats"),
            version,
            cumulative_sats,
            indexes,
        );
        let btc = LazyPerBlock::from_lazy::<SatsToBitcoin, Sats>(&cumulative_name, version, &sats);
        let cents = LazyPerBlock::from_height_source::<Identity<Cents>>(
            &format!("{cumulative_name}_cents"),
            version,
            cumulative_cents,
            indexes,
        );
        let usd = LazyPerBlock::from_lazy::<CentsUnsignedToDollars, Cents>(
            &format!("{cumulative_name}_usd"),
            version,
            &cents,
        );
        let cumulative = LazyCumulativeValuePerBlock {
            btc,
            sats,
            usd,
            cents,
        };
        let block = LazyValueBlock::from_cumulative_sources(
            name,
            version,
            &cumulative.sats.height,
            &cumulative.cents.height,
        );
        let sum = LazyRollingSumsAmountFromHeight::new(
            &format!("{name}_sum"),
            version,
            &cumulative.sats.height,
            &cumulative.cents.height,
            window_starts,
            indexes,
        );
        let average = LazyRollingAvgsAmountFromHeight::new(
            &format!("{name}_average"),
            version,
            &cumulative.sats.height,
            &cumulative.cents.height,
            window_starts,
            indexes,
        );

        Self {
            block,
            cumulative,
            sum,
            average,
        }
    }
}
