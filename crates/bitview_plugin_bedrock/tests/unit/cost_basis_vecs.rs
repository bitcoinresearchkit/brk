use std::collections::BTreeSet;

use bitview_traversable::Traversable;
use bitview_vecs::DailyMappings;
use brk_types::{Cents, Day1, Height, Version};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, Budgeted, Database, EagerVec, ImportableVec, LazyVec, PcoVec, PcoVecValue,
    ReadableCloneableVec, ReadableVec, VecIndex, WritableVec,
};

use crate::{CostBasisVecs, DayUrpds, WeightedPair};

fn mapping<I: VecIndex, T: PcoVecValue>(db: &Database, name: &str) -> LazyVec<I, Day1, I, T> {
    let source: EagerVec<PcoVec<I, T>> = EagerVec::forced_import(db, name, Version::ONE).unwrap();
    LazyVec::init(
        name,
        Version::ONE,
        source.read_only_boxed_clone(),
        |_, _| Day1::from(0_usize),
    )
}

#[test]
fn density_series_persist_reopen_rewind_and_expose_normal_percent_units() {
    Budgeted::init_global(64 * 1024 * 1024).unwrap();
    let root = tempdir().unwrap();
    let db = Database::open(root.path()).unwrap();
    let mut heights: EagerVec<PcoVec<Height, Day1>> =
        EagerVec::forced_import(&db, "height_day1", Version::ONE).unwrap();
    heights.push(Day1::from(0_usize));
    heights.push(Day1::from(1_usize));
    heights.write().unwrap();
    let mappings = DailyMappings {
        height: heights.read_only_boxed_clone(),
        minute10: mapping(&db, "minute10"),
        minute30: mapping(&db, "minute30"),
        hour1: mapping(&db, "hour1"),
        hour4: mapping(&db, "hour4"),
        hour12: mapping(&db, "hour12"),
        day3: mapping(&db, "day3"),
        week1: mapping(&db, "week1"),
        month1: mapping(&db, "month1"),
        month3: mapping(&db, "month3"),
        month6: mapping(&db, "month6"),
        year1: mapping(&db, "year1"),
        year10: mapping(&db, "year10"),
        halving: mapping(&db, "halving"),
        epoch: mapping(&db, "epoch"),
    };
    let import = || CostBasisVecs::forced_import(&db, Version::ONE, &mappings).unwrap();
    let mut vecs = import();
    let data = DayUrpds::repeated([(95, 20), (100, 30), (105, 10), (110, 20), (200, 20)])
        .cost_basis(Cents::new(100));
    vecs.push(&data);
    vecs.push(&WeightedPair::default());
    assert_eq!(vecs.minimum_len(), 2);
    let names = vecs
        .supply_density
        .iter_any_visible()
        .chain(vecs.supply_density_10pct.iter_any_visible())
        .map(|vec| vec.name())
        .collect::<BTreeSet<_>>();
    for mode in ["cointime", "coinflow"] {
        for side in ["", "_in_profit", "_in_loss"] {
            for unit in ["", "_ratio", "_ppm"] {
                for band in ["", "_10pct"] {
                    assert!(names.contains(
                        format!("bedrock_{mode}_supply_density{band}{side}{unit}").as_str()
                    ));
                }
            }
        }
    }
    for vec in vecs.stored_vecs_mut() {
        vec.write().unwrap();
    }
    drop(vecs);
    let mut vecs = import();
    for mode in vecs.supply_density_10pct.iter() {
        assert_eq!(
            mode.series
                .total
                .ppm
                .day1
                .collect_one(Day1::from(0_usize))
                .unwrap()
                .inner(),
            800_000
        );
        assert!(
            mode.series
                .total
                .ppm
                .day1
                .collect_one(Day1::from(1_usize))
                .unwrap()
                .is_nan()
        );
    }
    for mode in vecs.supply_density.iter() {
        let total = &mode.series.total;
        assert_eq!(
            total
                .ppm
                .day1
                .collect_one(Day1::from(0_usize))
                .unwrap()
                .inner(),
            600_000
        );
        assert!(
            (f32::from(total.percent.day1.collect_one(Day1::from(0_usize)).unwrap()) - 60.0).abs()
                < 1e-5
        );
        assert!(
            (f32::from(total.ratio.day1.collect_one(Day1::from(0_usize)).unwrap()) - 0.6).abs()
                < 1e-6
        );
        assert!(
            total
                .ppm
                .day1
                .collect_one(Day1::from(1_usize))
                .unwrap()
                .is_nan()
        );
        assert!(
            total
                .ppm
                .views
                .height
                .collect_one(Height::from(1_usize))
                .unwrap()
                .unwrap()
                .is_nan()
        );
    }
    for vec in vecs.stored_vecs_mut() {
        vec.any_truncate_if_needed_at(1).unwrap();
    }
    let changed = DayUrpds::repeated([(105, 10)]).cost_basis(Cents::new(100));
    vecs.push(&changed);
    assert_eq!(vecs.minimum_len(), 2);
    for vec in vecs.stored_vecs_mut() {
        vec.write().unwrap();
    }
    for mode in vecs
        .supply_density
        .iter()
        .chain(vecs.supply_density_10pct.iter())
    {
        assert_eq!(
            mode.series
                .in_loss
                .ppm
                .day1
                .collect_one(Day1::from(1_usize))
                .unwrap()
                .inner(),
            1_000_000
        );
        assert_eq!(
            mode.series
                .in_profit
                .ppm
                .day1
                .collect_one(Day1::from(1_usize))
                .unwrap()
                .inner(),
            0
        );
    }
}
