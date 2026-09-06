use super::*;

#[test]
fn completed_closes_are_causal_and_share_missing_data_rules() {
    let mapping = [0usize, 2, 2, 3, 4].map(Height::from);
    let values = [
        Cents::new(100),
        Cents::new(200),
        Cents::ZERO,
        Cents::new(400),
    ];
    let query = |target| {
        historical_prices(&mapping, values.len(), target, |height| {
            Ok(values[usize::from(height)])
        })
        .unwrap()
    };
    let all = query(None);
    assert_eq!(all.prices.len(), 3);
    assert_eq!(
        all.prices.iter().map(|p| *p.time).collect::<Vec<_>>(),
        [1, 3, 4].map(|i| INDEX_EPOCH + i * HOUR4_INTERVAL)
    );
    assert_eq!(all.prices[1].usd, Dollars::from(Cents::ZERO));
    for timestamp in [0, INDEX_EPOCH, INDEX_EPOCH + HOUR4_INTERVAL - 1] {
        assert!(query(Some(Timestamp::new(timestamp))).prices.is_empty());
    }
    for timestamp in [
        INDEX_EPOCH + HOUR4_INTERVAL,
        INDEX_EPOCH + 2 * HOUR4_INTERVAL,
        INDEX_EPOCH + 3 * HOUR4_INTERVAL,
        u32::MAX,
    ] {
        let point = query(Some(Timestamp::new(timestamp)));
        let expected = all
            .prices
            .iter()
            .rev()
            .find(|p| *p.time <= timestamp)
            .unwrap();
        assert_eq!(
            serde_json::to_value(&point.prices[0]).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
    }
    assert!(
        historical_prices(&[Height::ZERO], 1, None, |_| panic!(
            "partial tail must not read prices"
        ))
        .unwrap()
        .prices
        .is_empty()
    );
}

#[test]
fn invalid_mapping_or_source_price_is_not_silently_empty_or_zero() {
    for mapping in [vec![2usize, 1], vec![0, 3]] {
        let mapping: Vec<_> = mapping.into_iter().map(Height::from).collect();
        assert!(
            historical_prices(&mapping, 2, None, |_| panic!(
                "invalid mapping must fail before reads"
            ))
            .is_err()
        );
    }
    for result in [Ok(Cents::NAN), Err(Error::Internal("source read failed"))] {
        let mut result = Some(result);
        assert!(
            historical_prices(&[Height::ZERO, Height::from(1usize)], 1, None, |_| result
                .take()
                .unwrap())
            .is_err()
        );
    }
}
