use super::*;

#[test]
fn exact_nearest_rank_and_p999_threshold() {
    let g = Group {
        durations: (1..=100).collect(),
        ..Group::default()
    };
    assert_eq!(
        (g.percentile(50), g.percentile(95), g.percentile(99)),
        (50, 95, 99)
    );
    for (n, expected) in [
        (999, None),
        (1_000, Some(999)),
        (9_999, Some(9_990)),
        (10_000, Some(9_990)),
    ] {
        let g = Group {
            durations: (1..=n).collect(),
            ..Group::default()
        };
        assert_eq!(g.p999(), expected);
    }
    let g = Group {
        durations: vec![17],
        ..Group::default()
    };
    assert_eq!(g.percentile(99), 17);
}
