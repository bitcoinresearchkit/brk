use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    internal::{
        FiatPerBlockCumulativeWithSumsAndDeltas, LazyPerBlock, NegCentsUnsignedToDollars,
        PerBlockCumulativeWithSums, RatioCents64, RollingWindow24hPerBlock, Windows,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedMinimal;

#[derive(Clone, Traversable)]
pub struct NegRealizedLoss {
    #[traversable(flatten)]
    pub base: LazyPerBlock<Dollars, Cents>,
    pub sum: Windows<LazyPerBlock<Dollars, Cents>>,
}

#[derive(Traversable)]
pub struct RealizedSoprCore<M: StorageMode = Rw> {
    pub value_destroyed: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub ratio: RollingWindow24hPerBlock<StoredF64, M>,
}

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: RealizedMinimal<M>,

    #[traversable(wrap = "loss", rename = "negative")]
    pub neg_loss: NegRealizedLoss,
    pub net_pnl: FiatPerBlockCumulativeWithSumsAndDeltas<CentsSigned, CentsSigned, BasisPointsSigned32, M>,
    pub sopr: RealizedSoprCore<M>,
}

impl RealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let minimal = RealizedMinimal::forced_import(cfg)?;

        let neg_loss_base = LazyPerBlock::from_height_source::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_realized_loss"),
            cfg.version + Version::ONE,
            minimal.loss.base.cents.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        let neg_loss_sum = minimal.loss.sum.0.map_with_suffix(|suffix, slot| {
            LazyPerBlock::from_height_source::<NegCentsUnsignedToDollars>(
                &cfg.name(&format!("neg_realized_loss_sum_{suffix}")),
                cfg.version + Version::ONE,
                slot.cents.height.read_only_boxed_clone(),
                cfg.indexes,
            )
        });

        let neg_loss = NegRealizedLoss {
            base: neg_loss_base,
            sum: neg_loss_sum,
        };

        let net_pnl = FiatPerBlockCumulativeWithSumsAndDeltas::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            cfg.version + v1,
            Version::new(4),
            cfg.indexes,
            cfg.cached_starts,
        )?;

        let value_destroyed = PerBlockCumulativeWithSums::forced_import(
            cfg.db,
            &cfg.name("value_destroyed"),
            cfg.version + v1,
            cfg.indexes,
            cfg.cached_starts,
        )?;

        Ok(Self {
            minimal,
            neg_loss,
            net_pnl,
            sopr: RealizedSoprCore {
                value_destroyed,
                ratio: cfg.import("sopr", v1)?,
            },
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.minimal.min_stateful_len()
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &CohortState<impl RealizedOps, impl CostBasisOps>) {
        self.minimal.push_state(state);
        self.sopr.value_destroyed.base.height.push(state.realized.value_destroyed());
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.minimal.collect_vecs_mut();
        vecs.push(&mut self.sopr.value_destroyed.base.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let minimal_refs: Vec<&RealizedMinimal> = others.iter().map(|o| &o.minimal).collect();
        self.minimal
            .compute_from_stateful(starting_indexes, &minimal_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; sopr.value_destroyed.base.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part1(starting_indexes, exit)?;

        self.sopr
            .value_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        self.net_pnl.base.cents.height.compute_transform2(
            starting_indexes.height,
            &self.minimal.profit.base.cents.height,
            &self.minimal.loss.base.cents.height,
            |(i, profit, loss, ..)| {
                (
                    i,
                    CentsSigned::new(profit.inner() as i64 - loss.inner() as i64),
                )
            },
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        transfer_volume_sum_24h_cents: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part2(prices, starting_indexes, height_to_supply, exit)?;

        self.net_pnl
            .compute_rest(starting_indexes.height, exit)?;

        self.sopr
            .ratio
            ._24h
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                transfer_volume_sum_24h_cents,
                &self.sopr.value_destroyed.sum._24h.height,
                exit,
            )?;

        Ok(())
    }
}
