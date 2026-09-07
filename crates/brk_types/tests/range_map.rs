use brk_types::RangeMap;

fn check(map: &mut RangeMap<usize, usize>, starts: &[usize]) {
    for index in (0..40).chain((0..40).rev()) {
        let floor = starts.iter().rposition(|&first| first <= index);
        let ceil = starts.iter().position(|&first| first >= index);
        assert_eq!(map.get(index), floor);
        assert_eq!(map.get_shared(index), floor);
        assert_eq!(map.ceil(index), ceil);
    }
    let mut cursor = map.cursor();
    for index in (0..40)
        .chain((0..40).rev())
        .chain([usize::MAX, 0, 10, 10, 39])
    {
        assert_eq!(
            cursor.get(index),
            starts.iter().rposition(|&first| first <= index)
        );
    }
}

#[test]
fn range_map_constructors_clones_and_rewinds_agree() {
    let mut empty = RangeMap::default();
    check(&mut empty, &[]);
    let mut starts = vec![3, 3, 5, 10, 10, 20];
    let mut map = RangeMap::from(starts.clone());
    check(&mut map, &starts);
    let mut cloned = map.clone();
    check(&mut cloned, &starts);
    map.truncate(3);
    starts.truncate(3);
    check(&mut map, &starts);
    for first in [5, 15, 25] {
        map.push(first);
        starts.push(first);
        check(&mut map, &starts);
    }
    check(&mut cloned, &[3, 3, 5, 10, 10, 20]);
    map.truncate(0);
    check(&mut map, &[]);
}

#[test]
#[ignore = "Paired request-local interval hint benchmark"]
fn benchmark_range_map_cursor() {
    use std::{hint::black_box, time::Instant};
    const N: usize = 1_000_000;
    let map = RangeMap::<usize, usize>::from((0..N).map(|i| i * 2000).collect::<Vec<_>>());
    for (name, indices) in [
        ("one", vec![1234567]),
        (
            "same_block",
            (0..4096).map(|i| 1_000_000 + i % 2000).collect(),
        ),
        ("sequential", (0..4096).map(|i| 1_000_000 + i).collect()),
        (
            "random",
            (0..4096).map(|i| (i * 7919 % N) * 2000 + 1).collect(),
        ),
    ] {
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..22 {
            for variant in [round % 2, 1 - round % 2] {
                let start = Instant::now();
                let mut cursor = map.cursor();
                let values: Vec<_> = black_box(&indices)
                    .iter()
                    .map(|&i| {
                        if variant == 0 {
                            map.get_shared(i)
                        } else {
                            cursor.get(i)
                        }
                    })
                    .collect();
                let elapsed = start.elapsed();
                assert_eq!(
                    values,
                    indices.iter().map(|i| Some(i / 2000)).collect::<Vec<_>>()
                );
                black_box(values);
                if round > 1 {
                    samples[variant].push(elapsed);
                }
            }
        }
        for sample in &mut samples {
            sample.sort();
        }
        eprintln!(
            "{name}: shared {:?}, cursor {:?}",
            samples[0][10], samples[1][10]
        );
    }
}
