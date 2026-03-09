use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{
        metrics::{ImportConfig, unrealized::UnrealizedMinimal},
        state::UnrealizedState,
    },
    internal::{
        CentsSubtractToCentsSigned, FiatPerBlock, LazyPerBlock, NegCentsUnsignedToDollars,
    },
    prices,
};

use brk_types::Dollars;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: UnrealizedMinimal<M>,

    pub unrealized_profit: FiatPerBlock<Cents, M>,
    pub unrealized_loss: FiatPerBlock<Cents, M>,

    pub neg_unrealized_loss: LazyPerBlock<Dollars, Cents>,

    pub net_unrealized_pnl: FiatPerBlock<CentsSigned, M>,
}

impl UnrealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;

        let minimal = UnrealizedMinimal::forced_import(cfg)?;

        let unrealized_profit = cfg.import("unrealized_profit", v0)?;
        let unrealized_loss: FiatPerBlock<Cents> = cfg.import("unrealized_loss", v0)?;

        let neg_unrealized_loss = LazyPerBlock::from_computed::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            unrealized_loss.cents.height.read_only_boxed_clone(),
            &unrealized_loss.cents,
        );

        let net_unrealized_pnl = cfg.import("net_unrealized_pnl", v0)?;

        Ok(Self {
            minimal,
            unrealized_profit,
            unrealized_loss,
            neg_unrealized_loss,
            net_unrealized_pnl,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.minimal
            .min_stateful_height_len()
            .min(self.unrealized_profit.cents.height.len())
            .min(self.unrealized_loss.cents.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        height_state: &UnrealizedState,
    ) -> Result<()> {
        self.minimal.truncate_push(height, height_state)?;
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
        let mut vecs = self.minimal.collect_vecs_mut();
        vecs.push(&mut self.unrealized_profit.cents.height);
        vecs.push(&mut self.unrealized_loss.cents.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let minimal_refs: Vec<&UnrealizedMinimal> = others.iter().map(|o| &o.minimal).collect();
        self.minimal
            .compute_from_sources(starting_indexes, &minimal_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; unrealized_profit.cents.height);
        sum_others!(self, starting_indexes, others, exit; unrealized_loss.cents.height);

        Ok(())
    }

    /// Compute derived metrics from stored values.
    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest(prices, starting_indexes.height, exit)?;

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
