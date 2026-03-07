use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints16, BasisPointsSigned32, Dollars, Height, Sats, StoredF32, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::internal::{
    Bps32ToFloat, LazyFromHeight, PercentFromHeight, RatioDollarsBp16, RatioDollarsBps32,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedCore};

use super::RelativeBase;

/// Full relative metrics (Source/Extended tier).
#[derive(Deref, DerefMut, Traversable)]
pub struct RelativeFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase<M>,

    pub unrealized_profit_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub net_unrealized_pnl_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32, M>,
    pub nupl: LazyFromHeight<StoredF32, BasisPointsSigned32>,
}

impl RelativeFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let base = RelativeBase::forced_import(cfg)?;

        let v2 = Version::new(2);
        let v3 = Version::new(3);

        let net_unrealized_pnl_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32> =
            cfg.import("net_unrealized_pnl_rel_to_market_cap", v3)?;

        let nupl = LazyFromHeight::from_computed::<Bps32ToFloat>(
            &cfg.name("nupl"),
            cfg.version + v3,
            net_unrealized_pnl_rel_to_market_cap
                .bps
                .height
                .read_only_boxed_clone(),
            &net_unrealized_pnl_rel_to_market_cap.bps,
        );

        Ok(Self {
            base,
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
        self.base.compute(
            max_from,
            unrealized,
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
