use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredI8, Version};
use vecdb::{AnyVec, Database, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    indexes,
    internal::{PerBlock, Price, RatioPerBlockPercentiles},
};

#[derive(Traversable)]
pub struct RarityMeterInner<M: StorageMode = Rw> {
    pub pct0_5: Price<PerBlock<Cents, M>>,
    pub pct1: Price<PerBlock<Cents, M>>,
    pub pct2: Price<PerBlock<Cents, M>>,
    pub pct5: Price<PerBlock<Cents, M>>,
    pub pct95: Price<PerBlock<Cents, M>>,
    pub pct98: Price<PerBlock<Cents, M>>,
    pub pct99: Price<PerBlock<Cents, M>>,
    pub pct99_5: Price<PerBlock<Cents, M>>,
    pub index: PerBlock<StoredI8, M>,
    pub score: PerBlock<StoredI8, M>,
}

impl RarityMeterInner {
    pub(crate) fn forced_import(
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            pct0_5: Price::forced_import(db, &format!("{prefix}_pct0_5"), version, indexes)?,
            pct1: Price::forced_import(db, &format!("{prefix}_pct01"), version, indexes)?,
            pct2: Price::forced_import(db, &format!("{prefix}_pct02"), version, indexes)?,
            pct5: Price::forced_import(db, &format!("{prefix}_pct05"), version, indexes)?,
            pct95: Price::forced_import(db, &format!("{prefix}_pct95"), version, indexes)?,
            pct98: Price::forced_import(db, &format!("{prefix}_pct98"), version, indexes)?,
            pct99: Price::forced_import(db, &format!("{prefix}_pct99"), version, indexes)?,
            pct99_5: Price::forced_import(db, &format!("{prefix}_pct99_5"), version, indexes)?,
            index: PerBlock::forced_import(db, &format!("{prefix}_index"), version, indexes)?,
            score: PerBlock::forced_import(db, &format!("{prefix}_score"), version, indexes)?,
        })
    }

    pub(super) fn compute(
        &mut self,
        models: &[&RatioPerBlockPercentiles],
        spot: &impl ReadableVec<Height, Cents>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let gather = |f: fn(&RatioPerBlockPercentiles) -> &_| -> Vec<_> {
            models.iter().map(|m| f(m)).collect()
        };

        // Lower percentiles: max across all models (tightest lower bound)
        self.pct0_5.cents.height.compute_max_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct0_5.price.cents.height),
            exit,
        )?;
        self.pct1.cents.height.compute_max_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct1.price.cents.height),
            exit,
        )?;
        self.pct2.cents.height.compute_max_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct2.price.cents.height),
            exit,
        )?;
        self.pct5.cents.height.compute_max_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct5.price.cents.height),
            exit,
        )?;

        // Upper percentiles: min across all models (tightest upper bound)
        self.pct95.cents.height.compute_min_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct95.price.cents.height),
            exit,
        )?;
        self.pct98.cents.height.compute_min_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct98.price.cents.height),
            exit,
        )?;
        self.pct99.cents.height.compute_min_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct99.price.cents.height),
            exit,
        )?;
        self.pct99_5.cents.height.compute_min_of_others(
            starting_indexes.height,
            &gather(|m| &m.pct99_5.price.cents.height),
            exit,
        )?;

        self.compute_index(spot, starting_indexes, exit)?;

        self.compute_score(models, spot, starting_indexes, exit)?;

        Ok(())
    }

    fn compute_index(
        &mut self,
        spot: &impl ReadableVec<Height, Cents>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let bands = [
            &self.pct0_5.cents.height,
            &self.pct1.cents.height,
            &self.pct2.cents.height,
            &self.pct5.cents.height,
            &self.pct95.cents.height,
            &self.pct98.cents.height,
            &self.pct99.cents.height,
            &self.pct99_5.cents.height,
        ];

        let dep_version: Version =
            bands.iter().map(|b| b.version()).sum::<Version>() + spot.version();

        self.index
            .height
            .validate_computed_version_or_reset(dep_version)?;
        self.index
            .height
            .truncate_if_needed(starting_indexes.height)?;

        self.index.height.repeat_until_complete(exit, |vec| {
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

                if price < b[3][j] {
                    score -= 1;
                }
                if price < b[2][j] {
                    score -= 1;
                }
                if price < b[1][j] {
                    score -= 1;
                }
                if price < b[0][j] {
                    score -= 1;
                }
                if price > b[4][j] {
                    score += 1;
                }
                if price > b[5][j] {
                    score += 1;
                }
                if price > b[6][j] {
                    score += 1;
                }
                if price > b[7][j] {
                    score += 1;
                }

                vec.push(StoredI8::new(score));
            }

            Ok(())
        })?;

        Ok(())
    }

    fn compute_score(
        &mut self,
        models: &[&RatioPerBlockPercentiles],
        spot: &impl ReadableVec<Height, Cents>,
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

        self.score
            .height
            .validate_computed_version_or_reset(dep_version)?;
        self.score
            .height
            .truncate_if_needed(starting_indexes.height)?;

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
                    if price < model[3][j] {
                        total -= 1;
                    }
                    if price < model[2][j] {
                        total -= 1;
                    }
                    if price < model[1][j] {
                        total -= 1;
                    }
                    if price < model[0][j] {
                        total -= 1;
                    }
                    if price > model[4][j] {
                        total += 1;
                    }
                    if price > model[5][j] {
                        total += 1;
                    }
                    if price > model[6][j] {
                        total += 1;
                    }
                    if price > model[7][j] {
                        total += 1;
                    }
                }

                vec.push(StoredI8::new(total));
            }

            Ok(())
        })?;

        Ok(())
    }
}
