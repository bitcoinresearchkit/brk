use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints16, BasisPointsSigned32, Dollars, Height, Sats, StoredF32, Version,
};
use vecdb::{Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::internal::{
    Bps32ToFloat, LazyPerBlock, PercentPerBlock, RatioDollarsBp16, RatioDollarsBps32,
    RatioSatsBp16,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedCore};

/// Full relative metrics (sth/lth/all tier).
#[derive(Traversable)]
pub struct RelativeFull<M: StorageMode = Rw> {
    pub supply_in_profit_rel_to_own_supply: PercentPerBlock<BasisPoints16, M>,
    pub supply_in_loss_rel_to_own_supply: PercentPerBlock<BasisPoints16, M>,

    pub unrealized_profit_rel_to_market_cap: PercentPerBlock<BasisPoints16, M>,
    pub unrealized_loss_rel_to_market_cap: PercentPerBlock<BasisPoints16, M>,
    pub net_unrealized_pnl_rel_to_market_cap: PercentPerBlock<BasisPointsSigned32, M>,
    pub nupl: LazyPerBlock<StoredF32, BasisPointsSigned32>,
}

impl RelativeFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let v3 = Version::new(3);

        let net_unrealized_pnl_rel_to_market_cap: PercentPerBlock<BasisPointsSigned32> =
            cfg.import("net_unrealized_pnl_rel_to_market_cap", v3)?;

        let nupl = LazyPerBlock::from_computed::<Bps32ToFloat>(
            &cfg.name("nupl"),
            cfg.version + v3,
            net_unrealized_pnl_rel_to_market_cap
                .bps
                .height
                .read_only_boxed_clone(),
            &net_unrealized_pnl_rel_to_market_cap.bps,
        );

        Ok(Self {
            supply_in_profit_rel_to_own_supply: cfg
                .import("supply_in_profit_rel_to_own_supply", v1)?,
            supply_in_loss_rel_to_own_supply: cfg
                .import("supply_in_loss_rel_to_own_supply", v1)?,
            unrealized_profit_rel_to_market_cap: cfg
                .import("unrealized_profit_rel_to_market_cap", v2)?,
            unrealized_loss_rel_to_market_cap: cfg
                .import("unrealized_loss_rel_to_market_cap", v2)?,
            net_unrealized_pnl_rel_to_market_cap,
            nupl,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedCore,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &unrealized.supply_in_profit.sats.height,
                supply_total_sats,
                exit,
            )?;
        self.supply_in_loss_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &unrealized.supply_in_loss.sats.height,
                supply_total_sats,
                exit,
            )?;

        self.unrealized_profit_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_profit.usd.height,
                market_cap,
                exit,
            )?;
        self.unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                market_cap,
                exit,
            )?;
        self.net_unrealized_pnl_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBps32>(
                max_from,
                &unrealized.net_unrealized_pnl.usd.height,
                market_cap,
                exit,
            )?;
        Ok(())
    }
}
