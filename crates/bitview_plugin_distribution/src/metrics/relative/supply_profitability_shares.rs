use bitview_cohort::{UTXOAggregate, UTXOAggregateId};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::RatioSats;
use bitview_traversable::Traversable;
use bitview_vecs::{ColumnarPerBlock, LazyPercentPerBlock};
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, PartsPerMillion32, Sats, Version};
use vecdb::{AnyStoredVec, BinaryTransform, CacheBudget, Database, ReadOnlyClone, Rw, StorageMode};

use super::{RelativeSource, share_views};

const VERSION: Version = Version::ONE;

#[derive(Traversable)]
pub struct SupplyProfitabilityShares<M: StorageMode = Rw> {
    #[traversable(wrap = "supply/in_profit", rename = "share")]
    /// Share of an aggregate UTXO cohort's unspent satoshis whose creation price
    /// is less than or equal to the represented block's spot price. Exact
    /// break-even is assigned to profit. Returns zero when the cohort has no
    /// unspent supply.
    pub supply_in_profit_share: UTXOAggregate<LazyPercentPerBlock<PartsPerMillion32>>,
    #[traversable(wrap = "supply/in_loss", rename = "share")]
    /// Share of an aggregate UTXO cohort's unspent satoshis whose creation price
    /// is greater than the represented block's spot price. Returns zero when the
    /// cohort has no unspent supply.
    pub supply_in_loss_share: UTXOAggregate<LazyPercentPerBlock<PartsPerMillion32>>,
    #[traversable(hidden)]
    pub profit_share_source: ColumnarPerBlock<PartsPerMillion32, UTXOAggregateId, (), M>,
}

impl SupplyProfitabilityShares {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
    ) -> Result<Self> {
        let version = version + VERSION;
        let profit_share_source = ColumnarPerBlock::forced_import(
            cache,
            db,
            "supply_in_profit_share_ppm_by_aggregate",
            version,
            |_| (),
        )?;
        let source = profit_share_source.height.read_only_clone();
        let supply_in_profit_share = share_views(
            &source,
            "supply_in_profit_share",
            version,
            Self::public_profit_share,
            mappings,
        );
        let supply_in_loss_share = share_views(
            &source,
            "supply_in_loss_share",
            version,
            Self::public_loss_share,
            mappings,
        );

        Ok(Self {
            supply_in_profit_share,
            supply_in_loss_share,
            profit_share_source,
        })
    }

    #[inline(always)]
    fn stored_profit_share(profit: Sats, total: Sats) -> PartsPerMillion32 {
        if total.is_zero() {
            PartsPerMillion32::NAN
        } else {
            RatioSats::apply(profit, total)
        }
    }

    #[inline(always)]
    fn public_profit_share(_: Height, profit_share: PartsPerMillion32) -> PartsPerMillion32 {
        if profit_share.is_nan() {
            PartsPerMillion32::ZERO
        } else {
            profit_share
        }
    }

    #[inline(always)]
    fn public_loss_share(_: Height, profit_share: PartsPerMillion32) -> PartsPerMillion32 {
        if profit_share.is_nan() {
            PartsPerMillion32::ZERO
        } else {
            PartsPerMillion32::ONE - profit_share
        }
    }

    pub fn compute(
        &mut self,
        max_from: Height,
        sources: &UTXOAggregate<RelativeSource<'_>>,
        exit: &Exit,
    ) -> Result<()> {
        self.profit_share_source.compute_columns2(
            max_from,
            |id| &id.select(sources).supply.in_profit.sats.height,
            |id| &id.select(sources).supply.total.sats.height,
            |_, profit, total| Self::stored_profit_share(profit, total),
            exit,
        )
    }

    pub fn stored_mut(&mut self) -> &mut dyn AnyStoredVec {
        self.profit_share_source.stored_mut()
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{Height, PartsPerMillion32, Sats};

    use super::SupplyProfitabilityShares;

    #[test]
    fn derives_both_public_shares_from_profit_share() {
        let empty = SupplyProfitabilityShares::stored_profit_share(Sats::ZERO, Sats::ZERO);
        assert!(empty.is_nan());
        assert_eq!(
            SupplyProfitabilityShares::public_profit_share(Height::ZERO, empty),
            PartsPerMillion32::ZERO
        );
        assert_eq!(
            SupplyProfitabilityShares::public_loss_share(Height::ZERO, empty),
            PartsPerMillion32::ZERO
        );

        let profit_share =
            SupplyProfitabilityShares::stored_profit_share(Sats::new(25), Sats::new(100));
        assert_eq!(
            SupplyProfitabilityShares::public_profit_share(Height::ZERO, profit_share),
            PartsPerMillion32::from(0.25)
        );
        assert_eq!(
            SupplyProfitabilityShares::public_loss_share(Height::ZERO, profit_share),
            PartsPerMillion32::from(0.75)
        );
    }
}
