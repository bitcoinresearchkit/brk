use super::*;

fn check(raw: &[u32]) {
    let maximum: Vec<_> = raw
        .iter()
        .scan(0, |max, &value| {
            *max = (*max).max(value);
            Some(*max)
        })
        .collect();
    for target in (0..=*maximum.last().unwrap() + 1).chain([u32::MAX]) {
        let expected = raw
            .iter()
            .enumerate()
            .filter(|(_, value)| **value <= target)
            .max_by_key(|(h, value)| (**value, std::cmp::Reverse(*h)))
            .map(|(h, value)| (h, Timestamp::from(*value)));
        let actual = select_timestamp(
            raw.len(),
            target.into(),
            |h| Ok(maximum[h].into()),
            |h| Ok(raw[h].into()),
        );
        match expected {
            Some(value) => assert_eq!(actual.unwrap(), value, "target={target}"),
            None => assert!(matches!(actual, Err(Error::NotFound(_)))),
        }
    }
}

#[test]
fn selection_matches_predecessor_on_valid_clock_regressions() {
    check(&[1, 99, 101, 100]);
    check(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 99, 101, 98]);
    check(&[1, 2, 3, 4, 8, 6, 5, 7, 8, 9, 10, 11, 12]);
    let mut raw = vec![1u32];
    let mut seed = 17u32;
    for h in 1usize..1000 {
        let mut window = raw[h.saturating_sub(11)..h].to_vec();
        window.sort_unstable();
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        raw.push(window[window.len() / 2] + 1 + seed % 100);
    }
    check(&raw);
}

#[test]
fn future_lookup_is_logarithmic_and_empty_is_unavailable() {
    let mut reads = 0;
    let result = select_timestamp(
        1_000_000,
        Timestamp::from(u32::MAX),
        |h| {
            reads += 1;
            Ok(Timestamp::from((h / 2) as u32))
        },
        |_| panic!("future query read raw history"),
    );
    assert_eq!(result.unwrap(), (999_998, Timestamp::from(499_999u32)));
    assert!(reads <= 42, "{reads}");
    assert!(matches!(
        select_timestamp(0, Timestamp::ZERO, |_| unreachable!(), |_| unreachable!()),
        Err(Error::StateUpdating)
    ));
}
