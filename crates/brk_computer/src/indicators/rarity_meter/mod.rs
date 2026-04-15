mod inner;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{distribution, indexes, prices};

pub use inner::RarityMeterInner;

#[derive(Traversable)]
pub struct RarityMeter<M: StorageMode = Rw> {
    pub full: RarityMeterInner<M>,
    pub local: RarityMeterInner<M>,
    pub cycle: RarityMeterInner<M>,
}

const VERSION: Version = Version::new(4);

impl RarityMeter {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            full: RarityMeterInner::forced_import(db, "rarity_meter", v, indexes)?,
            local: RarityMeterInner::forced_import(db, "local_rarity_meter", v, indexes)?,
            cycle: RarityMeterInner::forced_import(db, "cycle_rarity_meter", v, indexes)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        distribution: &distribution::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let realized = &distribution.utxo_cohorts.all.metrics.realized;
        let sth_realized = &distribution.utxo_cohorts.sth.metrics.realized;
        let lth_realized = &distribution.utxo_cohorts.lth.metrics.realized;
        let spot = &prices.spot.cents.height;

        // Full: all + sth + lth (rp + ip), 6 models
        self.full.compute(
            &[
                &realized.price_ratio_percentiles,
                &realized.investor.price.percentiles,
                &sth_realized.price_ratio_percentiles,
                &sth_realized.investor.price.percentiles,
                &lth_realized.price_ratio_percentiles,
                &lth_realized.investor.price.percentiles,
            ],
            spot,
            starting_indexes,
            exit,
        )?;

        // Local: sth only, 2 models
        self.local.compute(
            &[
                &sth_realized.price_ratio_percentiles,
                &sth_realized.investor.price.percentiles,
            ],
            spot,
            starting_indexes,
            exit,
        )?;

        // Cycle: all + lth, 4 models
        self.cycle.compute(
            &[
                &realized.price_ratio_percentiles,
                &realized.investor.price.percentiles,
                &lth_realized.price_ratio_percentiles,
                &lth_realized.investor.price.percentiles,
            ],
            spot,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }
}
