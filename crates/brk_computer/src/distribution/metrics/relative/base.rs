use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned32, Dollars, Height, Sats, StoredF32, Version};
use vecdb::{Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::internal::{
    Bps32ToFloat, LazyFromHeight, NegRatioDollarsBps32, PercentFromHeight, RatioDollarsBp16,
    RatioDollarsBps32, RatioSatsBp16,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Relative metrics for the Complete tier (~6 fields).
///
/// Excludes source-only fields (invested_capital_in_profit/loss_rel_to_realized_cap).
#[derive(Traversable)]
pub struct RelativeBase<M: StorageMode = Rw> {
    pub supply_in_profit_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
    pub supply_in_loss_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,

    pub unrealized_profit_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub net_unrealized_pnl_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32, M>,
    pub neg_unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32, M>,
    pub nupl: LazyFromHeight<StoredF32, BasisPointsSigned32>,
}

impl RelativeBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        let net_unrealized_pnl_rel_to_market_cap =
            cfg.import_percent_bps32("net_unrealized_pnl_rel_to_market_cap", Version::new(3))?;

        let nupl = LazyFromHeight::from_computed::<Bps32ToFloat>(
            &cfg.name("nupl"),
            cfg.version + Version::new(3),
            net_unrealized_pnl_rel_to_market_cap
                .bps
                .height
                .read_only_boxed_clone(),
            &net_unrealized_pnl_rel_to_market_cap.bps,
        );

        Ok(Self {
            supply_in_profit_rel_to_own_supply: cfg
                .import_percent_bp16("supply_in_profit_rel_to_own_supply", v1)?,
            supply_in_loss_rel_to_own_supply: cfg
                .import_percent_bp16("supply_in_loss_rel_to_own_supply", v1)?,
            unrealized_profit_rel_to_market_cap: cfg
                .import_percent_bp16("unrealized_profit_rel_to_market_cap", v2)?,
            unrealized_loss_rel_to_market_cap: cfg
                .import_percent_bp16("unrealized_loss_rel_to_market_cap", v2)?,
            net_unrealized_pnl_rel_to_market_cap,
            neg_unrealized_loss_rel_to_market_cap: cfg
                .import_percent_bps32("neg_unrealized_loss_rel_to_market_cap", Version::new(3))?,
            nupl,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
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
        self.neg_unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, NegRatioDollarsBps32>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                market_cap,
                exit,
            )?;
        Ok(())
    }
}
