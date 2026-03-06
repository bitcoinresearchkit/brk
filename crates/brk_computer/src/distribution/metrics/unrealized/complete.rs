use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Height, Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::state::UnrealizedState,
    internal::{
        CentsSubtractToCentsSigned, FiatFromHeight, LazyFromHeight, NegCentsUnsignedToDollars,
        ValueFromHeight,
    },
};

use brk_types::Dollars;

use crate::distribution::metrics::ImportConfig;

/// Unrealized metrics for the Complete tier (~6 fields).
///
/// Excludes source-only fields (invested_capital, raw BytesVecs, unrealized_gross_pnl)
/// and extended-only fields (pain_index, greed_index, net_sentiment).
#[derive(Traversable)]
pub struct UnrealizedComplete<M: StorageMode = Rw> {
    pub supply_in_profit: ValueFromHeight<M>,
    pub supply_in_loss: ValueFromHeight<M>,

    pub unrealized_profit: FiatFromHeight<Cents, M>,
    pub unrealized_loss: FiatFromHeight<Cents, M>,

    pub neg_unrealized_loss: LazyFromHeight<Dollars, Cents>,

    pub net_unrealized_pnl: FiatFromHeight<CentsSigned, M>,
}

impl UnrealizedComplete {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let supply_in_profit = cfg.import_value("supply_in_profit", v0)?;
        let supply_in_loss = cfg.import_value("supply_in_loss", v0)?;

        let unrealized_profit = cfg.import_fiat("unrealized_profit", v0)?;
        let unrealized_loss = cfg.import_fiat("unrealized_loss", v0)?;

        let neg_unrealized_loss = LazyFromHeight::from_computed::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            unrealized_loss.cents.height.read_only_boxed_clone(),
            &unrealized_loss.cents,
        );

        let net_unrealized_pnl = cfg.import_fiat("net_unrealized_pnl", v0)?;

        Ok(Self {
            supply_in_profit,
            supply_in_loss,
            unrealized_profit,
            unrealized_loss,
            neg_unrealized_loss,
            net_unrealized_pnl,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .sats
            .height
            .len()
            .min(self.supply_in_loss.sats.height.len())
            .min(self.unrealized_profit.cents.height.len())
            .min(self.unrealized_loss.cents.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        height_state: &UnrealizedState,
    ) -> Result<()> {
        self.supply_in_profit
            .sats
            .height
            .truncate_push(height, height_state.supply_in_profit)?;
        self.supply_in_loss
            .sats
            .height
            .truncate_push(height, height_state.supply_in_loss)?;
        self.unrealized_profit
            .cents
            .height
            .truncate_push(height, height_state.unrealized_profit)?;
        self.unrealized_loss
            .cents
            .height
            .truncate_push(height, height_state.unrealized_loss)?;

        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.cents.height,
            &mut self.unrealized_loss.cents.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! sum_others {
            ($($field:tt).+) => {
                self.$($field).+.compute_sum_of_others(
                    starting_indexes.height,
                    &others.iter().map(|v| &v.$($field).+).collect::<Vec<_>>(),
                    exit,
                )?
            };
        }

        sum_others!(supply_in_profit.sats.height);
        sum_others!(supply_in_loss.sats.height);
        sum_others!(unrealized_profit.cents.height);
        sum_others!(unrealized_loss.cents.height);

        Ok(())
    }

    /// Compute derived metrics from stored values.
    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.net_unrealized_pnl
            .cents
            .height
            .compute_binary::<Cents, Cents, CentsSubtractToCentsSigned>(
                starting_indexes.height,
                &self.unrealized_profit.cents.height,
                &self.unrealized_loss.cents.height,
                exit,
            )?;

        Ok(())
    }
}
