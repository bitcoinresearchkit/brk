use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredI8, Version};
use vecdb::{AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    cointime,
    distribution,
    indexes,
    internal::{PerBlock, Price, RatioPerBlockPercentiles},
    prices,
};

#[derive(Traversable)]
pub struct Thermometer<M: StorageMode = Rw> {
    pub pct0_5: Price<PerBlock<Cents, M>>,
    pub pct1: Price<PerBlock<Cents, M>>,
    pub pct2: Price<PerBlock<Cents, M>>,
    pub pct5: Price<PerBlock<Cents, M>>,
    pub pct95: Price<PerBlock<Cents, M>>,
    pub pct98: Price<PerBlock<Cents, M>>,
    pub pct99: Price<PerBlock<Cents, M>>,
    pub pct99_5: Price<PerBlock<Cents, M>>,
    pub zone: PerBlock<StoredI8, M>,
    pub score: PerBlock<StoredI8, M>,
}

const VERSION: Version = Version::new(2);

impl Thermometer {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            pct0_5: Price::forced_import(db, "thermometer_pct0_5", v, indexes)?,
            pct1: Price::forced_import(db, "thermometer_pct01", v, indexes)?,
            pct2: Price::forced_import(db, "thermometer_pct02", v, indexes)?,
            pct5: Price::forced_import(db, "thermometer_pct05", v, indexes)?,
            pct95: Price::forced_import(db, "thermometer_pct95", v, indexes)?,
            pct98: Price::forced_import(db, "thermometer_pct98", v, indexes)?,
            pct99: Price::forced_import(db, "thermometer_pct99", v, indexes)?,
            pct99_5: Price::forced_import(db, "thermometer_pct99_5", v, indexes)?,
            zone: PerBlock::forced_import(db, "thermometer_zone", v, indexes)?,
            score: PerBlock::forced_import(db, "thermometer_score", v, indexes)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        distribution: &distribution::Vecs,
        cointime: &cointime::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let realized = &distribution.utxo_cohorts.all.metrics.realized;
        let ct = &cointime.prices;

        let sth_realized = &distribution.utxo_cohorts.sth.metrics.realized;
        let lth_realized = &distribution.utxo_cohorts.lth.metrics.realized;

        let models: [&RatioPerBlockPercentiles; 10] = [
            &realized.price_ratio_percentiles,
            &realized.investor.price.percentiles,
            &sth_realized.price_ratio_percentiles,
            &sth_realized.investor.price.percentiles,
            &lth_realized.price_ratio_percentiles,
            &lth_realized.investor.price.percentiles,
            &ct.vaulted.percentiles,
            &ct.active.percentiles,
            &ct.true_market_mean.percentiles,
            &ct.cointime.percentiles,
        ];

        macro_rules! sources {
            ($pct:ident) => {
                models.each_ref().map(|m| &m.$pct.price.cents.height)
            };
        }

        // Lower percentiles: max across all models (tightest lower bound)
        self.pct0_5.cents.height.compute_max_of_others(starting_indexes.height, &sources!(pct0_5), exit)?;
        self.pct1.cents.height.compute_max_of_others(starting_indexes.height, &sources!(pct1), exit)?;
        self.pct2.cents.height.compute_max_of_others(starting_indexes.height, &sources!(pct2), exit)?;
        self.pct5.cents.height.compute_max_of_others(starting_indexes.height, &sources!(pct5), exit)?;

        // Upper percentiles: min across all models (tightest upper bound)
        self.pct95.cents.height.compute_min_of_others(starting_indexes.height, &sources!(pct95), exit)?;
        self.pct98.cents.height.compute_min_of_others(starting_indexes.height, &sources!(pct98), exit)?;
        self.pct99.cents.height.compute_min_of_others(starting_indexes.height, &sources!(pct99), exit)?;
        self.pct99_5.cents.height.compute_min_of_others(starting_indexes.height, &sources!(pct99_5), exit)?;

        let spot = &prices.spot.cents.height;

        // Zone: spot vs own envelope bands (-4 to +4)
        self.compute_zone(spot, starting_indexes, exit)?;

        // Temperature: per-model band crossings (-40 to +40)
        self.compute_score(&models, spot, starting_indexes, exit)?;

        Ok(())
    }

    fn compute_zone(
        &mut self,
        spot: &EagerVec<PcoVec<Height, Cents>>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let bands: [&_; 8] = [
            &self.pct0_5.cents.height,
            &self.pct1.cents.height,
            &self.pct2.cents.height,
            &self.pct5.cents.height,
            &self.pct95.cents.height,
            &self.pct98.cents.height,
            &self.pct99.cents.height,
            &self.pct99_5.cents.height,
        ];

        let dep_version: Version = bands.iter().map(|b| b.version()).sum::<Version>() + spot.version();

        self.zone.height.validate_computed_version_or_reset(dep_version)?;
        self.zone.height.truncate_if_needed(starting_indexes.height)?;

        self.zone.height.repeat_until_complete(exit, |vec| {
            let skip = vec.len();
            let source_end = bands.iter().map(|b| b.len()).min().unwrap().min(spot.len());
            let end = vec.batch_end(source_end);

            if skip >= end {
                return Ok(());
            }

            let spot_batch = spot.collect_range_at(skip, end);
            let b: [Vec<Cents>; 8] = bands.each_ref().map(|v| v.collect_range_at(skip, end));

            for j in 0..(end - skip) {
                let price = spot_batch[j];
                let mut score: i8 = 0;

                if price < b[3][j] { score -= 1; }
                if price < b[2][j] { score -= 1; }
                if price < b[1][j] { score -= 1; }
                if price < b[0][j] { score -= 1; }
                if price > b[4][j] { score += 1; }
                if price > b[5][j] { score += 1; }
                if price > b[6][j] { score += 1; }
                if price > b[7][j] { score += 1; }

                vec.push(StoredI8::new(score));
            }

            Ok(())
        })?;

        Ok(())
    }

    fn compute_score(
        &mut self,
        models: &[&RatioPerBlockPercentiles; 10],
        spot: &EagerVec<PcoVec<Height, Cents>>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version: Version = models
            .iter()
            .map(|p| {
                p.pct0_5.price.cents.height.version()
                    + p.pct1.price.cents.height.version()
                    + p.pct2.price.cents.height.version()
                    + p.pct5.price.cents.height.version()
                    + p.pct95.price.cents.height.version()
                    + p.pct98.price.cents.height.version()
                    + p.pct99.price.cents.height.version()
                    + p.pct99_5.price.cents.height.version()
            })
            .sum::<Version>()
            + spot.version();

        self.score.height.validate_computed_version_or_reset(dep_version)?;
        self.score.height.truncate_if_needed(starting_indexes.height)?;

        self.score.height.repeat_until_complete(exit, |vec| {
            let skip = vec.len();
            let source_end = models
                .iter()
                .flat_map(|p| {
                    [
                        p.pct0_5.price.cents.height.len(),
                        p.pct1.price.cents.height.len(),
                        p.pct2.price.cents.height.len(),
                        p.pct5.price.cents.height.len(),
                        p.pct95.price.cents.height.len(),
                        p.pct98.price.cents.height.len(),
                        p.pct99.price.cents.height.len(),
                        p.pct99_5.price.cents.height.len(),
                    ]
                })
                .min()
                .unwrap()
                .min(spot.len());
            let end = vec.batch_end(source_end);

            if skip >= end {
                return Ok(());
            }

            let spot_batch = spot.collect_range_at(skip, end);

            let bands: Vec<[Vec<Cents>; 8]> = models
                .iter()
                .map(|p| {
                    [
                        p.pct0_5.price.cents.height.collect_range_at(skip, end),
                        p.pct1.price.cents.height.collect_range_at(skip, end),
                        p.pct2.price.cents.height.collect_range_at(skip, end),
                        p.pct5.price.cents.height.collect_range_at(skip, end),
                        p.pct95.price.cents.height.collect_range_at(skip, end),
                        p.pct98.price.cents.height.collect_range_at(skip, end),
                        p.pct99.price.cents.height.collect_range_at(skip, end),
                        p.pct99_5.price.cents.height.collect_range_at(skip, end),
                    ]
                })
                .collect();

            for j in 0..(end - skip) {
                let price = spot_batch[j];
                let mut total: i8 = 0;

                for model in &bands {
                    if price < model[3][j] { total -= 1; }
                    if price < model[2][j] { total -= 1; }
                    if price < model[1][j] { total -= 1; }
                    if price < model[0][j] { total -= 1; }
                    if price > model[4][j] { total += 1; }
                    if price > model[5][j] { total += 1; }
                    if price > model[6][j] { total += 1; }
                    if price > model[7][j] { total += 1; }
                }

                vec.push(StoredI8::new(total));
            }

            Ok(())
        })?;

        Ok(())
    }
}
