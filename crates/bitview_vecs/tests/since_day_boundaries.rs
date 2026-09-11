use bitview_vecs::{LazySinceDayVec, RangeMapVec};
use brk_types::{Day1, Height, Version};
use rangeindex::SharedRangeMap;
use vecdb::{ReadBounds, ReadableVec};

#[test]
fn direct_day_boundaries_match_all_read_paths_and_published_prefixes() {
    let values = [10u64, 30, 60, 100, 150, 210];
    let source = RangeMapVec::<Height, _>::new(
        "cumulative",
        Version::ONE,
        SharedRangeMap::new(values.to_vec()),
    );
    let starts = [0, 0, 2, 2, 5];
    let boundaries = RangeMapVec::<Day1, _>::new(
        "first_height",
        Version::ONE,
        SharedRangeMap::new(starts.map(Height::from).to_vec()),
    );
    for day in [0, 1, 2, 3, 4, 5, 100] {
        let view = LazySinceDayVec::new(
            "since_day",
            Version::ONE,
            &source,
            &boundaries,
            Day1::from(day),
            |current, before| current - before,
        );
        for limit in 0..=values.len() {
            let mut bounds = ReadBounds::new();
            bounds.set("height", limit);
            bounds.scope(|| {
                let start = starts.get(day).copied().unwrap_or(limit).min(limit);
                let before = start.checked_sub(1).map_or(0, |index| values[index]);
                let expected: Vec<_> = values[..limit]
                    .iter()
                    .enumerate()
                    .map(|(index, &value)| if index < start { 0 } else { value - before })
                    .collect();
                assert_eq!(view.collect_range_at(0, usize::MAX), expected);
                assert!(view.collect_range_at(limit, usize::MAX).is_empty());
                assert!(view.collect_range_at(3, 1).is_empty());
                for index in 0..=values.len() {
                    assert_eq!(view.collect_one_at(index), expected.get(index).copied());
                }
                let requested = [0, 1, 1, 3, 4, 5, 6, usize::MAX];
                assert_eq!(
                    view.read_sorted_at(&requested),
                    requested
                        .into_iter()
                        .filter_map(|index| expected.get(index).copied())
                        .collect::<Vec<_>>()
                );
                let mut chunks = Vec::new();
                view.for_each_chunk_at(0, usize::MAX, &mut |at, values| {
                    assert_eq!(at, chunks.len());
                    chunks.extend_from_slice(values);
                });
                assert_eq!(chunks, expected);
                assert_eq!(
                    view.fold_range_at(0, usize::MAX, 0, |a, b| a + b),
                    expected.iter().sum::<u64>()
                );
                let mut visited = Vec::new();
                let result = view.try_fold_range_at(0, usize::MAX, (), |(), value| {
                    visited.push(value);
                    if visited.len() == 2 {
                        Err("stop")
                    } else {
                        Ok(())
                    }
                });
                assert_eq!(visited, expected[..expected.len().min(2)]);
                assert_eq!(result, if limit >= 2 { Err("stop") } else { Ok(()) });
            });
        }
    }
}

#[test]
fn clones_follow_boundary_rewrites_truncations_and_appends() {
    let values = SharedRangeMap::new(vec![10u64, 30, 60, 100, 150, 210]);
    let source = RangeMapVec::<Height, _>::new("cumulative", Version::ONE, values.clone());
    let starts = SharedRangeMap::new([0usize, 0, 2, 2, 5].map(Height::from).to_vec());
    let boundaries = RangeMapVec::<Day1, _>::new("first_height", Version::ONE, starts.clone());
    let view = LazySinceDayVec::new(
        "since_day",
        Version::ONE,
        &source,
        &boundaries,
        Day1::from(2usize),
        |current, before| current - before,
    )
    .clone();
    assert_eq!(view.collect(), [0, 0, 30, 70, 120, 180]);
    values.update_at(4, [170, 230]);
    starts.update_at(2, [4usize, 4, 5].map(Height::from));
    assert_eq!(view.collect(), [0, 0, 0, 0, 70, 130]);
    values.update_at(2, []);
    starts.update_at(2, []);
    assert_eq!(view.collect(), [0, 0]);
    values.update_at(2, [40, 50]);
    starts.update_at(2, [2usize, 3].map(Height::from));
    assert_eq!(view.collect(), [0, 0, 10, 20]);
    values.update_at(0, []);
    starts.update_at(0, []);
    assert!(view.collect().is_empty());
}
