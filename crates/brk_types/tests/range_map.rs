use brk_types::RangeMap;

fn check(map: &mut RangeMap<usize, usize>, starts: &[usize]) {
    for index in (0..40).chain((0..40).rev()) {
        let floor = starts.iter().rposition(|&first| first <= index);
        assert_eq!(map.get(index), floor);
        assert_eq!(map.get_shared(index), floor);
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
