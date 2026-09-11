use bitview_vecs::{RangeMapLookupVec, RangeMapVec};
use brk_types::{Day1, Height, Version};
use rangeindex::SharedRangeMap;
use vecdb::{AnyVec, LazyVec, ReadBounds, ReadableCloneableVec, ReadableVec};

fn lookup(
    boundaries: &SharedRangeMap<Height, Day1>,
    domain: &SharedRangeMap<u64, Height>,
) -> RangeMapLookupVec<Height, Day1> {
    let values = RangeMapVec::<Height, u64>::new("metadata", Version::ONE, domain.clone());
    let source = LazyVec::init(
        "day1",
        Version::TWO,
        values.read_only_boxed_clone(),
        |_, _| -> Day1 { panic!("range-map lookup must not read the original mapping") },
    );
    RangeMapLookupVec::new(boundaries, &source)
}

#[test]
fn reverse_reads_skip_empty_days_and_share_append_reorg_and_truncation() {
    let boundaries = SharedRangeMap::new([0, 0, 0, 2, 2, 4].map(Height::new).to_vec());
    let domain = SharedRangeMap::new(vec![0; 5]);
    let reader = lookup(&boundaries, &domain);
    let clone = reader.clone();
    assert_eq!(reader.collect(), [2, 2, 4, 4, 5].map(Day1::from));
    assert_eq!(reader.name(), "day1");
    assert_eq!(reader.version(), Version::ONE + Version::TWO);
    assert_eq!(reader.len(), 5);
    assert_eq!(reader.collect_one_at(5), None);
    assert_eq!(reader.collect_one_at(usize::MAX), None);
    let mut values = vec![Day1::from(99)];
    reader.read_sorted_into_at(&[0, 0, 1, 2, 4, 5, usize::MAX], &mut values);
    assert_eq!(values, [99, 2, 2, 2, 4, 5].map(Day1::from));

    domain.update_at(5, [0, 0]);
    boundaries.update_at(6, [5, 5].map(Height::new));
    assert_eq!(clone.collect(), [2, 2, 4, 4, 5, 7, 7].map(Day1::from));

    boundaries.update_at(3, [2, 5].map(Height::new));
    assert_eq!(clone.collect(), [2, 2, 3, 3, 3, 4, 4].map(Day1::from));
    domain.update_at(3, []);
    boundaries.update_at(4, []);
    assert_eq!(clone.collect(), [2, 2, 3].map(Day1::from));
    assert_eq!(clone.collect_one_at(3), None);

    domain.update_at(0, []);
    boundaries.update_at(0, []);
    assert!(clone.collect().is_empty());
    assert_eq!(clone.collect_one_at(0), None);
}

#[test]
fn all_reads_respect_published_height_bounds_and_fallible_folds_stop_early() {
    let boundaries = SharedRangeMap::new([0, 0, 0, 2, 2, 4].map(Height::new).to_vec());
    let domain = SharedRangeMap::new(vec![0; 5]);
    let reader = lookup(&boundaries, &domain);
    let mut bounds = ReadBounds::new();
    bounds.set("height", 3);
    bounds.scope(|| {
        for (from, to, expected) in [
            (0, 99, vec![2, 2, 4]),
            (1, 2, vec![2]),
            (3, 99, vec![]),
            (4, 1, vec![]),
        ] {
            let expected: Vec<_> = expected.into_iter().map(Day1::from).collect();
            assert_eq!(reader.collect_range_at(from, to), expected);
            let mut visited = Vec::new();
            reader.for_each_chunk_at(from, to, &mut |at, values| {
                assert_eq!(at, from + visited.len());
                visited.extend_from_slice(values);
            });
            assert_eq!(visited, expected);
            assert_eq!(
                reader.fold_range_at(from, to, 0, |sum, day| sum + usize::from(day)),
                expected.iter().copied().map(usize::from).sum::<usize>(),
            );
        }
        assert_eq!(reader.collect_one_at(3), None);
        assert_eq!(reader.read_sorted_at(&[0, 2, 3, 4]), [2, 4].map(Day1::from));
        let mut visited = Vec::new();
        let result = reader.try_fold_range_at(0, 99, (), |(), day| {
            visited.push(day);
            if visited.len() == 2 {
                Err("stop")
            } else {
                Ok(())
            }
        });
        assert_eq!(result, Err("stop"));
        assert_eq!(visited, [2, 2].map(Day1::from));
    });
    assert_eq!(reader.collect_one_at(4), Some(Day1::from(5)));
}
