use std::{collections::BTreeMap, iter};

use brk_error::Error;
use brk_fetcher::compute_ohlc_from_range;
use brk_types::{Cents, Close, High, Low, OHLCCents, Open, Timestamp};
use serde_json::to_value;

fn candle(open: u64, high: u64, low: u64, close: u64) -> OHLCCents {
    OHLCCents::from((
        Open::new(Cents::from(open)),
        High::new(Cents::from(high)),
        Low::new(Cents::from(low)),
        Close::new(Cents::from(close)),
    ))
}

// Keep the original implementation as an independent boundary-behavior oracle.
fn reference(
    tree: &BTreeMap<Timestamp, OHLCCents>,
    end: Timestamp,
    previous: Option<Timestamp>,
) -> Result<OHLCCents, Error> {
    let previous_ohlc = previous.map_or(Some(OHLCCents::default()), |t| tree.get(&t).cloned());
    let last_ohlc = tree.get(&end);
    if previous_ohlc.is_none() || last_ohlc.is_none() {
        return Err(Error::NotFound("Couldn't find timestamp in fixture".into()));
    }
    let mut result = OHLCCents::from(previous_ohlc.unwrap().close);
    let start = previous.unwrap_or(Timestamp::new(0));
    if start < end {
        for (_, ohlc) in tree.range(start..=end).skip(1) {
            if ohlc.high > result.high {
                result.high = ohlc.high;
            }
            if ohlc.low < result.low {
                result.low = ohlc.low;
            }
            result.close = ohlc.close;
        }
    }
    Ok(result)
}

#[test]
fn all_sparse_boundaries_match_original_candles_and_errors() {
    // Includes missing endpoints, no previous candle, zero, gaps, equal and reversed ranges.
    for mask in 0..32 {
        let tree = (0..5)
            .filter(|i| mask & (1 << i) != 0)
            .map(|i| {
                (
                    Timestamp::new(i * 2),
                    candle(
                        10 + u64::from(i),
                        30 + u64::from(i),
                        2 + u64::from(i),
                        20 + u64::from(i),
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for previous in iter::once(None).chain((0..10).map(|i| Some(Timestamp::new(i)))) {
            for end in 0..10 {
                let end = Timestamp::new(end);
                let actual = compute_ohlc_from_range(&tree, end, previous, "fixture");
                let expected = reference(&tree, end, previous);
                match (actual, expected) {
                    (Ok(a), Ok(b)) => assert_eq!(to_value(a).unwrap(), to_value(b).unwrap()),
                    (Err(Error::NotFound(a)), Err(Error::NotFound(b))) => assert_eq!(a, b),
                    other => panic!("different results: {other:?}"),
                }
            }
        }
    }
}
