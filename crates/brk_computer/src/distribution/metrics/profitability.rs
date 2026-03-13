use brk_cohort::{Loss, Profit, ProfitabilityRange};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use vecdb::{AnyStoredVec, AnyVec, Database, Rw, StorageMode, WritableVec};

use crate::{indexes, internal::PerBlock};

/// Supply + realized cap for a single profitability bucket.
#[derive(Traversable)]
pub struct ProfitabilityBucket<M: StorageMode = Rw> {
    pub supply: PerBlock<Sats, M>,
    pub realized_cap: PerBlock<Dollars, M>,
}

impl<M: StorageMode> ProfitabilityBucket<M> {
    fn min_len(&self) -> usize {
        self.supply.height.len().min(self.realized_cap.height.len())
    }
}

impl ProfitabilityBucket {
    fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            supply: PerBlock::forced_import(
                db,
                &format!("{name}_supply"),
                version,
                indexes,
            )?,
            realized_cap: PerBlock::forced_import(
                db,
                &format!("{name}_realized_cap"),
                version,
                indexes,
            )?,
        })
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        supply: Sats,
        realized_cap: Dollars,
    ) -> Result<()> {
        self.supply.height.truncate_push(height, supply)?;
        self.realized_cap
            .height
            .truncate_push(height, realized_cap)?;
        Ok(())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply.height as &mut dyn AnyStoredVec,
            &mut self.realized_cap.height as &mut dyn AnyStoredVec,
        ]
    }
}

/// All profitability metrics: 25 ranges + 15 profit thresholds + 10 loss thresholds.
#[derive(Traversable)]
pub struct ProfitabilityMetrics<M: StorageMode = Rw> {
    pub range: ProfitabilityRange<ProfitabilityBucket<M>>,
    pub profit: Profit<ProfitabilityBucket<M>>,
    pub loss: Loss<ProfitabilityBucket<M>>,
}

impl<M: StorageMode> ProfitabilityMetrics<M> {
    pub(crate) fn min_stateful_len(&self) -> usize {
        self.range.iter()
            .chain(self.profit.iter())
            .chain(self.loss.iter())
            .map(|b| b.min_len())
            .min()
            .unwrap_or(0)
    }
}

impl ProfitabilityMetrics {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let range = ProfitabilityRange::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes)
        })?;

        let profit = Profit::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes)
        })?;

        let loss = Loss::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes)
        })?;

        Ok(Self {
            range,
            profit,
            loss,
        })
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = Vec::new();
        for bucket in self.range.iter_mut() {
            vecs.extend(bucket.collect_all_vecs_mut());
        }
        for bucket in self.profit.iter_mut() {
            vecs.extend(bucket.collect_all_vecs_mut());
        }
        for bucket in self.loss.iter_mut() {
            vecs.extend(bucket.collect_all_vecs_mut());
        }
        vecs
    }

}
