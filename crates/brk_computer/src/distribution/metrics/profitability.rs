use brk_cohort::{Loss, Profit, ProfitabilityRange};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Cents, Dollars, Indexes, Sats, StoredF32, Version,
};
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, Rw, StorageMode, WritableVec};

use crate::{
    indexes,
    internal::{
        AmountPerBlock, AmountPerBlockWithDeltas, CachedWindowStarts, Identity, LazyPerBlock,
        PerBlock, PriceWithRatioPerBlock, RatioPerBlock,
    },
    prices,
};

#[derive(Traversable)]
pub struct WithSth<All, Sth = All> {
    pub all: All,
    pub sth: Sth,
}

#[derive(Traversable)]
pub struct ProfitabilityBucket<M: StorageMode = Rw> {
    pub supply: WithSth<AmountPerBlockWithDeltas<M>, AmountPerBlock<M>>,
    pub realized_cap: WithSth<PerBlock<Dollars, M>>,
    pub realized_price: PriceWithRatioPerBlock<M>,
    pub mvrv: LazyPerBlock<StoredF32>,
    pub nupl: RatioPerBlock<BasisPointsSigned32, M>,
}

impl<M: StorageMode> ProfitabilityBucket<M> {
    fn min_len(&self) -> usize {
        self.supply
            .all
            .sats
            .height
            .len()
            .min(self.realized_cap.all.height.len())
    }
}

impl ProfitabilityBucket {
    fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let realized_price = PriceWithRatioPerBlock::forced_import(
            db,
            &format!("{name}_realized_price"),
            version,
            indexes,
        )?;

        let mvrv = LazyPerBlock::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &format!("{name}_mvrv"),
            version,
            &realized_price.ratio,
        );

        Ok(Self {
            supply: WithSth {
                all: AmountPerBlockWithDeltas::forced_import(
                    db,
                    &format!("{name}_supply"),
                    version,
                    indexes,
                    cached_starts,
                )?,
                sth: AmountPerBlock::forced_import(
                    db,
                    &format!("{name}_sth_supply"),
                    version,
                    indexes,
                )?,
            },
            realized_cap: WithSth {
                all: PerBlock::forced_import(
                    db,
                    &format!("{name}_realized_cap"),
                    version,
                    indexes,
                )?,
                sth: PerBlock::forced_import(
                    db,
                    &format!("{name}_sth_realized_cap"),
                    version,
                    indexes,
                )?,
            },
            realized_price,
            mvrv,
            nupl: RatioPerBlock::forced_import_raw(
                db,
                &format!("{name}_nupl"),
                version + Version::ONE,
                indexes,
            )?,
        })
    }

    #[inline(always)]
    pub(crate) fn push(
        &mut self,
        supply: Sats,
        sth_supply: Sats,
        realized_cap: Dollars,
        sth_realized_cap: Dollars,
    ) {
        self.supply.all.sats.height.push(supply);
        self.supply.sth.sats.height.push(sth_supply);
        self.realized_cap.all.height.push(realized_cap);
        self.realized_cap.sth.height.push(sth_realized_cap);
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let max_from = starting_indexes.height;

        self.supply.all.compute(prices, max_from, exit)?;
        self.supply.sth.compute(prices, max_from, exit)?;

        // Realized price cents = realized_cap_cents × ONE_BTC / supply_sats
        self.realized_price.cents.height.compute_transform2(
            max_from,
            &self.realized_cap.all.height,
            &self.supply.all.sats.height,
            |(i, cap_dollars, supply_sats, ..)| {
                let cap_cents = Cents::from(cap_dollars).as_u128();
                let supply = supply_sats.as_u128();
                if supply == 0 {
                    (i, Cents::ZERO)
                } else {
                    (i, Cents::from(cap_cents * Sats::ONE_BTC_U128 / supply))
                }
            },
            exit,
        )?;

        // Ratio (spot / realized_price) → feeds MVRV lazily
        self.realized_price
            .compute_ratio(starting_indexes, &prices.spot.cents.height, exit)?;

        // NUPL = (spot - realized_price) / spot
        self.nupl.bps.height.compute_transform2(
            max_from,
            &prices.spot.cents.height,
            &self.realized_price.cents.height,
            |(i, spot, realized, ..)| {
                let p = spot.as_u128();
                if p == 0 {
                    (i, BasisPointsSigned32::ZERO)
                } else {
                    let rp = realized.as_u128();
                    let bps = ((p as i128 - rp as i128) * 10000) / p as i128;
                    (i, BasisPointsSigned32::from(bps as i32))
                }
            },
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply.all.inner.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply.all.inner.cents.height as &mut dyn AnyStoredVec,
            &mut self.supply.sth.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply.sth.cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_cap.all.height as &mut dyn AnyStoredVec,
            &mut self.realized_cap.sth.height as &mut dyn AnyStoredVec,
            &mut self.realized_price.cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_price.bps.height as &mut dyn AnyStoredVec,
            &mut self.nupl.bps.height as &mut dyn AnyStoredVec,
        ]
    }
}

/// All profitability metrics: 25 ranges + 14 profit thresholds + 9 loss thresholds.
#[derive(Traversable)]
pub struct ProfitabilityMetrics<M: StorageMode = Rw> {
    pub range: ProfitabilityRange<ProfitabilityBucket<M>>,
    pub profit: Profit<ProfitabilityBucket<M>>,
    pub loss: Loss<ProfitabilityBucket<M>>,
}

impl<M: StorageMode> ProfitabilityMetrics<M> {
    pub fn iter(&self) -> impl Iterator<Item = &ProfitabilityBucket<M>> {
        self.range
            .iter()
            .chain(self.profit.iter())
            .chain(self.loss.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ProfitabilityBucket<M>> {
        self.range
            .iter_mut()
            .chain(self.profit.iter_mut())
            .chain(self.loss.iter_mut())
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.iter().map(|b| b.min_len()).min().unwrap_or(0)
    }
}

impl ProfitabilityMetrics {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let range = ProfitabilityRange::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes, cached_starts)
        })?;

        let profit = Profit::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes, cached_starts)
        })?;

        let loss = Loss::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes, cached_starts)
        })?;

        Ok(Self {
            range,
            profit,
            loss,
        })
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.iter_mut()
            .try_for_each(|b| b.compute(prices, starting_indexes, exit))
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = Vec::new();
        for bucket in self.iter_mut() {
            vecs.extend(bucket.collect_all_vecs_mut());
        }
        vecs
    }
}
