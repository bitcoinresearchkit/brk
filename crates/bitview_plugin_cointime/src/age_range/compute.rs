use brk_error::Result;

use bitview_cohort::{AgeRange, AgeRangeId};
use bitview_plugin_indexer::Indexer;
use brk_exit::Exit;
use brk_types::{Bitcoin, BoundedRatio, Height, Sats, StoredF64, Version};
use vecdb::{CheckedSub, ColumnId, ReadableVec, StoredVec, WritableVec};

use super::Vecs;

const HOURS_PER_DAY: f64 = 24.0;
const WRITE_INTERVAL: usize = 10_000;

pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    distribution: &bitview_plugin_distribution::Vecs,
    exit: &Exit,
) -> Result<()> {
    let starting_height = indexer.safe_lengths().height;
    let transfer_volumes = AgeRange::from_fn(|id| {
        &id.select(
            &distribution
                .cohorts
                .activity
                .transfer_volume
                .cohorts
                .age
                .range,
        )
        .block
        .sats
    });
    let coindays_destroyed = AgeRange::from_fn(|id| {
        &id.select(
            &distribution
                .cohorts
                .activity
                .coindays_destroyed
                .cohorts
                .age
                .range,
        )
        .block
    });
    let coindays_created = &distribution.coindays_created.cumulative;

    vecs.compute_consumed(
        starting_height,
        &transfer_volumes,
        &coindays_destroyed,
        exit,
    )?;
    vecs.compute_rest(starting_height, coindays_created, exit)
}

impl Vecs {
    fn compute_consumed<V, D>(
        &mut self,
        starting_height: Height,
        transfer_volumes: &AgeRange<&V>,
        source_coindays_destroyed: &AgeRange<&D>,
        exit: &Exit,
    ) -> Result<()>
    where
        V: ReadableVec<Height, Sats>,
        D: ReadableVec<Height, StoredF64>,
    {
        let version = Version::combine_all(
            transfer_volumes
                .iter()
                .map(|vec| vec.version())
                .chain(source_coindays_destroyed.iter().map(|vec| vec.version())),
        );
        let source_end = transfer_volumes
            .iter()
            .map(|vec| vec.len())
            .chain(source_coindays_destroyed.iter().map(|vec| vec.len()))
            .min()
            .unwrap_or_default();
        let bounds = age_bounds_days();
        self.coindays_consumed.cumulative.compute_batched_to(
            starting_height,
            source_end,
            version,
            WRITE_INTERVAL,
            |target, range| {
                let mut cumulative = target
                    .collect_last()
                    .unwrap_or_else(|| AgeRangeId::from_fn(|_| StoredF64::default()));
                let transfer_batches = AgeRange::from_fn(|id| {
                    id.select(transfer_volumes)
                        .collect_range_at(range.start, range.end)
                });
                let destroyed_batches = AgeRange::from_fn(|id| {
                    id.select(source_coindays_destroyed)
                        .collect_range_at(range.start, range.end)
                });

                for offset in 0..range.len() {
                    let volumes_btc = AgeRange::from_fn(|id| {
                        f64::from(Bitcoin::from(id.select(&transfer_batches)[offset]))
                    });
                    let coindays_destroyed =
                        AgeRange::from_fn(|id| f64::from(id.select(&destroyed_batches)[offset]));
                    let consumed =
                        allocate_consumed_coindays(volumes_btc, coindays_destroyed, &bounds);
                    target.push(AgeRangeId::from_fn(|column| {
                        let value = column.get_mut(&mut cumulative);
                        *value += StoredF64::from(*column.select(&consumed));
                        *value
                    }));
                }

                Ok(())
            },
            exit,
        )?;
        Ok(())
    }

    fn compute_rest<C>(&mut self, starting_height: Height, created: &C, exit: &Exit) -> Result<()>
    where
        C: ReadableVec<Height, AgeRange<StoredF64>>,
    {
        {
            let consumed = self.coindays_consumed.cumulative.read_only_clone();
            self.coindays_stored.cumulative.compute_transform2_batched(
                starting_height,
                created,
                &consumed,
                WRITE_INTERVAL,
                |(height, created, consumed, ..)| {
                    let stored = AgeRangeId::from_fn(|column| {
                        (*column.get(&created))
                            .checked_sub(*column.get(&consumed))
                            .expect("coindays stored underflow")
                    });
                    (height, stored)
                },
                exit,
            )?;
            self.activity.height.compute_transform2_batched(
                starting_height,
                &consumed,
                created,
                WRITE_INTERVAL,
                |(height, consumed, created, ..)| {
                    let wakefulness = AgeRangeId::from_fn(|column| {
                        BoundedRatio::from(
                            f64::from(*column.get(&consumed)) / f64::from(*column.get(&created)),
                        )
                    });
                    (height, wakefulness)
                },
                exit,
            )?;
        }

        Ok(())
    }
}

fn age_bounds_days() -> AgeRange<(f64, f64)> {
    AgeRange::from_fn(|id| {
        let bound = id.bounds();
        let lower = bound.start as f64 / HOURS_PER_DAY;
        let width = if id == AgeRangeId::Over15Y {
            0.0
        } else {
            (bound.end - bound.start) as f64 / HOURS_PER_DAY
        };
        (lower, width)
    })
}

fn allocate_consumed_coindays(
    transfer_volume_btc: AgeRange<f64>,
    coindays_destroyed: AgeRange<f64>,
    bounds: &AgeRange<(f64, f64)>,
) -> AgeRange<f64> {
    let mut result = AgeRange::default();
    let mut older_transfer_volume = 0.0;

    for &id in AgeRangeId::ALL.iter().rev() {
        let (lower_days, width_days) = *id.select(bounds);
        let transfer_volume = *id.select(&transfer_volume_btc);
        let within_cohort =
            (*id.select(&coindays_destroyed) - transfer_volume * lower_days).max(0.0);
        *id.select_mut(&mut result) = within_cohort + older_transfer_volume * width_days;
        older_transfer_volume += transfer_volume;
    }

    result
}

#[cfg(test)]
mod tests {
    use bitview_cohort::AGE_RANGE_COUNT;

    use super::*;

    #[test]
    fn destruction_at_a_boundary_stays_in_the_ranges_already_traversed() {
        let mut volumes = AgeRange::default();
        let mut cdd = AgeRange::default();
        volumes._1d_to_1w = 1.0;
        cdd._1d_to_1w = 1.0;

        let allocated = allocate_consumed_coindays(volumes, cdd, &age_bounds_days());

        assert!((allocated.under_1h - 1.0 / HOURS_PER_DAY).abs() < 1e-12);
        assert!((allocated._1h_to_1d - 23.0 / HOURS_PER_DAY).abs() < 1e-12);
        assert!(allocated._1d_to_1w.abs() < 1e-12);
        assert!((allocated.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn consumed_coindays_cover_every_traversed_cohort() {
        let mut volumes = AgeRange::default();
        let mut cdd = AgeRange::default();
        volumes._1d_to_1w = 2.0;
        cdd._1d_to_1w = 20.0;

        let allocated = allocate_consumed_coindays(volumes, cdd, &age_bounds_days());

        assert!((allocated.under_1h - 2.0 / HOURS_PER_DAY).abs() < 1e-12);
        assert!((allocated._1h_to_1d - 46.0 / HOURS_PER_DAY).abs() < 1e-12);
        assert!((allocated._1d_to_1w - 18.0).abs() < 1e-12);
        assert!((allocated.iter().sum::<f64>() - 20.0).abs() < 1e-12);
    }

    #[test]
    fn allocated_coindays_conserve_mixed_cohort_destruction() {
        let bounds = age_bounds_days();
        let mut volumes = AgeRange::default();
        let mut cdd = AgeRange::default();

        for id in [
            AgeRangeId::Under1H,
            AgeRangeId::From1HTo1D,
            AgeRangeId::From1DTo1W,
            AgeRangeId::From9MTo1Y,
            AgeRangeId::From10YTo12Y,
            AgeRangeId::From12YTo15Y,
            AgeRangeId::Over15Y,
        ] {
            let (lower, width) = *id.select(&bounds);
            let volume = id.index() as f64 + 1.0;
            let age = lower + if width > 0.0 { width / 2.0 } else { 30.0 };
            *id.select_mut(&mut volumes) = volume;
            *id.select_mut(&mut cdd) = volume * age;
        }

        let expected = cdd.iter().sum::<f64>();
        let allocated = allocate_consumed_coindays(volumes, cdd, &bounds);

        assert!((allocated.iter().sum::<f64>() - expected).abs() < 1e-9);
    }

    #[test]
    fn bounds_and_allocation_cover_every_canonical_age_range() {
        let bounds = age_bounds_days();
        let mut volumes = AgeRange::default();
        let mut cdd = AgeRange::default();

        volumes.over_15y = 1.0;
        cdd.over_15y = bounds.over_15y.0 + 30.0;

        let expected = cdd.iter().sum::<f64>();
        let allocated = allocate_consumed_coindays(volumes, cdd, &bounds);

        assert_eq!(bounds.iter().count(), AGE_RANGE_COUNT);
        assert!(allocated.over_15y > 0.0);
        assert!((allocated.iter().sum::<f64>() - expected).abs() < 1e-9);
    }
}
